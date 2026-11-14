//! The positional `write` under each of the three write members.
//!
//! The context in the test implements the column-write bridge over one small
//! column and counts every store it is asked for, which is where a runner
//! counts the store as changed. A bare column and one under `Written` store
//! every value. One under `Differs` stores a value only where it differs from
//! the one the record holds.

#![no_std]

use core::cell::Cell;

use arvo::USize;
use hilavitkutin_api::{
    AccessSet,
    Accum,
    Column,
    ColumnValue,
    ColumnWriterApi,
    Differs,
    ResolveColumnWrite,
    Virtual,
    Written,
    write,
};

#[derive(Clone, Copy, PartialEq, Debug)]
struct Temp(u32);

/// No comparison.
#[derive(Clone, Copy)]
struct Raw(u32);

struct Tick;

/// The one bundle index the test context resolves a column at.
struct Only;

struct Slot<T: ColumnValue> {
    held:   Cell<[T; 3]>,
    stores: Cell<u32>,
}

impl<T: ColumnValue> Slot<T> {
    fn over(held: [T; 3]) -> Self {
        Slot {
            held:   Cell::new(held),
            stores: Cell::new(0),
        }
    }
}

impl<T: ColumnValue> ResolveColumnWrite<T, Only> for Slot<T> {
    unsafe fn resolve_write(&self, i: USize, v: T) {
        let mut held = self.held.get();
        held[i.0] = v;
        self.held.set(held);
        self.stores.set(self.stores.get() + 1);
    }

    unsafe fn resolve_held(&self, i: USize) -> T {
        self.held.get()[i.0]
    }
}

// The accessor is the api's own method over the bridge.
impl<T: ColumnValue, W: AccessSet> ColumnWriterApi<W> for Slot<T> {}

fn temps() -> Slot<Temp> {
    Slot::over([Temp(1), Temp(2), Temp(3)])
}

#[test]
fn a_bare_column_stores_every_value() {
    let slot = temps();
    // SAFETY: records 0 and 1 are in the column and the test is its one writer.
    unsafe {
        ColumnWriterApi::<write![Column<Temp>]>::write(&slot, USize(0), Temp(7));
        // The value the record already holds: stored and counted all the same.
        ColumnWriterApi::<write![Column<Temp>]>::write(&slot, USize(1), Temp(2));
    }
    assert_eq!(slot.held.get(), [Temp(7), Temp(2), Temp(3)]);
    assert_eq!(slot.stores.get(), 2);
}

#[test]
fn a_column_under_written_stores_every_value() {
    let slot = temps();
    // SAFETY: as above.
    unsafe {
        ColumnWriterApi::<write![Written<Column<Temp>>]>::write(&slot, USize(0), Temp(7));
        ColumnWriterApi::<write![Written<Column<Temp>>]>::write(&slot, USize(1), Temp(2));
    }
    assert_eq!(slot.held.get(), [Temp(7), Temp(2), Temp(3)]);
    assert_eq!(slot.stores.get(), 2);
}

#[test]
fn a_column_under_differs_stores_only_a_value_that_differs() {
    let slot = temps();
    // SAFETY: as above.
    unsafe {
        ColumnWriterApi::<write![Differs<Column<Temp>>]>::write(&slot, USize(0), Temp(7));
        // Equal to what the record holds: neither stored nor counted.
        ColumnWriterApi::<write![Differs<Column<Temp>>]>::write(&slot, USize(1), Temp(2));
        ColumnWriterApi::<write![Differs<Column<Temp>>]>::write(&slot, USize(2), Temp(3));
    }
    assert_eq!(slot.held.get(), [Temp(7), Temp(2), Temp(3)]);
    assert_eq!(slot.stores.get(), 1);
}

// The member is found wherever it sits in the list, and its rule goes with it.
#[test]
fn the_rule_follows_the_member_past_the_head() {
    type Bare = write![Virtual<Tick>, Accum<Temp>, Column<Temp>];
    type Marked = write![Virtual<Tick>, Accum<Temp>, Written<Column<Temp>>];
    type Compared = write![Virtual<Tick>, Accum<Temp>, Differs<Column<Temp>>];

    let bare = temps();
    let marked = temps();
    let compared = temps();
    // SAFETY: as above.
    unsafe {
        ColumnWriterApi::<Bare>::write(&bare, USize(1), Temp(2));
        ColumnWriterApi::<Marked>::write(&marked, USize(1), Temp(2));
        ColumnWriterApi::<Compared>::write(&compared, USize(1), Temp(2));
    }
    assert_eq!(bare.stores.get(), 1);
    assert_eq!(marked.stores.get(), 1);
    assert_eq!(compared.stores.get(), 0);
}

// `Written` asks no comparison of the value type, by position as in a list.
#[test]
fn written_writes_a_value_type_with_no_comparison() {
    let slot = Slot::over([Raw(1), Raw(2), Raw(3)]);
    // SAFETY: as above.
    unsafe {
        ColumnWriterApi::<write![Written<Column<Raw>>]>::write(&slot, USize(2), Raw(3));
    }
    assert_eq!(slot.held.get()[2].0, 3);
    assert_eq!(slot.stores.get(), 1);
}
