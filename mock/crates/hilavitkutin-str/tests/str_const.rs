#![feature(const_trait_impl)]
//! `str_const!()` smoke + determinism.

use hilavitkutin_str::{StaticStrEntry, Str, str_const};

#[test]
fn same_input_same_handle() {
    let a = str_const!("alpha");
    let b = str_const!("alpha");
    assert_eq!(a, b);
}

#[test]
fn different_inputs_different_handles() {
    let a = str_const!("alpha");
    let b = str_const!("beta");
    assert_ne!(a, b);
}

#[test]
fn handle_is_const_origin() {
    let h = str_const!("gamma");
    assert!(h.is_const().0);
    assert!(!h.is_runtime().0);
}

/// The handle the macro returns is the entry's own, and its id is the low 28
/// bits of the FNV-1a hash, pinned here against the published reference value
/// for "foobar", 0x85944171f73967e8, so the formula is not only compared with
/// itself.
#[test]
fn the_macro_handle_is_the_entrys_and_the_low_28_bits_of_the_hash() {
    let h = str_const!("foobar");
    assert_eq!(h, StaticStrEntry::new("foobar").hash);
    assert_eq!(h.id().to_raw(), 0x0739_67E8);
}

#[test]
fn handle_id_fits_28_bits() {
    let h = str_const!("delta");
    assert_eq!(h.to_bits().to_raw() & !Str::ID_MASK.to_raw(), 0);
}
