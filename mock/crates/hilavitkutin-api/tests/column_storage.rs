//! Unified columnar storage contract conformance.
//!
//! Round 202605302100 phase 1: `ColumnStorage` and `Decompose` are
//! trait-only. This test proves the shapes are implementable and that
//! `Decompose`'s leaf set resolves to the cons-list the DESIGN
//! specifies. The behavioral arena test (reserve, write, read back over
//! real memory) ships with the naive impl next round, where backing
//! memory exists.

#![no_std]

use arvo::USize;
use hilavitkutin_api::{ColumnStorage, ColumnValue, Cons, Decompose, Empty, Field, StoreId};
use notko::Outcome;

// A conforming ColumnStorage stub. Proves the trait is implementable and
// that reserve / count interact as a sane impl would. No real backing
// memory this round; column_ptr returns null and is never dereferenced.
struct StubStore {
    last_count: USize,
}

impl ColumnStorage for StubStore {
    type Error = ();

    fn reserve<T: ColumnValue>(&mut self, _id: StoreId, len: USize) -> Outcome<(), ()> {
        self.last_count = len;
        Outcome::Ok(())
    }

    unsafe fn column_ptr<T: ColumnValue>(&self, _id: StoreId) -> *const T {
        core::ptr::null()
    }

    unsafe fn column_ptr_mut<T: ColumnValue>(&self, _id: StoreId) -> *mut T {
        core::ptr::null_mut()
    }

    fn count(&self, _id: StoreId) -> USize {
        self.last_count
    }

    fn release(&mut self, _id: StoreId) {}
}

#[test]
fn columnstorage_is_implementable_and_reserve_count_roundtrips() {
    let mut store = StubStore {
        last_count: USize(0),
    };
    let id = StoreId(USize(0));
    let outcome = store.reserve::<u32>(id, USize(8));
    assert!(matches!(outcome, Outcome::Ok(())));
    assert_eq!(store.count(id), USize(8));
}

// A two-field aggregate. Decompose maps it to its scalar column leaves
// in field order: a u32 leaf then a u16 leaf.
struct TwoField;

impl Decompose for TwoField {
    type Leaves = Cons<Field<u32>, Cons<Field<u16>, Empty>>;
}

// Compile-time type-equality assertion: only compiles if TwoField's
// Leaves is exactly the specified cons-list.
fn assert_leaves<D: Decompose<Leaves = Cons<Field<u32>, Cons<Field<u16>, Empty>>>>() {}

#[test]
fn decompose_resolves_leaves_in_field_order() {
    assert_leaves::<TwoField>();
}
