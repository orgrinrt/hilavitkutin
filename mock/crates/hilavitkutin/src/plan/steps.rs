//! The twelve plan-stage algorithm steps (domains 11-16).
//!
//! Every function here is a skeleton stub whose body is `todo!()`.
//! Each step lands as a separate downstream micro-round; see
//! BACKLOG → Engine 5a2 follow-ups.
//!
//! Signatures are stable: downstream rounds implement a body and
//! add tests, without changing the surface.

use super::column::ColumnClassification;
use super::dirty::DirtyMask;
use super::fiber::FiberGrouping;
use super::graph::DependencyGraph;
use super::inputs::{UnitId, PlanInputs};
use super::phase::PhaseBoundaries;

/// Step 1 — DAG construction from AccessMask overlap (domain 11).
pub fn build_dag<const MAX_UNITS: usize, const MAX_STORES: usize>(
    inputs: &PlanInputs<MAX_UNITS, MAX_STORES>,
) -> DependencyGraph<MAX_UNITS> {
    let _ = inputs;
    todo!("5a2 step 1: build DAG from AccessMask overlap")
}

/// Step 2 — Topological sort + node renumbering (domain 15).
pub fn topo_sort<const MAX_UNITS: usize>(
    graph: &DependencyGraph<MAX_UNITS>,
) -> [UnitId; MAX_UNITS] {
    let _ = graph;
    todo!("5a2 step 2: topological sort + node renumbering")
}

/// Step 3 — Upward rank + critical path (domain 15).
pub fn upward_rank<const MAX_UNITS: usize>(
    graph: &DependencyGraph<MAX_UNITS>,
) -> [u32; MAX_UNITS] {
    let _ = graph;
    todo!("5a2 step 3: upward rank + critical path")
}

/// Step 4 — Waist detection → phase boundaries (domain 11).
pub fn detect_waists<const MAX_UNITS: usize, const MAX_PHASES: usize>(
    graph: &DependencyGraph<MAX_UNITS>,
) -> PhaseBoundaries<MAX_PHASES> {
    let _ = graph;
    todo!("5a2 step 4: waist detection → phase boundaries")
}

/// Step 5 — RCM reordering → fiber grouping order (domain 15).
pub fn rcm_reorder<const MAX_UNITS: usize>(
    graph: &DependencyGraph<MAX_UNITS>,
) -> [UnitId; MAX_UNITS] {
    let _ = graph;
    todo!("5a2 step 5: RCM reordering → fiber grouping order")
}

/// Step 6 — Block diagonal + D-M → phase validation (domain 15).
pub fn block_diagonal<const MAX_UNITS: usize, const MAX_PHASES: usize>(
    graph: &DependencyGraph<MAX_UNITS>,
    phases: &PhaseBoundaries<MAX_PHASES>,
) -> bool {
    let _ = (graph, phases);
    todo!("5a2 step 6: block diagonal + D-M → phase validation")
}

/// Step 7 — Spectral partitioning for >5 fibers (domain 15).
pub fn spectral_partition<const MAX_UNITS: usize, const MAX_FIBERS: usize>(
    graph: &DependencyGraph<MAX_UNITS>,
) -> FiberGrouping<MAX_UNITS, MAX_FIBERS> {
    let _ = graph;
    todo!("5a2 step 7: spectral partitioning for >5 fibers")
}

/// Step 8 — Fiber grouping (greedy + matrix chain DP, domain 14).
pub fn group_fibers<const MAX_UNITS: usize, const MAX_FIBERS: usize>(
    graph: &DependencyGraph<MAX_UNITS>,
) -> FiberGrouping<MAX_UNITS, MAX_FIBERS> {
    let _ = graph;
    todo!("5a2 step 8: fiber grouping (greedy + matrix chain DP)")
}

/// Step 9 — Per-fiber morsel sizing (domain 12).
pub fn size_morsels<const MAX_FIBERS: usize>(record_count: u64) -> [u32; MAX_FIBERS] {
    let _ = record_count;
    todo!("5a2 step 9: per-fiber morsel sizing")
}

/// Step 10 — Per-phase adaptive configs (domain 11).
pub fn adaptive_config<const MAX_PHASES: usize>(
    phases: &PhaseBoundaries<MAX_PHASES>,
) -> [u8; MAX_PHASES] {
    let _ = phases;
    todo!("5a2 step 10: per-phase adaptive configs")
}

/// Step 11 — Column classification per fiber (domain 15).
pub fn classify_columns<const MAX_UNITS: usize, const MAX_FIBERS: usize>(
    fibers: &FiberGrouping<MAX_UNITS, MAX_FIBERS>,
) -> [ColumnClassification; MAX_FIBERS] {
    let _ = fibers;
    todo!("5a2 step 11: column classification per fiber")
}

/// Step 12 — Dirty propagation masks (domain 16).
pub fn propagate_dirty<const MAX_UNITS: usize, const MAX_STORES: usize>(
    inputs: &PlanInputs<MAX_UNITS, MAX_STORES>,
) -> DirtyMask<MAX_STORES> {
    let _ = inputs;
    todo!("5a2 step 12: dirty propagation masks")
}
