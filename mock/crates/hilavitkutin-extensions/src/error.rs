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
    /// Descriptor's `tag` field did not match `EXTENSION_DESCRIPTOR_TAG`.
    /// The layout is suspect; the host reads no further fields.
    DescriptorTagMismatch { found: u32 }, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
    /// Descriptor's `descriptor_size` is below the host's
    /// `core::mem::size_of::<ExtensionDescriptor>()`. The host cannot
    /// trust that the v1.1 fields it expects are actually present.
    DescriptorSizeTooSmall { advertised: u32, minimum: u32 }, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
    /// Descriptor `abi_version` does not match host.
    AbiVersionMismatch { expected: AbiVersion, got: AbiVersion },
    /// A descriptor length field exceeded `MAX_DESCRIPTOR_LIST_LEN`.
    /// `field` is one of `"name"`, `"capabilities"`, or
    /// `"required_host_caps"`.
    DescriptorBoundsExceeded { field: &'static str, len: u32 }, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
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
