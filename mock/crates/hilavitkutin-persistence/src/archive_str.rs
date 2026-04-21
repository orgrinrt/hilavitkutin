//! `Str` eviction and injection — pure bit-layout logic.
//!
//! The DESIGN describes an `Archive` trait relationship; the real
//! trait lands with rkyv. This round ships the pure bit-layout halves
//! as free functions so consumers can evict / inject handles today.
//!
//! Const handles (bit 31 = 0) carry content hashes directly (28-bit
//! FNV truncation); identity on evict. Runtime handles (bit 31 = 1)
//! resolve to bytes via the interner, then re-hash. Injection runs
//! the reverse: match the const table first; otherwise look up bytes
//! in the string table and intern.

use arvo_bits::{Bits, Hot};
use arvo_hash::ContentHash;
use hilavitkutin_str::{const_fnv1a, ArenaInterner, Str, StringInterner};
use notko::{Maybe, Outcome};

use crate::error::PersistenceError;
use crate::string_table::StringTable;

/// Evict a `Str` handle to its on-disk content-hash value.
///
/// Const handles (bit 31 = 0) already carry a 28-bit content hash;
/// their id portion is returned unchanged. Runtime handles resolve
/// via the interner and re-hash the bytes.
pub fn evict_str<A: ArenaInterner>(handle: Str, interner: &StringInterner<A>) -> ContentHash {
    if handle.is_const().0 {
        ContentHash::new(handle.id().bits())
    } else {
        let s = interner
            .resolve(handle)
            .unwrap();
        ContentHash::new(const_fnv1a(s) & Str::ID_MASK.bits())
    }
}

/// Inject a content hash back as a live `Str` handle.
///
/// First checks the const table (via the interner's resolve path):
/// if the hash matches a known const entry, returns that const
/// handle. Otherwise looks up the bytes in `string_table` and interns
/// them through the arena, returning a runtime handle.
pub fn inject_str<A: ArenaInterner>(
    content_hash: ContentHash,
    interner: &StringInterner<A>,
    string_table: &StringTable,
) -> Outcome<Str, PersistenceError> {
    let masked_bits: Bits<28, Hot> =
        (content_hash.bits() & Str::ID_MASK.bits()).into();

    // Consult const table via the interner. A const hit returns the
    // const handle unchanged; a miss falls through to the runtime
    // lookup path.
    let candidate = Str::__make(masked_bits);
    if let Maybe::Is(resolved) = interner.resolve(candidate) {
        // Confirm the const entry hashes back to the same masked id.
        let back: Bits<28, Hot> = (const_fnv1a(resolved) & Str::ID_MASK.bits()).into();
        if back == masked_bits {
            return Outcome::Ok(candidate);
        }
    }

    // Runtime path: look up bytes in the string table, intern via
    // the arena, return a runtime handle.
    let bytes = match string_table.lookup(ContentHash::new(masked_bits.bits())) {
        Maybe::Is(b) => b,
        Maybe::Isnt => return Outcome::Err(PersistenceError::Missing),
    };
    let s = match core::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => return Outcome::Err(PersistenceError::Archive),
    };
    Outcome::Ok(interner.intern(s))
}
