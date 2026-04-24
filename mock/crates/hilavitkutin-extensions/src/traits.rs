//! Author-side contracts.
//!
//! Extension crates implement these traits on their extension struct;
//! the `#[export_extension]` proc-macro consumes the impls to emit
//! the descriptor and trampolines.

use core::ffi::c_void;

use crate::descriptor::CapabilityId;

/// Author-side metadata contract.
///
/// Implemented by the extension's top-level struct. The macro reads
/// the associated consts to populate the descriptor's static fields.
pub trait ExtensionMeta {
    /// ASCII name the host uses in diagnostics and
    /// `ExtensionRequirement` lookups.
    const NAME: &'static str;

    /// Semver-style version triple. Defaults to `CARGO_PKG_VERSION`
    /// parsed at macro-expansion time; override here to pin manually.
    const VERSION: crate::descriptor::ExtensionVersion;

    /// Host capability ids the extension requires at load.
    ///
    /// Empty by default. The host enforces this list before calling
    /// the extension's `init_fn`; any missing id produces
    /// `ExtensionError::NotSupported`.
    const REQUIRED_HOST_CAPS: &'static [CapabilityId] = &[];
}

/// Optional init handler. Implemented on the extension struct.
///
/// Called once per load. Return non-zero to signal failure; the host
/// surfaces this as `ExtensionError::InitFailed`.
pub trait InitHandler {
    /// Extension-author init entry point.
    ///
    /// `host_ctx` is the opaque pointer the host threaded through at
    /// load time. Treat as opaque; never inspect, mutate, or release.
    ///
    /// # Safety
    ///
    /// Called by the emitted trampoline. Extensions implementing
    /// this handler must not assume `host_ctx` points to any
    /// author-known shape.
    unsafe fn init(host_ctx: *mut c_void) -> i32;
}

/// Optional shutdown handler. Implemented on the extension struct.
///
/// Called exactly once at drop. Return non-zero to signal a failure
/// that `close` surfaces but `Drop` swallows.
pub trait ShutdownHandler {
    /// Extension-author shutdown entry point.
    ///
    /// # Safety
    ///
    /// Called by the emitted trampoline. Extensions implementing
    /// this handler must not assume `host_ctx` points to any
    /// author-known shape.
    unsafe fn shutdown(host_ctx: *mut c_void) -> i32;
}

/// Per-capability export contract.
///
/// An extension implements this trait once per capability it exports.
/// The macro reads each impl to emit one `CapabilityEntry` plus an
/// `extern "C"` trampoline that calls the user's method through the
/// vtable pointer.
pub trait CapabilityExport {
    /// Compile-time capability id. Typically
    /// `CapabilityId::from_name("...")`.
    const ID: CapabilityId;
}
