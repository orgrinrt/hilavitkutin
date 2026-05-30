//! PlanInputs construction tests (5a2 skeleton).
//!
//! `PlanInputs` is now sized by the `Capacity` TYPE (`Dim<N>`), so no
//! `generic_const_exprs` gate is needed: the unit and store capacities are
//! types, not `Cap` const generics, and the backing arrays are plain
//! `[T; N]`.

use arvo::{Bool, Identity, USize};
use arvo_tensor::Dim;
use hilavitkutin::plan::{AccessMask, PlanInputs};

type C4 = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
type C8 = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
type C16 = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649

#[test]
fn plan_inputs_new_is_zero_filled() {
    let p: PlanInputs<C8, C16> = PlanInputs::new();
    assert_eq!(p.unit_count, USize::ZERO);
    assert_eq!(p.record_count, USize::ZERO);
    for m in p.access.as_ref().iter() {
        assert_eq!(m.is_empty(), Bool::TRUE);
    }
    for b in p.commutative.as_ref().iter() {
        assert_eq!(*b, Bool::FALSE);
    }
}

#[test]
fn plan_inputs_default_matches_new() {
    let a: PlanInputs<C4, C8> = PlanInputs::new();
    let b: PlanInputs<C4, C8> = PlanInputs::default();
    assert_eq!(a.unit_count, b.unit_count);
    assert_eq!(a.record_count, b.record_count);
    let a_access = a.access.as_ref();
    let b_access = b.access.as_ref();
    let a_reads = a.reads.as_ref();
    let b_reads = b.reads.as_ref();
    let a_writes = a.writes.as_ref();
    let b_writes = b.writes.as_ref();
    let a_comm = a.commutative.as_ref();
    let b_comm = b.commutative.as_ref();
    for i in 0..4 {
        assert_eq!(a_access[i], b_access[i]);
        assert_eq!(a_reads[i], b_reads[i]);
        assert_eq!(a_writes[i], b_writes[i]);
        assert_eq!(a_comm[i], b_comm[i]);
    }
}

#[test]
fn plan_inputs_access_can_be_populated() {
    let mut p: PlanInputs<C4, C16> = PlanInputs::new();
    p.access.as_mut()[0] = AccessMask::empty().set(USize(2)).set(USize(5)); // lint:allow(no-bare-numeric) reason: bit-index literals; tracked: #399
    p.unit_count = USize(1); // lint:allow(no-bare-numeric) reason: count literal; tracked: #399
    p.record_count = USize(10_000); // lint:allow(no-bare-numeric) reason: count literal; tracked: #399
    assert_eq!(p.access.as_ref()[0].contains(USize(2)), Bool::TRUE); // lint:allow(no-bare-numeric) reason: bit-index literal; tracked: #399
    assert_eq!(p.access.as_ref()[0].contains(USize(5)), Bool::TRUE); // lint:allow(no-bare-numeric) reason: bit-index literal; tracked: #399
    assert_eq!(p.unit_count, USize(1)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(p.record_count, USize(10_000)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
}
