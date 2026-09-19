//! `SchedulerBuilder`: the type-state builder that accumulates registrations
//! and finalises into a `Scheduler`, plus `Scheduler::builder()`.
//!
//! Split out of `scheduler/mod.rs` (file-size lint). No behaviour change.
//! `super::Scheduler`'s fields, `WorkerCtx` and `empty_pool_frame` stay
//! defined directly in `scheduler` (`mod.rs`): this module is a descendant,
//! so it reaches them without any widening. Calls `compute_plan`,
//! `derive_phase_dispatch_order` and `store_plan`, all `pub(super)` in
//! `scheduler::plan_handle` (a sibling).

use core::cell::Cell;
use core::marker::{PhantomData, PhantomPinned};
use core::sync::atomic::{AtomicBool, AtomicUsize};

use arvo::USize;
use arvo::strategy::Identity;
use hilavitkutin_api::ColumnStorage;
use hilavitkutin_api::access::{AccessSet, ContainsAll, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, Dispatch};
use hilavitkutin_api::platform::ClockApi;
use hilavitkutin_api::run_cfg::{DefaultRunCfg, RunCfg};
use hilavitkutin_api::store_values::{Place, RouterKind, StoreValues, SvEmpty};
use hilavitkutin_api::work_unit::WorkUnitBundle;
use hilavitkutin_api::work_unit_values::{WuAppend, WuNil};

use super::plan_handle::{compute_plan, derive_phase_dispatch_order, store_plan};
use super::{BuildError, DefaultClock, Scheduler, WorkerCtx, empty_pool_frame};
use crate::plan::grouping::{GATE2_MAX_ACCUMS, GATE2_MAX_UNITS};
use crate::plan::project::{AccumStoresMask, BundleProject, StoreSizes};
use crate::plan::{DefaultPlanDims, MorselBudget, PlanDims};
use crate::resource::bindings::{BindingsFor, DrainStores};
use crate::thread::class::MAX_CORES;

impl Scheduler<DefaultRunCfg, WuNil, SvEmpty, super::NullColumnStorage> {
    /// Start a fresh builder. Empty Wus + Stores + Platform typestate,
    /// empty store-value and WorkUnit-value lists; the builder grows via
    /// `.with(...)`.
    pub const fn builder() -> SchedulerBuilder<Empty, Empty, Empty, SvEmpty, WuNil, DefaultClock> {
        SchedulerBuilder {
            store_values: SvEmpty,
            wu_values:    WuNil,
            clock:        DefaultClock::new(),
            _phantom:     PhantomData,
        }
    }
}

/// Builder for `Scheduler`. Accumulates WU, store, and platform types
/// in a phantom-tuple type-state, and retains the registered store
/// values on the `StoreValues` list.
///
/// `Wus` is a cons-list of registered WU types. `Stores` is a
/// cons-list of registered store markers. `Platform` is a cons-list
/// of registered platform-provider types. `StoreValues` carries the
/// store VALUES aligned with `Stores`. All start empty from
/// `Scheduler::builder()` and grow via `.with(...)`.
pub struct SchedulerBuilder<Wus, Stores, Platform, Vals: StoreValues, WuVals, Clk = DefaultClock> {
    store_values: Vals,
    wu_values:    WuVals,
    clock:        Clk,
    _phantom:     PhantomData<(Wus, Stores, Platform)>,
}

