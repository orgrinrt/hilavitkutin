//! Host-level extension error shape.
//!
//! Disjoint from `hilavitkutin_linking::LinkError`. The linking layer
//! surfaces low-level load / resolve / close failures; this crate
//! surfaces contract-level failures (descriptor missing, abi skew,
//! init handler non-zero, required host capability unavailable, and
//! so on).

use arvo::USize;
use hilavitkutin_linking::LinkError;

use crate::descriptor::CapabilityId;

/// Host-side extension error.
#[non_exhaustive]
#[repr(C)]
pub enum ExtensionError {
    /// The underlying linking layer failed to load or resolve.
    Link(LinkError),
    /// The extension did not export `__hilavitkutin_extension_descriptor`.
    DescriptorMissing,
    /// The descriptor symbol resolved but pointed to invalid or
    /// unparseable payload.
    DescriptorInvalid,
    /// Abi version skew the host cannot accept.
    ExtensionVersionUnsupported,
    /// Extension's `init_fn` returned a non-zero platform code.
    InitFailed { platform_code: USize },
    /// Required host capability was not registered on the host.
    NotSupported { missing: CapabilityId },
    /// Argument passed by the host was malformed.
    InvalidArg,
    /// Host-internal error (resource exhaustion, unexpected state).
    Internal,
}
