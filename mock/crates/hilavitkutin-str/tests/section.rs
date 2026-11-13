#![feature(const_trait_impl)]
//! The strings section in a link that registers no string of its own.
//!
//! This binary never invokes `str_const!`, which is the case that left the
//! section's start and stop symbols undefined before the crate placed an
//! entry itself: it did not link at all.

use hilavitkutin_str::{StaticStrEntry, const_fnv1a, static_entries};

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

#[test]
fn an_entry_hash_is_the_hash_of_its_string() {
    let e = StaticStrEntry::new("anchor-check");
    let again = StaticStrEntry::new("anchor-check");
    assert_eq!(e.value, "anchor-check");
    assert_eq!(e.hash, again.hash);
    assert_ne!(e.hash, StaticStrEntry::new("anchor-check-2").hash);
    assert_ne!(const_fnv1a("anchor-check"), 0);
}
