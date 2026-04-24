//! Host-level extension error shape.
//!
//! Disjoint from `hilavitkutin_linking::LinkError`. The linking layer
//! surfaces load / resolve / close failures; this crate surfaces
//! contract-level failures (descriptor missing or malformed, abi skew,
//! init handler non-`Ok`, required host capability unavailable, and so
//! on).

use hilavitkutin_linking::LinkError;

use crate::descriptor::{AbiVersion, CapabilityId, ExtensionAbiStatus};

/// Host-side extension error.
#[non_exhaustive]
#[repr(C)]
pub enum ExtensionError {
    /// The underlying linking layer failed to load or resolve.
    LinkFailed { cause: LinkError },
    /// The extension did not export `__hilavitkutin_extension_descriptor`.
    DescriptorMissing,
    /// The descriptor symbol resolved but pointed to invalid payload.
    DescriptorInvalid,
    /// Descriptor `abi_version` does not match host.
    AbiVersionMismatch { expected: AbiVersion, got: AbiVersion },
    /// Host declines to accept the extension's semantic version (reserved
    /// for consumer-layer policies; the base host does not apply any).
    ExtensionVersionUnsupported,
    /// A capability the extension requires is not in the host's advertised set.
    RequiredHostCapabilityMissing { cap: CapabilityId },
    /// Extension's `init_fn` returned a non-`Ok` status.
    InitFailed { status: ExtensionAbiStatus },
    /// Extension's `shutdown_fn` returned a non-`Ok` status.
    ShutdownFailed { status: ExtensionAbiStatus },
}
