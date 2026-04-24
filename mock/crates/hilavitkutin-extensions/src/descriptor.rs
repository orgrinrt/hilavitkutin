//! `#[repr(C)]` descriptor shape and compile-time capability id.
//!
//! The descriptor is the sole contract between extension and host.
//! Extensions expose one symbol; the host reads a pointer to this
//! struct; lifecycle, capability dispatch, and version gating all
//! derive from the descriptor's fields.

use core::ffi::c_void;

/// Host-side ABI version the extensions crate speaks at build time.
///
/// Extensions declare the ABI version they target via
/// `ExtensionDescriptor::abi_version`; the host compares and rejects
/// mismatches via `ExtensionError::AbiVersionMismatch`.
pub const HOST_ABI_VERSION: u32 = 1;

/// Well-known exported symbol name that every extension `cdylib`
/// resolves to `extern "C" fn() -> *const ExtensionDescriptor`.
///
/// Null-terminated for direct use with `Library::resolve`.
pub const DESCRIPTOR_SYMBOL: &[u8] = b"__hilavitkutin_extension_descriptor\0";

/// Stable capability identifier. Compile-time hash of an ASCII name.
///
/// FNV-1a 64-bit. Layout is `#[repr(transparent)]` over `u64` so wire
/// representation matches a plain u64 across platforms.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CapabilityId(pub u64);

impl CapabilityId {
    /// Compute the capability id from an ASCII name at compile time.
    ///
    /// FNV-1a over the raw byte contents. Constant-folded at the call
    /// site; no runtime cost.
    pub const fn from_name(name: &str) -> Self {
        const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
        const FNV_PRIME: u64 = 0x100000001b3;
        let bytes = name.as_bytes();
        let mut hash: u64 = FNV_OFFSET_BASIS;
        let mut i = 0;
        while i < bytes.len() {
            hash ^= bytes[i] as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
            i += 1;
        }
        Self(hash)
    }
}

/// Four-component version record the extension reports.
///
/// Three-component semver plus one u16 padding slot reserved for
/// future extension (flags, build kind, locale).
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ExtensionVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub _reserved: u16,
}

/// C-ABI status returned by init and shutdown handler trampolines.
///
/// `#[repr(u32)]` so it transits the FFI boundary as a plain u32. The
/// host maps non-`Ok` statuses into `ExtensionError::InitFailed` or
/// `ExtensionError::ShutdownFailed` carrying this enum.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ExtensionAbiStatus {
    Ok = 0,
    InitFailed = 1,
    InvalidArg = 2,
    NotSupported = 3,
    Internal = 4,
}

/// Single capability entry in the descriptor's capability table.
///
/// `vtable_ptr` points to a thin extension-owned vtable. The layout
/// behind the pointer is specific to the capability's contract crate;
/// `hilavitkutin-extensions` treats it as opaque.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CapabilityEntry {
    pub id: CapabilityId,
    pub vtable_ptr: *const c_void,
}

// SAFETY: CapabilityEntry carries a raw pointer that is extension-
// owned and stable for the lifetime of the loaded library. Send +
// Sync are sound because the host never mutates through the pointer.
unsafe impl Send for CapabilityEntry {}
unsafe impl Sync for CapabilityEntry {}

/// Top-level `#[repr(C)]` descriptor the host reads from each
/// loaded extension.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtensionDescriptor {
    pub abi_version: u32,
    pub name_ptr: *const u8,
    pub name_len: usize,
    pub version: ExtensionVersion,
    pub capabilities_ptr: *const CapabilityEntry,
    pub capabilities_len: usize,
    pub required_host_caps_ptr: *const CapabilityId,
    pub required_host_caps_len: usize,
    pub init_fn: Option<
        unsafe extern "C" fn(host_ctx: *mut c_void) -> ExtensionAbiStatus,
    >,
    pub shutdown_fn: Option<
        unsafe extern "C" fn(host_ctx: *mut c_void) -> ExtensionAbiStatus,
    >,
}

// SAFETY: ExtensionDescriptor is a POD payload with raw pointers
// into extension-owned static memory. The host reads only; pointers
// are stable for the library's loaded lifetime.
unsafe impl Send for ExtensionDescriptor {}
unsafe impl Sync for ExtensionDescriptor {}
