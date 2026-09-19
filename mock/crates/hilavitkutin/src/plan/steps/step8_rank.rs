//! Step 8 (fused): upward rank + dirty propagation, and the
//! per-unit predecessor masks the runtime incremental-skip path reads.
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use arvo_bitmask::BitAccess;
use arvo_tensor::{Capacity, cap_size};
use hilavitkutin_api::UnitId;

use super::{DependencyGraph, DirtyMasks, FiberGrouping, PlanDims, PlanInputs};

/// Step 8 (fused, per Topic 3 S5 / P1.5): upward rank + dirty
/// propagation in a single reverse-topo walk.
///
/// Upward rank is the longest path from a unit to any sink. Dirty
/// masks track which stores changed since the last frame on a per-
/// fiber basis. Both walk the same data in reverse-topo order; fusion
/// avoids two passes over the unit set.
pub fn compute_upward_rank_and_dirty<D: PlanDims>(
    graph: &DependencyGraph<D>,
    topo: &<D::Units as Capacity>::Array<UnitId>,
    inputs: &PlanInputs<D::Units, D::Stores>,
    fibers: &FiberGrouping<D>,
) -> (
    <D::Units as Capacity>::Array<USize>,
    DirtyMasks<D::Fibers, D::Stores>,
) {
    let mut ranks: <D::Units as Capacity>::Array<USize> =
        <D::Units as Capacity>::filled(<USize as Identity<Additive>>::IDENTITY);
    let mut dirty: DirtyMasks<D::Fibers, D::Stores> = DirtyMasks::new();
    let n = graph.unit_count.0;
    if n == 0 {
        return (ranks, dirty);
    }
    let topo = topo.as_ref();
    let cols = graph.col_indices.as_ref();
    let row_offsets = graph.row_offsets.as_ref();
    let assignment = fibers.assignment.as_ref();
    let writes = inputs.writes.as_ref();
    // Reverse-topo walk: leaves get rank 0; predecessors take max
    // successor rank + 1.
    let mut i = n;
    while i > 0 {
        i -= 1;
        let u = topo[i].index().0;
        if u >= cap_size(<D::Units as Capacity>::CAP) || u >= graph.unit_count.0 {
            continue;
        }
        // Scan successors for max rank.
        let start = row_offsets[u].0;
        let end_excl = if u + 1 < graph.unit_count.0 {
            row_offsets[u + 1].0
        } else {
            graph.edge_count.0
        };
        let mut max_rank = <USize as Identity<Additive>>::IDENTITY;
        let mut k = start;
        while k < end_excl {
            let d = cols[k].index().0;
            if d < cap_size(<D::Units as Capacity>::CAP) && ranks.as_ref()[d].0 + 1 > max_rank.0 {
                max_rank = USize(ranks.as_ref()[d].0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
            }
            k += 1;
        }
        ranks.as_mut()[u] = max_rank;
        // Dirty propagation: union unit's writes into its fiber's
        // dirty mask. Fiber-level dirty drives incremental-skip.
        if u < inputs.unit_count.0 {
            let f = assignment[u].index().0;
            if f < cap_size(<D::Fibers as Capacity>::CAP) {
                let mut store = 0;
                while store < cap_size(<D::Stores as Capacity>::CAP) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: AccessMask uses USize backing with 64-bit window per skeleton; tracked: #72
                    && store < 64
                {
                    if writes[u]
                        .contains(USize(store)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
                        .0
                    {
                        let pf = dirty.per_fiber.as_ref()[f];
                        dirty.per_fiber.as_mut()[f] = pf.set(USize(store)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
                    }
                    store += 1;
                }
            }
        }
    }
    (ranks, dirty)
}

/// Per-unit predecessor masks for runtime incremental skip (domain 16,
/// canonical Step 9).
///
/// `masks[v]` has bit `u` set for every direct dependency edge `u -> v`,
/// so the bit names unit `u` as a direct predecessor of unit `v`. The bit
/// index is the `UnitId::index` carrier position, the same space the
/// dispatch walk threads, so the runtime dirty propagation can gate each
/// unit by its position: a unit is dirty when any predecessor bit
/// intersects the running dirty mask. Built from the CSR successor
/// adjacency by recording, for each row `from`, its source position into
/// every successor's mask. Sinks contribute nothing (empty rows), and the
/// reverse direction is exactly the predecessor relation the propagation
/// reads.
pub fn compute_predecessor_masks<D: PlanDims>(
    graph: &DependencyGraph<D>,
) -> <D::Units as Capacity>::Array<D::AdjRow> {
    let mut masks: <D::Units as Capacity>::Array<D::AdjRow> =
        <D::Units as Capacity>::filled(D::AdjRow::default());
    let cap = cap_size(<D::Units as Capacity>::CAP);
    let cols = graph.col_indices.as_ref();
    let row_offsets = graph.row_offsets.as_ref();
    let n = graph.unit_count.0;
    let mut from = 0;
    while from < n && from < cap {
        let start = row_offsets[from].0;
        let end_excl = graph.end_for(from);
        let mut k = start;
        while k < end_excl && k < graph.edge_count.0 {
            let to = cols[k].index().0;
            if to < cap {
                let m = masks.as_ref()[to];
                masks.as_mut()[to] = m.with_bit_set(USize(from)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal carrier position; tracked: #72
            }
            k += 1;
        }
        from += 1;
    }
    masks
}
