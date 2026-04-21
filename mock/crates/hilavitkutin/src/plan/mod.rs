//! Plan stage (domains 11-16).
//!
//! Pure analysis, no runtime state. Takes WU declarations (Read /
//! Write AccessSets, scheduling hints, COMMUTATIVE flag) and
//! produces a complete `ExecutionPlan`. Recomputes on any pipeline
//! structure change (new WUs, record-count change, DAG modification).
//!
//! This module is the *skeleton* for 5a2: public surface is
//! complete; every algorithm step (`build_dag`, `topo_sort`,
//! `upward_rank`, `detect_waists`, `rcm_reorder`, `block_diagonal`,
//! `spectral_partition`, `group_fibers`, `size_morsels`,
//! `adaptive_config`, `classify_columns`, `propagate_dirty`) stubs
//! to `todo!()`. Each becomes its own downstream round — see
//! BACKLOG → Engine 5a2 follow-ups.
//!
//! The top-level orchestration function `build_plan` is also a
//! `todo!()` stub: it depends on all 12 step bodies existing
//! before it can meaningfully wire them together.

pub mod access;
pub mod column;
pub mod dirty;
pub mod fiber;
pub mod graph;
pub mod inputs;
pub mod phase;
pub mod steps;

pub use access::AccessMask;
pub use column::ColumnClassification;
pub use dirty::DirtyMask;
pub use fiber::{FiberGrouping, FiberId};
pub use graph::DependencyGraph;
pub use inputs::{PlanInputs, UnitId};
pub use phase::{PhaseBoundaries, PhaseId};

/// Build an `ExecutionPlan` from `PlanInputs`.
///
/// Skeleton: `todo!()`. The real orchestration wires steps 1-12
/// (see [`steps`]) and lands once all 12 bodies exist — tracked
/// in BACKLOG → Engine 5a2 follow-ups.
pub fn build_plan<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_STORES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    inputs: &PlanInputs<MAX_UNITS, MAX_STORES>,
) -> crate::scheduler::ExecutionPlan<MAX_LANES> {
    let _ = inputs;
    todo!("skeleton — plan-stage algorithm split across follow-up rounds")
}
