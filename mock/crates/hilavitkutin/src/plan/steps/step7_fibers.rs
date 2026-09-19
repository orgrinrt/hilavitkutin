//! Step 7: greedy fiber grouping, its per-block and spectral-filtered
//! variants, the per-phase trunk/fiber component projection that
//! selects between them, and the flat-pool-to-`FiberGrouping`
//! reconstruction the later steps still consume.
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.
//! `group_fibers_in_block` and `spectral_grouping_in_block` stay
//! private: their only caller, `project_fiber_components`, is in this
//! same file, so no widening is needed.

use arvo::strategy::{Additive, Identity};
use arvo::{Bool, USize};
use arvo_bitmask::NodeId;
use arvo_tensor::{Capacity, cap_size};
use hilavitkutin_api::{TrunkId, UnitId};

use super::step6_spectral::spectral_partition;
use super::{
    BlockPartition,
    DependencyGraph,
    Fiber,
    FiberGrouping,
    FiberLayout,
    PhaseBoundaries,
    PlanDims,
    SpectralFloat,
    Trunk,
};

/// Step 7: greedy fiber grouping.
///
/// Assigns each unit to a fiber such that fibers respect topo order
/// and stay within the consumer's fiber capacity. The skeleton walks
/// the topo order and emits one fiber per leaf chain (a maximal
/// chain of units where each has exactly one in-degree and one out-
/// degree). Real heuristics (matrix-chain DP for non-trivial branch
/// merging) land in HILA-RUNTIME-C1.
pub fn group_fibers<D: PlanDims>(
    graph: &DependencyGraph<D>,
    topo: &<D::Units as Capacity>::Array<UnitId>,
) -> FiberGrouping<D> {
    use hilavitkutin_api::FiberId;
    let mut g: FiberGrouping<D> = FiberGrouping::new();
    let n = graph.unit_count.0;
    if n == 0 {
        return g;
    }
    let topo = topo.as_ref();
    let mut current_fiber: usize = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal counter; tracked: #72
    // Track which fiber actually received the last assignment so the
    // final count reflects fibers used, not fibers reached.
    let mut max_used_fiber: usize = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal counter; tracked: #72
    let mut any_assigned = false;
    let mut i = 0;
    while i < n {
        let idx = topo[i].index().0;
        if idx < cap_size(<D::Units as Capacity>::CAP) {
            let fid = FiberId::from_index(USize(current_fiber));
            g.assignment.as_mut()[idx] = fid;
            max_used_fiber = current_fiber;
            any_assigned = true;
            // Roll over to a new fiber whenever the unit's out-degree
            // is more than 1 (branching) or zero (leaf); single
            // chains pack into one fiber.
            let out_deg = graph.out_degree(USize(idx)).0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            if out_deg != 1 && current_fiber + 1 < cap_size(<D::Fibers as Capacity>::CAP) {
                current_fiber += 1;
            }
        }
        i += 1;
    }
    g.fiber_count = if any_assigned {
        USize(max_used_fiber + 1) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
    } else {
        <USize as Identity<Additive>>::IDENTITY
    };
    g
}

