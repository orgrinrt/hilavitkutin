//! Scheduler builder + execution plan (domain 23).
//!
//! Static composition (R6): all WUs registered at compile time.
//! No runtime registration.
//!
//! `SchedulerBuilder<Wus, Stores>` carries a phantom-tuple
//! type-state. `Wus` accumulates registered WU types (Cons-list).
//! `Stores` accumulates registered `Resource<T>` / `Column<T>` /
//! `Virtual<T>` markers (Cons-list). `.build()` carries
//! `Stores: ContainsAll<Wus::AccumRead> +
//! ContainsAll<Wus::AccumWrite>`, which proves at compile time that
//! every registered WU's `Read` and `Write` membership is satisfied
//! by the registered stores.
//!
//! Round 4 reshape: dropped `MAX_UNITS` / `MAX_STORES` /
//! `MAX_LANES` const generics. `.add_kit::<K: Kit>()` is type-level
//! only; no `install` body. `Scheduler::replace_resource::<T>` lands
//! with a `T: Replaceable` bound.

use core::marker::PhantomData;

use hilavitkutin_api::access::{Concat, Cons, ContainsAll, Empty};
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, ThreadPoolApi};
use hilavitkutin_api::store::{Column, Replaceable, Resource, Virtual};
use hilavitkutin_api::work_unit::{WorkUnit, WorkUnitBundle};
use hilavitkutin_kit::Kit;

pub mod metrics;
pub mod plan;
pub mod result;

pub use metrics::SchedulerMetrics;
pub use plan::{ExecutionPlan, LaneAssignment};
pub use result::PipelineResult;

/// Top-level scheduler.
pub struct Scheduler {
    _phantom: PhantomData<()>,
}

impl Scheduler {
    pub const fn builder() -> SchedulerBuilder<Empty, Empty> {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Replace the existing `Resource<T>` instance in the scheduler's
    /// data plane with `_new`.
    ///
    /// `T: Replaceable` is enforced statically. Apps that want a
    /// replaceable resource opt their type into the marker; types
    /// that should not be overridable stay locked. The implementation
    /// is a stub at this round; the runtime data plane lands with
    /// HILA-RUNTIME tasks.
    pub fn replace_resource<T: Replaceable>(&mut self, _new: T) {
        // stub: runtime data plane lands later
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self { _phantom: PhantomData }
    }
}

/// Builder for `Scheduler`. Accumulates WU and store types in a
/// phantom-tuple type-state.
///
/// `Wus` is a Cons-list of registered WU types: `Cons<W0, Cons<W1,
/// ..., Empty>>`. `Stores` is a Cons-list of registered store
/// markers (`Resource<T>` / `Column<T>` / `Virtual<T>` mixed). Both
/// start at `Empty` from `Scheduler::builder()` and grow via the
/// registration methods.
pub struct SchedulerBuilder<Wus, Stores> {
    _phantom: PhantomData<(Wus, Stores)>,
}

impl<Wus, Stores> SchedulerBuilder<Wus, Stores> {
    /// Register a WU type. Prepends `W` onto `Wus`.
    pub fn add<W: WorkUnit>(self) -> SchedulerBuilder<Cons<W, Wus>, Stores> {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Register a `Resource<T>` with an initial value. Prepends
    /// `Resource<T>` onto `Stores`.
    pub fn resource<T: 'static>(
        self,
        _init: T,
    ) -> SchedulerBuilder<Wus, Cons<Resource<T>, Stores>> {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Register a `Resource<T>` constructed via `Default`.
    pub fn resource_default<T: Default + 'static>(
        self,
    ) -> SchedulerBuilder<Wus, Cons<Resource<T>, Stores>> {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Register a `Column<T>`. Prepends `Column<T>` onto `Stores`.
    pub fn column<T: 'static>(self) -> SchedulerBuilder<Wus, Cons<Column<T>, Stores>> {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Register a `Virtual<T>`. Prepends `Virtual<T>` onto `Stores`.
    pub fn add_virtual<T: 'static>(
        self,
    ) -> SchedulerBuilder<Wus, Cons<Virtual<T>, Stores>> {
        SchedulerBuilder { _phantom: PhantomData }
    }

    /// Install a Kit, accumulating its declared `Units` and `Owned`
    /// type-level lists onto the builder's typestate.
    ///
    /// Type-level only: no value-level `install` body. The kit's
    /// declared `Units` (a `WorkUnitBundle`) and `Owned` (a
    /// `StoreBundle`) are concatenated onto the builder's accumulators.
    pub fn add_kit<K: Kit>(self) -> SchedulerBuilder<
        <K::Units as Concat<Wus>>::Out,
        <K::Owned as Concat<Stores>>::Out,
    >
    where
        K::Units: Concat<Wus>,
        K::Owned: Concat<Stores>,
    {
        SchedulerBuilder { _phantom: PhantomData }
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

impl<Wus, Stores> SchedulerBuilder<Wus, Stores>
where
    Wus: WorkUnitBundle,
    Stores: hilavitkutin_api::AccessSet
        + ContainsAll<<Wus as WorkUnitBundle>::AccumRead>
        + ContainsAll<<Wus as WorkUnitBundle>::AccumWrite>,
{
    /// Finalise the builder into a `Scheduler`.
    ///
    /// Carries `Stores: ContainsAll<Wus::AccumRead> +
    /// ContainsAll<Wus::AccumWrite>` as its where-clause. A
    /// registered WU referencing an unregistered store produces a
    /// compile error pointing at the missing store.
    pub fn build(self) -> Scheduler {
        Scheduler::default()
    }
}
