//! The 13-step plan algorithm chain.
//!
//! Each step is a free function with a stable signature. Steps
//! produce the per-stage intermediate analytical types and feed the
//! next step in the chain. The runner `compute_execution_plan`
//! orchestrates them and returns `Outcome<ExecutionPlan, PlanError>`.
//!
//! Step responsibilities (Topic 3 axis A + Domain 15):
//! 1. `build_dag`: AccessMask overlap to CSR `DependencyGraph`.
//! 2. `topo_sort`: Kahn's algorithm to produce a topological order.
//! 3. `compute_waists`: narrow cut detection to delimit phases.
//! 4. `rcm_reorder`: Reverse Cuthill-McKee bandwidth-reduction reordering.
//! 5. `block_diagonalise`: connected-component block detection to trunk skeletons.
//! 6. `spectral_partition`: spectral clustering via a symmetric Laplacian.
//! 7. `group_fibers`: greedy fiber assignment with bounded slack.
//! 8. `compute_upward_rank_and_dirty` (fused per Topic 3 S5):
//!    reverse-topo critical-path rank + per-fiber dirty propagation.
//! 9. `size_morsels`: per-fiber morsel sizing based on record count.
//! 10. `select_phase_configs`: pick MaxFuse/Balanced/MaxSplit per phase.
//! 11. `classify_columns`: per-fiber column role (Internal/Input/Output).
//! 12. `assign_cores`: map trunks onto concrete cores by `CoreClass`.
//! 13. `synthesise_core_programs`: per-core projection from plan.
//!
//! Steps 4 to 6 (`rcm_reorder`, `block_diagonalise`, `spectral_partition`)
//! are wired to arvo-sparse / arvo-spectral through the
//! `DependencyGraph::to_csr_bidirectional` adapter. The runner consumes
//! the rcm renumber (step 4) and the block-detection trunk skeletons
//! (step 5); step 6's spectral grouping awaits the C1d bench and the
//! fiber projection (HILA-RUNTIME-C1 follow-up slices).
//!
//! Steps 13 ships its body in a follow-up commit alongside
//! `plan/core_program.rs` (Pass 3 codegen feeds it).
//!
//! Every step is generic over one `D: PlanDims` that bundles the
//! capacity dimensions it sizes by; the dimensions are types, so no
//! `cap_size` sits in an array-length position.

use arvo::strategy::Identity;
use arvo::{Bits, Bool, FastFloat, Hot, Unsigned, USize};
use arvo_bitmask::{BitMatrix, Mask, NodeId};
use arvo_graph::waist_detect;
use arvo_sparse::{block_diagonal_via, rcm_reorder_via};
use arvo_spectral::k_way_partition;
use arvo_tensor::{cap_size, Capacity};

use hilavitkutin_api::{TrunkId, UnitId};

use super::column::{ColumnClassMap, ColumnClassification};
use super::dims::PlanDims;
use super::dirty::DirtyMasks;
use super::fiber::{Fiber, FiberGrouping};
use super::graph::{DependencyGraph, EdgeKind};
use super::inputs::PlanInputs;
use super::laplacian::SymmetricLaplacian;
use super::phase::{PhaseBoundaries, PhaseConfig};
use super::trunk::{BlockPartition, Trunk};

/// Eigenvector float for the spectral partition step. `f32` is the IEEE
/// width tag of arvo's `FastFloat`, not a bare numeric value.
type SpectralFloat = FastFloat<f32>; // lint:allow(no-bare-numeric) reason: f32 is the IEEE width tag of arvo FastFloat; tracked: #72

/// Flat CSR projection output: the plan-wide `trunks` and `fibers` pools
/// the projection writes, plus the per-phase trunk counts.
///
/// The runner copies the pools onto the `ExecutionPlan` and uses
/// `phase_trunks` to set each phase's `(trunk_offset, trunk_count)` CSR
/// range. `phase_trunks[p]` is the number of trunks the projection
/// actually emitted for phase `p` (capped by the plan-wide `D::Trunks`
/// budget), so the per-phase ranges always bracket the flat pool exactly.
pub struct FiberLayout<D: PlanDims> {
    pub trunks: <D::Trunks as Capacity>::Array<Trunk>,
    pub trunk_count: USize,
    pub fibers: <D::Fibers as Capacity>::Array<Fiber<D>>,
    pub fiber_count: USize,
    pub phase_trunks: <D::Phases as Capacity>::Array<USize>,
}

