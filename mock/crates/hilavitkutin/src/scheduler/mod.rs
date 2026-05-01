//! Scheduler builder + execution plan (domain 23).
//!
//! Static composition (R6): all WUs registered at compile time.
//! No runtime registration.
//!
//! `SchedulerBuilder<MAX_*, Wus, Stores>` carries a phantom-tuple
//! type-state. `Wus` accumulates registered WU types. `Stores`
//! unifies registered `Resource<T>` / `Column<T>` / `Virtual<T>`
//! markers as a cons-list. `.build()` carries `Wus:
//! Buildable<Stores>`, which proves at compile time that every
//! registered WU's `Read` and `Write` membership is satisfied by
//! the registered stores.
//!
//! Round 202605010900 (#255) introduced the type-state shape.
//! The runtime `.run()` loop is still 5a2/5a3/5a4 deferred work;
//! this module ships the build-time proof only.

use core::marker::PhantomData;

use hilavitkutin_api::access::AccessSet;
use hilavitkutin_api::builder::Buildable;
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, ThreadPoolApi};
use hilavitkutin_api::store::{Column, Resource, Virtual};
use hilavitkutin_api::work_unit::WorkUnit;
use hilavitkutin_kit::Kit;

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
    pub const fn builder() -> SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, (), ()> {
        SchedulerBuilder { _phantom: PhantomData }
    }
}

impl<const MAX_UNITS: usize, const MAX_STORES: usize, const MAX_LANES: usize> Default // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    for Scheduler<MAX_UNITS, MAX_STORES, MAX_LANES>
{
    fn default() -> Self {
        Self { _phantom: PhantomData }
    }
}

/// Builder for `Scheduler`. Accumulates WU and store types in
/// phantom-tuple type-state.
///
/// `Wus` is a cons-list of registered WU types: `(W0, (W1, (...,
/// ())))`. `Stores` is a cons-list of registered store markers
/// (`Resource<T>` / `Column<T>` / `Virtual<T>` mixed). Both start
/// at `()` from `Scheduler::builder()` and grow via the
/// registration methods.
pub struct SchedulerBuilder<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_STORES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    Wus,
    Stores,
> {
    _phantom: PhantomData<(Wus, Stores)>,
}

impl<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_STORES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    Wus,
    Stores,
> SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores>
where
    Wus: AccessSet,
    Stores: AccessSet,
{
    /// Register a WU type. Prepends `W` onto `Wus`.
    pub fn add<W: WorkUnit>(self) -> SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, (W, Wus), Stores>
    where
        (W, Wus): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Register a `Resource<T>` with an initial value. Prepends
    /// `Resource<T>` onto `Stores`.
    pub fn resource<T: 'static>(
        self,
        _init: T,
    ) -> SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, (Resource<T>, Stores)>
    where
        (Resource<T>, Stores): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Register a `Resource<T>` constructed via `Default`.
    /// Prepends `Resource<T>` onto `Stores`.
    pub fn resource_default<T: Default + 'static>(
        self,
    ) -> SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, (Resource<T>, Stores)>
    where
        (Resource<T>, Stores): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Register a `Column<T>`. Prepends `Column<T>` onto `Stores`.
    pub fn column<T: 'static>(
        self,
    ) -> SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, (Column<T>, Stores)>
    where
        (Column<T>, Stores): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Register a `Virtual<T>`. Prepends `Virtual<T>` onto `Stores`.
    pub fn virtual_<T: 'static>(
        self,
    ) -> SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, (Virtual<T>, Stores)>
    where
        (Virtual<T>, Stores): AccessSet,
    {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Install a Kit, returning the type-state the Kit's `install`
    /// produces.
    pub fn add_kit<K: Kit<Self>>(self, k: K) -> K::Output {
        k.install(self)
    }

    pub fn memory<M: MemoryProviderApi + 'static>(self, _provider: M) -> Self {
        self
    }

    pub fn threads<P: ThreadPoolApi + 'static>(self, _pool: P) -> Self {
        self
    }

    pub fn clock<C: ClockApi + 'static>(self, _clock: C) -> Self {
        self
    }
}

impl<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_STORES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    Wus,
    Stores,
> SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores>
where
    Wus: Buildable<Stores>,
    Stores: AccessSet,
{
    /// Finalise the builder into a `Scheduler`.
    ///
    /// Carries `Wus: Buildable<Stores>` as its where-clause. This
    /// unfolds into per-WU `Stores: WuSatisfied<Wᵢ::Read> +
    /// WuSatisfied<Wᵢ::Write>` proofs, which unfold into per-store
    /// `Stores: Contains<Tⱼ>` membership checks. A registered WU
    /// referencing an unregistered store produces a compile error
    /// naming the missing store directly.
    pub fn build(self) -> Scheduler<MAX_UNITS, MAX_STORES, MAX_LANES> {
        Scheduler::default()
    }
}
