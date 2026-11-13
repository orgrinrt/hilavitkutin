//! Store marker types must all be zero-sized.

#![no_std]

use core::mem::size_of;

use arvo::{Cap, USize};
use hilavitkutin_api::{
    Accum,
    Buffered,
    Column,
    Differs,
    Exchange,
    Field,
    Map,
    Reach,
    Resource,
    Seq,
    Snapshot,
    Virtual,
    Written,
};

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
fn buffered_is_zst_in_both_modes() {
    assert_eq!(size_of::<Buffered<Pos>>(), 0);
    assert_eq!(size_of::<Buffered<Pos, Snapshot>>(), 0);
    assert_eq!(size_of::<Buffered<Pos, Exchange>>(), 0);
    assert_eq!(size_of::<Snapshot>(), 0);
    assert_eq!(size_of::<Exchange>(), 0);
}

// The mode left out is `Snapshot`.
#[test]
fn buffered_defaults_to_snapshot() {
    trait SameAs<B> {}
    impl<T> SameAs<T> for T {}
    fn same<A: SameAs<B>, B>() {}
    same::<Buffered<Pos>, Buffered<Pos, Snapshot>>();
}

#[test]
fn accum_is_zst() {
    assert_eq!(size_of::<Accum<Pos>>(), 0);
}

#[test]
fn wrappers_are_zst() {
    assert_eq!(size_of::<Reach<Column<Pos>, { Cap(USize(2)) }>>(), 0);
    assert_eq!(size_of::<Written<Column<Pos>>>(), 0);
    assert_eq!(size_of::<Differs<Buffered<Pos, Exchange>>>(), 0);
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
