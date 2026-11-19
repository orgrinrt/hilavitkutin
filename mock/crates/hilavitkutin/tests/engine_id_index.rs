//! Round-trip property for the engine-id index accessors.
//!
//! `from_index` performs no range check (arvo `from_raw` is unchecked), so the
//! tests walk each id's whole `Uint<N>` logical range and assert the inverse
//! `from_index(USize(k)).index() == USize(k)` at every index, the width ceiling
//! included. The ceiling would fail loudly if arvo ever retuned the `Warm`
//! container for a width, which is the drift the accessors exist to absorb.
//! Every range is small enough to walk whole: the widest, `UnitId`, is 2^16.

#![no_std]

use arvo::USize;
use hilavitkutin::dispatch::codegen::{FiberId, PhaseId, TrunkId, UnitId};

#[test]
fn unit_id_index_roundtrips_over_its_whole_range() {
    for k in 0 .. (1usize << 16) {
        // lint:allow(no-bare-numeric) reason: the index range of Uint<16>; tracked: #339
        assert_eq!(UnitId::from_index(USize(k)).index(), USize(k));
    }
}

#[test]
fn fiber_id_index_roundtrips_over_its_whole_range() {
    for k in 0 .. (1usize << 7) {
        // lint:allow(no-bare-numeric) reason: the index range of Uint<7>; tracked: #339
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