impl<Wus, Stores, Platform, Vals: StoreValues, WuVals, Clk>
    SchedulerBuilder<Wus, Stores, Platform, Vals, WuVals, Clk>
{
    /// Register one provider on the scheduler.
    ///
    /// Accepts any `P: BuilderInput`: WorkUnit unit-structs, Kits,
    /// `Resource::new(value)`, `Column::<T>::new()`,
    /// `Virtual::<T>::new()`, `ExtensionSurface::<TraitFamily>::new()`,
    /// and platform impls. The per-kind typestate update flows through
    /// `P::Dispatch` and lands on the appropriate accumulator. The
    /// registered value routes through the `RouterKind` tag plus the
    /// `Place<P>` view, which routes onto both retained lists at once:
    /// store inputs prepend their value onto `store_values` (for the
    /// bindings drain); WorkUnit inputs prepend their instance onto
    /// `wu_values` (for the run walk); platform and run-config inputs
    /// drop their value (their TYPE is tracked in the typestate).
    ///
    /// Non-`BuilderInput` values fail the trait solver here, surfacing
    /// the `BuilderInput` `#[diagnostic::on_unimplemented]` message.
    pub fn with<P>(
        self,
        provider: P,
    ) -> SchedulerBuilder<
        <P::Dispatch as Dispatch<Wus, Stores, Platform>>::NextWus,
        <P::Dispatch as Dispatch<Wus, Stores, Platform>>::NextStores,
        <P::Dispatch as Dispatch<Wus, Stores, Platform>>::NextPlatform,
        <<P::Dispatch as RouterKind>::Kind as Place<P>>::NextStores<Vals>,
        <<P::Dispatch as RouterKind>::Kind as Place<P>>::NextWus<WuVals>,
        Clk,
    >
    where
        P: BuilderInput,
        P::Dispatch: Dispatch<Wus, Stores, Platform> + RouterKind,
        <P::Dispatch as RouterKind>::Kind: Place<P>,
        WuVals: WuAppend<P>,
    {
        let (store_values, wu_values) = <<P::Dispatch as RouterKind>::Kind as Place<P>>::place(
            provider,
            self.store_values,
            self.wu_values,
        );
        SchedulerBuilder {
            store_values,
            wu_values,
            clock: self.clock,
            _phantom: PhantomData,
        }
    }

    /// Replace the clock provider the built scheduler samples for the
    /// pass-duration EMA (E8 adapt).
    ///
    /// The slot starts on `DefaultClock` (`OsClock` under the default
    /// `platform-os` feature, the null clock otherwise); a no_os consumer
    /// supplies its own here (the DI path), and a test supplies a scripted
    /// clock for deterministic assertions. A dedicated method rather than a
    /// `with(...)` routing case because the clock VALUE must be retained
    /// (platform inputs through `with` drop their value and track only the
    /// type).
    pub fn clock<C2: ClockApi>(
        self,
        clock: C2,
    ) -> SchedulerBuilder<Wus, Stores, Platform, Vals, WuVals, C2> {
        SchedulerBuilder {
            store_values: self.store_values,
            wu_values: self.wu_values,
            clock,
            _phantom: PhantomData,
        }
    }

    /// Borrow the retained store-value list. Hidden test accessor: lets
    /// the value-retention test confirm a registered `Resource` value
    /// survived `.with`. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __store_values(&self) -> &Vals {
        &self.store_values
    }
}

impl<Wus, Stores, Platform, Vals, WuVals, Clk>
    SchedulerBuilder<Wus, Stores, Platform, Vals, WuVals, Clk>
