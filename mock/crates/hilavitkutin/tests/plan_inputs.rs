//! PlanInputs construction tests (5a2 skeleton).

use hilavitkutin::plan::{AccessMask, PlanInputs};

#[test]
fn plan_inputs_new_is_zero_filled() {
    let p: PlanInputs<8, 16> = PlanInputs::new();
    assert_eq!(p.unit_count, 0);
    assert_eq!(p.record_count, 0);
    for m in p.access.iter() {
        assert!(m.is_empty());
    }
    for b in p.commutative.iter() {
        assert!(!*b);
    }
}

#[test]
fn plan_inputs_default_matches_new() {
    let a: PlanInputs<4, 8> = PlanInputs::new();
    let b: PlanInputs<4, 8> = PlanInputs::default();
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
    let mut p: PlanInputs<4, 16> = PlanInputs::new();
    p.access[0] = AccessMask::empty().set(2).set(5);
    p.unit_count = 1;
    p.record_count = 10_000;
    assert!(p.access[0].contains(2));
    assert!(p.access[0].contains(5));
    assert_eq!(p.unit_count, 1);
    assert_eq!(p.record_count, 10_000);
}
