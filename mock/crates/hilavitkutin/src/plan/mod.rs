//! Plan stage: pure analysis, no runtime state.
//!
//! Takes WU declarations (Read / Write AccessSets, scheduling hints,
//! COMMUTATIVE flag) and produces a complete `ExecutionPlan`.
//! Recomputes on any pipeline structure change (new WUs, record-count
//! change, DAG modification).
//!
//! `ExecutionPlan` carries ten const generics. The plan-wide caps
//! (MAX_UNITS / MAX_PHASES / MAX_TRUNKS / MAX_FIBERS / MAX_LANES /
//! MAX_COLUMNS) size the top-level arrays. The four per-aggregate
//! caps (MAX_COMPONENTS_PER_TRUNK / MAX_UNITS_PER_FIBER /
//! MAX_COLUMNS_PER_FIBER / MAX_TRUNKS_PER_PHASE) size the nested
//! structures. Per Topic 3 audit-2 m3, the per-aggregate caps are
//! their own const generics rather than CeilingDiv-derived: this
//! decouples per-fiber footprint from pipeline-wide caps.

use arvo::strategy::Identity;
use arvo::{Cap, USize};
use arvo_tensor::{cap, cap_size};

pub mod access;
pub mod column;
pub mod core_program;
pub mod dirty;
pub mod fiber;
pub mod graph;
pub mod inputs;
pub mod phase;
pub mod steps;
pub mod trunk;
pub mod unit;

pub use access::AccessMask;
pub use column::{ColumnClassMap, ColumnClassification};
pub use dirty::{DirtyMask, DirtyMasks};
pub use fiber::{
    AccumSlot, AccumType, Fiber, FiberGrouping, HeadTailConvergence, MergeOp,
};
pub use graph::{DependencyGraph, EdgeKind};
pub use inputs::PlanInputs;
pub use steps::PlanError;
pub use phase::{Phase, PhaseBoundaries, PhaseConfig};
pub use trunk::{Branch, Bridge, Trunk, TrunkComponent};
pub use unit::{CostTable, UnitMeta};

pub use hilavitkutin_api::{FiberId, PhaseId, TrunkId, UnitId};

/// Complete plan-stage output.
///
/// Frozen once computed; the dispatch stage walks it without
/// mutation. The mutable sibling `CostTable<MAX_UNITS>` lives
/// alongside and refreshes between frames.
#[derive(Copy, Clone, Debug)]
pub struct ExecutionPlan<
    const MAX_UNITS: Cap,
    const MAX_PHASES: Cap,
    const MAX_TRUNKS: Cap,
    const MAX_FIBERS: Cap,
    const MAX_LANES: Cap,
    const MAX_COLUMNS: Cap,
    const MAX_COMPONENTS_PER_TRUNK: Cap,
    const MAX_UNITS_PER_FIBER: Cap,
    const MAX_COLUMNS_PER_FIBER: Cap,
    const MAX_TRUNKS_PER_PHASE: Cap,
>
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_PHASES)]:,
    [(); cap_size(MAX_FIBERS)]:,
    [(); cap_size(MAX_TRUNKS_PER_PHASE)]:,
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    /// Waist-delimited phases (in dispatch order).
    pub phases: [Phase<
        MAX_TRUNKS_PER_PHASE,
        MAX_COMPONENTS_PER_TRUNK,
        MAX_UNITS_PER_FIBER,
        MAX_COLUMNS_PER_FIBER,
    >; cap_size(MAX_PHASES)],
    pub phase_count: USize,
    /// Per-unit metadata array, addressed by `UnitId`.
    pub unit_meta: [UnitMeta; cap_size(MAX_UNITS)],
    pub unit_count: USize,
    /// Per-fiber column classification.
    pub column_class: ColumnClassMap<MAX_FIBERS, MAX_COLUMNS_PER_FIBER>,
    /// Per-fiber dirty masks (incremental-skip propagation).
    pub dirty: DirtyMasks<MAX_FIBERS, MAX_COLUMNS>,
    /// Per-fiber morsel sizes. `morsel_sizes[f]` is the number of
    /// records assigned to fiber `f`. Sum-preserving across the full
    /// record set (remainder distributed across the first
    /// `record_count % fiber_count` fibers). Read by dispatch codegen
    /// to emit per-fiber `RecordRange` slices.
    pub morsel_sizes: [USize; cap_size(MAX_FIBERS)],
    /// RCM renumber permutation: `rcm_order[new_pos]` is the `UnitId`
    /// placed at that position by the step-4 bandwidth-reduction pass.
    /// A locality renumber consumed by dispatch codegen for arena
    /// layout, not the dispatch order (dispatch stays topological via
    /// `unit_meta`). Zero-filled before the chain populates it.
    pub rcm_order: [UnitId; cap_size(MAX_UNITS)],
}

