//! ColumnValue::BIT_WIDTH: the spec-free trait-body default.
//!
//! `ColumnValue` is spec-free (no `specialization`): every `Copy + 'static`
//! type inherits `BIT_WIDTH = size_of * 8` through the empty blanket impl. arvo
//! sub-byte types are no exception, so they report their `#[repr(transparent)]`
//! container width here, not their logical bit width. The logical sub-byte
//! width is reconstructed from the arvo type's own const-generic width by the
//! future bitpacking storage layer, not reported by `ColumnValue`. See `#631`.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo::{fbits, ibits, USize};
use hilavitkutin_api::ColumnValue;

#[test]
fn blanket_u8() {
    assert_eq!(<u8 as ColumnValue>::BIT_WIDTH, USize(8));
}

#[test]
fn blanket_u16() {
    assert_eq!(<u16 as ColumnValue>::BIT_WIDTH, USize(16));
}

#[test]
fn blanket_u32() {
    assert_eq!(<u32 as ColumnValue>::BIT_WIDTH, USize(32));
}

#[test]
fn blanket_u64() {
    assert_eq!(<u64 as ColumnValue>::BIT_WIDTH, USize(64));
}

// Sub-byte arvo types fall through the blanket to their container width: the
// trait reports `size_of * 8`, not the logical bit count. These pin the
// spec-free contract (no per-type override remains) and that the logical width
// is a future bitpacking-layer concern, not a `ColumnValue` const. The
// container width is asserted via `size_of` so the test states the contract
// (blanket applies) rather than a hardcoded container choice.

#[test]
fn sub_byte_one_bit_reports_container_width() {
    type T = UFixed<{ ibits(1) }, { fbits(0) }, Hot>;
    assert_eq!(<T as ColumnValue>::BIT_WIDTH, USize(core::mem::size_of::<T>() * 8));
}

#[test]
fn sub_byte_two_bit_reports_container_width() {
    type T = UFixed<{ ibits(2) }, { fbits(0) }, Hot>;
    assert_eq!(<T as ColumnValue>::BIT_WIDTH, USize(core::mem::size_of::<T>() * 8));
}

#[test]
fn sub_byte_four_bit_reports_container_width() {
    type T = UFixed<{ ibits(4) }, { fbits(0) }, Hot>;
    assert_eq!(<T as ColumnValue>::BIT_WIDTH, USize(core::mem::size_of::<T>() * 8));
}
