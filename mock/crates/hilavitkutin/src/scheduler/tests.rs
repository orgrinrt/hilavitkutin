//! `#[cfg(test)]` module split out of `scheduler/mod.rs` (file-size lint).
//! No behaviour change. `PlanHandle`'s fields and `PlanColumn::COUNT` are
//! `pub(super)` in `scheduler::plan_handle` (a sibling of this module) for
//! exactly the construction and assertion this test performs.

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use hilavitkutin_api::StoreId;

use super::PlanHandle;
use super::plan_handle::PlanColumn;

// A handle at a nonzero base, so the assertions discriminate "offset off
// base" from "absolute StoreId".
fn handle_at(base: USize) -> PlanHandle {
    PlanHandle {
        base,
        phase_count: <USize as Identity<Additive>>::IDENTITY,
        trunk_count: <USize as Identity<Additive>>::IDENTITY,
        fiber_count: <USize as Identity<Additive>>::IDENTITY,
        unit_count: <USize as Identity<Additive>>::IDENTITY,
    }
}

// Pin every plan column's StoreId offset off a nonzero base, and bind the
// accessor count to `PlanColumn::COUNT`. Store and read both route through
// `column_id`, so the offsets are a stored contract, not free to drift: a
// wrong match arm shifts an `assert_eq!`, an absolute-vs-off-base confusion
// drops the base, and a `COUNT` that disagrees with the column set fails the
// length check (the span `store_plan`'s doc cites). The nonzero base is what
// discriminates "offset off base" from "absolute StoreId".
#[test]
fn column_ids_are_pinned_offsets_off_base() {
    let base = USize(7); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test base literal; tracked: #72
    let h = handle_at(base);
    let at = |off: usize| StoreId(USize(base.0 + off)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: expected offset off base; tracked: #72
    let ids = [
        h.phases_id(),
        h.trunks_id(),
        h.fibers_id(),
        h.unit_meta_id(),
        h.morsel_windows_id(),
        h.rcm_order_id(),
    ];
    assert_eq!(ids[0], at(0));
    assert_eq!(ids[1], at(1));
    assert_eq!(ids[2], at(2));
    assert_eq!(ids[3], at(3));
    assert_eq!(ids[4], at(4));
    assert_eq!(ids[5], at(5));
    assert_eq!(ids.len(), PlanColumn::COUNT);
}