where
    Wus: WorkUnitBundle,
    Stores: AccessSet
        + ContainsAll<<Wus as WorkUnitBundle>::AccumRead>
        + ContainsAll<<Wus as WorkUnitBundle>::AccumWrite>
        + AccumStoresMask<<DefaultPlanDims as PlanDims>::Stores>
        + StoreSizes<<DefaultPlanDims as PlanDims>::Stores>,
    Vals: StoreValues + BindingsFor + DrainStores,
{
    /// Finalise the builder into a `Scheduler<DefaultRunCfg, Stores, M>`.
    ///
    /// Carries `Stores: ContainsAll<Wus::AccumRead> +
    /// ContainsAll<Wus::AccumWrite>` as its where-clause. A registered
    /// WU referencing an unregistered store produces a compile error
    /// pointing at the missing store.
    ///
    /// Walks `Stores` and `store_values` in lockstep, reserving each
    /// `Resource<T>`'s one-record column via `storage` and recording its
    /// pointer in the bindings. Returns `Err(BuildError::AllocationFailed)`
    /// if any reservation fails; the store frees every column reserved
    /// before the failure when it drops at the end of this call.
    pub fn build<BWit, CS: ColumnStorage>(
        self,
        storage: CS,
        record_count: USize,
    ) -> notko::Outcome<
        Scheduler<DefaultRunCfg, WuVals, Vals, CS, DefaultPlanDims, Stores, Clk>,
        BuildError,
    >
    where
        Wus: BundleProject<
                Stores,
                BWit,
                <DefaultPlanDims as PlanDims>::Units,
                <DefaultPlanDims as PlanDims>::Stores,
            >,
    {
        let wu_values = self.wu_values;
        // Compute the plan from the registered bundle before draining the
        // store bindings, so a dependency cycle returns without allocating.
        // The morsel budget comes from the run-config consts (domain 12).
        let budget = MorselBudget {
            l1_usable:  DefaultRunCfg::L1_USABLE,
            min_morsel: DefaultRunCfg::MIN_MORSEL,
            max_morsel: DefaultRunCfg::MAX_MORSEL,
        };
        let plan = match compute_plan::<Wus, Stores, BWit>(record_count, budget) {
            notko::Outcome::Ok(p) => p,
            notko::Outcome::Err(e) => return notko::Outcome::Err(e),
        };
        let (topo_order, topo_count, fiber_dispatch, fiber_dispatch_count) =
            derive_phase_dispatch_order(&plan);
        let mut storage = storage;
        let mut next_id = <USize as Identity<arvo::strategy::Additive>>::IDENTITY;
        match <Vals as DrainStores>::drain(
            self.store_values,
            &mut storage,
            &mut next_id,
            record_count,
        ) {
            notko::Outcome::Ok(bindings) => {
                // Store-back the plan's flat pools at the `StoreId` namespace
                // continued past the resource columns the drain reserved.
                let plan_handle = match store_plan(&plan, &mut storage, next_id) {
                    notko::Outcome::Ok(h) => h,
                    notko::Outcome::Err(e) => return notko::Outcome::Err(e),
                };
                notko::Outcome::Ok(Scheduler {
                    _cfg: PhantomData,
                    _stores: PhantomData,
                    topo_order,
                    topo_count,
                    plan_handle,
                    record_count,
                    fiber_dispatch,
                    fiber_dispatch_count,
                    plan_dirty: <<DefaultPlanDims as PlanDims>::PlanAffecting as arvo_tensor::Capacity>::from_fn(
                        |_| AtomicBool::new(false),
                    ),
                    plan_cache: super::PlanCache::new(),
                    predecessor_masks: plan.predecessor_masks,
                    read_masks: plan.read_masks,
                    store_dirty: Cell::new(super::AccessMask::empty()),
                    first_frame: AtomicBool::new(true),
                    virtual_epoch: AtomicUsize::new(0),
                    meta_block: crate::meta::MetaBlock::default(),
                    clock: self.clock,
                    bindings,
                    storage,
                    wu_values,
                    pool: empty_pool_frame(),
                    worker_ctxs: [const {
                        WorkerCtx {
                            sched:   core::ptr::null(),
                            core_id: 0,
                        }
                    }; MAX_CORES], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: worker ctx init, core index; tracked: #121
                    spawned: arvo::Bool::FALSE,
                    gate2_phase: [<USize as Identity<arvo::strategy::Additive>>::IDENTITY; GATE2_MAX_UNITS],
                    gate2_trunk: [<USize as Identity<arvo::strategy::Additive>>::IDENTITY; GATE2_MAX_UNITS],
                    gate2_n: <USize as Identity<arvo::strategy::Additive>>::IDENTITY,
                    gate2_nphases: <USize as Identity<arvo::strategy::Additive>>::IDENTITY,
                    gate2_ncores: <USize as Identity<arvo::strategy::Additive>>::IDENTITY,
                    gate2_accum_live: [const { core::sync::atomic::AtomicUsize::new(0) };
                        MAX_CORES * GATE2_MAX_ACCUMS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic publish array init; tracked: #121
                    phase_ema: [const { Cell::new(hilavitkutin_api::platform::Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase EMA zero-init; tracked: #121
                    phase_accum: [const { Cell::new(hilavitkutin_api::platform::Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-frame accumulator reset; tracked: #121
                    adapt_reconfigure: Cell::new(arvo::Bool::FALSE),
                    _pin: PhantomPinned,
                })
            },
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }

    /// Finalise the builder with an explicit `RunCfg` type.
    ///
    /// Used when the consumer registered a custom `RunCfg` via
    /// `.with(MyRunCfg)`; the explicit type parameter threads the
    /// `Cfg::Out` shape through `Scheduler::run()`.
    pub fn build_with<Cfg: RunCfg, BWit, CS: ColumnStorage>(
        self,
        storage: CS,
        record_count: USize,
    ) -> notko::Outcome<Scheduler<Cfg, WuVals, Vals, CS, DefaultPlanDims, Stores, Clk>, BuildError>
    where
        Wus: BundleProject<
                Stores,
                BWit,
                <DefaultPlanDims as PlanDims>::Units,
                <DefaultPlanDims as PlanDims>::Stores,
            >,
    {
        let wu_values = self.wu_values;
        // Compute the plan from the registered bundle before draining the
        // store bindings, so a dependency cycle returns without allocating.
        // The morsel budget comes from the consumer's run-config consts.
        let budget = MorselBudget {
            l1_usable:  Cfg::L1_USABLE,
            min_morsel: Cfg::MIN_MORSEL,
            max_morsel: Cfg::MAX_MORSEL,
        };
        let plan = match compute_plan::<Wus, Stores, BWit>(record_count, budget) {
            notko::Outcome::Ok(p) => p,
            notko::Outcome::Err(e) => return notko::Outcome::Err(e),
        };
        let (topo_order, topo_count, fiber_dispatch, fiber_dispatch_count) =
            derive_phase_dispatch_order(&plan);
        let mut storage = storage;
        let mut next_id = <USize as Identity<arvo::strategy::Additive>>::IDENTITY;
        match <Vals as DrainStores>::drain(
            self.store_values,
            &mut storage,
            &mut next_id,
            record_count,
        ) {
            notko::Outcome::Ok(bindings) => {
                // Store-back the plan's flat pools at the `StoreId` namespace
                // continued past the resource columns the drain reserved.
                let plan_handle = match store_plan(&plan, &mut storage, next_id) {
                    notko::Outcome::Ok(h) => h,
                    notko::Outcome::Err(e) => return notko::Outcome::Err(e),
                };
                notko::Outcome::Ok(Scheduler {
                    _cfg: PhantomData,
                    _stores: PhantomData,
                    topo_order,
                    topo_count,
                    plan_handle,
                    record_count,
                    fiber_dispatch,
                    fiber_dispatch_count,
                    plan_dirty: <<DefaultPlanDims as PlanDims>::PlanAffecting as arvo_tensor::Capacity>::from_fn(
                        |_| AtomicBool::new(false),
                    ),
                    plan_cache: super::PlanCache::new(),
                    predecessor_masks: plan.predecessor_masks,
                    read_masks: plan.read_masks,
                    store_dirty: Cell::new(super::AccessMask::empty()),
                    first_frame: AtomicBool::new(true),
                    virtual_epoch: AtomicUsize::new(0),
                    meta_block: crate::meta::MetaBlock::default(),
                    clock: self.clock,
                    bindings,
                    storage,
                    wu_values,
                    pool: empty_pool_frame(),
                    worker_ctxs: [const {
                        WorkerCtx {
                            sched:   core::ptr::null(),
                            core_id: 0,
                        }
                    }; MAX_CORES], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: worker ctx init, core index; tracked: #121
                    spawned: arvo::Bool::FALSE,
                    gate2_phase: [<USize as Identity<arvo::strategy::Additive>>::IDENTITY; GATE2_MAX_UNITS],
                    gate2_trunk: [<USize as Identity<arvo::strategy::Additive>>::IDENTITY; GATE2_MAX_UNITS],
                    gate2_n: <USize as Identity<arvo::strategy::Additive>>::IDENTITY,
                    gate2_nphases: <USize as Identity<arvo::strategy::Additive>>::IDENTITY,
                    gate2_ncores: <USize as Identity<arvo::strategy::Additive>>::IDENTITY,
                    gate2_accum_live: [const { core::sync::atomic::AtomicUsize::new(0) };
                        MAX_CORES * GATE2_MAX_ACCUMS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic publish array init; tracked: #121
                    phase_ema: [const { Cell::new(hilavitkutin_api::platform::Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase EMA zero-init; tracked: #121
                    phase_accum: [const { Cell::new(hilavitkutin_api::platform::Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-frame accumulator reset; tracked: #121
                    adapt_reconfigure: Cell::new(arvo::Bool::FALSE),
                    _pin: PhantomPinned,
                })
            },
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}
