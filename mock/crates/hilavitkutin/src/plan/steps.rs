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

use arvo::strategy::Identity;
use arvo::{Bool, Cap, FastFloat, USize};
use arvo_sparse::{block_diagonal_via, rcm_reorder_via};
use arvo_spectral::k_way_partition;
use arvo_tensor::cap_size;

use hilavitkutin_api::{TrunkId, UnitId};

use super::column::{ColumnClassMap, ColumnClassification};
use super::dirty::DirtyMasks;
use super::fiber::{Fiber, FiberGrouping};
use super::graph::{DependencyGraph, EdgeKind};
use super::inputs::PlanInputs;
use super::laplacian::SymmetricLaplacian;
use super::phase::{PhaseBoundaries, PhaseConfig};
use super::trunk::{BlockPartition, Trunk, TrunkComponent};

/// Eigenvector float for the spectral partition step. `f32` is the IEEE
/// width tag of arvo's `FastFloat`, not a bare numeric value.
type SpectralFloat = FastFloat<f32>; // lint:allow(no-bare-numeric) reason: f32 is the IEEE width tag of arvo FastFloat; tracked: #72

/// Step 1: build the CSR `DependencyGraph` from `AccessMask` overlap.
///
/// For each pair of units `(i, j)` with `i < j` in input order: if
/// `j`'s reads overlap `i`'s writes (RAW), append a `Read` edge
/// `i j`; if `j`'s writes overlap `i`'s writes (WAW), append a
/// `Write` edge. The CSR append-order invariant is preserved because
/// the outer loop walks `i` in ascending order.
pub fn build_dag<const MAX_UNITS: Cap, const MAX_STORES: Cap, const MAX_EDGES: Cap>(
    inputs: &PlanInputs<MAX_UNITS, MAX_STORES>,
) -> DependencyGraph<MAX_UNITS, MAX_EDGES>
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_EDGES)]:,
{
    let mut g: DependencyGraph<MAX_UNITS, MAX_EDGES> = DependencyGraph::new();
    let n = inputs.unit_count.0;
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n {
            // RAW: j reads what i wrote.
            if inputs.reads[j].overlaps(&inputs.writes[i]).0 {
                g.add_edge_kind(USize(i), USize(j), EdgeKind::Read); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal loop counter; tracked: #72
            }
            // WAW: j writes what i wrote. Order-only dependency.
            if inputs.writes[j].overlaps(&inputs.writes[i]).0 {
                g.add_edge_kind(USize(i), USize(j), EdgeKind::Write); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal loop counter; tracked: #72
            }
            j += 1;
        }
        i += 1;
    }
    // Ensure every input unit has a row entry, even units with zero
    // out-degree. row_offsets for empty rows equals edge_count
    // (consistent with the CSR invariant: empty row = start == end).
    while g.unit_count.0 < n && g.unit_count.0 < cap_size(MAX_UNITS) {
        g.row_offsets[g.unit_count.0] = g.edge_count;
        g.unit_count = USize(g.unit_count.0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
    }
    g
}

/// Sentinel value marking an already-placed unit in the in-degree
/// counter array used by `topo_sort`. Distinguished from a real
/// in-degree count (which is bounded by `MAX_EDGES`) by being set
/// to `usize::MAX`, which no valid in-degree can ever reach.
const CONSUMED: USize = USize(usize::MAX); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sentinel definition; rust grammar requires raw usize literal here; tracked: #72

