//! Adapter test: DependencyGraph to CsrBidirectional (Phase C C1a).
//!
//! `to_csr_bidirectional` is sized by the `Capacity` TYPES projected from
//! `TestDims`, so no `generic_const_exprs` gate is needed: the unit and edge
//! capacities are types, not `Cap` const generics.

use arvo::USize;
use arvo_bitmask::NodeId;
use arvo_sparse::{BidirectionalSparseAdjacency, SparseAdjacency};
use arvo_tensor::Dim;
use hilavitkutin::plan::{DependencyGraph, PlanDims};

/// Eight units, sixteen edges for the diamond fixture.
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

// The diamond's nodes, named once so the per-literal lint:allow lives in
// one place and the assertions read as graph structure, not integers.
const N0: NodeId = NodeId::new(USize(0)); // lint:allow(no-bare-numeric) reason: diamond node index; tracked: #427
const N1: NodeId = NodeId::new(USize(1)); // lint:allow(no-bare-numeric) reason: diamond node index; tracked: #427
const N2: NodeId = NodeId::new(USize(2)); // lint:allow(no-bare-numeric) reason: diamond node index; tracked: #427
const N3: NodeId = NodeId::new(USize(3)); // lint:allow(no-bare-numeric) reason: diamond node index; tracked: #427
const N4: NodeId = NodeId::new(USize(4)); // lint:allow(no-bare-numeric) reason: first slack node index; tracked: #427

/// Build the diamond DAG 0->1, 0->2, 1->3, 2->3 in an eight-unit /
/// sixteen-edge cap, four units and four edges live.
///
/// Node 3 is a pure sink (no out-edges), so `add_edge` alone leaves
/// `unit_count` at 3. The engine's `build_dag` finishes by advancing
/// `unit_count` to the true node count and giving each trailing node an
/// empty row (`start == end == edge_count`); this mirrors that step so
/// node 3 is a live, zero-out-degree row rather than slack.
fn diamond() -> DependencyGraph<TestDims> {
    let mut g: DependencyGraph<TestDims> = DependencyGraph::new();
    g.add_edge(N0.0, N1.0);
    g.add_edge(N0.0, N2.0);
    g.add_edge(N1.0, N3.0);
    g.add_edge(N2.0, N3.0);
    let uc = g.unit_count.0;
    g.row_offsets.as_mut()[uc] = g.edge_count;
    g.unit_count = N4.0;
    g
}

#[test]
fn forward_successors_match_source_edges() {
    let csr = diamond().to_csr_bidirectional();
    // the source fans out to nodes 1 and 2.
    assert_eq!(csr.successors(N0).count(), 2); // lint:allow(no-bare-numeric) reason: expected out-degree; tracked: #399
    assert!(csr.successors(N0).any(|d| d == N1));
    assert!(csr.successors(N0).any(|d| d == N2));
    // nodes 1 and 2 each point at the sink.
    assert_eq!(csr.successors(N1).count(), 1); // lint:allow(no-bare-numeric) reason: expected out-degree; tracked: #399
    assert!(csr.successors(N1).any(|d| d == N3));
    assert_eq!(csr.successors(N2).count(), 1); // lint:allow(no-bare-numeric) reason: expected out-degree; tracked: #399
    assert!(csr.successors(N2).any(|d| d == N3));
    // the sink has no out-edges.
    assert_eq!(csr.successors(N3).count(), 0); // lint:allow(no-bare-numeric) reason: sink out-degree; tracked: #399
}

#[test]
fn transpose_carries_reverse_adjacency() {
    let csr = diamond().to_csr_bidirectional();
    // the sink is reached from nodes 1 and 2.
    assert_eq!(csr.predecessors(N3).count(), 2); // lint:allow(no-bare-numeric) reason: expected in-degree; tracked: #399
    assert!(csr.predecessors(N3).any(|p| p == N1));
    assert!(csr.predecessors(N3).any(|p| p == N2));
    // nodes 1 and 2 are reached only from the source.
    assert_eq!(csr.predecessors(N1).count(), 1); // lint:allow(no-bare-numeric) reason: expected in-degree; tracked: #399
    assert!(csr.predecessors(N1).any(|p| p == N0));
    assert_eq!(csr.predecessors(N2).count(), 1); // lint:allow(no-bare-numeric) reason: expected in-degree; tracked: #399
    assert!(csr.predecessors(N2).any(|p| p == N0));
}

#[test]
fn node_count_is_live_not_cap() {
    let csr = diamond().to_csr_bidirectional();
    // four live units in an eight-unit cap: node_count reports the live
    // count, never the cap. A packed adapter would report cap_size(UNITS).
    assert_eq!(csr.node_count(), N4.0);
}

#[test]
fn no_slack_tail_pollution() {
    let csr = diamond().to_csr_bidirectional();
    // the source has no predecessors. A packed `Csr::new()` would let the
    // default-NodeId(0) slack tail count as edges into node 0, inflating
    // its in-degree; `with_live_counts` confines iteration to the live
    // range, so the source stays clean. This assertion fails if the
    // adapter forgets the live counts.
    assert_eq!(csr.predecessors(N0).count(), 0); // lint:allow(no-bare-numeric) reason: source in-degree; tracked: #399
    // nodes past the live range carry no edges in either direction.
    assert_eq!(csr.successors(N4).count(), 0); // lint:allow(no-bare-numeric) reason: slack node out-degree; tracked: #399
    assert_eq!(csr.predecessors(N4).count(), 0); // lint:allow(no-bare-numeric) reason: slack node in-degree; tracked: #399
}
