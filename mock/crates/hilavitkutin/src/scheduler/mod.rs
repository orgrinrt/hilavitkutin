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
//! Every value passed to `.with` impls the sealed `BuilderInput`
//! trait from `hilavitkutin-api`; the per-kind typestate update
//! flows through `BuilderInput::Dispatch`. WorkUnit unit-structs,
//! Kits, `Resource::new(value)`, `Column::<T>::new()`,
//! `Virtual::<T>::new()`, `LinkedBin::<TraitFamily>::new()`,
//! and platform impls (memory / threads / clock) all share the one
//! signature.
//!
//! Pass 6 of the runtime megaround (`202605101036`): `Scheduler`
//! lifts to `Scheduler<Cfg: RunCfg = DefaultRunCfg>`. The
//! `RunCfg::Out` associated type drives `Scheduler::run()`'s
//! return shape. `Scheduler::replace_resource<R: PlanAffecting>`
//! sets the per-resource dirty bit; the cheap `replace_value` path
//! is reserved for non-`PlanAffecting` replacements. The
//! `PipelineResult` enum retires per the workspace no-legacy-shims
//! rule; consumers receive `RunCfg::Out` directly.

use core::marker::PhantomData;
use core::sync::atomic::AtomicBool;

use hilavitkutin_api::access::{ContainsAll, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, Dispatch};
use hilavitkutin_api::run_cfg::{DefaultRunCfg, PlanAffecting, RunCfg};
use hilavitkutin_api::store::Replaceable;
use hilavitkutin_api::work_unit::WorkUnitBundle;

pub mod metrics;
pub mod plan;
pub mod stage;

pub use metrics::SchedulerMetrics;
pub use plan::PlanCache;

use stage::{Stage, StageEmpty, StageList};

/// Top-level scheduler.
///
/// Generic over the consumer's `RunCfg`. The `Cfg::Out` associated
/// type parameterises `run()`'s return shape; `Cfg::Err` flows
/// through `Cfg::Out::Err`. The dirty bitmap width is driven by
/// `Cfg::MAX_PLAN_AFFECTING_RESOURCES` per Topic 8 axis B and
/// Topic 9 axis C: each consumer's RunCfg picks the cap that fits
/// its plan-affecting resource population, and the type system
/// verifies the per-RunCfg slot count via `generic_const_exprs`.
pub struct Scheduler<Cfg: RunCfg = DefaultRunCfg> {
    _cfg: PhantomData<Cfg>,
    // The dirty bitmap width matches DefaultRunCfg::MAX_PLAN_AFFECTING_RESOURCES = USize(256).
    // The intended lift is `[AtomicBool; Cfg::MAX_PLAN_AFFECTING_RESOURCES.0]` under
    // `feature(generic_const_exprs)`, but current rustc rejects field access on generic
    // constants ("overly complex generic constant: field access is not supported in
    // generic constants"). The lift waits on rustc gaining that capability; until then
    // the hardcoded 256 matches the documented default and lint:allow(no-bare-numeric)
    // covers the const-generic-array-dimension root.
    // lint:allow(no-bare-numeric) reason: const-generic array dimension at the L0 storage root; matches DefaultRunCfg::MAX_PLAN_AFFECTING_RESOURCES = USize(256); tracked: #345 (per-Cfg lift awaits rustc generic_const_exprs gaining field-access support)
    plan_dirty: [AtomicBool; 256],
    plan_cache: PlanCache,
}

impl Scheduler<DefaultRunCfg> {
    /// Start a fresh builder. Empty Wus + Stores typestate; the
    /// builder grows via `.with(...)` calls.
    pub const fn builder() -> SchedulerBuilder<Empty, Empty, Empty, StageEmpty> {
        SchedulerBuilder {
            staged: StageEmpty,
            _phantom: PhantomData,
        }
    }
}

impl<Cfg: RunCfg> Scheduler<Cfg> {
    /// Replace the existing `Resource<T>` instance in the data
    /// plane with `_new`, marking the plan dirty.
    ///
    /// `T: PlanAffecting` routes the call onto the dirty-marking
    /// path; the next `run()` recomputes the execution plan.
    /// Consumers that need a cheap value swap on a non-plan-
    /// affecting resource use `replace_value`.
    pub fn replace_resource<T: PlanAffecting>(&mut self, _new: T) {
        // body lands at Pass 7 + Pass 8 wiring: locate the
        // PlanAffectingId for T, set plan_dirty[id] = true with
        // Release ordering, install the new value in the data
        // plane.
        let _ = &self.plan_dirty;
    }

    /// Cheap value-swap path for non-plan-affecting resources.
    ///
    /// `T: Replaceable` opts the type into runtime replacement
    /// without signalling plan recompute. The `Replaceable` marker
    /// is consumer-driven per Topic 8 axis B (replaceable but not
    /// plan-affecting is the typical case for app-level state).
    pub fn replace_value<T: Replaceable>(&mut self, _new: T) {
        // body lands at Pass 7 + Pass 8 wiring: locate the slot
        // for T in the data plane, swap the value, no dirty bit.
    }

