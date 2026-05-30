//! Step 4 RCM reorder wiring test (Phase C C1b).
//!
//! `rcm_reorder` now returns `<TestDims::Units as Capacity>::Array<UnitId>`,
//! a plain fixed-length array sized by the `Capacity` TYPE, so no
//! `generic_const_exprs` gate is needed here: the capacity dimension is a
//! type, not a `Cap` const generic. The chain DAG, the renumber, and the
//! per-slot assertions read against the `Dim<8>` unit capacity.

use arvo::USize;
use arvo_tensor::{Capacity, Dim};
use hilavitkutin::plan::{steps, DependencyGraph, PlanDims};

/// Eight units, sixteen edges; the rest of the dimensions are sized
/// small (this test only exercises the unit / edge axes).
struct TestDims;

impl PlanDims for TestDims {
    type Units = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Stores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Edges = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Phases = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Trunks = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type TrunksPerPhase = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Fibers = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Lanes = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Columns = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ComponentsPerTrunk = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type UnitsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ColumnsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Cores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
}

// The chain's unit indices, named once so the per-literal lint:allow
// lives in one place and the edges / expectations read as structure.
const U0: USize = USize(0); // lint:allow(no-bare-numeric) reason: chain unit index; tracked: #427
const U1: USize = USize(1); // lint:allow(no-bare-numeric) reason: chain unit index; tracked: #427
const U2: USize = USize(2); // lint:allow(no-bare-numeric) reason: chain unit index; tracked: #427
const U3: USize = USize(3); // lint:allow(no-bare-numeric) reason: chain unit index; tracked: #427
const U4: USize = USize(4); // lint:allow(no-bare-numeric) reason: first slack unit index; tracked: #427

/// Build the chain DAG 0->1->2->3 in an eight-unit / sixteen-edge cap,
/// four units and three edges live.
///
/// Node 3 is a pure sink, so `add_edge` alone leaves `unit_count` at 3.
/// The engine's `build_dag` finishes by advancing `unit_count` to the
/// true node count and giving the trailing node an empty row; this
/// mirrors that single-trailing-sink finalise so node 3 is a live,
/// zero-out-degree row rather than slack.
fn chain() -> DependencyGraph<TestDims> {
    let mut g: DependencyGraph<TestDims> = DependencyGraph::new();
    g.add_edge(U0, U1);
    g.add_edge(U1, U2);
    g.add_edge(U2, U3);
    let uc = g.unit_count.0;
    g.row_offsets.as_mut()[uc] = g.edge_count;
    g.unit_count = U4;
    g
}

#[test]
fn rcm_renumbers_chain_to_bandwidth_reverse() {
    let order = steps::rcm_reorder::<TestDims>(&chain());
    // RCM of the undirected chain 0-1-2-3: min-degree start (node 0),
    // ascending-degree BFS, final reverse, so the live order is
    // [3, 2, 1, 0]. Positions past the live count (4..8) keep the
    // default-zero fill, because rcm_reorder_via seeds only over the
    // CSR's live node_count(). Asserted by unit index across all eight
    // slots. This fails on a passthrough stub (which returns the topo
    // order [0, 1, 2, 3, ...]) and on a packed CSR (where the degree-0
    // slack nodes 4-7 would seed first and scramble the order), so it
    // transitively guards the C1a live-count plumbing.
    let expect: <<TestDims as PlanDims>::Units as Capacity>::Array<USize> =
        [U3, U2, U1, U0, U0, U0, U0, U0];
    let order = order.as_ref();
    for (pos, want) in expect.iter().enumerate() {
        assert_eq!(order[pos].index(), *want, "rcm slot {pos}");
    }
}