/// Step 1: build the CSR `DependencyGraph` from `AccessMask` overlap.
///
/// RAW edges are order-independent: for every ordered pair `(s, t)`
/// with `s != t`, if the reader `t` reads what the writer `s` wrote,
/// append a `Read` edge `s to t` (the writer sorts before the reader,
/// regardless of input order, so a reader registered before its writer
/// still gets the dependency). WAW conflicts serialise
/// deterministically: a `Write` edge `s to t` only when `s < t` (the
/// lower input index first), so two writers of one store never produce
/// a back-edge and a spurious cycle. The CSR append-order invariant is
/// preserved because the outer loop walks the source `s` in ascending
/// order. WAR anti-dependencies are out of scope (a tracked plan-chain
/// follow-up).
pub fn build_dag<D: PlanDims>(
    inputs: &PlanInputs<D::Units, D::Stores>,
) -> DependencyGraph<D> {
    let mut g: DependencyGraph<D> = DependencyGraph::new();
    let n = inputs.unit_count.0;
    let reads = inputs.reads.as_ref();
    let writes = inputs.writes.as_ref();
    // Outer loop walks the source `s` in ascending order, so every
    // appended edge has a source no smaller than the previous one,
    // satisfying `add_edge_kind`'s CSR append-order invariant.
    let mut s = 0;
    while s < n {
        let mut t = 0;
        while t < n {
            if t != s {
                // RAW: `t` reads what `s` wrote, so the writer `s` runs
                // before the reader `t`. Checked for every ordered pair
                // (not only `s < t`), so a reader registered ahead of its
                // writer still gets the dependency.
                if reads[t].overlaps(&writes[s]).0 {
                    g.add_edge_kind(USize(s), USize(t), EdgeKind::Read); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal loop counter; tracked: #72
                }
                // WAW: both write the same store. Serialise
                // deterministically, the lower input index first, so the
                // back-direction is never added (two writers cannot form a
                // spurious cycle).
                if s < t && writes[t].overlaps(&writes[s]).0 {
                    g.add_edge_kind(USize(s), USize(t), EdgeKind::Write); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal loop counter; tracked: #72
                }
            }
            t += 1;
        }
        s += 1;
    }
    // Ensure every input unit has a row entry, even units with zero
    // out-degree. row_offsets for empty rows equals edge_count
    // (consistent with the CSR invariant: empty row = start == end).
    while g.unit_count.0 < n && g.unit_count.0 < cap_size(<D::Units as Capacity>::CAP) {
        let uc = g.unit_count.0;
        g.row_offsets.as_mut()[uc] = g.edge_count;
        g.unit_count = USize(g.unit_count.0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
    }
    g
}

/// Sentinel value marking an already-placed unit in the in-degree
/// counter array used by `topo_sort`. Distinguished from a real
/// in-degree count (which is bounded by the edge capacity) by being
/// set to `usize::MAX`, which no valid in-degree can ever reach.
const CONSUMED: USize = USize(usize::MAX); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sentinel definition; rust grammar requires raw usize literal here; tracked: #72

/// Step 2: topological sort via Kahn's algorithm.
///
/// Returns the units in topo order and the count of units that were
/// placed. The placed-count is the cycle-detection signal: when
/// `placed < graph.unit_count`, the input contains a cycle. The
/// runner (`compute_execution_plan`) is responsible for translating
/// that into `PlanError::Cycle`. Trailing entries in the returned
/// array (indices `placed..unit capacity`) are left as `UnitId::ZERO`
/// (the array's initial fill); they are NOT the cycle members. The
/// caller must use the placed count to slice the valid prefix.
pub fn topo_sort<D: PlanDims>(
    graph: &DependencyGraph<D>,
) -> (<D::Units as Capacity>::Array<UnitId>, USize) {
    let mut out: <D::Units as Capacity>::Array<UnitId> =
        <D::Units as Capacity>::filled(UnitId::ZERO);
    let n = graph.unit_count.0;
    if n == 0 {
        return (out, USize::ZERO);
    }
    // In-degree counter.
    let mut in_degree: <D::Units as Capacity>::Array<USize> =
        <D::Units as Capacity>::filled(USize::ZERO);
    let cols = graph.col_indices.as_ref();
    let row_offsets = graph.row_offsets.as_ref();
    let mut e = 0;
    while e < graph.edge_count.0 {
        let d = cols[e].index().0;
        if d < cap_size(<D::Units as Capacity>::CAP) {
            let id = in_degree.as_mut();
            id[d] = USize(id[d].0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
        }
        e += 1;
    }
    // Simple queue replacement: a placement cursor over a fixed array.
    // The outer loop is a fixed-point iteration over zero-in-degree
    // units. Cycles cause an iteration with no progress, at which
    // point the loop exits with `placed < n`; the runner reads the
    // count and produces `PlanError::Cycle`.
    let mut placed: usize = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal placement cursor; rust grammar requires usize; tracked: #72
    let mut progress = true;
    while progress && placed < n {
        progress = false;
        let mut i = 0;
        while i < n {
            // Skip already-placed units (in_degree set to CONSUMED).
            if in_degree.as_ref()[i].0 == 0 {
                let id = UnitId::from_index(USize(i));
                out.as_mut()[placed] = id;
                placed += 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal cursor increment; tracked: #72
                in_degree.as_mut()[i] = CONSUMED;
                progress = true;
                // Decrement successors of unit `i`.
                let start = row_offsets[i].0;
                let end_excl = graph.end_for(i);
                let mut k = start;
                while k < end_excl {
                    let d = cols[k].index().0;
                    let deg = in_degree.as_ref()[d];
                    if d < cap_size(<D::Units as Capacity>::CAP) && deg.0 != CONSUMED.0 && deg.0 > 0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sentinel + bound check on USize internal field; tracked: #72
                        in_degree.as_mut()[d] = USize(deg.0 - 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
                    }
                    k += 1;
                }
            }
            i += 1;
        }
    }
    (out, USize(placed)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-wrap internal cursor; tracked: #72
}

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
/// The bit-matrix is `Bits<64>`-wide, an exact fit for the engine's
/// default unit capacity (`Dim<64>`); arbitrary node counts above 64
/// are a separate arc (the dense / CSR-sparse / spectral node-count
/// branch).
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
    let mut adj: BitMatrix<Bits<64, Hot, Unsigned>, D::Units> = BitMatrix::empty();
    let mut from = 0;
    while from < n {
        let mut to = 0;
        while to < n {
            if graph.has_edge(USize(from), USize(to)).0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
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

    let waists: Mask<Bits<64, Hot, Unsigned>> =
        waist_detect::<D::Units, Bits<64, Hot, Unsigned>>(&adj, &topo_nodes);

    // Phase 0 starts at position 0; each waist position (with a successor)
    // opens a new phase at the next position.
    boundaries.boundaries.as_mut()[0] = USize::ZERO;
    boundaries.phase_count = USize(1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: at least one phase always; tracked: #72
    let mut p = 0;
    while p + 1 < n && boundaries.phase_count.0 < cap_size(<D::Phases as Capacity>::CAP) {
        if waists.contains(USize(p)).0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal position; tracked: #72
            let next_phase = boundaries.phase_count.0;
            boundaries.boundaries.as_mut()[next_phase] = USize(p + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            boundaries.phase_count = USize(next_phase + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
        }
        p += 1;
    }
    boundaries
}

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
pub fn rcm_reorder<D: PlanDims>(
    graph: &DependencyGraph<D>,
) -> <D::Units as Capacity>::Array<UnitId>
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
pub fn block_diagonalise<D: PlanDims>(
    graph: &DependencyGraph<D>,
) -> BlockPartition<D::Units>
where
    <D::Units as Capacity>::Array<USize>: Copy,
    <D::Edges as Capacity>::Array<NodeId>: Copy,
{
    let csr = graph.to_csr_bidirectional();
    let (block_count, block_of_unit) = block_diagonal_via::<_, D::Units>(&csr);
    BlockPartition { block_count, block_of_unit }
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
        <D::Phases as Capacity>::filled(USize::ZERO);
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

/// Step 6: spectral partitioning.
///
/// Builds the symmetric graph Laplacian over the bidirectional CSR
/// (`SymmetricLaplacian`) and runs arvo-spectral's `k_way_partition`
/// to assign each unit a fiber by spectral cut, with the fiber
/// capacity as `K`. Returns a `FiberGrouping` mapping each unit to its
/// spectral partition id.
///
/// The spectral-versus-greedy `group_fibers` (step 7) choice and the
/// projection onto trunk components land in later C1d slices; the
/// runner does not consume this output yet. `k_way_partition` operates
/// over the full unit capacity; on a loose CSR the slack rows are
/// isolated and a live-node-count-aware spectral path is a follow-up
/// gated on the bench adopting spectral.
pub fn spectral_partition<D: PlanDims>(
    graph: &DependencyGraph<D>,
) -> FiberGrouping<D>
where
    <D::Units as Capacity>::Array<SpectralFloat>: Copy,
    <D::Units as Capacity>::Array<USize>: Copy,
    <D::Edges as Capacity>::Array<NodeId>: Copy,
{
    use hilavitkutin_api::FiberId;
    let csr = graph.to_csr_bidirectional();
    let lap: SymmetricLaplacian<D::Units, D::Edges, SpectralFloat> =
        SymmetricLaplacian::new(&csr);
    let sigma = lap.lambda_max_bound();
    let iterations = USize(100); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: spectral power-iteration count; tracked: #72
    let (count, partition) =
        k_way_partition::<_, D::Units, D::Fibers, SpectralFloat>(&lap, sigma, iterations);
    let mut grouping: FiberGrouping<D> = FiberGrouping::new();
    grouping.fiber_count = count;
    // Map each unit's spectral partition id to its fiber.
    for (slot, part) in grouping.assignment.as_mut().iter_mut().zip(partition.as_ref().iter()) {
        *slot = FiberId::from_index(*part);
    }
    grouping
}

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
        USize::ZERO
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
        USize::ZERO
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
        <D::Fibers as Capacity>::filled(USize::ZERO);
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
        <D::Phases as Capacity>::filled(USize::ZERO);
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
            <D::Units as Capacity>::filled(USize::ZERO);
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
                // lint:allow(no-bare-numeric) reason: width-gate threshold (>5), tunable; tracked: #644
                spectral_grouping_in_block::<D>(
                    &spectral,
                    &block_units.as_ref()[0..bu_count],
                )
            } else {
                group_fibers_in_block::<D>(graph, &block_units.as_ref()[0..bu_count])
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
        USize::ZERO
    };
    g
}

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
        <D::Units as Capacity>::filled(USize::ZERO);
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
        let mut max_rank = USize::ZERO;
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
                while store < cap_size(<D::Stores as Capacity>::CAP) && store < 64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: AccessMask uses USize backing with 64-bit window per skeleton; tracked: #72
                    if writes[u].contains(USize(store)).0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
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

/// Step 9: per-fiber morsel sizing.
///
/// Splits the record count across fibers. The skeleton evenly
/// distributes records and spreads the integer-divide remainder
/// across the first `remainder` fibers (sum-preserving: every record
/// is assigned somewhere). Falls back to the record count itself
/// when only one fiber is active. Bench-driven SIMD-width-aware
/// sizing lands in HILA-RUNTIME-C1.
pub fn size_morsels<D: PlanDims>(
    record_count: USize,
    fiber_count: USize,
) -> <D::Fibers as Capacity>::Array<USize> {
    let mut sizes: <D::Fibers as Capacity>::Array<USize> =
        <D::Fibers as Capacity>::filled(USize::ZERO);
    // Divide-by-zero guard: fiber_count of zero falls back to 1 so
    // the division below is defined. The plan-stage runner only calls
    // this when fiber_count >= 1, but the guard makes the function
    // self-contained.
    let n = if fiber_count.0 == 0 { 1 } else { fiber_count.0 }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: divide-by-zero guard literal; tracked: #72
    let per_fiber = record_count.0 / n;
    let remainder = record_count.0 % n;
    // Distribute the remainder across the first `remainder` fibers.
    // Sum invariant: sum(sizes[0..n]) == record_count.
    let mut i = 0;
    while i < n && i < cap_size(<D::Fibers as Capacity>::CAP) {
        let extra = if i < remainder { 1 } else { 0 }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: remainder-distribution literal; tracked: #72
        sizes.as_mut()[i] = USize(per_fiber + extra); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal compute; tracked: #72
        i += 1;
    }
    sizes
}

/// Heuristic threshold below which a phase is treated as "small" and
/// picks `MaxFuse`. Substrate-default; consumers will be able to tune
/// this once `RunCfg`-level phase-policy lands in Pass 3 / Pass 6.
/// Tracked as a follow-up under task #429 (review-driven).
const SMALL_RECORD_COUNT_THRESHOLD: usize = 10_000; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: substrate-default policy threshold; rust grammar requires usize; tracked: #429

/// Heuristic phase-width threshold above which a phase picks
/// `MaxSplit`. Substrate-default; same tuning story as
/// `SMALL_RECORD_COUNT_THRESHOLD`. Tracked under #429.
const WIDE_PHASE_WIDTH_THRESHOLD: usize = 8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: substrate-default policy threshold; rust grammar requires usize; tracked: #429

/// Step 10: per-phase config selection (MaxFuse / Balanced / MaxSplit).
///
/// Picks based on phase width (number of fibers in the phase) and
/// record count: small phases pick `MaxFuse` to minimise dispatch
/// overhead; wide phases pick `MaxSplit` to maximise parallelism;
/// everything in between picks `Balanced`. Threshold values live as
/// substrate-default constants near this fn; consumer-tunable
/// policy lands when `RunCfg` ships its phase-policy axis (Pass 3 /
/// Pass 6 follow-up).
pub fn select_phase_configs<D: PlanDims>(
    phases: &PhaseBoundaries<D>,
    record_count: USize,
    unit_count: USize,
) -> <D::Phases as Capacity>::Array<PhaseConfig> {
    let mut configs: <D::Phases as Capacity>::Array<PhaseConfig> =
        <D::Phases as Capacity>::filled(PhaseConfig::Balanced);
    let n = phases.phase_count.0;
    let boundaries = phases.boundaries.as_ref();
    let mut i = 0;
    while i < n && i < cap_size(<D::Phases as Capacity>::CAP) {
        // Compute the width of this phase (units it spans).
        let start = boundaries[i].0;
        let end_excl = if i + 1 < n {
            boundaries[i + 1].0
        } else {
            // Last phase spans from its start through the total unit
            // count. Threading `unit_count` in from the runner avoids
            // the prior `start + 1` lower-bound that misclassified a
            // wide last phase as a singleton.
            unit_count.0
        };
        let width = if end_excl > start {
            end_excl - start
        } else {
            1 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: degenerate-width floor for malformed boundaries; tracked: #72
        };
        configs.as_mut()[i] = if record_count.0 < SMALL_RECORD_COUNT_THRESHOLD
            || width == 1 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: explicit-singleton case bound; tracked: #72
        {
            PhaseConfig::MaxFuse
        } else if width > WIDE_PHASE_WIDTH_THRESHOLD {
            PhaseConfig::MaxSplit
        } else {
            PhaseConfig::Balanced
        };
        i += 1;
    }
    configs
}

/// Step 11: per-fiber column classification.
///
/// Walks the fiber assignment and PlanInputs.access masks; classifies
/// each column relative to each fiber as `Internal` (touched only by
/// units in this fiber), `Input` (touched by a unit upstream and read
/// by this fiber), or `Output` (written by this fiber and read by a
/// downstream fiber). The skeleton classifies conservatively as
/// `Internal`; refinement lands in HILA-RUNTIME-C1.
pub fn classify_columns<D: PlanDims>(
    fibers: &FiberGrouping<D>,
    inputs: &PlanInputs<D::Units, D::Stores>,
) -> ColumnClassMap<D>
where
    <D::ColumnsPerFiber as Capacity>::Array<ColumnClassification>: Copy,
{
    let mut map: ColumnClassMap<D> = ColumnClassMap::new();
    let n_fibers = fibers.fiber_count.0;
    let n_units = inputs.unit_count.0;
    let assignment = fibers.assignment.as_ref();
    let access = inputs.access.as_ref();
    // First pass: collect each fiber's touched stores into its
    // column slot list. We treat each touched store as `Internal`
    // initially; the upgrade-to-Input/Output pass would compare
    // across-fiber overlap. The conservative default is sound: it
    // produces correct dispatch shape, just misses some dead-store-
    // elimination opportunities.
    let mut u = 0;
    while u < n_units {
        let f = assignment[u].index().0;
        if f < cap_size(<D::Fibers as Capacity>::CAP) && f < n_fibers {
            // Walk this unit's access mask, register touched stores
            // as columns for fiber f.
            let mut store = 0;
            while store < cap_size(<D::Stores as Capacity>::CAP) && store < 64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: AccessMask 64-bit window per skeleton; tracked: #72
                if access[u].contains(USize(store)).0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
                    let slot = map.column_count.as_ref()[f].0;
                    if slot < cap_size(<D::ColumnsPerFiber as Capacity>::CAP) {
                        map.class.as_mut()[f].as_mut()[slot] = ColumnClassification::Internal;
                        map.column_count.as_mut()[f] = USize(slot + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
                    }
                }
                store += 1;
            }
        }
        u += 1;
    }
    map
}

/// Step 12: map plan trunks onto concrete cores. The body lives in
/// `crate::thread::assign_cores`; this is a re-export for chain
/// consistency. Actual signature parameterised on the `D: PlanDims`
/// ExecutionPlan there.
///
/// The chain treats `assign_cores` as a step but its implementation
/// lives elsewhere; this stub names the step explicitly so the chain
/// reads end-to-end in this file.
pub fn assign_cores_stub() {
    // Real impl: see `crate::thread::assign_cores`. Body lands in
    // HILA-RUNTIME-C4.
}

/// Step 13: per-core program synthesis. Real body needs the per-core
/// projection types from `plan/core_program.rs` (NEW file landing
/// alongside Pass 3 codegen). Stubbed for now.
pub fn synthesise_core_programs_stub() {
    // Real impl lands in HILA-RUNTIME-C2 + plan/core_program.rs.
}

/// PlanError: reasons `compute_execution_plan` rejects the input.
///
/// Each variant signals a specific shape problem the consumer can
/// inspect and respond to. The runner returns these via
/// `Outcome::Err` for upstream propagation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PlanError {
    /// `topo_sort` did not place every unit: the input DAG contains
    /// a cycle.
    Cycle,
    /// Reserved: a trunk shares a write column with another trunk in
    /// the same phase, breaking the zero-sync invariant. Not raised
    /// yet. `block_diagonalise` detects the block partition, but column
    /// disjointness is only decidable after column classification
    /// (step 11), so this fires from that later check. Distinct blocks
    /// are column-disjoint by construction today, so block detection
    /// alone surfaces no alignment fault.
    PhaseAlignmentMismatch,
    /// Reserved: a deeper feasibility reason (matrix-chain DP found no
    /// valid grouping). Not raised yet; layered on with the
    /// Dulmage-Mendelsohn fine decomposition in a later round.
    FeasibilityCheckFailed,
    /// `group_fibers` produced more fibers than the fiber capacity
    /// accommodates, or zero fibers for a non-empty unit set.
    NoTrunkAssignment,
    /// `size_morsels` produced a morsel size below the engine's
    /// hardcoded minimum (1 record).
    MorselSizeBelowMin,
    /// `assign_cores` was asked to map more lanes than the runtime
    /// has cores available.
    CoreCountExceeded,
    /// The `PlanDims` declares a phase capacity larger than the
    /// fixed-width `PhaseId` can name (`PhaseId::ADDRESSABLE`). The high
    /// phase slots would be unaddressable, so the plan stage rejects the
    /// misconfigured dims up front rather than wrapping ids.
    PhaseCapacityExceedsIdWidth,
    /// The `PlanDims` declares a trunk capacity larger than the
    /// fixed-width `TrunkId` can name (`TrunkId::ADDRESSABLE`). The high
    /// trunk slots would be unaddressable, so the plan stage rejects the
    /// misconfigured dims up front rather than wrapping ids.
    TrunkCapacityExceedsIdWidth,
}
