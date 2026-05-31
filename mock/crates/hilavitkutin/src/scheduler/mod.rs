//! Scheduler builder + execution plan (domain 23).
//!
//! Static composition (R6): all WUs registered at compile time.
//! No runtime registration.
//!
//! `SchedulerBuilder<Wus, Stores, Platform, Vals, WuVals>` carries a
//! phantom-tuple type-state plus two real value fields: the
//! `store_values` list and the `wu_values` list. `Wus` accumulates
//! registered WU types (cons-list). `Stores` accumulates registered
//! `Resource<T>` / `Column<T>` / `Virtual<T>` markers (cons-list).
//! `Platform` accumulates platform-provider types. `Vals` retains the
//! registered store VALUES (the `Resource<T>` carrier, the `Column<T>`
//! / `Virtual<T>` markers) in `Stores`-aligned order so the arena drain
//! can move them into scheduler-owned storage at `build()`. `WuVals`
//! retains the registered WorkUnit instances so `build()` can carry
//! them into the `Scheduler`, where `run()` walks them.
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

use arvo::strategy::Identity;
use arvo::USize;
use arvo_tensor::Capacity;
use crate::plan::project::BundleProject;
use crate::plan::{compute_execution_plan, plan_inputs_from_bundle, DefaultPlanDims, PlanDims};
use hilavitkutin_api::access::{AccessSet, ContainsAll, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, Dispatch};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::{DefaultRunCfg, PlanAffecting, RunCfg};
use hilavitkutin_api::store::Replaceable;
use hilavitkutin_api::store_values::{Place, RouterKind, StoreValues, SvEmpty};
use hilavitkutin_api::work_unit::WorkUnitBundle;
use hilavitkutin_api::work_unit_values::WuNil;
use hilavitkutin_api::{ColumnStorage, ColumnValue, StoreId};

use crate::dispatch::fiber_codegen::{noop_fiber_shim, CollectFiber, FiberSlot};
use crate::dispatch::morsel::MorselRange;

pub mod metrics;
pub mod plan;

pub use metrics::SchedulerMetrics;
pub use plan::PlanCache;

use crate::resource::arena::{ArenaFor, DrainStores};

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
    /// The plan stage could not produce a valid execution plan from the
    /// registered bundle (a dependency cycle, surfaced by
    /// `compute_execution_plan` as `PlanError::Cycle`, or another
    /// feasibility failure). The plan is computed before any allocation,
    /// so no block is allocated on this path.
    PlanFailed,
}

/// Convenience alias for a built scheduler over the default run-config.
pub type BuiltScheduler<WuVals, Vals, CS> = Scheduler<DefaultRunCfg, WuVals, Vals, CS>;

/// Compute the topological dispatch permutation for a registered bundle.
///
/// Projects the `Wus` bundle into `PlanInputs` over the `Stores` access set
/// (resource-only this slice, so the record count is zero), runs
/// `compute_execution_plan` over `DefaultPlanDims`, and reads the topological
/// permutation off `unit_meta`: `topo_order[step]` is the registration-list
/// position of the unit dispatched at topological step `step`. Returns the
/// permutation plus the live unit count, or `BuildError::PlanFailed` on a
/// plan-stage failure (a dependency cycle). Computed before any allocation,
/// so a plan failure allocates nothing.
fn compute_topo_order<Wus, Stores, BWit>() -> notko::Outcome<
    (
        <<DefaultPlanDims as PlanDims>::Units as Capacity>::Array<USize>,
        USize,
    ),
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
    let inputs = plan_inputs_from_bundle::<
        Wus,
        Stores,
        BWit,
        <DefaultPlanDims as PlanDims>::Units,
        <DefaultPlanDims as PlanDims>::Stores,
    >(USize::ZERO);
    match compute_execution_plan::<DefaultPlanDims>(&inputs) {
        notko::Outcome::Ok(plan) => {
            let mut order =
                <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(USize::ZERO);
            let meta = plan.unit_meta.as_ref();
            let mut u = 0;
            while u < plan.unit_count.0 {
                order.as_mut()[u] = meta[u].id.index();
                u += 1;
            }
            notko::Outcome::Ok((order, plan.unit_count))
        }
        notko::Outcome::Err(_) => notko::Outcome::Err(BuildError::PlanFailed),
    }
}

