//! Author-side contracts.
//!
//! Extension crates implement these traits on their extension struct;
//! the `#[export_extension]` proc-macro consumes the impls to emit the
//! descriptor and trampolines.

use core::ffi::c_void;

use crate::descriptor::{CapabilityId, ExtensionAbiStatus, ExtensionVersion};

/// Author-side metadata contract.
///
/// Implemented by the extension's top-level struct. The macro reads
/// the associated consts to populate the descriptor's static fields.
pub trait ExtensionMeta {
    /// ASCII byte-string name the host uses in diagnostics and
    /// `ExtensionRequirement` lookups. No trailing null.
    const NAME: &'static [u8];

    /// Semver triple plus reserved slot. The macro can default this
    /// from `CARGO_PKG_VERSION`; explicit impls pin it manually.
    const VERSION: ExtensionVersion;

    /// Host capability ids the extension requires at load.
    ///
    /// Empty by default. The host enforces this list before calling
    /// `init_fn`; any missing id produces
    /// `ExtensionError::RequiredHostCapabilityMissing`.
    const REQUIRED_HOST_CAPS: &'static [CapabilityId] = &[];
}

/// Optional init handler. Implemented on the extension struct.
///
/// Called once per load. Return a non-`Ok` status to signal failure;
/// the host surfaces this as `ExtensionError::InitFailed`.
pub trait InitHandler {
    /// Extension-author init entry point.
    ///
    /// `host_ctx` is the opaque pointer the host threaded through at
    /// load time. Treat as opaque; never inspect, mutate, or release.
    ///
    /// # Safety
    ///
    /// Called by the emitted trampoline. Extensions implementing this
    /// handler must not assume `host_ctx` points to any author-known
    /// shape.
    unsafe fn init(host_ctx: *mut c_void) -> ExtensionAbiStatus;
}

/// Optional shutdown handler. Implemented on the extension struct.
///
/// Called exactly once at drop (or at explicit `close`). Return a
/// non-`Ok` status to signal a failure that `close` surfaces but
/// `Drop` swallows.
pub trait ShutdownHandler {
    /// Extension-author shutdown entry point.
    ///
    /// # Safety
    ///
    /// Called by the emitted trampoline. Extensions implementing this
    /// handler must not assume `host_ctx` points to any author-known
    /// shape.
    unsafe fn shutdown(host_ctx: *mut c_void) -> ExtensionAbiStatus;
}

/// Per-capability export contract.
///
/// An extension implements this trait once per capability it exports.
/// The macro reads each impl to emit one `CapabilityEntry` whose
/// `vtable_ptr` field carries `<T as CapabilityExport>::VTABLE_PTR`.
pub trait CapabilityExport {
    /// Compile-time capability id. Typically
    /// `CapabilityId::from_name("...")`.
    const ID: CapabilityId;

    /// Raw pointer to the extension's vtable for this capability.
    /// Layout is consumer-domain-specific; the host treats it as
    /// opaque and hands it through to the consumer trampoline.
    const VTABLE_PTR: *const c_void;
}
