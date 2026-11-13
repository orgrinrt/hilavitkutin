#![feature(const_trait_impl)]
//! The strings section in a link that registers no string of its own.
//!
//! This binary never invokes `str_const!`, which is the case that left the
//! section's start and stop symbols undefined before the crate placed an
//! entry itself: it did not link at all.

use arvo_bits::{Bits, BitsRefitCtor};
use hilavitkutin_str::{StaticStrEntry, Str, const_fnv1a, static_entries};

#[test]
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios"
))]
fn the_section_holds_the_crates_own_empty_entry() {
    let entries = static_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].value, "");
}

/// The handle an entry carries is the const handle `str_const!` returns for
/// the string: the low 28 bits of its FNV-1a hash, built here from the hash
/// function directly rather than through the entry's own constructor.
#[test]
fn an_entry_hash_is_the_const_handle_of_its_string() {
    let e = StaticStrEntry::new("anchor-check");
    assert_eq!(e.value, "anchor-check");
    assert_eq!(
        e.hash,
        Str::__make(Bits::<28>::from_narrowed(const_fnv1a("anchor-check")))
    );
    assert_ne!(
        e.hash,
        Str::__make(Bits::<28>::from_narrowed(const_fnv1a("anchor-check-2"))),
        "the control: a different string takes a different handle"
    );
}
