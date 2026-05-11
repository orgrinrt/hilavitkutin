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
use arvo::USize;

pub mod access;
pub mod column;
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
pub use graph::DependencyGraph;
pub use inputs::PlanInputs;
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
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_PHASES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_TRUNKS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_FIBERS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COLUMNS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COMPONENTS_PER_TRUNK: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_UNITS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COLUMNS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_TRUNKS_PER_PHASE: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
> {
    /// Waist-delimited phases (in dispatch order).
    pub phases: [Phase<
        MAX_TRUNKS_PER_PHASE,
        MAX_COMPONENTS_PER_TRUNK,
        MAX_UNITS_PER_FIBER,
        MAX_COLUMNS_PER_FIBER,
    >; MAX_PHASES],
    pub phase_count: USize,
    /// Per-unit metadata array, addressed by `UnitId`.
    pub unit_meta: [UnitMeta; MAX_UNITS],
    pub unit_count: USize,
    /// Per-fiber column classification.
    pub column_class: ColumnClassMap<MAX_FIBERS, MAX_COLUMNS_PER_FIBER>,
    /// Per-fiber dirty masks (incremental-skip propagation).
    pub dirty: DirtyMasks<MAX_FIBERS, MAX_COLUMNS>,
}

impl<
        const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_PHASES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_TRUNKS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_FIBERS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_COLUMNS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_COMPONENTS_PER_TRUNK: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_UNITS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_COLUMNS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_TRUNKS_PER_PHASE: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
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
{
    /// All-zero plan. Used as the default before the plan-stage
    /// chain populates real values, and as the constructor for
    /// `Default`.
    pub const fn new() -> Self {
        Self {
            phases: [Phase::new(); MAX_PHASES],
            phase_count: USize::ZERO,
            unit_meta: [UnitMeta::new(); MAX_UNITS],
            unit_count: USize::ZERO,
            column_class: ColumnClassMap::new(),
            dirty: DirtyMasks::new(),
        }
    }
}

impl<
        const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_PHASES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_TRUNKS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_FIBERS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_COLUMNS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_COMPONENTS_PER_TRUNK: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_UNITS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_COLUMNS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        const MAX_TRUNKS_PER_PHASE: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
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
{
    fn default() -> Self {
        Self::new()
    }
}

/// Build an `ExecutionPlan` from `PlanInputs`.
///
/// Skeleton: `todo!()`. The real orchestration chains the 13-step
/// algorithm in [`steps`] and lands in Session 2B of this round.
pub fn build_plan<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_STORES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_PHASES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_TRUNKS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_FIBERS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COLUMNS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COMPONENTS_PER_TRUNK: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_UNITS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COLUMNS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_TRUNKS_PER_PHASE: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    inputs: &PlanInputs<MAX_UNITS, MAX_STORES>,
) -> ExecutionPlan<
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
> {
    let _ = inputs;
    todo!("session 2B: chain the 13 plan-stage steps")
}
