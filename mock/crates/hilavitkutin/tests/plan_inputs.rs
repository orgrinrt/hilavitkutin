//! PlanInputs construction tests (5a2 skeleton).

// The lifted Cap-dimension types carry `[(); cap_size(N)]:` bounds. A
// downstream crate that instantiates them must itself enable generic_const_exprs
// so its own trait solver can normalise the bounds, mirroring arvo's cross-crate
// tests. adt_const_params is needed only where a Cap const-generic param is
// declared (the engine crate root), not where a lifted type is named. WATCH-tier
// per the unstable-feature soundness sweep (#626).
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Bool, Cap, Identity, USize};
use arvo_tensor::cap;
use hilavitkutin::plan::{AccessMask, PlanInputs};

const C4: Cap = cap(4);
const C8: Cap = cap(8);
const C16: Cap = cap(16);

#[test]
fn plan_inputs_new_is_zero_filled() {
    let p: PlanInputs<C8, C16> = PlanInputs::new();
    assert_eq!(p.unit_count, USize::ZERO);
    assert_eq!(p.record_count, USize::ZERO);
    for m in p.access.iter() {
        assert_eq!(m.is_empty(), Bool::TRUE);
    }
    for b in p.commutative.iter() {
        assert_eq!(*b, Bool::FALSE);
    }
}

#[test]
fn plan_inputs_default_matches_new() {
    let a: PlanInputs<C4, C8> = PlanInputs::new();
    let b: PlanInputs<C4, C8> = PlanInputs::default();
    assert_eq!(a.unit_count, b.unit_count);
    assert_eq!(a.record_count, b.record_count);
    for i in 0..4 {
        assert_eq!(a.access[i], b.access[i]);
        assert_eq!(a.reads[i], b.reads[i]);
        assert_eq!(a.writes[i], b.writes[i]);
        assert_eq!(a.commutative[i], b.commutative[i]);
    }
}

#[test]
fn plan_inputs_access_can_be_populated() {
    let mut p: PlanInputs<C4, C16> = PlanInputs::new();
    p.access[0] = AccessMask::empty().set(USize(2)).set(USize(5)); // lint:allow(no-bare-numeric) reason: bit-index literals; tracked: #399
    p.unit_count = USize(1); // lint:allow(no-bare-numeric) reason: count literal; tracked: #399
    p.record_count = USize(10_000); // lint:allow(no-bare-numeric) reason: count literal; tracked: #399
    assert_eq!(p.access[0].contains(USize(2)), Bool::TRUE); // lint:allow(no-bare-numeric) reason: bit-index literal; tracked: #399
    assert_eq!(p.access[0].contains(USize(5)), Bool::TRUE); // lint:allow(no-bare-numeric) reason: bit-index literal; tracked: #399
    assert_eq!(p.unit_count, USize(1)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(p.record_count, USize(10_000)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
}
