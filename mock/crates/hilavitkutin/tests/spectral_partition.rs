//! Step 6 spectral partitioning test (Phase C C1d-1).

// `spectral_partition` returns `FiberGrouping<MAX_UNITS, MAX_FIBERS>`, a
// type carrying `[(); cap_size(N)]:` bounds through the CSR adapter, so
// this crate enables generic_const_exprs to normalise it. adt_const_params
// is not needed (only Cap values via cap(N) are named, no Cap const param).
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_tensor::cap;
use hilavitkutin::plan::{steps, DependencyGraph};

const UNITS: Cap = cap(6); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121
const EDGES: Cap = cap(16); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121
const FIBERS: Cap = cap(2); // lint:allow(no-bare-numeric) reason: K=2 split target; tracked: #121

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
fn two_cliques() -> DependencyGraph<UNITS, EDGES> {
    let mut g: DependencyGraph<UNITS, EDGES> = DependencyGraph::new();
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
    g.row_offsets[g.unit_count.0] = g.edge_count;
    g.unit_count = U6;
    g
}

#[test]
fn spectral_splits_two_cliques() {
    let grouping = steps::spectral_partition::<UNITS, EDGES, FIBERS>(&two_cliques());
    // K = 2 on a two-clique graph with a single bridge: the Fiedler cut
    // runs through the bridge, so the cliques land in different fibers.
    // The empty-FiberGrouping stub gives fiber_count 0 and all-equal
    // assignments, failing both the count and the separation assertions.
    assert_eq!(grouping.fiber_count, USize(2)); // lint:allow(no-bare-numeric) reason: expected partition count; tracked: #427
    // Sign-robust: assert clique-internal consistency and separation, not
    // which fiber id each clique received.
    let a = grouping.assignment[U0.0];
    let b = grouping.assignment[U5.0];
    assert!(a != b, "cliques not separated");
    assert_eq!(grouping.assignment[U1.0], a, "clique A split");
    assert_eq!(grouping.assignment[U2.0], a, "clique A split");
    assert_eq!(grouping.assignment[U3.0], b, "clique B split");
    assert_eq!(grouping.assignment[U4.0], b, "clique B split");
}
