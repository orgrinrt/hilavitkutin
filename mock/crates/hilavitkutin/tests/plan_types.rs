//! Plan-stage type surface tests (5a2 skeleton).

// The lifted Cap-dimension types carry `[(); cap_size(N)]:` bounds. A
// downstream crate that instantiates them must itself enable generic_const_exprs
// so its own trait solver can normalise the bounds, mirroring arvo's cross-crate
// tests. adt_const_params is needed only where a Cap const-generic param is
// declared (the engine crate root), not where a lifted type is named. WATCH-tier
// per the unstable-feature soundness sweep (#626).
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, Identity, USize};
use arvo_tensor::cap;
use hilavitkutin::plan::{
    AccessMask, ColumnClassification, DependencyGraph, FiberId, PhaseId, UnitId,
};

const C8: Cap = cap(8);
const C16: Cap = cap(16);

#[test]
fn unit_id_copy_eq_default() {
    let a = UnitId::from_constant::<{ USize(7) }>(); // lint:allow(no-bare-numeric) reason: unit id literal; tracked: #426
    let b = a;
    assert_eq!(a, b);
    assert_eq!(UnitId::default(), UnitId::ZERO);
}

#[test]
fn fiber_id_copy_eq_default() {
    let a = FiberId::from_constant::<{ USize(3) }>(); // lint:allow(no-bare-numeric) reason: fiber id literal; tracked: #426
    let b = a;
    assert_eq!(a, b);
    assert_eq!(FiberId::default(), FiberId::ZERO);
}

#[test]
fn phase_id_copy_eq_default() {
    let a = PhaseId::from_constant::<{ USize(2) }>(); // lint:allow(no-bare-numeric) reason: phase id literal; tracked: #426
    let b = a;
    assert_eq!(a, b);
    assert_eq!(PhaseId::default(), PhaseId::ZERO);
}

#[test]
fn access_mask_empty_set_contains_overlaps() {
    let empty: AccessMask<C16> = AccessMask::empty();
    assert!(empty.is_empty().0);
    assert!(!empty.contains(USize::ZERO).0);

    let m = empty
        .set(USize(3)) // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
        .set(USize(7)); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
    assert!(!m.is_empty().0);
    assert!(m.contains(USize(3)).0); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
    assert!(m.contains(USize(7)).0); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
    assert!(!m.contains(USize(4)).0); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426

    let other: AccessMask<C16> = AccessMask::empty().set(USize(7)); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
    assert!(m.overlaps(&other).0);

    let disjoint: AccessMask<C16> = AccessMask::empty()
        .set(USize(1)) // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
        .set(USize(2)); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
    assert!(!m.overlaps(&disjoint).0);
}

#[test]
fn dependency_graph_default_and_edges() {
    // CSR graph: MAX_UNITS=8, MAX_EDGES=16.
    let mut g: DependencyGraph<C8, C16> = DependencyGraph::new();
    assert!(!g.has_edge(USize::ZERO, USize(1)).0); // lint:allow(no-bare-numeric) reason: node index; tracked: #427
    assert!(!g.has_edge(USize(3), USize(5)).0); // lint:allow(no-bare-numeric) reason: node index; tracked: #427

    // Append in ascending-from order (CSR invariant): 0 -> 1, then
    // 3 -> 5. Units 1 and 2 land as zero-out-degree implicitly.
    g.add_edge(USize::ZERO, USize(1)); // lint:allow(no-bare-numeric) reason: node index; tracked: #427
    g.add_edge(USize(3), USize(5)); // lint:allow(no-bare-numeric) reason: node index; tracked: #427
    assert!(g.has_edge(USize::ZERO, USize(1)).0); // lint:allow(no-bare-numeric) reason: node index; tracked: #427
    assert!(g.has_edge(USize(3), USize(5)).0); // lint:allow(no-bare-numeric) reason: node index; tracked: #427
    assert!(!g.has_edge(USize(1), USize::ZERO).0); // lint:allow(no-bare-numeric) reason: node index; tracked: #427

    // Out-of-range no-ops.
    g.add_edge(USize(100), USize(200)); // lint:allow(no-bare-numeric) reason: out-of-range probe; tracked: #427
    assert!(!g.has_edge(USize(100), USize(200)).0); // lint:allow(no-bare-numeric) reason: out-of-range probe; tracked: #427
}

#[test]
fn column_classification_variants_distinct() {
    assert_ne!(ColumnClassification::Internal, ColumnClassification::Input);
    assert_ne!(ColumnClassification::Internal, ColumnClassification::Output);
    assert_ne!(ColumnClassification::Input, ColumnClassification::Output);
    assert_eq!(ColumnClassification::default(), ColumnClassification::Internal);
}
