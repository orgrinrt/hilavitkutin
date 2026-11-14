//! `Str` eviction and injection: pure bit-layout logic.
//!
//! The DESIGN describes an `Archive` trait relationship; the real
//! trait lands with rkyv. Until then the pure bit-layout halves are
//! free functions so consumers can evict / inject handles today.
//!
//! A string is stored under its full 64-bit FNV-1a content hash. A const
//! handle carries only 28 bits of that hash, so it cannot be the key by
//! itself: evict resolves any handle to its bytes through the interner and
//! hashes the bytes. Injection runs the reverse: a const handle built from
//! the low 28 bits of the key is accepted only when the string it resolves
//! to hashes back to the whole key; otherwise the bytes come from the string
//! table and are interned.

use arvo_hash::ContentHash;
use arvo_refit::Narrow;
use hilavitkutin_str::{ArenaInterner, Str, StringInterner, const_fnv1a};
use notko::{Maybe, Outcome};

use crate::error::PersistenceError;
use crate::string_table::StringTable;

/// The on-disk content hash of a string: the full FNV-1a hash of its bytes.
#[rustfmt::skip]
fn content_hash_of(s: &str) -> ContentHash { // lint:allow(no-bare-string) reason: persistence boundary: the bytes being hashed; tracked: #72
    ContentHash::from_raw(const_fnv1a(s))
}

/// Evict a `Str` handle to its on-disk content-hash value.
///
/// The handle resolves to bytes through the interner, a const handle through
/// the const table and a runtime handle through the arena, and the bytes hash
/// to the key. A handle the interner cannot resolve evicts to
/// `PersistenceError::Missing`: a const handle holds 28 bits of the hash and
/// no longer stands for the key without the string behind it.
pub fn evict_str<A: ArenaInterner>(
    handle: Str,
    interner: &StringInterner<A>,
) -> Outcome<ContentHash, PersistenceError> {
    match interner.resolve(handle) {
        Maybe::Is(s) => Outcome::Ok(content_hash_of(s)),
        Maybe::Isnt => Outcome::Err(PersistenceError::Missing),
    }
}

/// Inject a content hash back as a live `Str` handle.
///
/// First checks the const table (via the interner's resolve path): the
/// candidate const handle is built from the low 28 bits of the key and taken
/// only if the string it resolves to hashes back to the whole key, so a
/// handle that shares 28 bits with the key and nothing else is not taken for
/// the string. Otherwise looks up the bytes in `string_table` and interns
/// them through the arena, returning a runtime handle.
pub fn inject_str<A: ArenaInterner>(
    content_hash: ContentHash,
    interner: &StringInterner<A>,
    string_table: &StringTable,
) -> Outcome<Str, PersistenceError> {
    let candidate = Str::__make(content_hash.narrow_to::<28>());
    if let Maybe::Is(resolved) = interner.resolve(candidate)
        && content_hash_of(resolved) == content_hash
    {
        return Outcome::Ok(candidate);
    }

    // Runtime path: look up bytes in the string table, intern via
    // the arena, return a runtime handle.
    let bytes = match string_table.lookup(content_hash) {
        Maybe::Is(b) => b,
        Maybe::Isnt => return Outcome::Err(PersistenceError::Missing),
    };
    let s = match core::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => return Outcome::Err(PersistenceError::Archive),
    };
    Outcome::Ok(interner.intern(s))
}
