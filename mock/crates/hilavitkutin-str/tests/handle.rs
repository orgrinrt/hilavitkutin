//! Bit-layout roundtrips for `Str`.

use hilavitkutin_str::Str;

#[test]
fn default_is_const_zero() {
    let s = Str::default();
    assert_eq!(s.0, 0);
    assert!(s.is_const());
    assert!(!s.is_runtime());
    assert_eq!(s.id(), 0);
}

#[test]
fn make_masks_to_28_bits() {
    let s = Str::__make(0xFFFF_FFFF);
    assert_eq!(s.0, 0x0FFF_FFFF);
    assert_eq!(s.id(), 0x0FFF_FFFF);
    assert!(s.is_const());
}

#[test]
fn make_preserves_low_bits() {
    let s = Str::__make(0x0012_3456);
    assert_eq!(s.id(), 0x0012_3456);
    assert!(s.is_const());
    assert!(!s.is_runtime());
}

#[test]
fn runtime_sets_bit_31() {
    let s = Str::__runtime(0x1234);
    assert_eq!(s.0 & Str::RUNTIME_MASK, Str::RUNTIME_MASK);
    assert!(!s.is_const());
    assert!(s.is_runtime());
    assert_eq!(s.id(), 0x1234);
}

#[test]
fn runtime_id_mask_excludes_bit_31() {
    let s = Str::__runtime(0xFFFF_FFFF);
    assert_eq!(s.id(), 0x0FFF_FFFF);
    assert!(s.is_runtime());
}

#[test]
fn const_and_runtime_with_same_id_differ() {
    let c = Str::__make(0x42);
    let r = Str::__runtime(0x42);
    assert_ne!(c, r);
    assert_eq!(c.id(), r.id());
}

#[test]
fn equality_is_integer() {
    let a = Str::__make(7);
    let b = Str::__make(7);
    assert_eq!(a, b);
    assert_eq!(core::mem::size_of::<Str>(), 4);
}
