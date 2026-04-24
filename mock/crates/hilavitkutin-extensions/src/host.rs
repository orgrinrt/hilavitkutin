//! Host state: extension loader + failure policy surface.

use core::ffi::c_void;
use hilavitkutin_linking::Library;
use notko::Outcome;

use crate::descriptor::{
    CapabilityId, DESCRIPTOR_SYMBOL, ExtensionDescriptor, HOST_ABI_VERSION,
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PolicyVerdict {
    /// Propagate the underlying error to the caller.
    Abort,
    /// Swallow the failure and report success with no loaded handle.
    Continue,
}

/// Host-provided policy function signature.
///
/// Invoked on optional-load failures. Host inspects the error and
/// requirement, then returns a verdict. The default policy aborts
/// on required failures and continues on optional failures.
pub type FailurePolicyFn =
    fn(requirement: ExtensionRequirement, error: &ExtensionError) -> PolicyVerdict;

/// Default failure policy.
pub fn default_policy(
    requirement: ExtensionRequirement,
    _error: &ExtensionError,
) -> PolicyVerdict {
    match requirement {
        ExtensionRequirement::Required => PolicyVerdict::Abort,
        ExtensionRequirement::Optional => PolicyVerdict::Continue,
    }
}

/// Host state that drives extension load and lifecycle.
///
/// Carries a failure policy and an opaque host context pointer that
/// the host threads through each extension's init and shutdown
/// handlers.
pub struct ExtensionHost {
    policy: FailurePolicyFn,
    host_ctx: *mut c_void,
}

impl ExtensionHost {
    /// Construct a host with the default policy and a null opaque
    /// context pointer.
    pub fn new() -> Self {
        Self {
            policy: default_policy,
            host_ctx: core::ptr::null_mut(),
        }
    }

    /// Override the failure policy.
    pub fn with_policy(mut self, policy: FailurePolicyFn) -> Self {
        self.policy = policy;
        self
    }

    /// Load an extension from a null-terminated path.
    pub fn load(
        &self,
        path: &[u8],
        requirement: ExtensionRequirement,
    ) -> Outcome<Option<Extension>, ExtensionError> {
        let library = match Library::load(path) {
            Outcome::Ok(lib) => lib,
            Outcome::Err(e) => {
                let err = ExtensionError::Link(e);
                return policy_translate(self.policy, requirement, err);
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
        // SAFETY: the descriptor is a static declared by the
        // extension and is valid for the loaded library lifetime.
        // Extending to 'static is load-bearing: Extension keeps the
        // Library alive alongside this borrow.
        let descriptor: &'static ExtensionDescriptor =
            unsafe { &*descriptor_ptr };

        if descriptor.abi_version > HOST_ABI_VERSION {
            return policy_translate(
                self.policy,
                requirement,
                ExtensionError::ExtensionVersionUnsupported,
            );
        }

        if let Some(init) = descriptor.init_fn {
            // SAFETY: init is declared by the extension. host_ctx is
            // the host-owned opaque pointer the contract requires
            // the extension to treat as opaque.
            let rc = unsafe { init(self.host_ctx) };
            if rc != 0 {
                return policy_translate(
                    self.policy,
                    requirement,
                    ExtensionError::InitFailed {
                        platform_code: arvo::USize(rc as usize),
                    },
                );
            }
        }

        Outcome::Ok(Some(Extension::from_parts(library, descriptor, self.host_ctx)))
    }

    /// Look up a host-registered capability id.
    ///
    /// v1 returns `false` unconditionally; the capability registry is
    /// populated in a follow-up round. The method exists so callers
    /// can write the call-site now. See `BACKLOG.md`.
    pub fn has_capability(&self, _id: CapabilityId) -> bool {
        false
    }
}

impl Default for ExtensionHost {
    fn default() -> Self {
        Self::new()
    }
}

fn policy_translate(
    policy: FailurePolicyFn,
    requirement: ExtensionRequirement,
    error: ExtensionError,
) -> Outcome<Option<Extension>, ExtensionError> {
    match policy(requirement, &error) {
        PolicyVerdict::Abort => Outcome::Err(error),
        PolicyVerdict::Continue => Outcome::Ok(None),
    }
}
