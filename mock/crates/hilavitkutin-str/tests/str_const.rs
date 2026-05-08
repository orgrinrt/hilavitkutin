#![feature(const_trait_impl)]
//! `str_const!()` smoke + determinism.

use hilavitkutin_str::{str_const, Str};

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

#[test]
fn handle_id_fits_28_bits() {
    let h = str_const!("delta");
    assert_eq!(h.to_bits().to_raw() & !Str::ID_MASK.to_raw(), 0);
}
