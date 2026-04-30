//! ColumnValue::BIT_WIDTH defaults and arvo specialisations.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo::{fbits, ibits};
use hilavitkutin_api::ColumnValue;

#[test]
fn blanket_u8() {
    assert_eq!(<u8 as ColumnValue>::BIT_WIDTH, 8);
}

#[test]
fn blanket_u16() {
    assert_eq!(<u16 as ColumnValue>::BIT_WIDTH, 16);
}

#[test]
fn blanket_u32() {
    assert_eq!(<u32 as ColumnValue>::BIT_WIDTH, 32);
}

#[test]
fn blanket_u64() {
    assert_eq!(<u64 as ColumnValue>::BIT_WIDTH, 64);
}

#[test]
fn specialised_one_bit() {
    assert_eq!(
        <UFixed<{ ibits(1) }, { fbits(0) }, Hot> as ColumnValue>::BIT_WIDTH,
        1
    );
}

#[test]
fn specialised_two_bit() {
    assert_eq!(
        <UFixed<{ ibits(2) }, { fbits(0) }, Hot> as ColumnValue>::BIT_WIDTH,
        2
    );
}

#[test]
fn specialised_four_bit() {
    assert_eq!(
        <UFixed<{ ibits(4) }, { fbits(0) }, Hot> as ColumnValue>::BIT_WIDTH,
        4
    );
}