/// Top-level scheduler.
///
/// Generic over the consumer's `RunCfg`, the retained WorkUnit-value
/// list `WuVals`, the registered store-value list `Vals`, and the
/// `ColumnStorage` `CS` that backs the resource data plane. `Cfg::Out`
/// parameterises `run()`'s return shape. The scheduler owns the resource
/// arena (`<Vals as ArenaFor>::Arena`, raw pointers into store columns)
/// and the store itself; the store frees every resource block on its own
/// `Drop`, so the scheduler needs no `Drop` of its own. It also holds the
/// registered WorkUnit instances on `WuVals`, the value-carrying unit
/// list `run()` walks.
pub struct Scheduler<
    Cfg: RunCfg = DefaultRunCfg,
    WuVals = WuNil,
    Vals: StoreValues + ArenaFor = SvEmpty,
    CS: ColumnStorage = NullColumnStorage,
    D: PlanDims = DefaultPlanDims,
> {
    _cfg: PhantomData<Cfg>,
    /// The plan's topological dispatch permutation, computed at `build`.
    /// `topo_order[step]` is the registration-list position of the unit
    /// dispatched at topological step `step`; `run` walks the live prefix
    /// `topo_order[0 .. topo_count]`. Sized by the unit-capacity dimension,
    /// so `D` is named by a real field and the scheduler needs no
    /// `PhantomData<D>`.
    topo_order: <D::Units as Capacity>::Array<USize>,
    /// How many of `topo_order`'s entries are live (the registered unit
    /// count). The tail past it is the zero-fill the array carries.
    topo_count: USize,
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
    /// values at `build()`. Holds only `Copy` pointers into the store's
    /// reserved columns; no destructor walk on drop.
    arena: <Vals as ArenaFor>::Arena,
    /// The `ColumnStorage` that backs the resource arena. Owns the
    /// reserved column memory and frees it on its own `Drop`.
    storage: CS,
    /// Registered WorkUnit instances, retained from the builder in
    /// registration order. `run()` walks this value-carrying unit list.
    wu_values: WuVals,
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

/// Null column storage: the default `CS` for a bare `Scheduler` type.
///
/// Reserves nothing and hands back null pointers. It exists so the
/// `Scheduler` type has a default `CS` parameter for type-level uses
/// (alias defaults, turbofish-free naming). A scheduler that actually
/// owns resources is always built with a real store via
/// `build(storage)`. Reserving on it fails, which is correct: a no-store
/// scheduler registers no resources, so the drain never reserves.
pub struct NullColumnStorage;

impl Default for NullColumnStorage {
    fn default() -> Self {
        NullColumnStorage
    }
}

impl ColumnStorage for NullColumnStorage {
    type Error = ();

    fn reserve<T: ColumnValue>(
        &mut self,
        _id: StoreId,
        _len: USize,
    ) -> notko::Outcome<(), ()> {
        notko::Outcome::Err(())
    }

    unsafe fn column_ptr<T: ColumnValue>(&self, _id: StoreId) -> *const T {
        core::ptr::null()
    }

    unsafe fn column_ptr_mut<T: ColumnValue>(&self, _id: StoreId) -> *mut T {
        core::ptr::null_mut()
    }

    fn count(&self, _id: StoreId) -> USize {
        USize::ZERO
    }

    fn release(&mut self, _id: StoreId) {}
}

impl Scheduler<DefaultRunCfg, WuNil, SvEmpty, NullColumnStorage> {
    /// Start a fresh builder. Empty Wus + Stores + Platform typestate,
    /// empty store-value and WorkUnit-value lists; the builder grows via
    /// `.with(...)`.
    pub const fn builder() -> SchedulerBuilder<Empty, Empty, Empty, SvEmpty, WuNil> {
        SchedulerBuilder {
            store_values: SvEmpty,
            wu_values: WuNil,
            _phantom: PhantomData,
        }
    }
}

impl<Cfg: RunCfg, WuVals, Vals: StoreValues + ArenaFor, CS: ColumnStorage, D: PlanDims>
    Scheduler<Cfg, WuVals, Vals, CS, D>
{
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

    /// Dispatch the retained WorkUnit instances in the plan's topological
    /// order over a single full-range morsel, then return
    /// `Cfg::Out::default()`.
    ///
    /// `build` stored the topological permutation on the scheduler. This
    /// method materialises the type-erased slot array from the retained
    /// units via `CollectFiber` (one `FiberSlot` per unit, the unused tail
    /// filled with `noop_fiber_shim` placeholders), then walks
    /// `topo_order[0 .. topo_count]`, dispatching each step's slot. Each
    /// slot's shim projects that unit's `EngineCtx` from the arena and runs
    /// `execute`. Resource-only this slice. The `Witnesses` parameter is the
    /// per-unit projection-index list, inferred at the call site, so
    /// `scheduler.run()` needs no turbofish.
    ///
    /// The slot array is rebuilt each call because a stored instance pointer
    /// would make the scheduler self-referential. The real morsel loop
    /// (plan-dirty check, per-core dispatch build, executor spawn, morsel
    /// walk, phase barriers, meta-WU firing, persistence drain) waits on the
    /// `codegen_fiber` / `codegen_core` LLVM tier. The full real-body
    /// contract lives in `Scheduler::run real morsel loop body` in
    /// `BACKLOG.md.tmpl`.
    pub fn run<Witnesses>(&mut self) -> Cfg::Out
    where
        Cfg::Out: Default,
        WuVals: CollectFiber<<Vals as ArenaFor>::Arena, Witnesses>,
    {
        let _ = &self.plan_dirty;
        let _ = &self.plan_cache;
        // Materialise the erased dispatch slots from the retained units. The
        // tail past the live unit count keeps its `noop_fiber_shim`
        // placeholder and is never dispatched (the walk reads only the
        // `topo_count` live prefix).
        let placeholder: FiberSlot<<Vals as ArenaFor>::Arena> =
            (core::ptr::null(), noop_fiber_shim::<<Vals as ArenaFor>::Arena>);
        let mut slots = <D::Units as Capacity>::filled(placeholder);
        self.wu_values.collect(slots.as_mut());
        let morsel = MorselRange::new(USize::ZERO, USize::ZERO);
        let order = self.topo_order.as_ref();
        let live = slots.as_ref();
        let count = self.topo_count.0;
        let mut step = 0;
        while step < count {
            // `topo_order[step]` is the registration-list position of the
            // unit to dispatch at this step, which equals its slot index:
            // the builder prepends the unit bundle and the value list in
            // lockstep. The slot's shim performs the back-cast under its own
            // `// SAFETY:` note.
            let pos = order[step].0;
            let (ptr, shim) = live[pos];
            shim(ptr, &self.arena, morsel);
            step += 1;
        }
        Cfg::Out::default()
    }

    /// Borrow the resource arena. Hidden test accessor: lets in-crate
    /// and integration tests walk the arena nodes to confirm the
    /// moved-in resource values. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __arena(&self) -> &<Vals as ArenaFor>::Arena {
        &self.arena
    }

    /// Borrow the backing store. Hidden test accessor mirroring
    /// `__arena`: lets tests inspect reserved columns. The field is also
    /// held for its `Drop`, which frees every reserved resource column.
    /// Not part of the supported surface.
    #[doc(hidden)]
    pub fn __storage(&self) -> &CS {
        &self.storage
    }
}

