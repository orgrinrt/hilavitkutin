//! Store marker types must all be zero-sized.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use core::mem::size_of;
use hilavitkutin_api::{Column, Field, Map, Resource, Seq, Virtual};

struct Pos;
struct Gravity;

#[test]
fn resource_is_zst() {
    assert_eq!(size_of::<Resource<Gravity>>(), 0);
}

#[test]
fn column_is_zst() {
    assert_eq!(size_of::<Column<Pos>>(), 0);
}

#[test]
fn virtual_is_zst() {
    assert_eq!(size_of::<Virtual<Pos>>(), 0);
}

#[test]
fn field_is_zst() {
    assert_eq!(size_of::<Field<u32>>(), 0);
}

#[test]
fn seq_is_zst() {
    assert_eq!(size_of::<Seq<u32, { Cap(USize(8)) }>>(), 0);
}

#[test]
fn map_is_zst() {
    assert_eq!(size_of::<Map<u32, u64, { Cap(USize(8)) }>>(), 0);
}
