//! Scheduler builder + execution plan (domain 23).
//!
//! Static composition (R6): all WUs registered at compile time.
//! No runtime registration.
//!
//! `SchedulerBuilder<Wus, Stores, Platform, StoreValues>` carries a
//! phantom-tuple type-state plus one real field: the `store_values`
//! list. `Wus` accumulates registered WU types (cons-list). `Stores`
//! accumulates registered `Resource<T>` / `Column<T>` / `Virtual<T>`
//! markers (cons-list). `Platform` accumulates platform-provider
//! types. `StoreValues` retains the registered store VALUES (the
//! `Resource<T>` carrier, the `Column<T>` / `Virtual<T>` markers) in
//! `Stores`-aligned order so the arena drain can move them into
//! scheduler-owned storage at `build()`.
//!
//! `.build(memory_provider)` carries `Stores: ContainsAll<Wus::AccumRead>
//! + ContainsAll<Wus::AccumWrite>`, which proves at compile time that
//! every registered WU's `Read` and `Write` membership is satisfied by
//! the registered stores. It walks `Stores` and `store_values` in
//! lockstep, allocating each `Resource<T>`'s block via the supplied
//! `MemoryProviderApi` and recording its `ResourcePtr<T>` in the arena.
//!
//! Round 4 reshape: dropped `MAX_UNITS` / `MAX_STORES` / `MAX_LANES`
//! const generics. `Scheduler::replace_resource::<T>` lands with a
//! `T: Replaceable` bound.
//!
//! Round 202605091700 reshape: the nine `.add_*` and `.with_*` methods
//! retire in favour of one unified verb, `.with(value)`. Every value
//! passed to `.with` impls the sealed `BuilderInput` trait from
//! `hilavitkutin-api`; the per-kind typestate update flows through
//! `BuilderInput::Dispatch`.
//!
//! Round 202605290018 (B2a): store values route onto a
//! `Stores`-aligned `StoreValues` list under the single `.with` verb
//! via the `RouterKind` tag plus the `Place<P>` view. `Scheduler`
//! gains `<Stores, M>` parameters, an owned resource arena, and a
//! `Drop` that deallocates it. `build` takes the `MemoryProvider` as
//! an argument and returns `Outcome<_, BuildError>`.

use core::marker::PhantomData;
use core::sync::atomic::AtomicBool;

use hilavitkutin_api::access::{AccessSet, ContainsAll, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, Dispatch};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::{DefaultRunCfg, PlanAffecting, RunCfg};
use hilavitkutin_api::store::Replaceable;
use hilavitkutin_api::store_values::{Place, RouterKind, StoreValues, SvEmpty};
use hilavitkutin_api::work_unit::WorkUnitBundle;

pub mod metrics;
pub mod plan;

pub use metrics::SchedulerMetrics;
pub use plan::PlanCache;

use crate::resource::arena::{ArenaFor, DrainStores, DropArena};

/// The default empty store-value list, used as the `Vals` default for a
/// bare `Scheduler` type.
pub use hilavitkutin_api::store_values::SvEmpty as DefaultStoreValues;

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
}

/// Convenience alias for a built scheduler over the default run-config.
pub type BuiltScheduler<Vals, M> = Scheduler<DefaultRunCfg, Vals, M>;

/// Top-level scheduler.
///
/// Generic over the consumer's `RunCfg`, the registered store-value
/// list `Vals`, and the host `MemoryProvider` `M`. `Cfg::Out`
/// parameterises `run()`'s return shape. The scheduler owns the
/// resource arena (`<Vals as ArenaFor>::Arena`) and the memory provider
/// that backs it; `Drop` runs the arena's destructors and deallocates
/// each block.
pub struct Scheduler<
    Cfg: RunCfg = DefaultRunCfg,
    Vals: StoreValues + ArenaFor = SvEmpty,
    M: MemoryProviderApi = NullMemoryProvider,
