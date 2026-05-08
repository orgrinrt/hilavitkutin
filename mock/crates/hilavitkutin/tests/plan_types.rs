//! Plan-stage type surface tests (5a2 skeleton).

use arvo::USize;
use hilavitkutin::plan::{
    AccessMask, ColumnClassification, DependencyGraph, FiberId, UnitId, PhaseId,
};

#[test]
fn unit_id_copy_eq_default() {
    let a = UnitId(7);
    let b = a;
    assert_eq!(a, b);
    assert_eq!(UnitId::default(), UnitId(0));
}

#[test]
fn fiber_id_copy_eq_default() {
    let a = FiberId(3);
    let b = a;
    assert_eq!(a, b);
    assert_eq!(FiberId::default(), FiberId(0));
}

#[test]
fn phase_id_copy_eq_default() {
    let a = PhaseId(2);
    let b = a;
    assert_eq!(a, b);
    assert_eq!(PhaseId::default(), PhaseId(0));
}

#[test]
fn access_mask_empty_set_contains_overlaps() {
    let empty: AccessMask<16> = AccessMask::empty();
    assert!(empty.is_empty().0);
    assert!(!empty.contains(USize(0)).0);

    let m = empty.set(USize(3)).set(USize(7));
    assert!(!m.is_empty().0);
    assert!(m.contains(USize(3)).0);
    assert!(m.contains(USize(7)).0);
    assert!(!m.contains(USize(4)).0);

    let other: AccessMask<16> = AccessMask::empty().set(USize(7));
    assert!(m.overlaps(&other).0);

    let disjoint: AccessMask<16> = AccessMask::empty().set(USize(1)).set(USize(2));
    assert!(!m.overlaps(&disjoint).0);
}

#[test]
fn dependency_graph_default_and_edges() {
    let mut g: DependencyGraph<8> = DependencyGraph::new();
    assert!(!g.has_edge(USize(0), USize(1)).0);
    assert!(!g.has_edge(USize(3), USize(5)).0);

    g.add_edge(USize(0), USize(1));
    g.add_edge(USize(3), USize(5));
    assert!(g.has_edge(USize(0), USize(1)).0);
    assert!(g.has_edge(USize(3), USize(5)).0);
    assert!(!g.has_edge(USize(1), USize(0)).0);

    // Out-of-range no-ops.
    g.add_edge(USize(100), USize(200));
    assert!(!g.has_edge(USize(100), USize(200)).0);
}

#[test]
fn column_classification_variants_distinct() {
    assert_ne!(ColumnClassification::Internal, ColumnClassification::Input);
    assert_ne!(ColumnClassification::Internal, ColumnClassification::Output);
    assert_ne!(ColumnClassification::Input, ColumnClassification::Output);
    assert_eq!(ColumnClassification::default(), ColumnClassification::Internal);
}