impl<
        const MAX_UNITS: Cap,
        const MAX_PHASES: Cap,
        const MAX_TRUNKS: Cap,
        const MAX_FIBERS: Cap,
        const MAX_LANES: Cap,
        const MAX_COLUMNS: Cap,
        const MAX_COMPONENTS_PER_TRUNK: Cap,
        const MAX_UNITS_PER_FIBER: Cap,
        const MAX_COLUMNS_PER_FIBER: Cap,
        const MAX_TRUNKS_PER_PHASE: Cap,
    >
    ExecutionPlan<
        MAX_UNITS,
        MAX_PHASES,
        MAX_TRUNKS,
        MAX_FIBERS,
        MAX_LANES,
        MAX_COLUMNS,
        MAX_COMPONENTS_PER_TRUNK,
        MAX_UNITS_PER_FIBER,
        MAX_COLUMNS_PER_FIBER,
        MAX_TRUNKS_PER_PHASE,
    >
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_PHASES)]:,
    [(); cap_size(MAX_FIBERS)]:,
    [(); cap_size(MAX_TRUNKS_PER_PHASE)]:,
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    /// All-zero plan. Used as the default before the plan-stage
    /// chain populates real values, and as the constructor for
    /// `Default`.
    pub const fn new() -> Self {
        Self {
            phases: [Phase::new(); cap_size(MAX_PHASES)],
            phase_count: USize::ZERO,
            unit_meta: [UnitMeta::new(); cap_size(MAX_UNITS)],
            unit_count: USize::ZERO,
            column_class: ColumnClassMap::new(),
            dirty: DirtyMasks::new(),
            morsel_sizes: [USize::ZERO; cap_size(MAX_FIBERS)],
            rcm_order: [UnitId::ZERO; cap_size(MAX_UNITS)],
        }
    }
}

impl<
        const MAX_UNITS: Cap,
        const MAX_PHASES: Cap,
        const MAX_TRUNKS: Cap,
        const MAX_FIBERS: Cap,
        const MAX_LANES: Cap,
        const MAX_COLUMNS: Cap,
        const MAX_COMPONENTS_PER_TRUNK: Cap,
        const MAX_UNITS_PER_FIBER: Cap,
        const MAX_COLUMNS_PER_FIBER: Cap,
        const MAX_TRUNKS_PER_PHASE: Cap,
    > Default
    for ExecutionPlan<
        MAX_UNITS,
        MAX_PHASES,
        MAX_TRUNKS,
        MAX_FIBERS,
        MAX_LANES,
        MAX_COLUMNS,
        MAX_COMPONENTS_PER_TRUNK,
        MAX_UNITS_PER_FIBER,
        MAX_COLUMNS_PER_FIBER,
        MAX_TRUNKS_PER_PHASE,
    >
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_PHASES)]:,
    [(); cap_size(MAX_FIBERS)]:,
    [(); cap_size(MAX_TRUNKS_PER_PHASE)]:,
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    fn default() -> Self {
        Self::new()
    }
}

