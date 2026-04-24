//! Host state: extension loader + failure policy surface.

use arvo::Bool;
use core::ffi::c_void;
use hilavitkutin_linking::Library;
use notko::{Maybe, Outcome};

use crate::descriptor::{
    CapabilityId, DESCRIPTOR_SYMBOL, ExtensionAbiStatus, ExtensionDescriptor,
    HOST_ABI_VERSION,
};
use crate::error::ExtensionError;
use crate::extension::Extension;

/// Required-versus-optional load-time intent.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ExtensionRequirement {
    /// Failure surfaces as an error to the caller.
    Required,
    /// Failure is routed through the policy function and may be
    /// downgraded to a successful no-op.
    Optional,
}

/// Per-extension policy verdict returned by `FailurePolicyFn`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PolicyVerdict {
    /// Propagate the underlying error to the caller.
    Abort,
    /// Swallow the failure and report success with no loaded handle.
    Continue,
}

/// Host-provided policy function signature.
///
/// Invoked on any load failure. Host inspects the error and
/// requirement, then returns a verdict. The default policy aborts on
/// required failures and continues on optional failures.
pub type FailurePolicyFn =
    fn(error: &ExtensionError, requirement: ExtensionRequirement) -> PolicyVerdict;

/// Default failure policy.
pub fn default_policy(
    _error: &ExtensionError,
    requirement: ExtensionRequirement,
) -> PolicyVerdict {
    match requirement {
        ExtensionRequirement::Required => PolicyVerdict::Abort,
        ExtensionRequirement::Optional => PolicyVerdict::Continue,
    }
}

/// Host state that drives extension load and lifecycle.
///
/// Carries the host's advertised capability set and the failure
/// policy. The opaque `host_ctx` pointer is per-load, not per-host,
/// so two simultaneously-loaded extensions never share state through
/// this channel.
pub struct ExtensionHost {
    host_capabilities: &'static [CapabilityId],
    policy: FailurePolicyFn,
}

impl ExtensionHost {
    /// Construct a host advertising `host_capabilities`.
    pub fn new(host_capabilities: &'static [CapabilityId]) -> Self {
        Self { host_capabilities, policy: default_policy }
    }

    /// Override the failure policy.
    pub fn with_policy(mut self, policy: FailurePolicyFn) -> Self {
        self.policy = policy;
        self
    }

    /// Return `Bool::TRUE` if `id` appears in the host's advertised set.
    pub fn has_capability(&self, id: CapabilityId) -> Bool {
        let mut i = 0;
        while i < self.host_capabilities.len() {
            if self.host_capabilities[i] == id {
                return Bool(true);
            }
            i += 1;
        }
        Bool(false)
    }

    /// Load one extension from disk.
    pub fn load(
        &self,
        path: &[u8], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: null-terminated byte path for dlopen/LoadLibraryW; byte string is the loader input unit; tracked: #206
        requirement: ExtensionRequirement,
        host_ctx: *mut c_void,
    ) -> Outcome<Maybe<Extension>, ExtensionError> {
        let library = match Library::load(path) {
            Outcome::Ok(lib) => lib,
            Outcome::Err(cause) => {
                return policy_translate(
                    self.policy,
                    requirement,
                    ExtensionError::LinkFailed { cause },
                );
            }
        };

        type DescriptorFn = extern "C" fn() -> *const ExtensionDescriptor;
        let sym = match library.resolve::<DescriptorFn>(DESCRIPTOR_SYMBOL) {
            Outcome::Ok(sym) => sym,
            Outcome::Err(_) => {
                return policy_translate(
                    self.policy,
                    requirement,
                    ExtensionError::DescriptorMissing,
                );
            }
        };

        let descriptor_ptr = (sym.get())();
        if descriptor_ptr.is_null() {
            return policy_translate(
                self.policy,
                requirement,
                ExtensionError::DescriptorInvalid,
            );
        }

        // SAFETY: descriptor points at extension-static memory valid for
        // the library's loaded lifetime. Extension keeps Library alive
        // alongside this borrow so 'static tightening is load-bearing.
        let descriptor: &'static ExtensionDescriptor =
            unsafe { &*descriptor_ptr };

        if descriptor.abi_version != HOST_ABI_VERSION {
            return policy_translate(
                self.policy,
                requirement,
                ExtensionError::AbiVersionMismatch {
                    expected: HOST_ABI_VERSION,
                    got: descriptor.abi_version,
                },
            );
        }

        // Verify every required-host-cap is in our advertised set.
        let req_len = descriptor.required_host_caps_len;
        if !descriptor.required_host_caps_ptr.is_null() && req_len > 0 {
            let mut i = 0;
            while i < req_len {
                // SAFETY: required_host_caps_ptr + _len valid static slice.
                let required =
                    unsafe { *descriptor.required_host_caps_ptr.add(i) };
                if !self.has_capability(required).0 {
                    return policy_translate(
                        self.policy,
                        requirement,
                        ExtensionError::RequiredHostCapabilityMissing {
                            cap: required,
                        },
                    );
                }
                i += 1;
            }
        }

        if let Maybe::Is(init) = descriptor.init_fn.into_maybe() {
            // SAFETY: init is declared by the extension; host_ctx is
            // the per-load opaque pointer the contract requires.
            let status = unsafe { init(host_ctx) };
            if status != ExtensionAbiStatus::Ok {
                return policy_translate(
                    self.policy,
                    requirement,
                    ExtensionError::InitFailed { status },
                );
            }
        }

        Outcome::Ok(Maybe::Is(Extension::from_parts(library, descriptor, host_ctx)))
    }
}

fn policy_translate(
    policy: FailurePolicyFn,
    requirement: ExtensionRequirement,
    error: ExtensionError,
) -> Outcome<Maybe<Extension>, ExtensionError> {
    match policy(&error, requirement) {
        PolicyVerdict::Abort => Outcome::Err(error),
        PolicyVerdict::Continue => Outcome::Ok(Maybe::Isnt),
    }
}