/// Step 2: topological sort via Kahn's algorithm.
///
/// Returns the units in topo order and the count of units that were
/// placed. The placed-count is the cycle-detection signal: when
/// `placed < graph.unit_count`, the input contains a cycle. The
/// runner (`compute_execution_plan`) is responsible for translating
/// that into `PlanError::Cycle`. Trailing entries in the returned
/// array (indices `placed..MAX_UNITS`) are left as `UnitId::ZERO`
/// (the array's initial fill); they are NOT the cycle members. The
/// caller must use the placed count to slice the valid prefix.
pub fn topo_sort<const MAX_UNITS: Cap, const MAX_EDGES: Cap>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
) -> ([UnitId; cap_size(MAX_UNITS)], USize)
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_EDGES)]:,
{
    let mut out: [UnitId; cap_size(MAX_UNITS)] = [UnitId::ZERO; cap_size(MAX_UNITS)];
    let n = graph.unit_count.0;
    if n == 0 {
        return (out, USize::ZERO);
    }
    // In-degree counter.
    let mut in_degree: [USize; cap_size(MAX_UNITS)] = [USize::ZERO; cap_size(MAX_UNITS)];
    let mut e = 0;
    while e < graph.edge_count.0 {
        let d = graph.col_indices[e].index().0;
        if d < cap_size(MAX_UNITS) {
            in_degree[d] = USize(in_degree[d].0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
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
            if in_degree[i].0 == 0 {
                let id = UnitId::from_index(USize(i));
                out[placed] = id;
                placed += 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal cursor increment; tracked: #72
                in_degree[i] = CONSUMED;
                progress = true;
                // Decrement successors of unit `i`.
                let start = graph.row_offsets[i].0;
                let end_excl = graph.end_for(i);
                let mut k = start;
                while k < end_excl {
                    let d = graph.col_indices[k].index().0;
                    if d < cap_size(MAX_UNITS) && in_degree[d].0 != CONSUMED.0 && in_degree[d].0 > 0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sentinel + bound check on USize internal field; tracked: #72
                        in_degree[d] = USize(in_degree[d].0 - 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
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
/// A waist is a unit whose dispatch reduces the active set to a
/// narrow width; phases delimit at waists. The skeleton walks the
/// topo order and treats any unit with no fan-out edges as a waist,
/// emitting a phase boundary after it. Real bench-driven heuristics
/// land in a HILA-RUNTIME-C1 follow-up; this body produces a sane
/// default phase layout (one phase for simple pipelines, splits at
/// natural narrowing points).
pub fn compute_waists<const MAX_UNITS: Cap, const MAX_EDGES: Cap, const MAX_PHASES: Cap>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
    topo: &[UnitId; cap_size(MAX_UNITS)],
) -> PhaseBoundaries<MAX_PHASES>
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_EDGES)]:,
    [(); cap_size(MAX_PHASES)]:,
{
    let mut boundaries = PhaseBoundaries::<MAX_PHASES>::new();
    let n = graph.unit_count.0;
    if n == 0 {
        return boundaries;
    }
    // Phase 0 starts at unit 0 always.
    boundaries.boundaries[0] = USize::ZERO;
    boundaries.phase_count = USize(1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: at least one phase always; tracked: #72
    let mut i = 0;
    while i + 1 < n && boundaries.phase_count.0 < cap_size(MAX_PHASES) {
        let idx = topo[i].index().0;
        // Out-degree zero in topo order means this unit's output
        // funnels through nothing else; treat as a waist.
        if idx < cap_size(MAX_UNITS) && graph.out_degree(USize(idx)).0 == 0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            let next_phase = boundaries.phase_count.0;
            boundaries.boundaries[next_phase] = USize(i + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            boundaries.phase_count = USize(next_phase + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
        }
        i += 1;
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
pub fn rcm_reorder<const MAX_UNITS: Cap, const MAX_EDGES: Cap>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
) -> [UnitId; cap_size(MAX_UNITS)]
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_EDGES)]:,
{
    let csr = graph.to_csr_bidirectional();
    let order = rcm_reorder_via::<_, MAX_UNITS>(&csr);
    // Convert the arvo NodeId permutation back to the engine UnitId.
    let mut out: [UnitId; cap_size(MAX_UNITS)] = [UnitId::ZERO; cap_size(MAX_UNITS)];
    for (dst, src) in out.iter_mut().zip(order.iter()) {
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
pub fn block_diagonalise<const MAX_UNITS: Cap, const MAX_EDGES: Cap>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
) -> BlockPartition<MAX_UNITS>
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_EDGES)]:,
{
    let csr = graph.to_csr_bidirectional();
    let (block_count, block_of_unit) = block_diagonal_via::<_, MAX_UNITS>(&csr);
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
pub fn phase_trunk_counts<const MAX_UNITS: Cap, const MAX_PHASES: Cap>(
    partition: &BlockPartition<MAX_UNITS>,
    waists: &PhaseBoundaries<MAX_PHASES>,
    topo: &[UnitId; cap_size(MAX_UNITS)],
    unit_count: USize,
) -> [USize; cap_size(MAX_PHASES)]
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_PHASES)]:,
{
    let mut counts: [USize; cap_size(MAX_PHASES)] = [USize::ZERO; cap_size(MAX_PHASES)];
    let pc = waists.phase_count.0;
    let n = unit_count.0;
    let mut p = 0;
    while p < pc && p < cap_size(MAX_PHASES) {
        let start = waists.boundaries[p].0;
        // Phase p ends where phase p+1 starts, or at unit_count for the
        // last phase.
        let end = if p + 1 < pc { waists.boundaries[p + 1].0 } else { n };
        // Count distinct block ids in this phase, deduped through a
        // per-phase seen-flag array indexed by block id.
        let mut seen: [Bool; cap_size(MAX_UNITS)] = [Bool::FALSE; cap_size(MAX_UNITS)];
        let mut distinct = 0;
        let mut i = start;
        while i < end && i < cap_size(MAX_UNITS) {
            let unit_idx = topo[i].index().0;
            if unit_idx < cap_size(MAX_UNITS) {
                let block = partition.block_of_unit[unit_idx].0;
                if block < cap_size(MAX_UNITS) && !seen[block].0 {
                    seen[block] = Bool::TRUE;
                    distinct += 1;
                }
            }
            i += 1;
        }
        counts[p] = USize(distinct); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal count; tracked: #72
        p += 1;
    }
    counts
}

/// Step 6: spectral partitioning.
///
/// Builds the symmetric graph Laplacian over the bidirectional CSR
/// (`SymmetricLaplacian`) and runs arvo-spectral's `k_way_partition`
/// to assign each unit a fiber by spectral cut, with `K = MAX_FIBERS`.
/// Returns a `FiberGrouping` mapping each unit to its spectral
/// partition id.
///
/// The spectral-versus-greedy `group_fibers` (step 7) choice and the
/// projection onto trunk components land in later C1d slices; the
/// runner does not consume this output yet. `k_way_partition` operates
/// over the full `cap_size(MAX_UNITS)`; on a loose CSR the slack rows
/// are isolated and a live-node-count-aware spectral path is a
/// follow-up gated on the bench adopting spectral.
pub fn spectral_partition<const MAX_UNITS: Cap, const MAX_EDGES: Cap, const MAX_FIBERS: Cap>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
) -> FiberGrouping<MAX_UNITS, MAX_FIBERS>
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_EDGES)]:,
    [(); cap_size(MAX_FIBERS)]:,
{
    use hilavitkutin_api::FiberId;
    let csr = graph.to_csr_bidirectional();
    let lap: SymmetricLaplacian<MAX_UNITS, MAX_EDGES, SpectralFloat> =
        SymmetricLaplacian::new(&csr);
    let sigma = lap.lambda_max_bound();
    let iterations = USize(100); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: spectral power-iteration count; tracked: #72
    let (count, partition) =
        k_way_partition::<_, MAX_UNITS, MAX_FIBERS, SpectralFloat>(&lap, sigma, iterations);
    let mut grouping: FiberGrouping<MAX_UNITS, MAX_FIBERS> = FiberGrouping::new();
    grouping.fiber_count = count;
    // Map each unit's spectral partition id to its fiber.
    for (slot, part) in grouping.assignment.iter_mut().zip(partition.iter()) {
        *slot = FiberId::from_index(*part);
    }
    grouping
}

