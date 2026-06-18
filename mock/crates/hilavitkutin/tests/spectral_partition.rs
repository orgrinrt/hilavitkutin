//! Step 6 spectral partitioning test (Phase C C1d-1).
//!
//! `spectral_partition` returns `FiberGrouping<TestDims>`, sized by the
//! `Capacity` TYPES projected from `TestDims`, so no `generic_const_exprs`
//! gate is needed. `Fibers = Dim<2>` sets the K=2 split target.

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_tensor::Dim;
use hilavitkutin::plan::{steps, DependencyGraph, PlanDims};

/// Six units, sixteen edges, K = 2 (two fibers) for the spectral cut.
struct TestDims;

impl PlanDims for TestDims {
    type Units = Dim<6>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Stores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Edges = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Phases = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Trunks = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type TrunksPerPhase = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Fibers = Dim<2>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: K=2 split target; Dim<N> array-length root; tracked: #649
    type Lanes = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Columns = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ComponentsPerTrunk = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type UnitsPerFiber = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ColumnsPerFiber = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Cores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AccumsPerCore = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type PlanAffecting = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AdjRow = Bits<64, Hot, Unsigned>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-wide row covers the small test units; Bits width literal; tracked: #649
}

// Unit indices, named once so the per-literal lint:allow lives in one place.
const U0: USize = USize(0); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U1: USize = USize(1); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U2: USize = USize(2); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U3: USize = USize(3); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U4: USize = USize(4); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U5: USize = USize(5); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U6: USize = USize(6); // lint:allow(no-bare-numeric) reason: node count; tracked: #427

/// Two 3-cliques joined by a single bridge edge (2-3): a graph with a
/// clear spectral cut. Six units fill the cap (no slack). `add_edge`
/// appends in row-major (from non-decreasing) order; node 5 is the
/// trailing sink, finalised the build_dag way.
fn two_cliques() -> DependencyGraph<TestDims> {
    let mut g: DependencyGraph<TestDims> = DependencyGraph::new();
    // Clique A = {0, 1, 2}.
    g.add_edge(U0, U1);
    g.add_edge(U0, U2);
    g.add_edge(U1, U2);
    // Bridge A -> B.
    g.add_edge(U2, U3);
    // Clique B = {3, 4, 5}.
    g.add_edge(U3, U4);
    g.add_edge(U3, U5);
    g.add_edge(U4, U5);
    let uc = g.unit_count.0;
    g.row_offsets.as_mut()[uc] = g.edge_count;
    g.unit_count = U6;
    g
}

#[test]
fn spectral_splits_two_cliques() {
    let grouping = steps::spectral_partition::<TestDims>(&two_cliques());
    // K = 2 on a two-clique graph with a single bridge: the Fiedler cut
    // runs through the bridge, so the cliques land in different fibers.
    // The empty-FiberGrouping stub gives fiber_count 0 and all-equal
    // assignments, failing both the count and the separation assertions.
    assert_eq!(grouping.fiber_count, USize(2)); // lint:allow(no-bare-numeric) reason: expected partition count; tracked: #427
    // Sign-robust: assert clique-internal consistency and separation, not
    // which fiber id each clique received.
    let assignment = grouping.assignment.as_ref();
    let a = assignment[U0.0];
    let b = assignment[U5.0];
    assert!(a != b, "cliques not separated");
    assert_eq!(assignment[U1.0], a, "clique A split");
    assert_eq!(assignment[U2.0], a, "clique A split");
    assert_eq!(assignment[U3.0], b, "clique B split");
    assert_eq!(assignment[U4.0], b, "clique B split");
}
