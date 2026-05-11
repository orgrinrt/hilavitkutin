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
//! 4. `rcm_reorder`: Reverse Cuthill-McKee bandwidth-reduction. *Stub*.
//! 5. `block_diagonalise`: Dulmage-Mendelsohn block detection. *Stub*.
//! 6. `spectral_partition`: spectral clustering for wide pipelines. *Stub*.
//! 7. `group_fibers`: greedy fiber assignment with bounded slack.
//! 8. `compute_upward_rank_and_dirty` (fused per Topic 3 S5):
//!    reverse-topo critical-path rank + per-fiber dirty propagation.
//! 9. `size_morsels`: per-fiber morsel sizing based on record count.
//! 10. `select_phase_configs`: pick MaxFuse/Balanced/MaxSplit per phase.
//! 11. `classify_columns`: per-fiber column role (Internal/Input/Output).
//! 12. `assign_cores`: map trunks onto concrete cores by `CoreClass`.
//! 13. `synthesise_core_programs`: per-core projection from plan.
//!
//! Steps 4 to 6 are stubs awaiting arvo-graph and arvo-spectral primitives:
//! their bodies depend on
//! arvo-graph / arvo-spectral primitives that have not yet shipped
//! the analytical helpers this engine needs. They stub `todo!()` with
//! BACKLOG entries (HILA-RUNTIME-C1 follow-up rounds).
//!
//! Steps 13 ships its body in a follow-up commit alongside
//! `plan/core_program.rs` (Pass 3 codegen feeds it).

use arvo::strategy::Identity;
use arvo::{Bool, USize};

use hilavitkutin_api::UnitId;

use super::column::{ColumnClassMap, ColumnClassification};
use super::dirty::{DirtyMask, DirtyMasks};
use super::fiber::FiberGrouping;
use super::graph::{DependencyGraph, EdgeKind};
use super::inputs::PlanInputs;
use super::phase::{PhaseBoundaries, PhaseConfig};

