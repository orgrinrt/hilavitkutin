//! Plan stage: pure analysis, no runtime state.
//!
//! Takes WU declarations (Read / Write AccessSets, scheduling hints,
//! COMMUTATIVE flag) and produces a complete `ExecutionPlan`.
//! Recomputes on any pipeline structure change (new WUs, record-count
//! change, DAG modification).
//!
//! `ExecutionPlan` is generic over one `D: PlanDims` bundling the
//! plan capacity dimensions. The plan-wide dimensions (units / phases /
//! trunks / fibers / lanes / columns) size the top-level arrays; the
//! per-aggregate dimensions (components-per-trunk / units-per-fiber /
//! columns-per-fiber / trunks-per-phase) size the nested structures.
//! Per Topic 3 audit-2 m3, the per-aggregate dimensions are their own
//! capacity types rather than CeilingDiv-derived: this decouples
//! per-fiber footprint from pipeline-wide caps.

use arvo::strategy::Identity;
use arvo::USize;
use arvo_bitmask::NodeId;
use arvo_tensor::Capacity;

pub mod access;
pub mod column;
pub mod core_program;
pub mod dims;
pub mod dirty;
pub mod fiber;
pub mod graph;
pub mod inputs;
pub mod laplacian;
pub mod phase;
pub mod project;
pub mod steps;
pub mod trunk;
pub mod unit;

pub use access::AccessMask;
pub use column::{ColumnClassMap, ColumnClassification};
pub use dims::{DefaultPlanDims, PlanDims};
pub use dirty::{DirtyMask, DirtyMasks};
pub use fiber::{
    AccumSlot, AccumType, Fiber, FiberGrouping, HeadTailConvergence, MergeOp,
};
pub use graph::{DependencyGraph, EdgeKind};
pub use inputs::PlanInputs;
pub use project::plan_inputs_from_bundle;
pub use steps::{FiberLayout, PlanError};
pub use phase::{Phase, PhaseBoundaries, PhaseConfig};
pub use trunk::{BlockPartition, Branch, Bridge, Trunk, TrunkComponent};
pub use unit::{CostTable, UnitMeta};

pub use hilavitkutin_api::{FiberId, PhaseId, TrunkId, UnitId};

/// Complete plan-stage output.
///
/// Frozen once computed; the dispatch stage walks it without
/// mutation. The mutable sibling `CostTable<D::Units>` lives
/// alongside and refreshes between frames.
pub struct ExecutionPlan<D: PlanDims> {
    /// Waist-delimited phases (in dispatch order). Each `Phase` carries a
    /// `(trunk_offset, trunk_count)` range into the flat `trunks` pool.
    pub phases: <D::Phases as Capacity>::Array<Phase>,
    pub phase_count: USize,
    /// Flat plan-wide trunk pool. Each `Trunk` carries a `(fiber_offset,
    /// fiber_count)` range into the flat `fibers` pool.
    pub trunks: <D::Trunks as Capacity>::Array<Trunk>,
    pub trunk_count: USize,
    /// Flat plan-wide fiber pool. The CSR flatten collapses the dense
    /// per-phase, per-trunk fiber nesting onto this single pool, sized by
    /// the plan-wide `D::Fibers` cap.
    pub fibers: <D::Fibers as Capacity>::Array<Fiber<D>>,
    pub fiber_count: USize,
    /// Per-unit metadata array, addressed by `UnitId`.
    pub unit_meta: <D::Units as Capacity>::Array<UnitMeta>,
    pub unit_count: USize,
    /// Per-fiber column classification.
    pub column_class: ColumnClassMap<D>,
    /// Per-fiber dirty masks (incremental-skip propagation).
    pub dirty: DirtyMasks<D::Fibers, D::Columns>,
    /// Per-fiber morsel sizes. `morsel_sizes[f]` is the number of
    /// records assigned to fiber `f`. Sum-preserving across the full
    /// record set (remainder distributed across the first
    /// `record_count % fiber_count` fibers). Read by dispatch codegen
    /// to emit per-fiber `RecordRange` slices.
    pub morsel_sizes: <D::Fibers as Capacity>::Array<USize>,
    /// RCM renumber permutation: `rcm_order[new_pos]` is the `UnitId`
    /// placed at that position by the step-4 bandwidth-reduction pass.
    /// A locality renumber consumed by dispatch codegen for arena
    /// layout, not the dispatch order (dispatch stays topological via
    /// `unit_meta`). Zero-filled before the chain populates it.
    pub rcm_order: <D::Units as Capacity>::Array<UnitId>,
}