/// Default-construct an empty scheduler over the null store.
///
/// Only available for the no-store (`SvEmpty`) shape with the
/// `NullColumnStorage`: the empty arena (`ArenaTail`) owns nothing and
/// the null store reserves nothing, so no real store is needed. A
/// scheduler that owns resources is built via `build(storage)`.
impl<Cfg: RunCfg> Default for Scheduler<Cfg, WuNil, SvEmpty, NullColumnStorage> {
    fn default() -> Self {
        Self {
            _cfg: PhantomData,
            topo_order: <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(USize::ZERO),
            topo_count: USize::ZERO,
            plan_dirty: [const { AtomicBool::new(false) }; 256],
            plan_cache: PlanCache::new(),
            arena: crate::resource::arena::ArenaTail,
            storage: NullColumnStorage,
            wu_values: WuNil,
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
pub struct SchedulerBuilder<Wus, Stores, Platform, Vals: StoreValues, WuVals> {
    store_values: Vals,
    wu_values: WuVals,
    _phantom: PhantomData<(Wus, Stores, Platform)>,
}

impl<Wus, Stores, Platform, Vals: StoreValues, WuVals>
    SchedulerBuilder<Wus, Stores, Platform, Vals, WuVals>
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
    /// arena drain); WorkUnit inputs prepend their instance onto
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
    >
    where
        P: BuilderInput,
        P::Dispatch: Dispatch<Wus, Stores, Platform> + RouterKind,
        <P::Dispatch as RouterKind>::Kind: Place<P>,
    {
        let (store_values, wu_values) =
            <<P::Dispatch as RouterKind>::Kind as Place<P>>::place(
                provider,
                self.store_values,
                self.wu_values,
            );
        SchedulerBuilder {
            store_values,
            wu_values,
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

impl<Wus, Stores, Platform, Vals, WuVals> SchedulerBuilder<Wus, Stores, Platform, Vals, WuVals>
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
    /// Walks `Stores` and `store_values` in lockstep, reserving each
    /// `Resource<T>`'s one-record column via `storage` and recording its
    /// pointer in the arena. Returns `Err(BuildError::AllocationFailed)`
    /// if any reservation fails; the store frees every column reserved
    /// before the failure when it drops at the end of this call.
    pub fn build<BWit, CS: ColumnStorage>(
        self,
        storage: CS,
    ) -> notko::Outcome<Scheduler<DefaultRunCfg, WuVals, Vals, CS>, BuildError>
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
        // store arena, so a dependency cycle returns without allocating.
        let (topo_order, topo_count) = match compute_topo_order::<Wus, Stores, BWit>() {
            notko::Outcome::Ok(pair) => pair,
            notko::Outcome::Err(e) => return notko::Outcome::Err(e),
        };
        let mut storage = storage;
        let mut next_id = USize::ZERO;
        match <Vals as DrainStores>::drain(self.store_values, &mut storage, &mut next_id) {
            notko::Outcome::Ok(arena) => notko::Outcome::Ok(Scheduler {
                _cfg: PhantomData,
                topo_order,
                topo_count,
                plan_dirty: [const { AtomicBool::new(false) }; 256],
                plan_cache: PlanCache::new(),
                arena,
                storage,
                wu_values,
            }),
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
    ) -> notko::Outcome<Scheduler<Cfg, WuVals, Vals, CS>, BuildError>
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
        // store arena, so a dependency cycle returns without allocating.
        let (topo_order, topo_count) = match compute_topo_order::<Wus, Stores, BWit>() {
            notko::Outcome::Ok(pair) => pair,
            notko::Outcome::Err(e) => return notko::Outcome::Err(e),
        };
        let mut storage = storage;
        let mut next_id = USize::ZERO;
        match <Vals as DrainStores>::drain(self.store_values, &mut storage, &mut next_id) {
            notko::Outcome::Ok(arena) => notko::Outcome::Ok(Scheduler {
                _cfg: PhantomData,
                topo_order,
                topo_count,
                plan_dirty: [const { AtomicBool::new(false) }; 256],
                plan_cache: PlanCache::new(),
                arena,
                storage,
                wu_values,
            }),
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}