// Static `Send + Sync` assertion for `ExecutionPlan`. The plan crosses
// thread boundaries when Pass 3 hands `&ExecutionPlan` slices to per-core
// dispatch closures. Today every field auto-derives `Send + Sync` from
// its arvo-newtype leaves (USize, Bool, UnitId, FiberId, TrunkId, PhaseId)
// plus the nested plan types. The auto-impl is silent: a future field
// that introduces a raw pointer, interior mutability, or PhantomData of
// a non-Send/Sync type would break the bound without a load-bearing
// diagnostic. This monomorphised assertion forces a compile-time check
// against a representative instantiation.
const _: fn() = || {
    // The assertion is generic over the same const-dimension params as
    // `ExecutionPlan` and carries its array-length where-bounds, so the
    // `Send + Sync` check holds for any well-formed instantiation rather
    // than one concrete shape. The const params never bind to values
    // here; naming the function below forces the bound check.
    fn assert_send_sync<T: Send + Sync>() {}
    fn check<
        const MAX_UNITS: Cap,
        const MAX_PHASES: Cap,
        const MAX_TRUNKS: Cap,
        const MAX_FIBERS: Cap,
        const MAX_LANES: Cap,
        const MAX_COLUMNS: Cap,
        const MAX_COMPONENTS_PER_TRUNK: Cap,
        const MAX_UNITS_PER_FIBER: Cap,
        const MAX_COLUMNS_PER_FIBER: Cap,
        const MAX_TRUNKS_PER_PHASE: Cap,
    >()
    where
        [(); cap_size(MAX_UNITS)]:,
        [(); cap_size(MAX_PHASES)]:,
        [(); cap_size(MAX_FIBERS)]:,
        [(); cap_size(MAX_TRUNKS_PER_PHASE)]:,
        [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
        [(); cap_size(MAX_UNITS_PER_FIBER)]:,
        [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
    {
        // The const-eval array-length bounds are in scope here, so
        // naming the type to check `Send + Sync` is well-formed.
        assert_send_sync::<
            ExecutionPlan<
                MAX_UNITS,
                MAX_PHASES,
                MAX_TRUNKS,
                MAX_FIBERS,
                MAX_LANES,
                MAX_COLUMNS,
                MAX_COMPONENTS_PER_TRUNK,
                MAX_UNITS_PER_FIBER,
                MAX_COLUMNS_PER_FIBER,
                MAX_TRUNKS_PER_PHASE,
            >,
        >();
    }
    // Smallest meaningful instantiation; the bound is structural and
    // independent of the const-generic values. A named const is used
    // rather than inline `{ cap(1) }` because the const-eval normaliser
    // discharges the array-length bounds against a named `Cap` const but
    // not an inline construction in turbofish position.
    const ONE: Cap = cap(1);
    let _ = check::<ONE, ONE, ONE, ONE, ONE, ONE, ONE, ONE, ONE, ONE>;
};

/// Chain the 13 plan-stage steps and assemble an `ExecutionPlan`.
///
/// Walks the algorithm chain in order:
/// `build_dag` to `topo_sort` to `compute_waists` to (`rcm_reorder`,
/// `block_diagonalise`, `spectral_partition`. These depend on arvo-graph
/// and arvo-spectral primitives not yet shipped; their bodies are stubs)
/// to `group_fibers` to `compute_upward_rank_and_dirty` to
/// `size_morsels` to `select_phase_configs` to `classify_columns`.
/// Steps 12 (`assign_cores`) and 13 (`synthesise_core_programs`) run
/// in `crate::thread::assign_cores` and `plan/core_program.rs`
/// respectively; this runner produces the input they consume.
///
/// Returns `Outcome::Err(PlanError::Cycle)` when `topo_sort` fails to
/// place every input unit (cycle in the dependency graph), or other
/// `PlanError` variants for feasibility / size / core-count issues.
pub fn compute_execution_plan<
    const MAX_UNITS: Cap,
    const MAX_STORES: Cap,
    const MAX_EDGES: Cap,
    const MAX_PHASES: Cap,
    const MAX_TRUNKS: Cap,
    const MAX_FIBERS: Cap,
    const MAX_LANES: Cap,
    const MAX_COLUMNS: Cap,
    const MAX_COMPONENTS_PER_TRUNK: Cap,
    const MAX_UNITS_PER_FIBER: Cap,
    const MAX_COLUMNS_PER_FIBER: Cap,
    const MAX_TRUNKS_PER_PHASE: Cap,
>(
    inputs: &PlanInputs<MAX_UNITS, MAX_STORES>,
) -> notko::Outcome<
    ExecutionPlan<
        MAX_UNITS,
        MAX_PHASES,
        MAX_TRUNKS,
        MAX_FIBERS,
        MAX_LANES,
        MAX_COLUMNS,
        MAX_COMPONENTS_PER_TRUNK,
        MAX_UNITS_PER_FIBER,
        MAX_COLUMNS_PER_FIBER,
        MAX_TRUNKS_PER_PHASE,
    >,
    PlanError,
>
where
    [(); cap_size(MAX_UNITS)]:,
    [(); cap_size(MAX_STORES)]:,
    [(); cap_size(MAX_EDGES)]:,
    [(); cap_size(MAX_PHASES)]:,
    [(); cap_size(MAX_FIBERS)]:,
    [(); cap_size(MAX_TRUNKS_PER_PHASE)]:,
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS)]:,
{
    // Empty input → empty plan (valid).
    let n = inputs.unit_count.0;
    let mut plan: ExecutionPlan<
        MAX_UNITS,
        MAX_PHASES,
        MAX_TRUNKS,
        MAX_FIBERS,
        MAX_LANES,
        MAX_COLUMNS,
        MAX_COMPONENTS_PER_TRUNK,
        MAX_UNITS_PER_FIBER,
        MAX_COLUMNS_PER_FIBER,
        MAX_TRUNKS_PER_PHASE,
    > = ExecutionPlan::new();
    plan.unit_count = inputs.unit_count;
    if n == 0 {
        return notko::Outcome::Ok(plan);
    }

    // Step 1: build the DAG.
    let dag = steps::build_dag::<MAX_UNITS, MAX_STORES, MAX_EDGES>(inputs);

    // Step 2: topo sort with explicit placed-count for cycle detection.
    let (topo, topo_placed) = steps::topo_sort::<MAX_UNITS, MAX_EDGES>(&dag);
    // Cycle detection: when Kahn's iteration runs out of zero-in-degree
    // units before placing every unit, the remainder is a cycle. The
    // placed count is the canonical signal; UnitId::ZERO is a valid
    // index value and so an array-walk-for-defaults check is unsound.
    if topo_placed.0 < n {
        return notko::Outcome::Err(PlanError::Cycle);
    }

    // Step 3: phase boundaries from waist detection.
    let waists = steps::compute_waists::<MAX_UNITS, MAX_EDGES, MAX_PHASES>(&dag, &topo);
    plan.phase_count = waists.phase_count;

    // Step 4: RCM bandwidth-reduction reordering. Persisted on the
    // plan as the arena-layout renumber permutation (consumed by
    // dispatch codegen), distinct from the topological dispatch order
    // in `unit_meta`. Steps 5 to 6 (block-diagonal, spectral) remain
    // stubs tracked under HILA-RUNTIME-C1.
    plan.rcm_order = steps::rcm_reorder::<MAX_UNITS, MAX_EDGES>(&dag);
    let feasible = steps::block_diagonalise::<MAX_UNITS, MAX_EDGES, MAX_PHASES>(&dag, &waists);
    if !feasible.0 {
        return notko::Outcome::Err(PlanError::PhaseAlignmentMismatch);
    }
    let _clusters = steps::spectral_partition::<MAX_UNITS, MAX_EDGES, MAX_FIBERS>(&dag);

    // Step 7: fiber grouping.
    let fibers = steps::group_fibers::<MAX_UNITS, MAX_EDGES, MAX_FIBERS>(&dag, &topo);
    if fibers.fiber_count.0 == 0 && n > 0 {
        return notko::Outcome::Err(PlanError::NoTrunkAssignment);
    }

    // Step 8 (fused): upward rank + per-fiber dirty propagation.
    let (ranks, dirty) = steps::compute_upward_rank_and_dirty::<
        MAX_UNITS,
        MAX_EDGES,
        MAX_FIBERS,
        MAX_STORES,
    >(&dag, &topo, inputs, &fibers);
    // Stash a subset of the per-fiber dirty info onto the plan's
    // MAX_COLUMNS-shaped DirtyMasks. The compatibility cast assumes
    // MAX_STORES <= MAX_COLUMNS (typical); larger MAX_STORES would
    // need explicit truncation handled in a follow-up round.
    let mut f = 0;
    while f < cap_size(MAX_FIBERS) {
        // Reuse the same bit layout: DirtyMask::raw + manual restore.
        let raw = dirty.per_fiber[f].raw();
        // Move bits into the MAX_COLUMNS-shaped mask one by one.
        let mut store = 0;
        while store < cap_size(MAX_STORES) && store < 64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: AccessMask 64-bit window per skeleton; tracked: #72
            let bit = (raw.0 >> store) & 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bit extraction internal; tracked: #72
            if bit == 1 {
                plan.dirty.per_fiber[f] = plan.dirty.per_fiber[f].set(USize(store)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            }
            store += 1;
        }
        f += 1;
    }

    // Step 9: morsel sizing per fiber. Stored on the plan so Pass 3
    // dispatch codegen can emit per-fiber `RecordRange` slices without
    // recomputing.
    plan.morsel_sizes = steps::size_morsels::<MAX_FIBERS>(inputs.record_count, fibers.fiber_count);

    // Step 10: phase configs. Store onto plan.phases[i].config. Pass
    // the unit count so the last phase's width is computed against
    // the real range, not the prior `start + 1` lower-bound.
    let configs =
        steps::select_phase_configs::<MAX_PHASES>(&waists, inputs.record_count, inputs.unit_count);
    let mut i = 0;
    while i < plan.phase_count.0 && i < cap_size(MAX_PHASES) {
        plan.phases[i].config = configs[i];
        i += 1;
    }

    // Step 11: per-fiber column classification.
    plan.column_class = steps::classify_columns::<
        MAX_UNITS,
        MAX_FIBERS,
        MAX_COLUMNS_PER_FIBER,
        MAX_STORES,
    >(&fibers, inputs);

    // Populate the unit meta array with the topo order. `unit_meta` is
    // indexed by topo-position (matching the dispatch order), so each
    // slot's `id` is `topo[u]` and the per-unit fields are looked up
    // by the raw unit-id index that `topo[u]` projects to. `ranks` is
    // also unit-id-indexed; project once and read both.
    let mut u = 0;
    while u < n && u < cap_size(MAX_UNITS) {
        plan.unit_meta[u].id = topo[u];
        let unit_id_idx = topo[u].index().0;
        if unit_id_idx < cap_size(MAX_UNITS) {
            plan.unit_meta[u].commutative = inputs.commutative[unit_id_idx];
            plan.unit_meta[u].upward_rank = ranks[unit_id_idx];
        }
        u += 1;
    }

    // Steps 12 + 13 (core assignment + per-core program synthesis)
    // happen on the dispatch stage entry; not part of this runner.
    notko::Outcome::Ok(plan)
}
