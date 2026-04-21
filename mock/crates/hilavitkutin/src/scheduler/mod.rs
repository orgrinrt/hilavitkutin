//! Scheduler builder + execution plan (domain 23).
//!
//! Static composition (R6): all WUs registered at compile time.
//! No runtime registration.
//!
//! This round ships the builder skeleton only; the actual
//! execution loop requires 5a2 (plan-stage) + 5a3 (dispatch) +
//! 5a4 (thread) to be in place first.

use core::marker::PhantomData;

pub mod metrics;
pub mod plan;
pub mod result;

pub use metrics::SchedulerMetrics;
pub use plan::{ExecutionPlan, LaneAssignment};
pub use result::PipelineResult;

/// Top-level scheduler with const-sized capacity.
pub struct Scheduler<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_STORES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
> {
    _phantom: PhantomData<()>,
}

impl<const MAX_UNITS: usize, const MAX_STORES: usize, const MAX_LANES: usize> // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    Scheduler<MAX_UNITS, MAX_STORES, MAX_LANES>
{
    pub const fn builder() -> SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES> {
        SchedulerBuilder {
            _phantom: PhantomData,
        }
    }
}

impl<const MAX_UNITS: usize, const MAX_STORES: usize, const MAX_LANES: usize> Default // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    for Scheduler<MAX_UNITS, MAX_STORES, MAX_LANES>
{
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

/// Builder for Scheduler. Chains static composition.
pub struct SchedulerBuilder<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_STORES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
> {
    _phantom: PhantomData<()>,
}

impl<const MAX_UNITS: usize, const MAX_STORES: usize, const MAX_LANES: usize> // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES>
{
    pub fn add<WU: 'static>(self) -> Self {
        self
    }

    pub fn resource<T: 'static>(self, _init: T) -> Self {
        self
    }

    pub fn resource_default<T: Default + 'static>(self) -> Self {
        self
    }

    pub fn column<T: 'static>(self) -> Self {
        self
    }

    pub fn memory<M: 'static>(self, _provider: M) -> Self {
        self
    }

    pub fn threads<P: 'static>(self, _pool: P) -> Self {
        self
    }

    pub fn clock<C: 'static>(self, _clock: C) -> Self {
        self
    }

    pub fn build(self) -> Scheduler<MAX_UNITS, MAX_STORES, MAX_LANES> {
        Scheduler::default()
    }
}
