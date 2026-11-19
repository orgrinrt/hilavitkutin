//! Round-trip property for the engine-id index accessors.
//!
//! `from_index` performs no range check (arvo `from_raw` is unchecked), so the
//! tests walk each id's whole `Uint<N>` logical range and assert the inverse
//! `from_index(USize(k)).index() == USize(k)` at every index, the width ceiling
//! included. The ceiling would fail loudly if arvo ever retuned the `Warm`
//! container for a width, which is the drift the accessors exist to absorb.
//! Every range is small enough to walk whole: the widest, `UnitId`, is 2^16.
//! Each walk reads its bound from the id's own `ADDRESSABLE`, so it follows a
//! width change.

#![no_std]

use arvo::USize;
use hilavitkutin::dispatch::codegen::{FiberId, PhaseId, TrunkId, UnitId};

#[test]
fn unit_id_index_roundtrips_over_its_whole_range() {
    for k in 0 .. UnitId::ADDRESSABLE {
        assert_eq!(UnitId::from_index(USize(k)).index(), USize(k));
    }
}

#[test]
fn fiber_id_index_roundtrips_over_its_whole_range() {
    for k in 0 .. FiberId::ADDRESSABLE {
        assert_eq!(FiberId::from_index(USize(k)).index(), USize(k));
    }
}

#[test]
fn phase_id_index_roundtrips_over_its_whole_range() {
    for k in 0 .. PhaseId::ADDRESSABLE {
        assert_eq!(PhaseId::from_index(USize(k)).index(), USize(k));
    }
}

#[test]
fn trunk_id_index_roundtrips_over_its_whole_range() {
    for k in 0 .. TrunkId::ADDRESSABLE {
        assert_eq!(TrunkId::from_index(USize(k)).index(), USize(k));
    }
}
