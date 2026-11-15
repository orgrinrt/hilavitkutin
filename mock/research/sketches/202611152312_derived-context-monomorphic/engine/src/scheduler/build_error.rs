//! `SchedulerBuilder::build` failure modes and the recommended-order carrier.
//!
//! Split out of `scheduler/mod.rs` (file-size lint): the `BuildError` enum
//! and its `RecommendedOrder` payload type. No behaviour change.

use core::fmt;

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use arvo_tensor::Capacity;
use hilavitkutin_api::UnitId;

use crate::plan::{DefaultPlanDims, PlanDims};

/// Failure modes for `SchedulerBuilder::build`.
///
/// `#[non_exhaustive]` so future failure modes (column-buffer OOM in
/// B2b, plan-stage failures) do not break consumers.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum BuildError {
    /// The `MemoryProvider` returned null for a resource allocation.
    /// Every block allocated before the failure is freed before the
    /// error returns, so no block leaks.
    AllocationFailed,
    /// The plan stage could not produce a valid execution plan from the
    /// registered bundle (a dependency cycle, surfaced by
    /// `compute_execution_plan` as `PlanError::Cycle`, or another
    /// feasibility failure). The plan is computed before any allocation,
    /// so no block is allocated on this path.
    PlanFailed,
    /// The registration order is acyclic but not a topological order of the
    /// dependency DAG: a `producer` slot writes a store that a `consumer` slot
    /// reads, yet `producer` is registered after `consumer` (carrier slot index
    /// `producer >= consumer`). Distinct from `PlanFailed` (a cycle): there is
    /// a valid topological order, the registration just is not one. The static
    /// dispatch walk follows carrier order directly, so an anti-topological
    /// carrier would dispatch a reader before its writer. The fields name the
    /// offending carrier slots. This is the provisional producer-before-consumer
    /// constraint (engine roadmap r2 §8, op call b): it relaxes when the engine
    /// auto-applies the cache-optimal (RCM / topological) order. The check runs
    /// before any allocation, so a rejected registration allocates nothing.
    NonTopologicalRegistration {
        /// Carrier slot index of the writer registered after its reader.
        producer:    USize,
        /// Carrier slot index of the reader registered before its writer.
        consumer:    USize,
        /// A registration order that satisfies the gate: the RCM-reordered
        /// topological order (canonical Step 5, the cache-optimal order among
        /// valid topological orders), the same order the auto-ordering
        /// relaxation applies. Each entry is a carrier slot index; registering
        /// in this order makes the carrier topological.
        recommended: RecommendedOrder,
    },
}

/// A recommended registration order: a carrier-slot sequence the consumer can
/// register in to satisfy the topological-registration gate.
///
/// Built from the plan's `rcm_order` (the RCM-reordered topological order, the
/// cache-optimal order among valid topological orders per canonical Step 5).
/// Fixed-capacity, no allocation: the slot sequence lives inline, sized by the
/// engine's default unit capacity, with `count` naming the live prefix.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RecommendedOrder {
    /// Carrier slot indices in recommended order; only `[0 .. count]` is live.
    order: <<DefaultPlanDims as PlanDims>::Units as Capacity>::Array<USize>,
    /// Number of live entries in `order`.
    count: USize,
}

impl RecommendedOrder {
    /// Build the recommended order from the plan's `rcm_order` permutation.
    ///
    /// `rcm_order[new_pos]` is the original `UnitId` placed at `new_pos`; its
    /// carrier slot index is `.index()`. The live prefix is `[0 .. count]`.
    pub(super) fn from_rcm_order(rcm: &[UnitId], count: USize) -> Self {
        let mut order = <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(
            <USize as Identity<Additive>>::IDENTITY,
        );
        let n = count.0.min(rcm.len()).min(order.as_ref().len());
        let slots = order.as_mut();
        let mut i = 0;
        while i < n {
            slots[i] = rcm[i].index();
            i += 1;
        }
        Self {
            order,
            count: USize(n), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-wrap clamped live count; tracked: #72
        }
    }

    /// The recommended carrier-slot sequence, live prefix only.
    pub fn as_slice(&self) -> &[USize] {
        &self.order.as_ref()[.. self.count.0]
    }
}

impl fmt::Debug for RecommendedOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.as_slice().iter().map(|s| s.0))
            .finish()
    }
}