/// Greedy fiber former restricted to one block's units.
///
/// Walks `block_units` (the block's units in topo order) and assigns
/// block-local FiberIds via the same out-degree roll-over the global
/// `group_fibers` uses, writing into `assignment[global_unit_index]`.
/// Forming fibers per-block keeps every fiber inside one block so
/// fibers nest within their trunk: a global walk can roll a fiber
/// across a block boundary when topo order interleaves blocks.
fn group_fibers_in_block<D: PlanDims>(
    graph: &DependencyGraph<D>,
    block_units: &[UnitId],
) -> FiberGrouping<D> {
    use hilavitkutin_api::FiberId;
    let mut g: FiberGrouping<D> = FiberGrouping::new();
    let n = block_units.len();
    if n == 0 {
        return g;
    }
    let mut current_fiber = 0;
    let mut max_used_fiber = 0;
    let mut any_assigned = false;
    let mut i = 0;
    while i < n {
        let idx = block_units[i].index().0;
        if idx < cap_size(<D::Units as Capacity>::CAP) {
            g.assignment.as_mut()[idx] = FiberId::from_index(USize(current_fiber)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            max_used_fiber = current_fiber;
            any_assigned = true;
            let out_deg = graph.out_degree(USize(idx)).0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            if out_deg != 1 && current_fiber + 1 < cap_size(<D::Fibers as Capacity>::CAP) {
                current_fiber += 1;
            }
        }
        i += 1;
    }
    g.fiber_count = if any_assigned {
        USize(max_used_fiber + 1) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
    } else {
        <USize as Identity<Additive>>::IDENTITY
    };
    g
}

/// Filter the global spectral grouping to one block and remap.
///
/// Takes the whole-graph spectral `FiberGrouping` and a block's units,
/// keeps only those units' assignments, and remaps the block's distinct
/// spectral ids to contiguous block-local FiberIds, returning the same
/// block-local shape `group_fibers_in_block` returns. Spectral respects
/// block boundaries (disconnected blocks have independent Fiedler
/// vectors), so a block's units carry a self-contained id set.
fn spectral_grouping_in_block<D: PlanDims>(
    global: &FiberGrouping<D>,
    block_units: &[UnitId],
) -> FiberGrouping<D> {
    use hilavitkutin_api::FiberId;
    let mut g: FiberGrouping<D> = FiberGrouping::new();
    // Remap global spectral id -> block-local id in first-seen order.
    let mut remap: <D::Fibers as Capacity>::Array<USize> =
        <D::Fibers as Capacity>::filled(<USize as Identity<Additive>>::IDENTITY);
    let mut seen: <D::Fibers as Capacity>::Array<Bool> =
        <D::Fibers as Capacity>::filled(Bool::FALSE);
    let global_assign = global.assignment.as_ref();
    let mut local_count = 0;
    let mut i = 0;
    while i < block_units.len() {
        let uidx = block_units[i].index().0;
        if uidx < cap_size(<D::Units as Capacity>::CAP) {
            let gid = global_assign[uidx].index().0;
            if gid < cap_size(<D::Fibers as Capacity>::CAP) {
                if !seen.as_ref()[gid].0 {
                    seen.as_mut()[gid] = Bool::TRUE;
                    remap.as_mut()[gid] = USize(local_count); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
                    local_count += 1;
                }
                let remapped = remap.as_ref()[gid];
                g.assignment.as_mut()[uidx] = FiberId::from_index(remapped);
            }
        }
        i += 1;
    }
    g.fiber_count = USize(local_count); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal count; tracked: #72
    g
}

/// Project the per-block fiber grouping onto per-phase trunk components.
///
/// For each phase, blocks (connected components) map to trunks in
/// first-seen topo order (the same dedup `phase_trunk_counts` does).
/// Within each trunk, fibers form over that block's units in topo order
/// via the greedy former, so every fiber nests in its trunk. Each
/// block-local fiber becomes a `TrunkComponent::Fiber` carrying a
/// plan-wide `FiberId`, the fiber's units, and the unit count; the
/// remaining `Fiber` fields (columns, head+tail, dispatch shape) fill
/// in at later steps. `TrunkId`s are plan-wide running ids.
///
/// The former is greedy for every block in this slice; the width-gated
/// spectral former for wide blocks lands in a follow-on slice at the
/// marked selection point.
pub fn project_fiber_components<D: PlanDims>(
    graph: &DependencyGraph<D>,
    partition: &BlockPartition<D::Units>,
    waists: &PhaseBoundaries<D>,
    topo: &<D::Units as Capacity>::Array<UnitId>,
    unit_count: USize,
) -> FiberLayout<D>
where
    Fiber<D>: Copy,
    <D::Trunks as Capacity>::Array<Trunk>: Copy,
    <D::Fibers as Capacity>::Array<Fiber<D>>: Copy,
    <D::Units as Capacity>::Array<SpectralFloat>: Copy,
    <D::Units as Capacity>::Array<USize>: Copy,
    <D::Edges as Capacity>::Array<NodeId>: Copy,
{
    use hilavitkutin_api::FiberId;
    let mut trunks: <D::Trunks as Capacity>::Array<Trunk> =
        <D::Trunks as Capacity>::filled(Trunk::new());
    let mut fibers: <D::Fibers as Capacity>::Array<Fiber<D>> =
        <D::Fibers as Capacity>::filled(Fiber::new());
    let mut phase_trunks: <D::Phases as Capacity>::Array<USize> =
        <D::Phases as Capacity>::filled(<USize as Identity<Additive>>::IDENTITY);
    let pc = waists.phase_count.0;
    let n = unit_count.0;
    let topo_s = topo.as_ref();
    let boundaries = waists.boundaries.as_ref();
    let block_of_unit = partition.block_of_unit.as_ref();
    // Plan-wide running write cursors into the flat pools. A fiber's `id`
    // equals its flat `fibers` index; a trunk's `(fiber_offset,
    // fiber_count)` brackets the fibers it wrote.
    let mut next_trunk = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal flat-pool cursor; tracked: #72
    let mut next_fiber = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal flat-pool cursor; tracked: #72
    // Global spectral grouping, computed once and filtered per wide block
    // by the width-gate below. It respects block boundaries, so a wide
    // block's units carry a self-contained partition. Computed
    // unconditionally for now; skipping it when no block is wide is a
    // follow-up optimisation.
    let spectral = spectral_partition::<D>(graph);
    let mut p = 0;
    while p < pc && p < cap_size(<D::Phases as Capacity>::CAP) {
        let start = boundaries[p].0;
        let end = if p + 1 < pc { boundaries[p + 1].0 } else { n };
        // Map block id -> trunk index within the phase, first-seen order.
        let mut block_to_trunk: <D::Units as Capacity>::Array<USize> =
            <D::Units as Capacity>::filled(<USize as Identity<Additive>>::IDENTITY);
        let mut block_seen: <D::Units as Capacity>::Array<Bool> =
            <D::Units as Capacity>::filled(Bool::FALSE);
        let mut phase_trunk_count = 0;
        let mut i = start;
        while i < end && i < cap_size(<D::Units as Capacity>::CAP) {
            let unit_idx = topo_s[i].index().0;
            if unit_idx < cap_size(<D::Units as Capacity>::CAP) {
                let block = block_of_unit[unit_idx].0;
                if block < cap_size(<D::Units as Capacity>::CAP) && !block_seen.as_ref()[block].0 {
                    block_seen.as_mut()[block] = Bool::TRUE;
                    if phase_trunk_count < cap_size(<D::TrunksPerPhase as Capacity>::CAP) {
                        block_to_trunk.as_mut()[block] = USize(phase_trunk_count); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
                        phase_trunk_count += 1;
                    }
                }
            }
            i += 1;
        }
        // Trunks emitted for this phase = flat-pool cursor delta; capped
        // by the plan-wide `D::Trunks` budget via the break below.
        let phase_trunk_start = next_trunk;
        // For each trunk in the phase, form fibers within its block and
        // write them into the flat pools.
        let mut t = 0;
        while t < phase_trunk_count && t < cap_size(<D::TrunksPerPhase as Capacity>::CAP) {
            if next_trunk >= cap_size(<D::Trunks as Capacity>::CAP) {
                break;
            }
            // Gather this trunk's block units in topo order.
            let mut block_units: <D::Units as Capacity>::Array<UnitId> =
                <D::Units as Capacity>::filled(UnitId::ZERO);
            let mut bu_count = 0;
            let mut j = start;
            while j < end && j < cap_size(<D::Units as Capacity>::CAP) {
                let unit_idx = topo_s[j].index().0;
                if unit_idx < cap_size(<D::Units as Capacity>::CAP) {
                    let block = block_of_unit[unit_idx].0;
                    if block < cap_size(<D::Units as Capacity>::CAP)
                        && block_seen.as_ref()[block].0
                        && block_to_trunk.as_ref()[block].0 == t
                        && bu_count < cap_size(<D::Units as Capacity>::CAP)
                    {
                        block_units.as_mut()[bu_count] = topo_s[j];
                        bu_count += 1;
                    }
                }
                j += 1;
            }
            // Width-gate: a wide block (more units than the threshold)
            // forms fibers spectrally (filtered from the global
            // grouping); a narrow block keeps the greedy former. The
            // threshold is DESIGN.md.tmpl's ">5 fibers" applied to block
            // unit count for now (tunable). Spectral and greedy agree for
            // narrow chains, so the gate only diverges where it matters.
            let grouping = if bu_count > 5 {
                spectral_grouping_in_block::<D>(&spectral, &block_units.as_ref()[0 .. bu_count]) // lint:allow(no-bare-numeric) reason: width-gate threshold (>5), tunable; tracked: #644
            } else {
                group_fibers_in_block::<D>(graph, &block_units.as_ref()[0 .. bu_count])
            };
            // Emit each block-local fiber into the flat `fibers` pool.
            let trunk_fiber_offset = next_fiber;
            let fc = grouping.fiber_count.0;
            let grouping_assign = grouping.assignment.as_ref();
            let mut local_fid = 0;
            let mut emitted = 0;
            while local_fid < fc && next_fiber < cap_size(<D::Fibers as Capacity>::CAP) {
                let mut fib: Fiber<D> = Fiber::new();
                let mut fu = 0;
                let mut k = 0;
                while k < bu_count {
                    let uidx = block_units.as_ref()[k].index().0;
                    if uidx < cap_size(<D::Units as Capacity>::CAP)
                        && grouping_assign[uidx].index().0 == local_fid
                        && fu < cap_size(<D::UnitsPerFiber as Capacity>::CAP)
                    {
                        fib.units.as_mut()[fu] = block_units.as_ref()[k];
                        fu += 1;
                    }
                    k += 1;
                }
                if fu > 0 {
                    fib.id = FiberId::from_index(USize(next_fiber)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from plan-wide flat index; tracked: #72
                    fib.unit_count = USize(fu); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal count; tracked: #72
                    fibers.as_mut()[next_fiber] = fib;
                    next_fiber += 1;
                    emitted += 1;
                }
                local_fid += 1;
            }
            let mut trunk = Trunk::new();
            trunk.id = TrunkId::from_index(USize(next_trunk)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from plan-wide id; tracked: #72
            trunk.fiber_offset = USize(trunk_fiber_offset); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from flat-pool offset; tracked: #72
            trunk.fiber_count = USize(emitted); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal count; tracked: #72
            trunks.as_mut()[next_trunk] = trunk;
            next_trunk += 1;
            t += 1;
        }
        phase_trunks.as_mut()[p] = USize(next_trunk - phase_trunk_start); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from per-phase emitted count; tracked: #72
        p += 1;
    }
    FiberLayout {
        trunks,
        trunk_count: USize(next_trunk), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from plan-wide trunk total; tracked: #72
        fibers,
        fiber_count: USize(next_fiber), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from plan-wide fiber total; tracked: #72
        phase_trunks,
    }
}

/// Reconstruct a global per-unit `FiberGrouping` from the flat `fibers`
/// pool.
///
/// Walks the flat `fibers` pool and records each unit's plan-wide
/// `FiberId`, so steps 8 to 11 keep consuming a `FiberGrouping` unchanged
/// after the CSR flatten.
pub fn fiber_grouping_from_trunks<D: PlanDims>(
    fibers: &<D::Fibers as Capacity>::Array<Fiber<D>>,
    fiber_count: USize,
) -> FiberGrouping<D> {
    let mut g: FiberGrouping<D> = FiberGrouping::new();
    let fc = fiber_count.0;
    let fibers = fibers.as_ref();
    let mut max_fid = 0;
    let mut any = false;
    let mut f = 0;
    while f < fc && f < cap_size(<D::Fibers as Capacity>::CAP) {
        let fib = &fibers[f];
        let fid = fib.id.index().0;
        let uc = fib.unit_count.0;
        let fib_units = fib.units.as_ref();
        let mut u = 0;
        while u < uc && u < cap_size(<D::UnitsPerFiber as Capacity>::CAP) {
            let uidx = fib_units[u].index().0;
            if uidx < cap_size(<D::Units as Capacity>::CAP) {
                g.assignment.as_mut()[uidx] = fib.id;
                if fid > max_fid {
                    max_fid = fid;
                }
                any = true;
            }
            u += 1;
        }
        f += 1;
    }
    g.fiber_count = if any {
        USize(max_fid + 1) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
    } else {
        <USize as Identity<Additive>>::IDENTITY
    };
    g
}