/// Step 7: greedy fiber grouping.
///
/// Assigns each unit to a fiber such that fibers respect topo order
/// and stay within the consumer's MAX_FIBERS cap. The skeleton walks
/// the topo order and emits one fiber per leaf chain (a maximal
/// chain of units where each has exactly one in-degree and one out-
/// degree). Real heuristics (matrix-chain DP for non-trivial branch
/// merging) land in HILA-RUNTIME-C1.
pub fn group_fibers<const MAX_UNITS: Cap, const MAX_EDGES: Cap, const MAX_FIBERS: Cap>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
    topo: &[UnitId; cap_size(MAX_UNITS)],
) -> FiberGrouping<MAX_UNITS, MAX_FIBERS>
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_EDGES)]:,
{
    use hilavitkutin_api::FiberId;
    let mut g: FiberGrouping<MAX_UNITS, MAX_FIBERS> = FiberGrouping::new();
    let n = graph.unit_count.0;
    if n == 0 {
        return g;
    }
    let mut current_fiber: usize = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal counter; tracked: #72
    // Track which fiber actually received the last assignment so the
    // final count reflects fibers used, not fibers reached. The prior
    // shape used `current_fiber + 1` directly, which over-counted by
    // one whenever the last unit's out-degree triggered a roll-over
    // (e.g. a single-unit pipeline with no successor still tripped
    // the `out_deg != 1` branch).
    let mut max_used_fiber: usize = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal counter; tracked: #72
    let mut any_assigned = false;
    let mut i = 0;
    while i < n {
        let idx = topo[i].index().0;
        if idx < cap_size(MAX_UNITS) {
            let fid = FiberId::from_index(USize(current_fiber));
            g.assignment[idx] = fid;
            max_used_fiber = current_fiber;
            any_assigned = true;
            // Roll over to a new fiber whenever the unit's out-degree
            // is more than 1 (branching) or zero (leaf); single
            // chains pack into one fiber.
            let out_deg = graph.out_degree(USize(idx)).0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            if out_deg != 1 && current_fiber + 1 < cap_size(MAX_FIBERS) {
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
fn group_fibers_in_block<const MAX_UNITS: Cap, const MAX_EDGES: Cap, const MAX_FIBERS: Cap>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
    block_units: &[UnitId],
) -> FiberGrouping<MAX_UNITS, MAX_FIBERS>
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_EDGES)]:,
{
    use hilavitkutin_api::FiberId;
    let mut g: FiberGrouping<MAX_UNITS, MAX_FIBERS> = FiberGrouping::new();
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
        if idx < cap_size(MAX_UNITS) {
            g.assignment[idx] = FiberId::from_index(USize(current_fiber)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            max_used_fiber = current_fiber;
            any_assigned = true;
            let out_deg = graph.out_degree(USize(idx)).0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            if out_deg != 1 && current_fiber + 1 < cap_size(MAX_FIBERS) {
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
fn spectral_grouping_in_block<const MAX_UNITS: Cap, const MAX_FIBERS: Cap>(
    global: &FiberGrouping<MAX_UNITS, MAX_FIBERS>,
    block_units: &[UnitId],
) -> FiberGrouping<MAX_UNITS, MAX_FIBERS>
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_FIBERS)]:,
{
    use hilavitkutin_api::FiberId;
    let mut g: FiberGrouping<MAX_UNITS, MAX_FIBERS> = FiberGrouping::new();
    // Remap global spectral id -> block-local id in first-seen order.
    let mut remap: [USize; cap_size(MAX_FIBERS)] = [USize::ZERO; cap_size(MAX_FIBERS)];
    let mut seen: [Bool; cap_size(MAX_FIBERS)] = [Bool::FALSE; cap_size(MAX_FIBERS)];
    let mut local_count = 0;
    let mut i = 0;
    while i < block_units.len() {
        let uidx = block_units[i].index().0;
        if uidx < cap_size(MAX_UNITS) {
            let gid = global.assignment[uidx].index().0;
            if gid < cap_size(MAX_FIBERS) {
                if !seen[gid].0 {
                    seen[gid] = Bool::TRUE;
                    remap[gid] = USize(local_count); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
                    local_count += 1;
                }
                g.assignment[uidx] = FiberId::from_index(remap[gid]);
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
pub fn project_fiber_components<
    const MAX_UNITS: Cap,
    const MAX_EDGES: Cap,
    const MAX_FIBERS: Cap,
    const MAX_PHASES: Cap,
    const MAX_TRUNKS_PER_PHASE: Cap,
    const MAX_COMPONENTS_PER_TRUNK: Cap,
    const MAX_UNITS_PER_FIBER: Cap,
    const MAX_COLUMNS_PER_FIBER: Cap,
>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
    partition: &BlockPartition<MAX_UNITS>,
    waists: &PhaseBoundaries<MAX_PHASES>,
    topo: &[UnitId; cap_size(MAX_UNITS)],
    unit_count: USize,
) -> [[Trunk<MAX_COMPONENTS_PER_TRUNK, MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>;
        cap_size(MAX_TRUNKS_PER_PHASE)]; cap_size(MAX_PHASES)]
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_EDGES)]:,
    [(); cap_size(MAX_FIBERS)]:,
    [(); cap_size(MAX_PHASES)]:,
    [(); cap_size(MAX_TRUNKS_PER_PHASE)]:,
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    use hilavitkutin_api::FiberId;
    let mut out: [[Trunk<MAX_COMPONENTS_PER_TRUNK, MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>;
        cap_size(MAX_TRUNKS_PER_PHASE)]; cap_size(MAX_PHASES)] =
        [[Trunk::new(); cap_size(MAX_TRUNKS_PER_PHASE)]; cap_size(MAX_PHASES)];
    let pc = waists.phase_count.0;
    let n = unit_count.0;
    let mut next_trunk_id = 0;
    let mut next_fiber_id = 0;
    // Global spectral grouping, computed once and filtered per wide block
    // by the width-gate below. It respects block boundaries, so a wide
    // block's units carry a self-contained partition. Computed
    // unconditionally for now; skipping it when no block is wide is a
    // follow-up optimisation.
    let spectral = spectral_partition::<MAX_UNITS, MAX_EDGES, MAX_FIBERS>(graph);
    let mut p = 0;
    while p < pc && p < cap_size(MAX_PHASES) {
        let start = waists.boundaries[p].0;
        let end = if p + 1 < pc { waists.boundaries[p + 1].0 } else { n };
        // Map block id -> trunk index within the phase, first-seen order.
        let mut block_to_trunk: [USize; cap_size(MAX_UNITS)] =
            [USize::ZERO; cap_size(MAX_UNITS)];
        let mut block_seen: [Bool; cap_size(MAX_UNITS)] = [Bool::FALSE; cap_size(MAX_UNITS)];
        let mut trunk_count = 0;
        let mut i = start;
        while i < end && i < cap_size(MAX_UNITS) {
            let unit_idx = topo[i].index().0;
            if unit_idx < cap_size(MAX_UNITS) {
                let block = partition.block_of_unit[unit_idx].0;
                if block < cap_size(MAX_UNITS) && !block_seen[block].0 {
                    block_seen[block] = Bool::TRUE;
                    if trunk_count < cap_size(MAX_TRUNKS_PER_PHASE) {
                        block_to_trunk[block] = USize(trunk_count); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
                        trunk_count += 1;
                    }
                }
            }
            i += 1;
        }
        // For each trunk in the phase, form fibers within its block.
        let mut t = 0;
        while t < trunk_count && t < cap_size(MAX_TRUNKS_PER_PHASE) {
            // Gather this trunk's block units in topo order.
            let mut block_units: [UnitId; cap_size(MAX_UNITS)] =
                [UnitId::ZERO; cap_size(MAX_UNITS)];
            let mut bu_count = 0;
            let mut j = start;
            while j < end && j < cap_size(MAX_UNITS) {
                let unit_idx = topo[j].index().0;
                if unit_idx < cap_size(MAX_UNITS) {
                    let block = partition.block_of_unit[unit_idx].0;
                    if block < cap_size(MAX_UNITS)
                        && block_seen[block].0
                        && block_to_trunk[block].0 == t
                        && bu_count < cap_size(MAX_UNITS)
                    {
                        block_units[bu_count] = topo[j];
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
                spectral_grouping_in_block::<MAX_UNITS, MAX_FIBERS>(
                    &spectral,
                    &block_units[0..bu_count],
                )
            } else {
                group_fibers_in_block::<MAX_UNITS, MAX_EDGES, MAX_FIBERS>(
                    graph,
                    &block_units[0..bu_count],
                )
            };
            out[p][t].id = TrunkId::from_index(USize(next_trunk_id)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from plan-wide id; tracked: #72
            next_trunk_id += 1;
            // Emit one Fiber component per block-local fiber.
            let fc = grouping.fiber_count.0;
            let mut local_fid = 0;
            let mut comp_count = 0;
            while local_fid < fc && comp_count < cap_size(MAX_COMPONENTS_PER_TRUNK) {
                let mut fib: Fiber<MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER> = Fiber::new();
                let mut fu = 0;
                let mut k = 0;
                while k < bu_count {
                    let uidx = block_units[k].index().0;
                    if uidx < cap_size(MAX_UNITS)
                        && grouping.assignment[uidx].index().0 == local_fid
                        && fu < cap_size(MAX_UNITS_PER_FIBER)
                    {
                        fib.units[fu] = block_units[k];
                        fu += 1;
                    }
                    k += 1;
                }
                if fu > 0 {
                    fib.id = FiberId::from_index(USize(next_fiber_id)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from plan-wide id; tracked: #72
                    next_fiber_id += 1;
                    fib.unit_count = USize(fu); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal count; tracked: #72
                    out[p][t].components[comp_count] = TrunkComponent::Fiber(fib);
                    comp_count += 1;
                }
                local_fid += 1;
            }
            out[p][t].component_count = USize(comp_count); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal count; tracked: #72
            t += 1;
        }
        p += 1;
    }
    out
}

/// Reconstruct a global per-unit `FiberGrouping` from populated trunks.
///
/// Walks every trunk's `TrunkComponent::Fiber` and records each unit's
/// plan-wide `FiberId`, so steps 8 to 11 keep consuming a
/// `FiberGrouping` unchanged after the per-block projection.
pub fn fiber_grouping_from_trunks<
    const MAX_UNITS: Cap,
    const MAX_FIBERS: Cap,
    const MAX_PHASES: Cap,
    const MAX_TRUNKS_PER_PHASE: Cap,
    const MAX_COMPONENTS_PER_TRUNK: Cap,
    const MAX_UNITS_PER_FIBER: Cap,
    const MAX_COLUMNS_PER_FIBER: Cap,
>(
    trunks: &[[Trunk<MAX_COMPONENTS_PER_TRUNK, MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>;
        cap_size(MAX_TRUNKS_PER_PHASE)]; cap_size(MAX_PHASES)],
    trunk_counts: &[USize; cap_size(MAX_PHASES)],
    phase_count: USize,
) -> FiberGrouping<MAX_UNITS, MAX_FIBERS>
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_PHASES)]:,
    [(); cap_size(MAX_TRUNKS_PER_PHASE)]:,
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    let mut g: FiberGrouping<MAX_UNITS, MAX_FIBERS> = FiberGrouping::new();
    let pc = phase_count.0;
    let mut max_fid = 0;
    let mut any = false;
    let mut p = 0;
    while p < pc && p < cap_size(MAX_PHASES) {
        let tc = trunk_counts[p].0;
        let mut t = 0;
        while t < tc && t < cap_size(MAX_TRUNKS_PER_PHASE) {
            let cc = trunks[p][t].component_count.0;
            let mut c = 0;
            while c < cc && c < cap_size(MAX_COMPONENTS_PER_TRUNK) {
                if let TrunkComponent::Fiber(fib) = &trunks[p][t].components[c] {
                    let fid = fib.id.index().0;
                    let uc = fib.unit_count.0;
                    let mut u = 0;
                    while u < uc && u < cap_size(MAX_UNITS_PER_FIBER) {
                        let uidx = fib.units[u].index().0;
                        if uidx < cap_size(MAX_UNITS) {
                            g.assignment[uidx] = fib.id;
                            if fid > max_fid {
                                max_fid = fid;
                            }
                            any = true;
                        }
                        u += 1;
                    }
                }
                c += 1;
            }
            t += 1;
        }
        p += 1;
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
pub fn compute_upward_rank_and_dirty<
    const MAX_UNITS: Cap,
    const MAX_EDGES: Cap,
    const MAX_FIBERS: Cap,
    const MAX_STORES: Cap,
>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
    topo: &[UnitId; cap_size(MAX_UNITS)],
    inputs: &PlanInputs<MAX_UNITS, MAX_STORES>,
    fibers: &FiberGrouping<MAX_UNITS, MAX_FIBERS>,
) -> ([USize; cap_size(MAX_UNITS)], DirtyMasks<MAX_FIBERS, MAX_STORES>)
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_EDGES)]:,
    [(); cap_size(MAX_FIBERS)]:,
{
    let mut ranks: [USize; cap_size(MAX_UNITS)] = [USize::ZERO; cap_size(MAX_UNITS)];
    let mut dirty: DirtyMasks<MAX_FIBERS, MAX_STORES> = DirtyMasks::new();
    let n = graph.unit_count.0;
    if n == 0 {
        return (ranks, dirty);
    }
    // Reverse-topo walk: leaves get rank 0; predecessors take max
    // successor rank + 1.
    let mut i = n;
    while i > 0 {
        i -= 1;
        let u = topo[i].index().0;
        if u >= cap_size(MAX_UNITS) || u >= graph.unit_count.0 {
            continue;
        }
        // Scan successors for max rank.
        let start = graph.row_offsets[u].0;
        let end_excl = if u + 1 < graph.unit_count.0 {
            graph.row_offsets[u + 1].0
        } else {
            graph.edge_count.0
        };
        let mut max_rank = USize::ZERO;
        let mut k = start;
        while k < end_excl {
            let d = graph.col_indices[k].index().0;
            if d < cap_size(MAX_UNITS) && ranks[d].0 + 1 > max_rank.0 {
                max_rank = USize(ranks[d].0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
            }
            k += 1;
        }
        ranks[u] = max_rank;
        // Dirty propagation: union unit's writes into its fiber's
        // dirty mask. Fiber-level dirty drives incremental-skip.
        if u < inputs.unit_count.0 {
            let f = fibers.assignment[u].index().0;
            if f < cap_size(MAX_FIBERS) {
                let mut store = 0;
                while store < cap_size(MAX_STORES) && store < 64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: AccessMask uses USize backing with 64-bit window per skeleton; tracked: #72
                    if inputs.writes[u].contains(USize(store)).0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
                        dirty.per_fiber[f] = dirty.per_fiber[f].set(USize(store)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
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
pub fn size_morsels<const MAX_FIBERS: Cap>(
    record_count: USize,
    fiber_count: USize,
) -> [USize; cap_size(MAX_FIBERS)]
where
    [(); cap_size(MAX_FIBERS)]:,
{
    let mut sizes: [USize; cap_size(MAX_FIBERS)] = [USize::ZERO; cap_size(MAX_FIBERS)];
    // Divide-by-zero guard: fiber_count of zero falls back to 1 so
    // the division below is defined. The plan-stage runner only calls
    // this when fiber_count >= 1, but the guard makes the function
    // self-contained.
    let n = if fiber_count.0 == 0 { 1 } else { fiber_count.0 }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: divide-by-zero guard literal; tracked: #72
    let per_fiber = record_count.0 / n;
    let remainder = record_count.0 % n;
    // Distribute the remainder across the first `remainder` fibers.
    // Sum invariant: sum(sizes[0..n]) == record_count. Without this
    // every record past `per_fiber * n` was silently dropped, which
    // would propagate as a missing morsel range at dispatch time.
    let mut i = 0;
    while i < n && i < cap_size(MAX_FIBERS) {
        let extra = if i < remainder { 1 } else { 0 }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: remainder-distribution literal; tracked: #72
        sizes[i] = USize(per_fiber + extra); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal compute; tracked: #72
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
pub fn select_phase_configs<const MAX_PHASES: Cap>(
    phases: &PhaseBoundaries<MAX_PHASES>,
    record_count: USize,
    unit_count: USize,
) -> [PhaseConfig; cap_size(MAX_PHASES)]
where
    [(); cap_size(MAX_PHASES)]:,
{
    let mut configs: [PhaseConfig; cap_size(MAX_PHASES)] =
        [PhaseConfig::Balanced; cap_size(MAX_PHASES)];
    let n = phases.phase_count.0;
    let mut i = 0;
    while i < n && i < cap_size(MAX_PHASES) {
        // Compute the width of this phase (units it spans).
        let start = phases.boundaries[i].0;
        let end_excl = if i + 1 < n {
            phases.boundaries[i + 1].0
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
        configs[i] = if record_count.0 < SMALL_RECORD_COUNT_THRESHOLD
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
pub fn classify_columns<
    const MAX_UNITS: Cap,
    const MAX_FIBERS: Cap,
    const MAX_COLUMNS_PER_FIBER: Cap,
    const MAX_STORES: Cap,
>(
    fibers: &FiberGrouping<MAX_UNITS, MAX_FIBERS>,
    inputs: &PlanInputs<MAX_UNITS, MAX_STORES>,
) -> ColumnClassMap<MAX_FIBERS, MAX_COLUMNS_PER_FIBER>
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_FIBERS)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    let mut map: ColumnClassMap<MAX_FIBERS, MAX_COLUMNS_PER_FIBER> = ColumnClassMap::new();
    let n_fibers = fibers.fiber_count.0;
    let n_units = inputs.unit_count.0;
    // First pass: collect each fiber's touched stores into its
    // column slot list. We treat each touched store as `Internal`
    // initially; the upgrade-to-Input/Output pass would compare
    // across-fiber overlap. The conservative default is sound: it
    // produces correct dispatch shape, just misses some dead-store-
    // elimination opportunities.
    let mut u = 0;
    while u < n_units {
        let f = fibers.assignment[u].index().0;
        if f < cap_size(MAX_FIBERS) && f < n_fibers {
            // Walk this unit's access mask, register touched stores
            // as columns for fiber f.
            let mut store = 0;
            while store < cap_size(MAX_STORES) && store < 64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: AccessMask 64-bit window per skeleton; tracked: #72
                if inputs.access[u].contains(USize(store)).0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
                    let slot = map.column_count[f].0;
                    if slot < cap_size(MAX_COLUMNS_PER_FIBER) {
                        map.class[f][slot] = ColumnClassification::Internal;
                        map.column_count[f] = USize(slot + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
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
/// consistency. Actual signature parameterised on the 10-const-
/// generic ExecutionPlan there.
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
    /// `group_fibers` produced more fibers than `MAX_FIBERS`
    /// accommodates, or zero fibers for a non-empty unit set.
    NoTrunkAssignment,
    /// `size_morsels` produced a morsel size below the engine's
    /// hardcoded minimum (1 record).
    MorselSizeBelowMin,
    /// `assign_cores` was asked to map more lanes than the runtime
    /// has cores available.
    CoreCountExceeded,
}

