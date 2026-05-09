//! `#[repr(C)]` descriptor shape and compile-time provider id.
//!
//! The descriptor is the sole contract between extension and host.
//! Extensions expose one symbol; the host reads a pointer to this
//! struct; lifecycle, provider dispatch, and version gating all
//! derive from the descriptor's fields.

use core::ffi::{CStr, c_void};

/// Host-side ABI version newtype.
///
/// `#[repr(transparent)]` over `u32`, so the wire layout is identical
/// to a plain `u32`. Carries the host-facing semantic name through
/// the error path and the `HOST_ABI_VERSION` constant.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct AbiVersion(pub u32); // lint:allow(no-public-raw-field) tracked: #206

/// Host-side ABI version the extensions crate speaks at build time.
///
/// Bumped from 1 to 2 alongside the v1.1 descriptor reshape: the
/// descriptor gained `tag` and `descriptor_size` fields, the
/// `abi_version` field type changed from bare `u32` to `AbiVersion`,
/// and the three length fields flipped from `USize` to bare `u32` for
/// platform-stable wire layout. Per the workspace
/// no-legacy-shims-pre-1.0 rule, no v1 read path exists; descriptors
/// built against `AbiVersion(1)` fail the abi_version check on load.
pub const HOST_ABI_VERSION: AbiVersion = AbiVersion(2);

/// Magic tag the host validates first when reading any
/// `ExtensionDescriptor`. The four bytes spell ASCII "HILE" when read
/// in memory order on little-endian targets, which is the platform set
/// the linking layer commits to. A descriptor with a wrong tag
/// surfaces `ExtensionError::DescriptorTagMismatch` and aborts the
/// load before any further field is read.
pub const EXTENSION_DESCRIPTOR_TAG: u32 = 0x454C4948; // lint:allow(arvo-types-only, no-bare-numeric) tracked: #206

/// Hard ceiling on per-descriptor list lengths (`name_len`,
/// `providers_len`, `required_host_providers_len`). Validated at
/// descriptor-read time; a value above the ceiling surfaces
/// `ExtensionError::DescriptorBoundsExceeded`. The one-million-entry
/// bound is well above any plausible extension's surface area; combined
/// with the largest entry size (`ProviderEntry` at 16 bytes on
/// 64-bit) the worst-case slice byte size bounds at roughly 16 MiB.
pub const MAX_DESCRIPTOR_LIST_LEN: u32 = 1 << 20; // lint:allow(arvo-types-only, no-bare-numeric) tracked: #206

/// Well-known exported symbol name that every extension `cdylib`
/// resolves to `extern "C" fn() -> *const ExtensionDescriptor`.
///
/// Typed as `&CStr` so callers see the nul-terminated-C-string
/// intent. Linking-layer resolve functions that take `&[u8]` receive
/// `DESCRIPTOR_SYMBOL.to_bytes_with_nul()` at the call site.
//
// SAFETY: the byte literal contains a single trailing nul and no
// interior nul. `from_bytes_with_nul_unchecked` is const since 1.59.
pub const DESCRIPTOR_SYMBOL: &CStr = unsafe {
    CStr::from_bytes_with_nul_unchecked(b"__hilavitkutin_extension_descriptor\0")
};

/// Stable provider identifier. Compile-time hash of an ASCII name.
///
/// FNV-1a 64-bit. Layout is `#[repr(transparent)]` over `u64` so wire
/// representation matches a plain u64 across platforms.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ProviderId(pub u64); // lint:allow(no-public-raw-field) tracked: #206

impl ProviderId {
    /// Compute the provider id from an ASCII name at compile time.
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
// The four u16 fields below are bare FFI-wire primitives by deliberate
// choice. arvo has no wire-stable 16-bit newtype yet; see
// BACKLOG.md.tmpl `Flip bare-primitive FFI-wire sites to UWire<N>`.
pub struct ExtensionVersion {
    pub major: u16, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
    pub minor: u16, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
    pub patch: u16, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
    pub _reserved: u16, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
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

/// Single provider entry in the descriptor's provider table.
///
/// `vtable_ptr` points to a thin extension-owned vtable. The layout
/// behind the pointer is specific to the provider's contract crate;
/// `hilavitkutin-extensions` treats it as opaque.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProviderEntry {
    pub id: ProviderId,
    pub vtable_ptr: *const c_void,
}

// SAFETY: ProviderEntry carries a raw pointer that is extension-
// owned and stable for the lifetime of the loaded library. Send +
// Sync are sound because the host never mutates through the pointer.
unsafe impl Send for ProviderEntry {}
unsafe impl Sync for ProviderEntry {}

/// Top-level `#[repr(C)]` descriptor the host reads from each
/// loaded extension.
///
/// The field order is fixed and load-bearing. The host validates
/// `tag` first (so a wrong layout is caught before any other field is
/// trusted), then `descriptor_size` (so the host knows what byte
/// range it can rely on for the v1.1 prefix), then `abi_version`
/// (which gates the rest of the layout under the v1.1 contract), and
/// only after that does it read the pointer-and-length pairs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtensionDescriptor {
    pub tag: u32, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
    pub descriptor_size: u32, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
    pub abi_version: AbiVersion,
    pub name_ptr: *const u8,
    pub name_len: u32, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
    pub version: ExtensionVersion,
    pub providers_ptr: *const ProviderEntry,
    pub providers_len: u32, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
    pub required_host_providers_ptr: *const ProviderId,
    pub required_host_providers_len: u32, // lint:allow(arvo-types-only, no-bare-numeric, no-public-raw-field) tracked: #206
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