> {
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
    /// Scheduler-owned resource arena, built from the registered store
    /// values at `build()`. Held by value so `Drop` can run each
    /// resource's destructor and free its block.
    arena: <Vals as ArenaFor>::Arena,
    /// The host memory provider that backs the arena. Retained so
    /// `Drop` can deallocate every block the arena holds.
    memory_provider: M,
}

/// Null memory provider: the default `M` for a bare `Scheduler` type.
///
/// Every allocation returns null. It exists so the `Scheduler` type
/// has a default `M` parameter for type-level uses (alias defaults,
/// turbofish-free naming). A scheduler that actually owns resources is
/// always built with a real provider via `build(memory_provider)`.
pub struct NullMemoryProvider;

// SAFETY: zero-sized, holds no state; trivially Send + Sync.
unsafe impl Send for NullMemoryProvider {}
unsafe impl Sync for NullMemoryProvider {}

impl Default for NullMemoryProvider {
    fn default() -> Self {
        NullMemoryProvider
    }
}

impl MemoryProviderApi for NullMemoryProvider {
    unsafe fn allocate(&self, _len: arvo::USize, _align: arvo::USize) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI returns raw pointer by contract; tracked: #72
        core::ptr::null_mut()
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: arvo::USize) {} // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer by contract; tracked: #72

    unsafe fn protect(
        &self,
        _ptr: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer by contract; tracked: #72
        _len: arvo::USize,
        _read: arvo::Bool,
        _write: arvo::Bool,
    ) {
    }
}

impl Scheduler<DefaultRunCfg, SvEmpty, NullMemoryProvider> {
    /// Start a fresh builder. Empty Wus + Stores + Platform typestate,
    /// empty store-value list; the builder grows via `.with(...)`.
    pub const fn builder() -> SchedulerBuilder<Empty, Empty, Empty, SvEmpty> {
        SchedulerBuilder {
            store_values: SvEmpty,
            _phantom: PhantomData,
        }
    }
}

impl<Cfg: RunCfg, Vals: StoreValues + ArenaFor, M: MemoryProviderApi> Scheduler<Cfg, Vals, M> {
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
    pub fn run(&mut self) -> Cfg::Out
    where
        Cfg::Out: Default,
    {
        let _ = &self.plan_dirty;
        let _ = &self.plan_cache;
        Cfg::Out::default()
    }

    /// Borrow the resource arena. Hidden test accessor: lets in-crate
    /// and integration tests walk the arena nodes to confirm the
    /// moved-in resource values. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __arena(&self) -> &<Vals as ArenaFor>::Arena {
        &self.arena
    }

    /// Borrow the retained memory provider. Hidden test accessor: lets
    /// tests read the provider's allocation counters. Not part of the
    /// supported surface.
    #[doc(hidden)]
    pub fn __memory_provider(&self) -> &M {
        &self.memory_provider
    }
}

/// Drop the resource arena: run each moved-in resource value's
/// destructor in place, then deallocate its block via the retained
/// provider. The arena is owned by value and dropped once, so no
/// double free.
impl<Cfg: RunCfg, Vals: StoreValues + ArenaFor, M: MemoryProviderApi> Drop
    for Scheduler<Cfg, Vals, M>
{
    fn drop(&mut self) {
        self.arena.drop_arena(&self.memory_provider);
    }
}

