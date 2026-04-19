//! Known-input / known-output pairs for `const_fnv1a`.

use hilavitkutin_str::{const_fnv1a, FNV_OFFSET, FNV_PRIME};

#[test]
fn empty_string_is_offset_basis() {
    assert_eq!(const_fnv1a(""), FNV_OFFSET);
}

#[test]
fn single_byte_a() {
    // FNV-1a of "a": (offset ^ 'a') * prime
    let expected = (FNV_OFFSET ^ (b'a' as u64)).wrapping_mul(FNV_PRIME);
    assert_eq!(const_fnv1a("a"), expected);
}

#[test]
fn foobar_matches_reference() {
    // Reference FNV-1a 64-bit for "foobar" = 0x85944171f73967e8
    assert_eq!(const_fnv1a("foobar"), 0x85944171f73967e8);
}

#[test]
fn different_inputs_differ() {
    assert_ne!(const_fnv1a("foo"), const_fnv1a("bar"));
    assert_ne!(const_fnv1a("foo"), const_fnv1a("Foo"));
}

#[test]
fn is_const_context() {
    const H: u64 = const_fnv1a("clause");
    assert_ne!(H, 0);
}
