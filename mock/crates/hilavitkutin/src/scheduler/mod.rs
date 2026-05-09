//! Scheduler builder + execution plan (domain 23).
//!
//! Static composition (R6): all WUs registered at compile time.
//! No runtime registration.
//!
//! `SchedulerBuilder<Wus, Stores>` carries a phantom-tuple
//! type-state. `Wus` accumulates registered WU types (Cons-list).
//! `Stores` accumulates registered `Resource<T>` / `Column<T>` /
//! `Virtual<T>` / `LinkedBin<T>` markers (Cons-list). `.build()`
//! carries `Stores: ContainsAll<Wus::AccumRead> +
//! ContainsAll<Wus::AccumWrite>`, which proves at compile time that
//! every registered WU's `Read` and `Write` membership is satisfied
//! by the registered stores.
//!
//! Round 4 reshape: dropped `MAX_UNITS` / `MAX_STORES` /
//! `MAX_LANES` const generics. `Scheduler::replace_resource::<T>`
//! lands with a `T: Replaceable` bound.
//!
//! Round 202605091700 reshape: the nine `.add_*` and `.with_*`
//! methods retire in favour of one unified verb, `.with(value)`.
//! Every value passed to `.with` impls the sealed `Provider` trait
//! from `hilavitkutin-api`; the per-kind typestate update flows
//! through `Provider::Dispatch`. WorkUnit unit-structs, Kits,
//! `Resource::new(value)`, `Column::<T>::new()`,
//! `Virtual::<T>::new()`, `LinkedBin::<dyn TraitFamily>::new()`, and
//! platform impls (memory / threads / clock) all share the one
//! signature.

use core::marker::PhantomData;

use hilavitkutin_api::access::{ContainsAll, Empty};
use hilavitkutin_api::provider::{Dispatch, Provider};
use hilavitkutin_api::store::Replaceable;
use hilavitkutin_api::work_unit::WorkUnitBundle;

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
    /// Register one provider on the scheduler.
    ///
    /// Accepts any `P: Provider`: WorkUnit unit-structs, Kits,
    /// `Resource::new(value)`, `Column::<T>::new()`,
    /// `Virtual::<T>::new()`, `LinkedBin::<dyn TraitFamily>::new()`,
    /// and platform impls (memory provider, thread pool, clock). The
    /// per-kind typestate update flows through `P::Dispatch` and
    /// lands on the appropriate accumulator.
    ///
    /// Non-`Provider` values fail the trait solver here, surfacing
    /// the `Provider` `#[diagnostic::on_unimplemented]` message which
    /// names the constructors a consumer reaches for.
    ///
    /// The platform-tuple accumulator `Empty` is the placeholder
    /// until the data plane (HILA-RUNTIME-C4) introduces a third
    /// builder type parameter.
    pub fn with<P>(self, _provider: P) -> SchedulerBuilder<
        <P::Dispatch as Dispatch<Wus, Stores, Empty>>::NextWus,
        <P::Dispatch as Dispatch<Wus, Stores, Empty>>::NextStores,
    >
    where
        P: Provider,
        P::Dispatch: Dispatch<Wus, Stores, Empty>,
    {
        SchedulerBuilder { _phantom: PhantomData }
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