impl<D: PlanDims> ExecutionPlan<D>
where
    Fiber<D>: Copy,
    <D::ColumnsPerFiber as Capacity>::Array<ColumnClassification>: Copy,
{
    /// All-zero plan. Used as the default before the plan-stage
    /// chain populates real values, and as the constructor for
    /// `Default`.
    pub fn new() -> Self {
        Self {
            phases: <D::Phases as Capacity>::filled(Phase::new()),
            phase_count: USize::ZERO,
            trunks: <D::Trunks as Capacity>::filled(Trunk::new()),
            trunk_count: USize::ZERO,
            fibers: <D::Fibers as Capacity>::filled(Fiber::new()),
            fiber_count: USize::ZERO,
            unit_meta: <D::Units as Capacity>::filled(UnitMeta::new()),
            unit_count: USize::ZERO,
            column_class: ColumnClassMap::new(),
            dirty: DirtyMasks::new(),
            morsel_sizes: <D::Fibers as Capacity>::filled(USize::ZERO),
            rcm_order: <D::Units as Capacity>::filled(UnitId::ZERO),
        }
    }
}

impl<D: PlanDims> Copy for ExecutionPlan<D>
where
    <D::Phases as Capacity>::Array<Phase>: Copy,
    <D::Trunks as Capacity>::Array<Trunk>: Copy,
    <D::Fibers as Capacity>::Array<Fiber<D>>: Copy,
    <D::Units as Capacity>::Array<UnitMeta>: Copy,
    ColumnClassMap<D>: Copy,
    DirtyMasks<D::Fibers, D::Columns>: Copy,
    <D::Fibers as Capacity>::Array<USize>: Copy,
    <D::Units as Capacity>::Array<UnitId>: Copy,
{
}

impl<D: PlanDims> Clone for ExecutionPlan<D>
where
    <D::Phases as Capacity>::Array<Phase>: Copy,
    <D::Trunks as Capacity>::Array<Trunk>: Copy,
    <D::Fibers as Capacity>::Array<Fiber<D>>: Copy,
    <D::Units as Capacity>::Array<UnitMeta>: Copy,
    ColumnClassMap<D>: Copy,
    DirtyMasks<D::Fibers, D::Columns>: Copy,
    <D::Fibers as Capacity>::Array<USize>: Copy,
    <D::Units as Capacity>::Array<UnitId>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: PlanDims> core::fmt::Debug for ExecutionPlan<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ExecutionPlan")
            .field("phase_count", &self.phase_count.0)
            .field("unit_count", &self.unit_count.0)
            .finish_non_exhaustive()
    }
}

impl<D: PlanDims> Default for ExecutionPlan<D>
where
    Fiber<D>: Copy,
    <D::ColumnsPerFiber as Capacity>::Array<ColumnClassification>: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

