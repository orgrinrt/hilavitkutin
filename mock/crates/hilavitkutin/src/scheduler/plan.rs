//! Plan cache attached to the `Scheduler`.
//!
//! Topic 9 axes D + E of the runtime megaround: the cache lives
//! with the `Scheduler`; no eviction beyond the dirty signal; single
//! cached plan v1. Pass 7 + Pass 8 wire the cache against
//! `compute_execution_plan` and the per-pass `Scheduler::run` body;
//! this module locks the storage shape.

use core::marker::PhantomData;

use arvo::Bool;

/// Single-slot cached execution plan.
///
/// `present` distinguishes the pre-first-run state (no cached plan
/// available) from the post-recompute state (cached plan ready to
/// reuse). Pass 7 + Pass 8 populate the slot.
pub struct PlanCache {
    _phantom: PhantomData<()>,
    /// `Bool::TRUE` when the slot holds a valid cached plan.
    present: Bool,
}

impl PlanCache {
    /// Construct an empty cache. The first `Scheduler::run` triggers
    /// recompute and populates the slot.
    pub const fn new() -> Self {
        Self { _phantom: PhantomData, present: Bool::FALSE }
    }

    /// Is a cached plan available for reuse?
    pub const fn is_present(&self) -> Bool {
        self.present
    }
}

impl Default for PlanCache {
    fn default() -> Self {
        Self::new()
    }
}
