//! `PlanError`: reasons `compute_execution_plan` rejects the input.
//!
//! Split out of `plan/steps.rs` (file-size lint). No behaviour change.

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
    /// `compute_fiber_morsel_windows` produced a morsel size below the engine's
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
