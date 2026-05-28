//! Round-trip property for the engine-id index accessors.
//!
//! `from_index` performs no range check (arvo `from_raw` is unchecked), so the
//! tests stay inside each id's `Uint<N>` logical range and assert the inverse
//! `from_index(USize(k)).index() == USize(k)` holds, including at the width
//! ceiling. The ceiling cases would fail loudly if arvo ever retuned the `Warm`
//! container for a width, which is the drift the accessors exist to absorb.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::USize;
use hilavitkutin_api::{FiberId, PhaseId, TrunkId, UnitId};

#[test]
fn unit_id_index_roundtrips() {
    for k in [USize(0), USize(1), USize(42), USize(65535)] {
        assert_eq!(UnitId::from_index(k).index(), k);
    }
}

#[test]
fn fiber_id_index_roundtrips() {
    for k in [USize(0), USize(1), USize(127)] {
        assert_eq!(FiberId::from_index(k).index(), k);
    }
}

#[test]
fn phase_id_index_roundtrips() {
    for k in [USize(0), USize(1), USize(31)] {
        assert_eq!(PhaseId::from_index(k).index(), k);
    }
}

#[test]
fn trunk_id_index_roundtrips() {
    for k in [USize(0), USize(1), USize(63)] {
        assert_eq!(TrunkId::from_index(k).index(), k);
    }
}
