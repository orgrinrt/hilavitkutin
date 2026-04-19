//! Plan-stage type surface tests (5a2 skeleton).

use hilavitkutin::plan::{
    AccessMask, ColumnClassification, DependencyGraph, FiberId, NodeId, PhaseId,
};

#[test]
fn node_id_copy_eq_default() {
    let a = NodeId(7);
    let b = a;
    assert_eq!(a, b);
    assert_eq!(NodeId::default(), NodeId(0));
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
    assert!(empty.is_empty());
    assert!(!empty.contains(0));

    let m = empty.set(3).set(7);
    assert!(!m.is_empty());
    assert!(m.contains(3));
    assert!(m.contains(7));
    assert!(!m.contains(4));

    let other: AccessMask<16> = AccessMask::empty().set(7);
    assert!(m.overlaps(&other));

    let disjoint: AccessMask<16> = AccessMask::empty().set(1).set(2);
    assert!(!m.overlaps(&disjoint));
}

#[test]
fn dependency_graph_default_and_edges() {
    let mut g: DependencyGraph<8> = DependencyGraph::new();
    assert!(!g.has_edge(0, 1));
    assert!(!g.has_edge(3, 5));

    g.add_edge(0, 1);
    g.add_edge(3, 5);
    assert!(g.has_edge(0, 1));
    assert!(g.has_edge(3, 5));
    assert!(!g.has_edge(1, 0));

    // Out-of-range no-ops.
    g.add_edge(100, 200);
    assert!(!g.has_edge(100, 200));
}

#[test]
fn column_classification_variants_distinct() {
    assert_ne!(ColumnClassification::Internal, ColumnClassification::Input);
    assert_ne!(ColumnClassification::Internal, ColumnClassification::Output);
    assert_ne!(ColumnClassification::Input, ColumnClassification::Output);
    assert_eq!(ColumnClassification::default(), ColumnClassification::Internal);
}
