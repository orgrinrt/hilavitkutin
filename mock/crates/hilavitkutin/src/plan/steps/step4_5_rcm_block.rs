//! Step 4 (RCM reordering) and step 5 (connected-component block
//! detection, plus its per-phase trunk-count projection).
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.

use arvo::strategy::{Additive, Identity};
use arvo::{Bool, USize};
use arvo_bitmask::NodeId;
use arvo_sparse::{block_diagonal_via, rcm_reorder_via};
use arvo_tensor::{Capacity, cap_size};
use hilavitkutin_api::UnitId;

use super::{BlockPartition, DependencyGraph, PhaseBoundaries, PlanDims};

/// Step 4: Reverse Cuthill-McKee bandwidth-reduction reordering.
///
/// Builds the bidirectional CSR via the `to_csr_bidirectional`
/// adapter and runs arvo-sparse `rcm_reorder_via` over it, returning
/// a renumber permutation where `result[new_pos]` is the `UnitId`
/// placed at that position. This is a locality renumber for arena
/// layout, not the dispatch order: dispatch stays topological (the
/// runner keeps populating `unit_meta` from the topo order). The
/// permutation is computed once at plan time at zero runtime cost.
///
/// `rcm_reorder_via` seeds over the CSR's live `node_count()`, so the
/// slack tail past `unit_count` never enters the permutation; the
/// trailing slots stay `UnitId::ZERO`.
pub fn rcm_reorder<D: PlanDims>(graph: &DependencyGraph<D>) -> <D::Units as Capacity>::Array<UnitId>
where
    <D::Units as Capacity>::Array<USize>: Copy,
    <D::Edges as Capacity>::Array<NodeId>: Copy,
{
    let csr = graph.to_csr_bidirectional();
    let order = rcm_reorder_via::<_, D::Units>(&csr);
    // Convert the arvo NodeId permutation back to the engine UnitId.
    let mut out: <D::Units as Capacity>::Array<UnitId> =
        <D::Units as Capacity>::filled(UnitId::ZERO);
    for (dst, src) in out.as_mut().iter_mut().zip(order.as_ref().iter()) {
        *dst = UnitId::from_index(src.0);
    }
    out
}

/// Step 5: connected-component block detection.
///
/// Detects the block partition of the dependency graph via arvo-sparse
/// `block_diagonal_via` over the `to_csr_bidirectional` adapter. Each
/// block is a weakly-connected component: an independent sub-graph
/// sharing no edges with the others, hence column-disjoint. Blocks map
/// to the column-disjoint trunks that run with zero sync within a
/// phase; `phase_trunk_counts` projects them per phase.
///
/// `block_diagonal_via` seeds over the live `node_count()`, so the
/// slack tail past `unit_count` stays block 0. The Dulmage-Mendelsohn
/// fine decomposition and dead-column elimination layer onto this in a
/// later round.
pub fn block_diagonalise<D: PlanDims>(graph: &DependencyGraph<D>) -> BlockPartition<D::Units>
where
    <D::Units as Capacity>::Array<USize>: Copy,
    <D::Edges as Capacity>::Array<NodeId>: Copy,
{
    let csr = graph.to_csr_bidirectional();
    let (block_count, block_of_unit) = block_diagonal_via::<_, D::Units>(&csr);
    BlockPartition {
        block_count,
        block_of_unit,
    }
}

/// Step 5 projection: trunk count per phase.
///
/// Within each phase (the topo-position range delimited by
/// `waists.boundaries`, ending at `unit_count` for the last phase),
/// the units partition by block id; each distinct block in a phase is
/// one trunk, since trunks within a phase are column-disjoint and run
/// with zero sync. Returns the trunk count per phase; the runner
/// assigns the `Phase` and `Trunk` ids from these counts. A block that
/// straddles a waist contributes a trunk to each phase it touches.
pub fn phase_trunk_counts<D: PlanDims>(
    partition: &BlockPartition<D::Units>,
    waists: &PhaseBoundaries<D>,
    topo: &<D::Units as Capacity>::Array<UnitId>,
    unit_count: USize,
) -> <D::Phases as Capacity>::Array<USize> {
    let mut counts: <D::Phases as Capacity>::Array<USize> =
        <D::Phases as Capacity>::filled(<USize as Identity<Additive>>::IDENTITY);
    let pc = waists.phase_count.0;
    let n = unit_count.0;
    let topo = topo.as_ref();
    let boundaries = waists.boundaries.as_ref();
    let block_of_unit = partition.block_of_unit.as_ref();
    let mut p = 0;
    while p < pc && p < cap_size(<D::Phases as Capacity>::CAP) {
        let start = boundaries[p].0;
        // Phase p ends where phase p+1 starts, or at unit_count for the
        // last phase.
        let end = if p + 1 < pc { boundaries[p + 1].0 } else { n };
        // Count distinct block ids in this phase, deduped through a
        // per-phase seen-flag array indexed by block id.
        let mut seen: <D::Units as Capacity>::Array<Bool> =
            <D::Units as Capacity>::filled(Bool::FALSE);
        let mut distinct = 0;
        let mut i = start;
        while i < end && i < cap_size(<D::Units as Capacity>::CAP) {
            let unit_idx = topo[i].index().0;
            if unit_idx < cap_size(<D::Units as Capacity>::CAP) {
                let block = block_of_unit[unit_idx].0;
                if block < cap_size(<D::Units as Capacity>::CAP) && !seen.as_ref()[block].0 {
                    seen.as_mut()[block] = Bool::TRUE;
                    distinct += 1;
                }
            }
            i += 1;
        }
        counts.as_mut()[p] = USize(distinct); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal count; tracked: #72
        p += 1;
    }
    counts
}
