//! Step 3: waist detection. Produces phase boundaries.
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.

use arvo::strategy::{Additive, Identity};
use arvo::{Bool, USize};
use arvo_bitmask::{BitMatrix, Mask, NodeId};
use arvo_graph::waist_detect;
use arvo_tensor::{Capacity, cap_size};
use hilavitkutin_api::UnitId;

use super::{DependencyGraph, PhaseBoundaries, PlanDims};

/// Step 3: waist detection. Produces phase boundaries.
///
/// A waist is a depth in the dependency DAG whose level width is a
/// strict local minimum, the natural narrowing point where a phase
/// barrier belongs. Detection runs through `arvo_graph::waist_detect`
/// over a bit-matrix adjacency built from the `DependencyGraph`: it
/// returns the topo-order positions whose depth is a width-local-minimum,
/// and each such position opens a phase boundary at its successor (the
/// waist unit is the last of its phase). A pipeline with no interior
/// narrowing is one phase.
///
/// The bit-matrix row word is `D::AdjRow`, the concrete row type the
/// `PlanDims` impl pins to cover its `D::Units` node count.
/// `DefaultPlanDims` uses a 64-wide row; a consumer with a larger unit
/// budget pins a wider one, lifting the former 64-node cap.
pub fn compute_waists<D: PlanDims>(
    graph: &DependencyGraph<D>,
    topo: &<D::Units as Capacity>::Array<UnitId>,
) -> PhaseBoundaries<D>
where
    <D::Units as Capacity>::Array<USize>: Copy,
    <D::Units as Capacity>::Array<Bool>: Copy,
{
    let mut boundaries = PhaseBoundaries::<D>::new();
    let n = graph.unit_count.0;
    if n == 0 {
        return boundaries;
    }
    let cap = cap_size(<D::Units as Capacity>::CAP);

    // Build the bit-matrix adjacency `waist_detect` consumes: one edge bit per
    // directed dependency edge `from -> to`, over the unit capacity.
    let mut adj: BitMatrix<D::AdjRow, D::Units> = BitMatrix::empty();
    let mut from = 0;
    while from < n {
        let mut to = 0;
        while to < n {
            if graph.has_edge(USize(from), USize(to)).0 {
                adj.set_edge(NodeId(USize(from)), NodeId(USize(to))); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            }
            to += 1;
        }
        from += 1;
    }

    // Project the topo `UnitId` order into the `NodeId` order `waist_detect`
    // walks. `waist_detect` walks the full capacity, so the slack tail past the
    // live count is filled with an out-of-range node id (>= cap) it skips.
    // Unused node slots have no edges, so they sit at depth 0 and only inflate
    // the depth-0 width, which is the first occupied depth and never an interior
    // local-minimum candidate, so they do not affect the detected waists.
    let topo_s = topo.as_ref();
    let mut topo_nodes: <D::Units as Capacity>::Array<NodeId> =
        <D::Units as Capacity>::filled(NodeId(USize(cap))); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct out-of-range sentinel; tracked: #72
    let mut k = 0;
    while k < n && k < cap {
        topo_nodes.as_mut()[k] = NodeId(topo_s[k].index());
        k += 1;
    }

    let waists: Mask<D::AdjRow> = waist_detect::<D::Units, D::AdjRow>(&adj, &topo_nodes);

    // Phase 0 starts at position 0; each waist position (with a successor)
    // opens a new phase at the next position.
    boundaries.boundaries.as_mut()[0] = <USize as Identity<Additive>>::IDENTITY;
    boundaries.phase_count = USize(1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: at least one phase always; tracked: #72
    let mut p = 0;
    while p + 1 < n && boundaries.phase_count.0 < cap_size(<D::Phases as Capacity>::CAP) {
        if waists
            .contains(USize(p)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal position; tracked: #72
            .0
        {
            let next_phase = boundaries.phase_count.0;
            boundaries.boundaries.as_mut()[next_phase] = USize(p + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            boundaries.phase_count = USize(next_phase + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
        }
        p += 1;
    }
    boundaries
}