/// Default-construct an empty scheduler over the null provider.
///
/// Only available for the no-store (`SvEmpty`) shape with the
/// `NullMemoryProvider`: the empty arena (`ArenaTail`) owns nothing, so
/// `Drop` is a no-op and no real provider is needed. A scheduler that
/// owns resources is built via `build(memory_provider)`.
impl<Cfg: RunCfg> Default for Scheduler<Cfg, SvEmpty, NullMemoryProvider> {
    fn default() -> Self {
        Self {
            _cfg: PhantomData,
            plan_dirty: [const { AtomicBool::new(false) }; 256],
            plan_cache: PlanCache::new(),
            arena: crate::resource::arena::ArenaTail,
            memory_provider: NullMemoryProvider,
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
pub struct SchedulerBuilder<Wus, Stores, Platform, Vals: StoreValues> {
    store_values: Vals,
    _phantom: PhantomData<(Wus, Stores, Platform)>,
}

impl<Wus, Stores, Platform, Vals: StoreValues> SchedulerBuilder<Wus, Stores, Platform, Vals> {
    /// Register one provider on the scheduler.
    ///
    /// Accepts any `P: BuilderInput`: WorkUnit unit-structs, Kits,
    /// `Resource::new(value)`, `Column::<T>::new()`,
    /// `Virtual::<T>::new()`, `ExtensionSurface::<TraitFamily>::new()`,
    /// and platform impls. The per-kind typestate update flows through
    /// `P::Dispatch` and lands on the appropriate accumulator. The
    /// registered value routes through the `RouterKind` tag plus the
    /// `Place<P>` view: store inputs prepend their value onto
    /// `store_values` (retained for the arena drain); WorkUnit and
    /// platform inputs drop their value (their TYPE is tracked in the
    /// `Wus` / `Platform` typestate).
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
        <<P::Dispatch as RouterKind>::Kind as Place<P>>::Next<Vals>,
    >
    where
        P: BuilderInput,
        P::Dispatch: Dispatch<Wus, Stores, Platform> + RouterKind,
        <P::Dispatch as RouterKind>::Kind: Place<P>,
    {
        SchedulerBuilder {
            store_values: <<P::Dispatch as RouterKind>::Kind as Place<P>>::place(
                provider,
                self.store_values,
            ),
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

impl<Wus, Stores, Platform, Vals> SchedulerBuilder<Wus, Stores, Platform, Vals>
where
    Wus: WorkUnitBundle,
    Stores: AccessSet
        + ContainsAll<<Wus as WorkUnitBundle>::AccumRead>
        + ContainsAll<<Wus as WorkUnitBundle>::AccumWrite>,
    Vals: StoreValues + ArenaFor + DrainStores,
{
    /// Finalise the builder into a `Scheduler<DefaultRunCfg, Stores, M>`.
    ///
    /// Carries `Stores: ContainsAll<Wus::AccumRead> +
    /// ContainsAll<Wus::AccumWrite>` as its where-clause. A registered
    /// WU referencing an unregistered store produces a compile error
    /// pointing at the missing store.
    ///
    /// Walks `Stores` and `store_values` in lockstep, allocating each
    /// `Resource<T>`'s block via `memory_provider` and recording its
    /// pointer in the arena. Returns `Err(BuildError::AllocationFailed)`
    /// (after freeing the prefix already built) if any allocation
    /// returns null.
    pub fn build<M: MemoryProviderApi>(
        self,
        memory_provider: M,
    ) -> notko::Outcome<Scheduler<DefaultRunCfg, Vals, M>, BuildError> {
        match <Vals as DrainStores>::drain(self.store_values, &memory_provider) {
            notko::Outcome::Ok(arena) => notko::Outcome::Ok(Scheduler {
                _cfg: PhantomData,
                plan_dirty: [const { AtomicBool::new(false) }; 256],
                plan_cache: PlanCache::new(),
                arena,
                memory_provider,
            }),
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }

    /// Finalise the builder with an explicit `RunCfg` type.
    ///
    /// Used when the consumer registered a custom `RunCfg` via
    /// `.with(MyRunCfg)`; the explicit type parameter threads the
    /// `Cfg::Out` shape through `Scheduler::run()`.
    pub fn build_with<Cfg: RunCfg, M: MemoryProviderApi>(
        self,
        memory_provider: M,
    ) -> notko::Outcome<Scheduler<Cfg, Vals, M>, BuildError> {
        match <Vals as DrainStores>::drain(self.store_values, &memory_provider) {
            notko::Outcome::Ok(arena) => notko::Outcome::Ok(Scheduler {
                _cfg: PhantomData,
                plan_dirty: [const { AtomicBool::new(false) }; 256],
                plan_cache: PlanCache::new(),
                arena,
                memory_provider,
            }),
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}
