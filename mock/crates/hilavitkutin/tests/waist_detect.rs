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

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_tensor::Dim;
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
    type AdjRow = Bits<64, Hot, Unsigned>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-wide row covers the small test units; Bits width literal; tracked: #649
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

// A 128-unit capacity with a 128-wide adjacency row, proving the waist cap is
// lifted past the former hardcoded 64. Every dimension is small except Units
// (128) and Edges (256); AdjRow is the 128-wide row that covers the 69-node
// fixture below.
struct WideDims;

impl PlanDims for WideDims {
    type Units = Dim<128>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: >64 capacity proving the cap lift; Dim<N> array-length root; tracked: #649
    type Stores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Edges = Dim<256>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 68 edges live; Dim<N> array-length root; tracked: #649
    type Phases = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Trunks = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type TrunksPerPhase = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Fibers = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Lanes = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Columns = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ComponentsPerTrunk = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type UnitsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ColumnsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Cores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AdjRow = Bits<128, Hot, Unsigned>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 128-wide row lifts the 64-node cap; Bits width literal; tracked: #649
}

// Two parallel length-33 chains feeding one merge node, which fans out to two
// sinks: 69 nodes. Node indices interleave the chains (a_i = 2i, b_i = 2i+1)
// so edges append in ascending `from` order (the CSR invariant). The chains'
// tails (a_32 = 64, b_32 = 65) both point at the merge node m = 66; m fans to
// the two sinks c0 = 67, c1 = 68. All 66 chain nodes precede m topologically,
// so m sits at topo position 66, past the former 64-wide cap. The merge depth
// is the sole strict local-minimum level (width 1 between two width-2 chain
// levels), so m is the only waist and one interior boundary opens at position
// 67. Under the former hardcoded `Bits<64>` row word the column bits for nodes
// >= 64 overflow the 64-bit word and the high-index edges are dropped, so the
// waist would be misdetected; the 128-wide `AdjRow` carries them.
fn twin_chain_merge() -> DependencyGraph<WideDims> {
    let mut g: DependencyGraph<WideDims> = DependencyGraph::new();
    let mut from = 0usize;
    while from < 64 {
        // chain links: a_i -> a_{i+1} (even from) and b_i -> b_{i+1} (odd from)
        g.add_edge(USize(from), USize(from + 2));
        from += 1;
    }
    // chain tails a_32 (64) and b_32 (65) both feed the merge node m = 66
    g.add_edge(USize(64), USize(66));
    g.add_edge(USize(65), USize(66));
    // m fans out to the two sinks
    g.add_edge(USize(66), USize(67));
    g.add_edge(USize(66), USize(68));
    // finalise the two trailing sinks (67, 68): empty rows, node count 69
    g.row_offsets.as_mut()[67] = g.edge_count;
    g.row_offsets.as_mut()[68] = g.edge_count;
    g.unit_count = USize(69);
    g
}

#[test]
fn waist_detected_past_the_former_64_cap() {
    let g = twin_chain_merge();
    let (topo, placed) = steps::topo_sort::<WideDims>(&g);
    assert_eq!(placed.0, 69, "the twin-chain graph has no cycle; all 69 units place");

    let pb = steps::compute_waists::<WideDims>(&g, &topo);

    // The merge node is the sole waist, at topo position 66 (after all 66 chain
    // nodes), so one interior boundary opens at position 67. This position lies
    // past the former 64-wide row cap, so it can only be detected with the
    // widened `WideDims::AdjRow` (`Bits<128>`).
    assert_eq!(pb.phase_count.0, 2, "one interior waist (the merge node), so two phases");
    assert_eq!(pb.boundaries.as_ref()[0].0, 0, "phase 0 always starts at position 0");
    assert_eq!(
        pb.boundaries.as_ref()[1].0,
        67,
        "the merge node is at topo position 66 (past the old 64 cap), so phase 1 \
         opens at position 67; with a 64-wide row the high-index edges drop and \
         this boundary cannot be detected"
    );
}
