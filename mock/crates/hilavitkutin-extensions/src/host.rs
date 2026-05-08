//! Host state: extension loader + failure policy surface.

use core::ffi::c_void;
use hilavitkutin_linking::Library;
use notko::{Maybe, Outcome};

use crate::descriptor::{
    CapabilityId, DESCRIPTOR_SYMBOL, EXTENSION_DESCRIPTOR_TAG,
    ExtensionAbiStatus, ExtensionDescriptor, HOST_ABI_VERSION,
    MAX_DESCRIPTOR_LIST_LEN,
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

/// Observer signature for extension shutdown completion.
///
/// Receives the extension's declared name and the status returned
/// from the extension's `shutdown_fn` (or `ExtensionAbiStatus::Ok`
/// if the extension declared no shutdown). Fires once per extension
/// from either `Extension::close()` or the `Drop` path.
pub type ShutdownObserverFn = fn(name: &[u8], status: ExtensionAbiStatus);

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
    observer: Maybe<ShutdownObserverFn>,
}

impl ExtensionHost {
    /// Construct a host advertising `host_capabilities`.
    pub fn new(host_capabilities: &'static [CapabilityId]) -> Self {
        Self {
            host_capabilities,
            policy: default_policy,
            observer: Maybe::Isnt,
        }
    }

    /// Override the failure policy.
    pub fn with_policy(mut self, policy: FailurePolicyFn) -> Self {
        self.policy = policy;
        self
    }

    /// Install a shutdown observer that fires from `Extension::close`
    /// and the `Drop` path.
    pub fn with_shutdown_observer(
        mut self,
        observer: ShutdownObserverFn,
    ) -> Self {
        self.observer = Maybe::Is(observer);
        self
    }

    /// Return true if `id` appears in the host's advertised set.
    pub fn has_capability(&self, id: CapabilityId) -> bool {
        let mut i = 0;
        while i < self.host_capabilities.len() {
            if self.host_capabilities[i] == id {
                return true;
            }
            i += 1;
        }
        false
    }

    /// Load one extension from disk.
    pub fn load(
        &self,
        path: &[u8],
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
        let sym = match library.resolve::<DescriptorFn>(DESCRIPTOR_SYMBOL.to_bytes_with_nul()) {
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

        if let Outcome::Err(err) = validate_descriptor(descriptor) {
            return policy_translate(self.policy, requirement, err);
        }

        // Verify every required-host-cap is in our advertised set.
        let req_len = descriptor.required_host_caps_len as usize;
        if !descriptor.required_host_caps_ptr.is_null() && req_len > 0 {
            let mut i = 0;
            while i < req_len {
                // SAFETY: required_host_caps_ptr + _len valid static slice.
                let required =
                    unsafe { *descriptor.required_host_caps_ptr.add(i) };
                if !self.has_capability(required) {
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

        if let Some(init) = descriptor.init_fn {
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

        Outcome::Ok(Maybe::Is(Extension::from_parts(
            library,
            descriptor,
            host_ctx,
            self.observer,
        )))
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

/// Validate descriptor structural invariants in the order required for
/// safe field access: tag, descriptor_size, abi_version, length
/// bounds. The required-host-capability check happens at the call
/// site after this returns, because it depends on the host's
/// advertised capability set.
///
/// Returns `Outcome::Ok(())` on success. The first failed check
/// surfaces; subsequent fields are not read. Public so consumers can
/// validate a descriptor pointer they obtained outside the standard
/// load path (e.g. from a probe-only inspection or a custom loader),
/// and so the integration test suite can exercise the contract
/// directly.
pub fn validate_descriptor(
    descriptor: &ExtensionDescriptor,
) -> Outcome<(), ExtensionError> {
    if descriptor.tag != EXTENSION_DESCRIPTOR_TAG {
        return Outcome::Err(ExtensionError::DescriptorTagMismatch {
            found: descriptor.tag,
        });
    }

    let host_size = core::mem::size_of::<ExtensionDescriptor>() as u32;
    if descriptor.descriptor_size < host_size {
        return Outcome::Err(ExtensionError::DescriptorSizeTooSmall {
            advertised: descriptor.descriptor_size,
            minimum: host_size,
        });
    }

    if descriptor.abi_version != HOST_ABI_VERSION {
        return Outcome::Err(ExtensionError::AbiVersionMismatch {
            expected: HOST_ABI_VERSION,
            got: descriptor.abi_version,
        });
    }

    if descriptor.name_len > MAX_DESCRIPTOR_LIST_LEN {
        return Outcome::Err(ExtensionError::DescriptorBoundsExceeded {
            field: "name",
            len: descriptor.name_len,
        });
    }

    if descriptor.capabilities_len > MAX_DESCRIPTOR_LIST_LEN {
        return Outcome::Err(ExtensionError::DescriptorBoundsExceeded {
            field: "capabilities",
            len: descriptor.capabilities_len,
        });
    }

    if descriptor.required_host_caps_len > MAX_DESCRIPTOR_LIST_LEN {
        return Outcome::Err(ExtensionError::DescriptorBoundsExceeded {
            field: "required_host_caps",
            len: descriptor.required_host_caps_len,
        });
    }

    Outcome::Ok(())
}
