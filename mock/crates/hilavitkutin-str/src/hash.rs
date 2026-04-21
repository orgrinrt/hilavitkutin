//! FNV-1a hash computable in const context.
//!
//! Used for `str_const!` handle derivation and by
//! `hilavitkutin-persistence` for stable on-disk string identity.

/// FNV-1a 64-bit offset basis.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FNV-1a 64-bit algorithm constant; hash width fixed by algorithm; tracked: #72

/// FNV-1a 64-bit prime.
pub const FNV_PRIME: u64 = 0x100000001b3; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FNV-1a 64-bit algorithm constant; hash width fixed by algorithm; tracked: #72

/// FNV-1a 64-bit hash of a string slice, computable in const context.
pub const fn const_fnv1a(s: &str) -> u64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-bare-string) reason: FNV-1a input/output boundary; byte-addressable &str; u64 hash width; tracked: #72
    let bytes = s.as_bytes();
    let mut hash = FNV_OFFSET;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FNV-1a step cast to algorithm width; tracked: #72
        hash = hash.wrapping_mul(FNV_PRIME);
        i += 1;
    }
    hash
}
