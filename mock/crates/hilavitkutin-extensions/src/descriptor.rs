//! `#[repr(C)]` descriptor shape and compile-time capability id.
//!
//! The descriptor is the sole contract between extension and host.
//! Extensions expose one symbol; the host reads a pointer to this
//! struct; lifecycle, capability dispatch, and version gating all
//! derive from the descriptor's fields.

use core::ffi::c_void;
use notko::MaybeNull;

/// Host-side ABI version the extensions crate speaks at build time.
///
/// Extensions declare the ABI version they target via
/// `ExtensionDescriptor::abi_version`; the host compares and rejects
/// mismatches via `ExtensionError::AbiVersionMismatch`.
pub const HOST_ABI_VERSION: u32 = 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C-ABI version counter width fixed by contract; tracked: #206

/// Well-known exported symbol name that every extension `cdylib`
/// resolves to `extern "C" fn() -> *const ExtensionDescriptor`.
///
/// Null-terminated for direct use with `Library::resolve`.
pub const DESCRIPTOR_SYMBOL: &[u8] = b"__hilavitkutin_extension_descriptor\0"; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: null-terminated byte string matching the dlsym linker name; tracked: #206

/// Stable capability identifier. Compile-time hash of an ASCII name.
///
/// FNV-1a 64-bit. Layout is `#[repr(transparent)]` over `u64` so wire
/// representation matches a plain u64 across platforms.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CapabilityId(pub u64); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: #[repr(transparent)] over u64; FNV-1a hash width fixed by algorithm and ABI; tracked: #206

impl CapabilityId {
    /// Compute the capability id from an ASCII name at compile time.
    ///
    /// FNV-1a over the raw byte contents. Constant-folded at the call
    /// site; no runtime cost.
    pub const fn from_name(name: &str) -> Self { // lint:allow(no-bare-string) reason: const-fn input; Str has no const constructor at macro-expansion time; tracked: #206
        const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FNV-1a algorithm constant; tracked: #206
        const FNV_PRIME: u64 = 0x100000001b3; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FNV-1a algorithm constant; tracked: #206
        let bytes = name.as_bytes();
        let mut hash: u64 = FNV_OFFSET_BASIS; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FNV-1a state width; tracked: #206
        let mut i = 0;
        while i < bytes.len() {
            hash ^= bytes[i] as u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FNV-1a step cast; tracked: #206
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
    pub major: u16, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: #[repr(C)] ABI field; tracked: #206
    pub minor: u16, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: #[repr(C)] ABI field; tracked: #206
    pub patch: u16, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: #[repr(C)] ABI field; tracked: #206
    pub _reserved: u16, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: #[repr(C)] ABI padding slot; tracked: #206
}

/// C-ABI status returned by init and shutdown handler trampolines.
///
/// `#[repr(u32)]` so it transits the FFI boundary as a plain u32. The
/// host maps non-`Ok` statuses into `ExtensionError::InitFailed` or
/// `ExtensionError::ShutdownFailed` carrying this enum.
#[repr(u32)] // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: #[repr(u32)] attribute itself; C-ABI return value representation; tracked: #206
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ExtensionAbiStatus { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: #[repr(u32)] C-ABI return value discriminants; tracked: #206
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
    pub abi_version: u32, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: #[repr(C)] ABI field; tracked: #206
    pub name_ptr: *const u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: byte pointer to extension name; tracked: #206
    pub name_len: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: #[repr(C)] ABI length field; tracked: #206
    pub version: ExtensionVersion,
    pub capabilities_ptr: *const CapabilityEntry,
    pub capabilities_len: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: #[repr(C)] ABI length field; tracked: #206
    pub required_host_caps_ptr: *const CapabilityId,
    pub required_host_caps_len: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: #[repr(C)] ABI length field; tracked: #206
    pub init_fn: MaybeNull<
        unsafe extern "C" fn(host_ctx: *mut c_void) -> ExtensionAbiStatus,
    >,
    pub shutdown_fn: MaybeNull<
        unsafe extern "C" fn(host_ctx: *mut c_void) -> ExtensionAbiStatus,
    >,
}

// SAFETY: ExtensionDescriptor is a POD payload with raw pointers
// into extension-owned static memory. The host reads only; pointers
// are stable for the library's loaded lifetime.
unsafe impl Send for ExtensionDescriptor {}
unsafe impl Sync for ExtensionDescriptor {}