// Static `Send + Sync` assertion for `ExecutionPlan` over the default
// dimensions. The plan crosses thread boundaries when Pass 3 hands
// `&ExecutionPlan` slices to per-core dispatch closures. Today every
// field auto-derives `Send + Sync` from its arvo-newtype leaves (USize,
// Bool, UnitId, FiberId, TrunkId, PhaseId) plus the nested plan types.
// The auto-impl is silent: a future field that introduces a raw
// pointer, interior mutability, or PhantomData of a non-Send/Sync type
// would break the bound without a load-bearing diagnostic. This
// monomorphised assertion forces a compile-time check against the
// default dimension instantiation.
const _: fn() = || {
    fn assert_send_sync<T: Send + Sync>() {}
    let _ = assert_send_sync::<ExecutionPlan<DefaultPlanDims>>;
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
pub fn compute_execution_plan<D: PlanDims>(
    inputs: &PlanInputs<D::Units, D::Stores>,
) -> notko::Outcome<ExecutionPlan<D>, PlanError>
where
    <D::Trunks as Capacity>::Array<Trunk>: Copy,
    <D::Fibers as Capacity>::Array<Fiber<D>>: Copy,
    <D::ColumnsPerFiber as Capacity>::Array<ColumnClassification>: Copy,
    Fiber<D>: Copy,
    <D::Units as Capacity>::Array<SpectralFloatVec>: Copy,
    <D::Units as Capacity>::Array<USize>: Copy,
    // `compute_waists` runs `arvo_graph::waist_detect`, which needs the
    // per-node depth (`USize`) and waist-flag (`Bool`) scratch arrays `Copy`.
    <D::Units as Capacity>::Array<arvo::Bool>: Copy,
    <D::Edges as Capacity>::Array<NodeId>: Copy,
{
    // #641: a `PlanDims` whose phase or trunk capacity exceeds the fixed-width
    // id types' addressable range (`PhaseId` names 32, `TrunkId` names 64)
    // cannot name its high slots; reject the misconfigured dims loudly rather
    // than letting id construction wrap past the addressable range. The check
    // is a property of `D`, independent of the input, so it guards the empty
    // plan too. `DefaultPlanDims` aligns its capacities to the id widths, so
    // the guard never fires for it.
    if cap_size_phases::<D>() > PhaseId::ADDRESSABLE {
        return notko::Outcome::Err(PlanError::PhaseCapacityExceedsIdWidth);
    }
    if cap_size(<D::Trunks as Capacity>::CAP) > TrunkId::ADDRESSABLE {
        return notko::Outcome::Err(PlanError::TrunkCapacityExceedsIdWidth);
    }

    // Empty input → empty plan (valid).
    let n = inputs.unit_count.0;
    let mut plan: ExecutionPlan<D> = ExecutionPlan::new();
    plan.unit_count = inputs.unit_count;
    if n == 0 {
        return notko::Outcome::Ok(plan);
    }

    // Step 1: build the DAG.
    let dag = steps::build_dag::<D>(inputs);

    // Step 2: topo sort with explicit placed-count for cycle detection.
    let (topo, topo_placed) = steps::topo_sort::<D>(&dag);
    // Cycle detection: when Kahn's iteration runs out of zero-in-degree
    // units before placing every unit, the remainder is a cycle. The
    // placed count is the canonical signal; UnitId::ZERO is a valid
    // index value and so an array-walk-for-defaults check is unsound.
    if topo_placed.0 < n {
        return notko::Outcome::Err(PlanError::Cycle);
    }

    // Step 3: phase boundaries from waist detection.
    let waists = steps::compute_waists::<D>(&dag, &topo);
    plan.phase_count = waists.phase_count;

    // Step 4: RCM bandwidth-reduction reordering. Persisted on the
    // plan as the arena-layout renumber permutation (consumed by
    // dispatch codegen), distinct from the topological dispatch order
    // in `unit_meta`. Steps 5 to 6 (block-diagonal, spectral) remain
    // stubs tracked under HILA-RUNTIME-C1.
    plan.rcm_order = steps::rcm_reorder::<D>(&dag);
    // Step 5: connected-component block detection, projected to
    // per-phase trunk skeletons. Each block is a column-disjoint
    // independent sub-graph; within a phase, distinct blocks become
    // trunks (zero sync between trunks per Domain 11). The runner then
    // sets `Phase.id` / `trunk_count` / `trunk_offset` and the flat
    // `trunks` / `fibers` pools land in the fiber-formation projection
    // (steps 6 + 7). No alignment check
    // fires yet: distinct blocks are column-disjoint by construction, so
    // PhaseAlignmentMismatch has no concrete condition before column
    // classification.
    let partition = steps::block_diagonalise::<D>(&dag);

    // Steps 6 + 7 (per C1d): per-block fiber formation written into the
    // flat `trunks` / `fibers` pools. `project_fiber_components` owns the
    // plan-wide `Trunk` / `Fiber` ids and the CSR layout; the runner
    // copies the pools onto the plan, sets each phase's `(trunk_offset,
    // trunk_count)` range from the layout's per-phase emitted counts, and
    // reconstructs a global `FiberGrouping` so step 8 keeps its interface.
    // The width-gated spectral former for wide blocks lands in a follow-on
    // slice.
    //
    // `PhaseId` is `Uint<5>` (up to 32 phases) and `TrunkId` is `Uint<6>`
    // (up to 64 trunks plan-wide): the deliberate engine width cap. The
    // layout's per-phase trunk count is the authority for the CSR range
    // (the projection caps emission at the plan-wide `D::Trunks` budget),
    // so the running prefix sum always brackets the flat pool exactly. A
    // hard bound-check that errors past the id-width cap is a follow-up
    // (#641).
    let layout = steps::project_fiber_components::<D>(
        &dag,
        &partition,
        &waists,
        &topo,
        inputs.unit_count,
    );
    plan.trunk_count = layout.trunk_count;
    plan.fiber_count = layout.fiber_count;
    plan.trunks = layout.trunks;
    plan.fibers = layout.fibers;
    let phase_trunks = layout.phase_trunks.as_ref();
    let mut p = 0;
    let mut running_trunk = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal CSR prefix-sum cursor; tracked: #72
    while p < waists.phase_count.0 && p < cap_size_phases::<D>() {
        plan.phases.as_mut()[p].id = PhaseId::from_index(USize(p)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
        let tc = phase_trunks[p];
        plan.phases.as_mut()[p].trunk_count = tc;
        plan.phases.as_mut()[p].trunk_offset = USize(running_trunk); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from CSR offset; tracked: #72
        running_trunk += tc.0;
        p += 1;
    }
    let fibers = steps::fiber_grouping_from_trunks::<D>(&plan.fibers, plan.fiber_count);
    if fibers.fiber_count.0 == 0 && n > 0 {
        return notko::Outcome::Err(PlanError::NoTrunkAssignment);
    }

    // Step 8 (fused): upward rank + per-fiber dirty propagation.
    let (ranks, dirty) = steps::compute_upward_rank_and_dirty::<D>(&dag, &topo, inputs, &fibers);
    // Stash a subset of the per-fiber dirty info onto the plan's
    // columns-shaped DirtyMasks. The compatibility cast assumes the
    // store capacity <= the column capacity (typical); a larger store
    // capacity would need explicit truncation handled in a follow-up
    // round.
    let mut f = 0;
    while f < cap_size(<D::Fibers as Capacity>::CAP) {
        // Reuse the same bit layout: DirtyMask::raw + manual restore.
        let raw = dirty.per_fiber.as_ref()[f].raw();
        // Move bits into the columns-shaped mask one by one.
        let mut store = 0;
        while store < cap_size(<D::Stores as Capacity>::CAP) && store < 64 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: AccessMask 64-bit window per skeleton; tracked: #72
            let bit = (raw.0 >> store) & 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bit extraction internal; tracked: #72
            if bit == 1 {
                let pf = plan.dirty.per_fiber.as_ref()[f];
                plan.dirty.per_fiber.as_mut()[f] = pf.set(USize(store)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            }
            store += 1;
        }
        f += 1;
    }

    // Step 9: morsel sizing per fiber. Stored on the plan so Pass 3
    // dispatch codegen can emit per-fiber `RecordRange` slices without
    // recomputing.
    plan.morsel_sizes = steps::size_morsels::<D>(inputs.record_count, fibers.fiber_count);

    // Step 10: phase configs. Store onto plan.phases[i].config. Pass
    // the unit count so the last phase's width is computed against
    // the real range, not the prior `start + 1` lower-bound.
    let configs =
        steps::select_phase_configs::<D>(&waists, inputs.record_count, inputs.unit_count);
    let mut i = 0;
    while i < plan.phase_count.0 && i < cap_size_phases::<D>() {
        plan.phases.as_mut()[i].config = configs.as_ref()[i];
        i += 1;
    }

    // Step 11: per-fiber column classification.
    plan.column_class = steps::classify_columns::<D>(&fibers, inputs);

    // Populate the unit meta array with the topo order. `unit_meta` is
    // indexed by topo-position (matching the dispatch order), so each
    // slot's `id` is `topo[u]` and the per-unit fields are looked up
    // by the raw unit-id index that `topo[u]` projects to. `ranks` is
    // also unit-id-indexed; project once and read both.
    let topo_s = topo.as_ref();
    let mut u = 0;
    while u < n && u < cap_size(<D::Units as Capacity>::CAP) {
        plan.unit_meta.as_mut()[u].id = topo_s[u];
        let unit_id_idx = topo_s[u].index().0;
        if unit_id_idx < cap_size(<D::Units as Capacity>::CAP) {
            plan.unit_meta.as_mut()[u].commutative = inputs.commutative.as_ref()[unit_id_idx];
            plan.unit_meta.as_mut()[u].upward_rank = ranks.as_ref()[unit_id_idx];
        }
        u += 1;
    }

    // Steps 12 + 13 (core assignment + per-core program synthesis)
    // happen on the dispatch stage entry; not part of this runner.
    notko::Outcome::Ok(plan)
}

use arvo_tensor::cap_size;

/// The spectral eigenvector float, named here so the runner's `Copy`
/// where-bound on `<D::Units as Capacity>::Array<_>` matches the one
/// `project_fiber_components` and `spectral_partition` carry.
type SpectralFloatVec = arvo::FastFloat<f32>; // lint:allow(no-bare-numeric) reason: f32 is the IEEE width tag of arvo FastFloat; tracked: #72

/// Phase-capacity cap as a `usize`, factored so the runner reads
/// cleanly. The dimension is a type; this is the value-position
/// projection of its `CAP`.
#[inline]
fn cap_size_phases<D: PlanDims>() -> usize { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-position cap projection; rust grammar requires usize loop bound; tracked: #72
    cap_size(<D::Phases as Capacity>::CAP)
}
