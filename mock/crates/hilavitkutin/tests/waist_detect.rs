//! Real waist-detection test (GATE-1 Phase C, #339, slice 1).
//!
//! `compute_waists` delimits phases at waists: depths in the dependency DAG
//! whose level width is a strict local minimum, the natural narrowing points
//! where a phase barrier belongs. This pins that the step uses real
//! concurrent-path-minimum detection (`arvo_graph::waist_detect`) and not the
//! crude out-degree-zero heuristic it replaced.
//!
//! The fixture is a bowtie: two sources fan into one middle unit, which fans
//! out to two sinks. `{0, 1} -> 2 -> {3, 4}`. By depth, the source level
//! (depth 0) has width 2, the middle (depth 1) has width 1, the sink level
//! (depth 2) has width 2. The middle is the sole strict local-minimum depth,
//! so it is the only waist. The topological order places the middle at
//! position 2 (after both sources, before both sinks), so the real detector
//! opens one interior phase boundary at position 3: phase 0 is the two sources
//! plus the middle, phase 1 is the two sinks.
//!
//! Red first: the crude out-degree-zero placeholder treats the two sinks as
//! waists and opens its boundary at a sink position (4), not at the middle's
//! successor (3). The two parallel sinks are not a sequential boundary, so the
//! placeholder over-segments. Asserting the boundary at position 3 fails
//! against the placeholder and passes once `waist_detect` drives the phases.
//!
//! Lives under `tests/` so the bare numeric node indices do not trip the
//! src-tree primitive lints.

use arvo::USize;
use arvo_tensor::{Capacity, Dim};
use hilavitkutin::plan::{steps, DependencyGraph, PlanDims};

// Eight-unit / sixteen-edge capacity; the bowtie uses five units and four
// edges. The rest of the dimensions are sized small.
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

// Bowtie node indices, named once so the per-literal lint:allow lives in one
// place and the edges read as structure.
const N0: USize = USize(0); // lint:allow(no-bare-numeric) reason: bowtie source; tracked: #427
const N1: USize = USize(1); // lint:allow(no-bare-numeric) reason: bowtie source; tracked: #427
const N2: USize = USize(2); // lint:allow(no-bare-numeric) reason: bowtie middle (the waist); tracked: #427
const N3: USize = USize(3); // lint:allow(no-bare-numeric) reason: bowtie sink; tracked: #427
const N4: USize = USize(4); // lint:allow(no-bare-numeric) reason: bowtie sink; tracked: #427
const N5: USize = USize(5); // lint:allow(no-bare-numeric) reason: live unit count past the last sink; tracked: #427

/// Build the bowtie `{0, 1} -> 2 -> {3, 4}` in an eight-unit capacity, five
/// units and four edges live. Edges append in ascending `from` order (the CSR
/// append-order invariant). Nodes 3 and 4 are pure sinks, so `add_edge` alone
/// leaves `unit_count` at 3; the finalise gives each trailing sink an empty
/// row and advances `unit_count` to the true node count, mirroring `build_dag`.
fn bowtie() -> DependencyGraph<TestDims> {
    let mut g: DependencyGraph<TestDims> = DependencyGraph::new();
    g.add_edge(N0, N2);
    g.add_edge(N1, N2);
    g.add_edge(N2, N3);
    g.add_edge(N2, N4);
    // Finalise the two trailing sinks (3, 4): each starts at the current edge
    // count with no outgoing edges, and the node count becomes 5.
    g.row_offsets.as_mut()[N3.0] = g.edge_count;
    g.row_offsets.as_mut()[N4.0] = g.edge_count;
    g.unit_count = N5;
    g
}

#[test]
fn waist_at_bowtie_middle_not_at_sinks() {
    let g = bowtie();
    let (topo, placed) = steps::topo_sort::<TestDims>(&g);
    assert_eq!(placed.0, 5, "the bowtie has no cycle; all five units place");

    let pb = steps::compute_waists::<TestDims>(&g, &topo);

    // The middle (node 2) is the sole waist, at topo position 2, so one
    // interior boundary opens at position 3: phase 0 = the two sources plus the
    // middle, phase 1 = the two sinks.
    assert_eq!(pb.phase_count.0, 2, "the bowtie has one interior waist, so two phases");
    assert_eq!(
        pb.boundaries.as_ref()[0].0, 0,
        "phase 0 always starts at position 0"
    );
    assert_eq!(
        pb.boundaries.as_ref()[1].0, 3,
        "the waist is the bowtie middle (topo position 2), so phase 1 opens at \
         position 3; the crude out-degree heuristic instead splits at a sink \
         position (4), which is the over-segmentation this replaces"
    );
}