/// Step 1: build the CSR `DependencyGraph` from `AccessMask` overlap.
///
/// For each pair of units `(i, j)` with `i < j` in input order: if
/// `j`'s reads overlap `i`'s writes (RAW), append a `Read` edge
/// `i j`; if `j`'s writes overlap `i`'s writes (WAW), append a
/// `Write` edge. The CSR append-order invariant is preserved because
/// the outer loop walks `i` in ascending order.
pub fn build_dag<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_STORES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_EDGES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    inputs: &PlanInputs<MAX_UNITS, MAX_STORES>,
) -> DependencyGraph<MAX_UNITS, MAX_EDGES> {
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
    while g.unit_count.0 < n && g.unit_count.0 < MAX_UNITS {
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
pub fn topo_sort<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_EDGES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
) -> ([UnitId; MAX_UNITS], USize) {
    let mut out: [UnitId; MAX_UNITS] = [UnitId::ZERO; MAX_UNITS];
    let n = graph.unit_count.0;
    if n == 0 {
        return (out, USize::ZERO);
    }
    // In-degree counter.
    let mut in_degree: [USize; MAX_UNITS] = [USize::ZERO; MAX_UNITS];
    let mut e = 0;
    while e < graph.edge_count.0 {
        let dest_raw: u32 = unsafe { core::mem::transmute_copy(&graph.col_indices[e]) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout UnitId chain; tracked: #428
        let d = dest_raw as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging projection to usize index; tracked: #428
        if d < MAX_UNITS {
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
                let id_raw = i as u32; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging usize to u32 for repr(transparent) projection; tracked: #428
                let id: UnitId = unsafe { core::mem::transmute_copy(&id_raw) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout UnitId chain; tracked: #428
                out[placed] = id;
                placed += 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal cursor increment; tracked: #72
                in_degree[i] = CONSUMED;
                progress = true;
                // Decrement successors of unit `i`.
                let start = graph.row_offsets[i].0;
                let end_excl = graph.end_for(i);
                let mut k = start;
                while k < end_excl {
                    let dest_raw: u32 = unsafe { core::mem::transmute_copy(&graph.col_indices[k]) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout UnitId chain; tracked: #428
                    let d = dest_raw as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging projection to usize index; tracked: #428
                    if d < MAX_UNITS && in_degree[d].0 != CONSUMED.0 && in_degree[d].0 > 0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sentinel + bound check on USize internal field; tracked: #72
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
pub fn compute_waists<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_EDGES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_PHASES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
    topo: &[UnitId; MAX_UNITS],
) -> PhaseBoundaries<MAX_PHASES> {
    let mut boundaries = PhaseBoundaries::<MAX_PHASES>::new();
    let n = graph.unit_count.0;
    if n == 0 {
        return boundaries;
    }
    // Phase 0 starts at unit 0 always.
    boundaries.boundaries[0] = USize::ZERO;
    boundaries.phase_count = USize(1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: at least one phase always; tracked: #72
    let mut i = 0;
    while i + 1 < n && boundaries.phase_count.0 < MAX_PHASES {
        let raw: u32 = unsafe { core::mem::transmute_copy(&topo[i]) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout UnitId chain; tracked: #428
        let idx = raw as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging projection to usize index; tracked: #428
        // Out-degree zero in topo order means this unit's output
        // funnels through nothing else; treat as a waist.
        if idx < MAX_UNITS && graph.out_degree(USize(idx)).0 == 0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
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
/// Substrate-heavy stub: real body requires arvo-graph's banded-
/// matrix utilities + the Cuthill-McKee BFS variant. Tracked as
/// HILA-RUNTIME-C1 follow-up. Returns the topo order unchanged for
/// pipelines that don't need the reorder.
pub fn rcm_reorder<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_EDGES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    _graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
    topo: &[UnitId; MAX_UNITS],
) -> [UnitId; MAX_UNITS] {
    // Pass-through stub: ship topo unchanged. Real reordering lands
    // when arvo-graph provides the banded-matrix support.
    *topo
}

/// Step 5: Dulmage-Mendelsohn block diagonalisation.
///
/// Substrate-heavy stub: block-detection + cross-phase validation.
/// Tracked as HILA-RUNTIME-C1 follow-up. Returns `Bool::TRUE` to
/// signal "shape accepted as-is" so the chain proceeds.
pub fn block_diagonalise<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_EDGES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_PHASES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    _graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
    _phases: &PhaseBoundaries<MAX_PHASES>,
) -> Bool {
    Bool::TRUE
}

/// Step 6: spectral partitioning for wide pipelines.
///
/// Substrate-heavy stub: real body requires arvo-spectral's
/// eigenvalue solver for the Laplacian + Fiedler-vector clustering.
/// Tracked as HILA-RUNTIME-C1 follow-up. For now defers to
/// `group_fibers` (step 7) for the actual grouping; this step does
/// not contribute a useful intermediate.
pub fn spectral_partition<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_EDGES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_FIBERS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    _graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
) -> FiberGrouping<MAX_UNITS, MAX_FIBERS> {
    FiberGrouping::new()
}

/// Step 7: greedy fiber grouping.
///
/// Assigns each unit to a fiber such that fibers respect topo order
/// and stay within the consumer's MAX_FIBERS cap. The skeleton walks
/// the topo order and emits one fiber per leaf chain (a maximal
/// chain of units where each has exactly one in-degree and one out-
/// degree). Real heuristics (matrix-chain DP for non-trivial branch
/// merging) land in HILA-RUNTIME-C1.
pub fn group_fibers<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_EDGES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_FIBERS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
    topo: &[UnitId; MAX_UNITS],
) -> FiberGrouping<MAX_UNITS, MAX_FIBERS> {
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
        let raw: u32 = unsafe { core::mem::transmute_copy(&topo[i]) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout UnitId chain; tracked: #428
        let idx = raw as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging projection to usize index; tracked: #428
        if idx < MAX_UNITS {
            let fid_raw = current_fiber as u16; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging usize to u16 for repr(transparent) projection; tracked: #428
            let fid: FiberId = unsafe { core::mem::transmute_copy(&fid_raw) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout FiberId chain; tracked: #428
            g.assignment[idx] = fid;
            max_used_fiber = current_fiber;
            any_assigned = true;
            // Roll over to a new fiber whenever the unit's out-degree
            // is more than 1 (branching) or zero (leaf); single
            // chains pack into one fiber.
            let out_deg = graph.out_degree(USize(idx)).0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            if out_deg != 1 && current_fiber + 1 < MAX_FIBERS {
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

/// Step 8 (fused, per Topic 3 S5 / P1.5): upward rank + dirty
/// propagation in a single reverse-topo walk.
///
/// Upward rank is the longest path from a unit to any sink. Dirty
/// masks track which stores changed since the last frame on a per-
/// fiber basis. Both walk the same data in reverse-topo order; fusion
/// avoids two passes over the unit set.
pub fn compute_upward_rank_and_dirty<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_EDGES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_FIBERS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_STORES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
    topo: &[UnitId; MAX_UNITS],
    inputs: &PlanInputs<MAX_UNITS, MAX_STORES>,
    fibers: &FiberGrouping<MAX_UNITS, MAX_FIBERS>,
) -> ([USize; MAX_UNITS], DirtyMasks<MAX_FIBERS, MAX_STORES>) {
    let mut ranks: [USize; MAX_UNITS] = [USize::ZERO; MAX_UNITS];
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
        let raw: u32 = unsafe { core::mem::transmute_copy(&topo[i]) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout UnitId chain; tracked: #428
        let u = raw as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging projection to usize index; tracked: #428
        if u >= MAX_UNITS || u >= graph.unit_count.0 {
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
            let dest_raw: u32 = unsafe { core::mem::transmute_copy(&graph.col_indices[k]) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout UnitId chain; tracked: #428
            let d = dest_raw as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging projection to usize index; tracked: #428
            if d < MAX_UNITS && ranks[d].0 + 1 > max_rank.0 {
                max_rank = USize(ranks[d].0 + 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal; tracked: #72
            }
            k += 1;
        }
        ranks[u] = max_rank;
        // Dirty propagation: union unit's writes into its fiber's
        // dirty mask. Fiber-level dirty drives incremental-skip.
        if u < inputs.unit_count.0 {
            let fid_raw: u16 = unsafe { core::mem::transmute_copy(&fibers.assignment[u]) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout FiberId chain; tracked: #428
            let f = fid_raw as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging projection to usize index; tracked: #428
            if f < MAX_FIBERS {
                let mut store = 0;
                while store < MAX_STORES && store < 64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: AccessMask uses USize backing with 64-bit window per skeleton; tracked: #72
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
/// distributes records, falling back to the record count itself when
/// only one fiber is active. Bench-driven SIMD-width-aware sizing
/// lands in HILA-RUNTIME-C1.
pub fn size_morsels<const MAX_FIBERS: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    record_count: USize,
    fiber_count: USize,
) -> [USize; MAX_FIBERS] {
    let mut sizes: [USize; MAX_FIBERS] = [USize::ZERO; MAX_FIBERS];
    // Divide-by-zero guard: fiber_count of zero falls back to 1 so
    // the division below is defined. The plan-stage runner only calls
    // this when fiber_count >= 1, but the guard makes the function
    // self-contained.
    let n = if fiber_count.0 == 0 { 1 } else { fiber_count.0 }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: divide-by-zero guard literal; tracked: #72
    let per_fiber = record_count.0 / n;
    let mut i = 0;
    while i < n && i < MAX_FIBERS {
        sizes[i] = USize(per_fiber); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal compute; tracked: #72
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
pub fn select_phase_configs<const MAX_PHASES: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    phases: &PhaseBoundaries<MAX_PHASES>,
    record_count: USize,
    unit_count: USize,
) -> [PhaseConfig; MAX_PHASES] {
    let mut configs: [PhaseConfig; MAX_PHASES] = [PhaseConfig::Balanced; MAX_PHASES];
    let n = phases.phase_count.0;
    let mut i = 0;
    while i < n && i < MAX_PHASES {
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
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_FIBERS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COLUMNS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_STORES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    fibers: &FiberGrouping<MAX_UNITS, MAX_FIBERS>,
    inputs: &PlanInputs<MAX_UNITS, MAX_STORES>,
) -> ColumnClassMap<MAX_FIBERS, MAX_COLUMNS_PER_FIBER> {
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
        let fid_raw: u16 = unsafe { core::mem::transmute_copy(&fibers.assignment[u]) }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: repr(transparent) projection through guaranteed-layout FiberId chain; tracked: #428
        let f = fid_raw as usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bridging projection to usize index; tracked: #428
        if f < MAX_FIBERS && f < n_fibers {
            // Walk this unit's access mask, register touched stores
            // as columns for fiber f.
            let mut store = 0;
            while store < MAX_STORES && store < 64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: AccessMask 64-bit window per skeleton; tracked: #72
                if inputs.access[u].contains(USize(store)).0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
                    let slot = map.column_count[f].0;
                    if slot < MAX_COLUMNS_PER_FIBER {
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
    /// `block_diagonalise` returned `Bool::FALSE`: phase boundaries
    /// don't align with the unit shape.
    PhaseAlignmentMismatch,
    /// `block_diagonalise` returned `Bool::FALSE` for a deeper
    /// feasibility reason (matrix-chain DP found no valid grouping).
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