    /// Transitional no-op body: returns `Cfg::Out::default()`.
    ///
    /// The real morsel loop (plan-dirty check, per-core dispatch
    /// build, executor spawn, morsel walk, phase barriers, meta-WU
    /// firing, persistence drain) waits on `codegen_fiber` and
    /// `codegen_core` producing monomorphised bodies, which
    /// themselves wait on `hilavitkutin-build` LLVM plugin hooks.
    /// The full real-body contract lives in `Scheduler::run real
    /// morsel loop body` in `BACKLOG.md.tmpl`.
    ///
    /// The `where Cfg::Out: Default` clause is method-level (not
    /// on the impl block), so `Scheduler::builder`,
    /// `replace_resource`, `replace_value`, and `Default for
    /// Scheduler<Cfg>` stay unaffected for consumers whose
    /// `Cfg::Out` is not `Default`. `DefaultRunCfg` satisfies the
    /// bound via notko's `Default for Outcome<T, E>` impl.
    pub fn run(&mut self) -> Cfg::Out
    where
        Cfg::Out: Default,
    {
        let _ = &self.plan_dirty;
        let _ = &self.plan_cache;
        Cfg::Out::default()
    }
}

impl<Cfg: RunCfg> Default for Scheduler<Cfg> {
    fn default() -> Self {
        Self {
            _cfg: PhantomData,
            plan_dirty: [const { AtomicBool::new(false) }; 256],
            plan_cache: PlanCache::new(),
        }
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
pub struct SchedulerBuilder<Wus, Stores, Platform, Staged: StageList> {
    staged: Staged,
    _phantom: PhantomData<(Wus, Stores, Platform)>,
}

impl<Wus, Stores, Platform, Staged: StageList> SchedulerBuilder<Wus, Stores, Platform, Staged> {
    /// Register one provider on the scheduler.
    ///
    /// Accepts any `P: BuilderInput`: WorkUnit unit-structs, Kits,
    /// `Resource::new(value)`, `Column::<T>::new()`,
    /// `Virtual::<T>::new()`, `ExtensionSurface::<TraitFamily>::new()`,
    /// and platform impls (memory provider, thread pool, clock).
    /// The per-kind typestate update flows through `P::Dispatch`
    /// and lands on the appropriate accumulator (`Wus`, `Stores`, or
    /// `Platform`).
    ///
    /// The registered value `provider` is moved onto the `Staged`
    /// value list so it is retained until `build()`, rather than
    /// dropped at the call site. The arena drain (HILA-RUNTIME-C6)
    /// moves each staged value into scheduler-owned storage.
    ///
    /// Non-`BuilderInput` values fail the trait solver here,
    /// surfacing the `BuilderInput`
    /// `#[diagnostic::on_unimplemented]` message which names the
    /// constructors a consumer reaches for.
    pub fn with<P>(
        self,
        provider: P,
    ) -> SchedulerBuilder<
        <P::Dispatch as Dispatch<Wus, Stores, Platform>>::NextWus,
        <P::Dispatch as Dispatch<Wus, Stores, Platform>>::NextStores,
        <P::Dispatch as Dispatch<Wus, Stores, Platform>>::NextPlatform,
        Stage<P, Staged>,
    >
    where
        P: BuilderInput,
        P::Dispatch: Dispatch<Wus, Stores, Platform>,
    {
        SchedulerBuilder {
            staged: Stage {
                head: provider,
                tail: self.staged,
            },
            _phantom: PhantomData,
        }
    }
}

impl<Wus, Stores, Platform, Staged> SchedulerBuilder<Wus, Stores, Platform, Staged>
where
    Wus: WorkUnitBundle,
    Stores: hilavitkutin_api::AccessSet
        + ContainsAll<<Wus as WorkUnitBundle>::AccumRead>
        + ContainsAll<<Wus as WorkUnitBundle>::AccumWrite>,
    Staged: StageList,
{
    /// Finalise the builder into a `Scheduler<DefaultRunCfg>`.
    ///
    /// Carries `Stores: ContainsAll<Wus::AccumRead> +
    /// ContainsAll<Wus::AccumWrite>` as its where-clause. A
    /// registered WU referencing an unregistered store produces a
    /// compile error pointing at the missing store. Consumers that
    /// supplied an explicit `RunCfg` via `.with(MyRunCfg)` use
    /// `build_with::<MyRunCfg>()` to thread the Cfg type through.
    ///
    /// The staged value list is dropped here at this round; the arena
    /// drain (HILA-RUNTIME-C6) moves each retained value into
    /// scheduler-owned storage instead.
    pub fn build(self) -> Scheduler<DefaultRunCfg> {
        Scheduler::default()
    }

    /// Finalise the builder with an explicit `RunCfg` type.
    ///
    /// Used when the consumer registered a custom `RunCfg` via
    /// `.with(MyRunCfg)`; the explicit type parameter threads the
    /// `Cfg::Out` shape through `Scheduler::run()`.
    pub fn build_with<Cfg: RunCfg>(self) -> Scheduler<Cfg> {
        Scheduler::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Scheduler;
    use hilavitkutin_api::Resource;

    #[test]
    fn builder_retains_registered_value() {
        // `.with(Resource::new(v))` moves the value onto the staged
        // list rather than dropping it; the value is reachable on the
        // builder afterward.
        let builder = Scheduler::builder().with(Resource::new(42u32));
        assert_eq!(builder.staged.head.into_inner(), 42u32);
    }
}
