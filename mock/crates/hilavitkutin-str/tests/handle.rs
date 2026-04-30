//! Bit-layout roundtrips for `Str`.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo_bits::Bits;
use hilavitkutin_str::Str;

#[test]
fn default_is_const_zero() {
    let s = Str::default();
    assert_eq!(s.to_bits().to_raw(), 0);
    assert!(s.is_const().0);
    assert!(!s.is_runtime().0);
    assert_eq!(s.id().to_raw(), 0);
}

#[test]
fn make_masks_to_28_bits() {
    let s = Str::__make(Bits::<28>::from_raw(0xFFFF_FFFF));
    assert_eq!(s.to_bits().to_raw(), 0x0FFF_FFFF);
    assert_eq!(s.id().to_raw(), 0x0FFF_FFFF);
    assert!(s.is_const().0);
}

#[test]
fn make_preserves_low_bits() {
    let s = Str::__make(Bits::<28>::from_raw(0x0012_3456));
    assert_eq!(s.id().to_raw(), 0x0012_3456);
    assert!(s.is_const().0);
    assert!(!s.is_runtime().0);
}

#[test]
fn runtime_sets_bit_31() {
    let s = Str::__runtime(Bits::<28>::from_raw(0x1234));
    assert_eq!(
        s.to_bits().to_raw() & Str::RUNTIME_MASK.to_raw(),
        Str::RUNTIME_MASK.to_raw(),
    );
    assert!(!s.is_const().0);
    assert!(s.is_runtime().0);
    assert_eq!(s.id().to_raw(), 0x1234);
}

#[test]
fn runtime_id_mask_excludes_bit_31() {
    let s = Str::__runtime(Bits::<28>::from_raw(0xFFFF_FFFF));
    assert_eq!(s.id().to_raw(), 0x0FFF_FFFF);
    assert!(s.is_runtime().0);
}

#[test]
fn const_and_runtime_with_same_id_differ() {
    let c = Str::__make(Bits::<28>::from_raw(0x42));
    let r = Str::__runtime(Bits::<28>::from_raw(0x42));
    assert_ne!(c, r);
    assert_eq!(c.id(), r.id());
}

#[test]
fn equality_is_integer() {
    let a = Str::__make(Bits::<28>::from_raw(7));
    let b = Str::__make(Bits::<28>::from_raw(7));
    assert_eq!(a, b);
    assert_eq!(core::mem::size_of::<Str>(), 4);
}
