//! `Str` eviction and injection — pure bit-layout logic.
//!
//! The DESIGN describes an `Archive` trait relationship; the real
//! trait lands with rkyv. This round ships the pure bit-layout halves
//! as free functions so consumers can evict / inject handles today.
//!
//! Const handles (bit 31 = 0) carry content hashes directly (28-bit
//! FNV truncation) — identity on evict. Runtime handles (bit 31 = 1)
//! resolve to bytes via the interner, then re-hash. Injection runs
//! the reverse: match the const table first; otherwise look up bytes
//! in the string table and intern.

use hilavitkutin_str::{const_fnv1a, ArenaInterner, Str, StringInterner};

use crate::error::PersistenceError;
use crate::string_table::StringTable;

/// Evict a `Str` handle to its on-disk u32 representation.
///
/// Const handles (bit 31 = 0) already carry a 28-bit content hash;
/// their id portion is returned unchanged. Runtime handles resolve
/// via the interner and re-hash the bytes.
pub fn evict_str<A: ArenaInterner>(handle: Str, interner: &StringInterner<A>) -> u32 {
    if handle.is_const() {
        handle.id()
    } else {
        let s = interner
            .resolve(handle)
            .expect("runtime handles always resolve via arena");
        (const_fnv1a(s) & Str::ID_MASK as u64) as u32
    }
}

/// Inject a content-hashed u32 back as a live `Str` handle.
///
/// First checks the const table (via the interner's resolve path):
/// if the hash matches a known const entry, returns that const
/// handle. Otherwise looks up the bytes in `string_table` and interns
/// them through the arena, returning a runtime handle.
pub fn inject_str<A: ArenaInterner>(
    content_hash: u32,
    interner: &StringInterner<A>,
    string_table: &StringTable,
) -> Result<Str, PersistenceError> {
    let masked = content_hash & Str::ID_MASK;

    // Consult const table via the interner. A const hit returns the
    // const handle unchanged; a miss falls through to the runtime
    // lookup path.
    let candidate = Str::__make(masked);
    if let Some(resolved) = interner.resolve(candidate) {
        // Confirm the const entry hashes back to the same masked id.
        let back = (const_fnv1a(resolved) & Str::ID_MASK as u64) as u32;
        if back == masked {
            return Ok(candidate);
        }
    }

    // Runtime path: look up bytes in the string table, intern via
    // the arena, return a runtime handle.
    let bytes = string_table
        .lookup(masked)
        .ok_or(PersistenceError::Missing)?;
    let s = core::str::from_utf8(bytes).map_err(|_| PersistenceError::Archive)?;
    Ok(interner.intern(s))
}
