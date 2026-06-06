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
//! / `Virtual<T>` markers) in `Stores`-aligned order so the bindings drain
//! can move them into scheduler-owned storage at `build()`. `WuVals`
//! retains the registered WorkUnit instances so `build()` can carry
//! them into the `Scheduler`, where `run()` walks them.
//!
//! `.build(memory_provider)` carries `Stores: ContainsAll<Wus::AccumRead>
//! + ContainsAll<Wus::AccumWrite>`, which proves at compile time that
//! every registered WU's `Read` and `Write` membership is satisfied by
//! the registered stores. It walks `Stores` and `store_values` in
//! lockstep, allocating each `Resource<T>`'s block via the supplied
//! `MemoryProviderApi` and recording its `ResourcePtr<T>` in the bindings.
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
//! gains `<Stores, M>` parameters, an owned resource bindings, and a
//! `Drop` that deallocates it. `build` takes the `MemoryProvider` as
//! an argument and returns `Outcome<_, BuildError>`.

use core::fmt;
use core::marker::PhantomData;
use core::sync::atomic::AtomicBool;

use arvo::strategy::Identity;
use arvo::Bool;
use arvo::USize;
use arvo_bitmask::{BitAccess, BitLogic, BitSequence};
use arvo_tensor::Capacity;
use crate::plan::project::{AccumStoresMask, BundleProject, Locate, WitnessIndex};
use crate::plan::{
    compute_execution_plan, plan_inputs_from_bundle, AccessMask, DefaultPlanDims, ExecutionPlan,
    PlanDims,
};
use hilavitkutin_api::access::{AccessSet, ContainsAll, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, Dispatch};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::{DefaultRunCfg, PlanAffecting, RunCfg};
use hilavitkutin_api::store::Replaceable;
use hilavitkutin_api::store_values::{Place, RouterKind, StoreValues, SvEmpty};
use hilavitkutin_api::work_unit::WorkUnitBundle;
use hilavitkutin_api::work_unit_values::{WuAppend, WuCons, WuNil};
use hilavitkutin_api::{ColumnStorage, ColumnValue, StoreId, UnitId};

use crate::dispatch::fiber_run::RunFiber;
use crate::dispatch::fusion::{ChainWu, FuseCarrier};
use crate::dispatch::morsel::MorselRange;

pub mod metrics;
pub mod plan;

pub use metrics::SchedulerMetrics;
pub use plan::PlanCache;

use crate::resource::bindings::{BindingsFor, DrainStores, ResetAccumulators};

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
        producer: USize,
        /// Carrier slot index of the reader registered before its writer.
        consumer: USize,
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
    fn from_rcm_order(rcm: &[UnitId], count: USize) -> Self {
        let mut order = <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(USize::ZERO);
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
        &self.order.as_ref()[..self.count.0]
    }
}

impl fmt::Debug for RecommendedOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.as_slice().iter().map(|s| s.0)).finish()
    }
}

/// Convenience alias for a built scheduler over the default run-config.
pub type BuiltScheduler<WuVals, Vals, CS> = Scheduler<DefaultRunCfg, WuVals, Vals, CS>;

/// Locator for the store-backed execution plan columns.
///
/// The plan's flat CSR pools live as columns in the scheduler's
/// `ColumnStorage`, reserved at a contiguous `StoreId` range continued past
/// the resource columns. `PlanHandle` is the `Copy` record of where: the base
/// column index plus the live phase / trunk / fiber / unit counts. The plan
/// columns are a closed set, so each column's `StoreId` is a fixed offset off
/// the base, named by `PlanColumn`. The dispatch consumer reads the plan back
/// through these ids and counts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PlanHandle {
    base: USize,
    phase_count: USize,
    trunk_count: USize,
    fiber_count: USize,
    unit_count: USize,
}

/// The closed set of store-backed plan columns. The variant's position is the
/// `StoreId` offset off a `PlanHandle`'s base.
#[derive(Copy, Clone)]
enum PlanColumn {
    Phases,
    Trunks,
    Fibers,
    UnitMeta,
    MorselSizes,
    RcmOrder,
}

impl PlanColumn {
    /// Number of plan columns: the count of `StoreId`s `store_plan` reserves
    /// past the resource base. Must equal the variant count above (a new
    /// variant breaks the `column_id` match, which is the compile-time guard;
    /// this const is the named source for the reservation-span prose). Its
    /// consumers are the `store_plan` doc and the offset-pinning unit test, so
    /// the non-test build sees no use site: that is expected, not drift.
    #[cfg_attr(not(test), allow(dead_code))]
    const COUNT: usize = 6; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: closed-set cardinality used as a reservation span; tracked: #72
}

impl PlanHandle {
    /// The empty handle: no plan store-backed (the bare and `Default`
    /// scheduler, whose store reserves nothing).
    pub const fn empty() -> Self {
        Self {
            base: USize::ZERO,
            phase_count: USize::ZERO,
            trunk_count: USize::ZERO,
            fiber_count: USize::ZERO,
            unit_count: USize::ZERO,
        }
    }

    /// `StoreId` of plan column `c`, a fixed offset off the base.
    fn column_id(&self, c: PlanColumn) -> StoreId {
        // Explicit per-variant offset, not `c as usize`: reordering the enum
        // does not shift the stored offsets, and adding a variant is a
        // compile-forced change here (the match goes non-exhaustive).
        let offset = match c {
            PlanColumn::Phases => 0,
            PlanColumn::Trunks => 1,
            PlanColumn::Fibers => 2,
            PlanColumn::UnitMeta => 3,
            PlanColumn::MorselSizes => 4,
            PlanColumn::RcmOrder => 5,
        }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: closed-set column offsets; tracked: #72
        StoreId(USize(self.base.0 + offset)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: StoreId-construct from base + closed-set column offset; tracked: #72
    }

    /// `StoreId` of the phases column.
    pub fn phases_id(&self) -> StoreId {
        self.column_id(PlanColumn::Phases)
    }
    /// `StoreId` of the trunks column.
    pub fn trunks_id(&self) -> StoreId {
        self.column_id(PlanColumn::Trunks)
    }
    /// `StoreId` of the fibers column.
    pub fn fibers_id(&self) -> StoreId {
        self.column_id(PlanColumn::Fibers)
    }
    /// `StoreId` of the per-unit metadata column.
    pub fn unit_meta_id(&self) -> StoreId {
        self.column_id(PlanColumn::UnitMeta)
    }
    /// `StoreId` of the per-fiber morsel-sizes column.
    pub fn morsel_sizes_id(&self) -> StoreId {
        self.column_id(PlanColumn::MorselSizes)
    }
    /// `StoreId` of the RCM renumber column.
    pub fn rcm_order_id(&self) -> StoreId {
        self.column_id(PlanColumn::RcmOrder)
    }

    /// Live phase count (records in the phases column).
    pub fn phase_count(&self) -> USize {
        self.phase_count
    }
    /// Live trunk count (records in the trunks column).
    pub fn trunk_count(&self) -> USize {
        self.trunk_count
    }
    /// Live fiber count (records in the fibers and morsel-sizes columns).
    pub fn fiber_count(&self) -> USize {
        self.fiber_count
    }
    /// Live unit count (records in the unit-meta and rcm-order columns).
    pub fn unit_count(&self) -> USize {
        self.unit_count
    }
}

/// Compute the execution plan for a registered bundle.
///
/// Projects the `Wus` bundle into `PlanInputs` over the `Stores` access set
/// with the frame `record_count` (the dimension that sizes the per-fiber
/// morsels and selects the phase configs) and runs `compute_execution_plan`
/// over `DefaultPlanDims`. Returns the plan or `BuildError::PlanFailed` on a
/// plan-stage failure (a dependency cycle). Computed before any allocation, so
/// a plan failure allocates nothing.
fn compute_plan<Wus, Stores, BWit>(
    record_count: USize,
) -> notko::Outcome<ExecutionPlan<DefaultPlanDims>, BuildError>
where
    Wus: BundleProject<
        Stores,
        BWit,
        <DefaultPlanDims as PlanDims>::Units,
        <DefaultPlanDims as PlanDims>::Stores,
    >,
    Stores: AccumStoresMask<<DefaultPlanDims as PlanDims>::Stores>,
{
    let inputs = plan_inputs_from_bundle::<
        Wus,
        Stores,
        BWit,
        <DefaultPlanDims as PlanDims>::Units,
        <DefaultPlanDims as PlanDims>::Stores,
    >(record_count);
    // Cycle detection runs first: a dependency cycle has no topological order at
    // all, so it cannot be fixed by reordering registration and stays
    // `PlanFailed`. `compute_execution_plan` succeeding proves the graph acyclic.
    match compute_execution_plan::<DefaultPlanDims>(&inputs) {
        notko::Outcome::Ok(plan) => {
            // Provisional registration constraint (op call b): the static
            // dispatch walk follows the carrier (registration) order directly,
            // so the carrier must already be a topological order. The graph is
            // acyclic here, so any back-edge in registration order is a genuine
            // anti-topological registration (a valid topological order exists,
            // this just is not one). Reject it, naming the offending carrier
            // slots. The plan layer stays order-independent; this is a
            // scheduler-build precondition that relaxes when the engine
            // auto-applies the cache-optimal order. No allocation has happened
            // yet, so a rejected registration allocates nothing.
            if let notko::Maybe::Is((producer, consumer)) =
                crate::plan::steps::first_back_edge::<DefaultPlanDims>(&inputs)
            {
                // The plan is in hand, so name the recommended registration order
                // (the RCM-reordered topological order) without recomputing it.
                let recommended =
                    RecommendedOrder::from_rcm_order(plan.rcm_order.as_ref(), plan.unit_count);
                return notko::Outcome::Err(BuildError::NonTopologicalRegistration {
                    producer,
                    consumer,
                    recommended,
                });
            }
            notko::Outcome::Ok(plan)
        }
        notko::Outcome::Err(_) => notko::Outcome::Err(BuildError::PlanFailed),
    }
}

/// Derive the phase-sequential dispatch order off a computed plan.
///
/// Flattens the plan's phase structure into the per-step dispatch order:
/// `plan.phases[0..phase_count]`, each phase's trunks
/// (`trunks[trunk_offset .. +trunk_count]`), each trunk's fibers
/// (`fibers[fiber_offset .. +fiber_count]`), each fiber's units
/// (`fiber.units[0..unit_count]`). The collected value is each unit's slot
/// index (`UnitId::index`), so `topo_order[step]` remains the registration-list
/// position of the unit dispatched at `step`, now ordered by the plan's
/// phase/trunk/fiber grouping rather than the flat `unit_meta` permutation. The
/// grouping is a topological order (phase boundaries sit where a dependency
/// crosses them; a fiber's units are gathered in topological order), so the
/// dispatch stays dependency-respecting. Returns the order array plus the count
/// of units emitted (the flattened total, which equals the live unit count when
/// the fiber partition is complete).
fn derive_phase_dispatch_order(
    plan: &ExecutionPlan<DefaultPlanDims>,
) -> (
    <<DefaultPlanDims as PlanDims>::Units as Capacity>::Array<USize>,
    USize,
    <<DefaultPlanDims as PlanDims>::Fibers as Capacity>::Array<FiberDispatch>,
    USize,
) {
    let mut order = <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(USize::ZERO);
    let cap = order.as_ref().len();
    let mut descriptors =
        <<DefaultPlanDims as PlanDims>::Fibers as Capacity>::filled(FiberDispatch::default());
    let fd_cap = descriptors.as_ref().len();
    let mut fd = 0;
    let phases = plan.phases.as_ref();
    let trunks = plan.trunks.as_ref();
    let fibers = plan.fibers.as_ref();
    let mut next = 0;
    let mut p = 0;
    while p < plan.phase_count.0 && p < phases.len() {
        let t_end = phases[p].trunk_offset.0 + phases[p].trunk_count.0;
        let mut t = phases[p].trunk_offset.0;
        while t < t_end && t < trunks.len() {
            let f_end = trunks[t].fiber_offset.0 + trunks[t].fiber_count.0;
            let mut f = trunks[t].fiber_offset.0;
            while f < f_end && f < fibers.len() {
                let units = fibers[f].units.as_ref();
                let uc = fibers[f].unit_count.0;
                let fib_start = next;
                let mut u = 0;
                while u < uc && u < units.len() && next < cap {
                    order.as_mut()[next] = units[u].index();
                    next += 1;
                    u += 1;
                }
                if fd < fd_cap {
                    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal dispatch-order cursors; tracked: #72
                    descriptors.as_mut()[fd] = FiberDispatch {
                        start: USize(fib_start),
                        len: USize(next - fib_start),
                        morsel_local: fibers[f].morsel_local,
                    };
                    fd += 1;
                }
                f += 1;
            }
            t += 1;
        }
        p += 1;
    }
    // A complete fiber partition places every registered unit in exactly one
    // fiber, so the flattened total equals the unit count. A mismatch means the
    // plan dropped or duplicated a unit (a silent dispatch error); surface it in
    // debug and test builds rather than dispatching a truncated order.
    debug_assert_eq!(
        next, plan.unit_count.0,
        "phase flatten emitted a different unit count than the plan registered: \
         the fiber partition is incomplete (a unit landed in no fiber, or a \
         capacity guard tripped)"
    );
    // Every fiber must get a descriptor; otherwise `run` would skip the units
    // of a fiber that landed in `order` but past the descriptor capacity. The
    // fiber count is bounded by the same `D::Fibers` budget the descriptor
    // array is sized to, so this holds for any valid plan.
    debug_assert_eq!(
        fd, plan.fiber_count.0,
        "phase flatten emitted a different fiber-descriptor count than the plan \
         registered: a fiber exceeded the descriptor capacity, so its units \
         would dispatch with no descriptor"
    );
    (order, USize(next), descriptors, USize(fd))
}

/// One fiber's slice of the flat dispatch order plus its morsel-locality
/// bit: the compact per-fiber dispatch program `run` walks.
///
/// `run` dispatches a `morsel_local` fiber morsel-outer (one morsel runs the
/// fiber's whole unit sequence before the next, keeping its intermediate
/// columns cache-resident) and an accumulator-bearing fiber unit-outer (a
/// unit completes its record range before the next, the cross-record-safe
/// form).
#[derive(Copy, Clone)]
pub struct FiberDispatch {
    /// Start index of this fiber's units in `topo_order`.
    pub start: USize,
    /// Number of this fiber's units (its slice length in `topo_order`).
    pub len: USize,
    /// True when the fiber writes no accumulator, so it dispatches
    /// morsel-outer.
    pub morsel_local: Bool,
}

impl Default for FiberDispatch {
    #[inline]
    fn default() -> Self {
        Self {
            start: USize::ZERO,
            len: USize::ZERO,
            morsel_local: Bool::TRUE,
        }
    }
}

/// Reserve one plan column and copy its live prefix in.
///
/// Reserves `id` for `count` records of `T`, then copies the first `count`
/// elements of `src` into the reserved column. Maps any reservation failure to
/// `BuildError::AllocationFailed`.
fn store_column<T: ColumnValue, CS: ColumnStorage>(
    storage: &mut CS,
    id: StoreId,
    src: &[T],
    count: USize,
) -> notko::Outcome<(), BuildError> {
    match storage.reserve::<T>(id, count) {
        notko::Outcome::Ok(()) => {}
        notko::Outcome::Err(_) => return notko::Outcome::Err(BuildError::AllocationFailed),
    }
    if count.0 > 0 {
        // SAFETY: `id` was just reserved for `count` records of `T`, so
        // `column_ptr_mut` returns a valid base for `count` writes; `src` is
        // the plan's flat pool, with at least `count` initialised elements
        // (the pool is `Capacity`-sized and `count` is the live prefix). No
        // aliasing read pointer to this freshly reserved column is live.
        unsafe {
            let dst = storage.column_ptr_mut::<T>(id);
            core::ptr::copy_nonoverlapping(src.as_ptr(), dst, count.0);
        }
    }
    notko::Outcome::Ok(())
}

/// Store-back the plan's flat CSR pools as columns at `base .. base +
/// PlanColumn::COUNT`.
///
/// Reserves and copies the phases, trunks, fibers, per-unit metadata,
/// per-fiber morsel sizes, and RCM renumber pools (one column per `PlanColumn`
/// variant), then returns the `PlanHandle` locating them. Per-fiber column
/// classification and the dirty masks stay off the store this round (their
/// columnar form and consumers are later rounds).
fn store_plan<CS: ColumnStorage>(
    plan: &ExecutionPlan<DefaultPlanDims>,
    storage: &mut CS,
    base: USize,
) -> notko::Outcome<PlanHandle, BuildError> {
    let handle = PlanHandle {
        base,
        phase_count: plan.phase_count,
        trunk_count: plan.trunk_count,
        fiber_count: plan.fiber_count,
        unit_count: plan.unit_count,
    };
    match store_column(storage, handle.phases_id(), plan.phases.as_ref(), plan.phase_count) {
        notko::Outcome::Ok(()) => {}
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    match store_column(storage, handle.trunks_id(), plan.trunks.as_ref(), plan.trunk_count) {
        notko::Outcome::Ok(()) => {}
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    match store_column(storage, handle.fibers_id(), plan.fibers.as_ref(), plan.fiber_count) {
        notko::Outcome::Ok(()) => {}
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    match store_column(storage, handle.unit_meta_id(), plan.unit_meta.as_ref(), plan.unit_count) {
        notko::Outcome::Ok(()) => {}
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    match store_column(storage, handle.morsel_sizes_id(), plan.morsel_sizes.as_ref(), plan.fiber_count) {
        notko::Outcome::Ok(()) => {}
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    match store_column(storage, handle.rcm_order_id(), plan.rcm_order.as_ref(), plan.unit_count) {
        notko::Outcome::Ok(()) => {}
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    notko::Outcome::Ok(handle)
}

/// Top-level scheduler.
///
/// Generic over the consumer's `RunCfg`, the retained WorkUnit-value
/// list `WuVals`, the registered store-value list `Vals`, and the
/// `ColumnStorage` `CS` that backs the resource data plane. `Cfg::Out`
/// parameterises `run()`'s return shape. The scheduler owns the resource
/// bindings (`<Vals as BindingsFor>::Bindings`, raw pointers into store columns)
/// and the store itself; the store frees every resource block on its own
/// `Drop`, so the scheduler needs no `Drop` of its own. It also holds the
/// registered WorkUnit instances on `WuVals`, the value-carrying unit
/// list `run()` walks.
pub struct Scheduler<
    Cfg: RunCfg = DefaultRunCfg,
    WuVals = WuNil,
    Vals: StoreValues + BindingsFor = SvEmpty,
    CS: ColumnStorage = NullColumnStorage,
    D: PlanDims = DefaultPlanDims,
    Stores = Empty,
> {
    _cfg: PhantomData<Cfg>,
    /// The registered store access set, retained so `mark_dirty` /
    /// `replace_resource` / `replace_value` can resolve a store type to its
    /// Stores-list bit position (the space `read_masks` / `store_dirty`
    /// index) via the `Locate` witness. Carried as `PhantomData` because the
    /// access set is purely type-level; no runtime value.
    _stores: PhantomData<Stores>,
    /// The plan's topological dispatch permutation, computed at `build`.
    /// `topo_order[step]` is the registration-list position of the unit
    /// dispatched at topological step `step`; `run` walks the live prefix
    /// `topo_order[0 .. topo_count]`. Sized by the unit-capacity dimension,
    /// so `D` is named by a real field and the scheduler needs no
    /// `PhantomData<D>`.
    topo_order: <D::Units as Capacity>::Array<USize>,
    /// How many of `topo_order`'s entries are live: the flattened dispatch
    /// total (equals the registered unit count when the fiber partition is
    /// complete, which `derive_phase_dispatch_order` debug-asserts). The tail
    /// past it is the zero-fill the array carries.
    topo_count: USize,
    /// Locator for the plan's store-backed flat CSR columns (phases, trunks,
    /// fibers, per-unit metadata, per-fiber morsel sizes, the RCM renumber),
    /// reserved in `storage` at a `StoreId` range continued past the resource
    /// columns. `PlanHandle::empty()` when no plan is store-backed (the bare
    /// scheduler). The dispatch consumer reads the plan back through it.
    plan_handle: PlanHandle,
    /// The frame record count fixed at `build`. Input columns are reserved
    /// to it, and `run` windows it into morsels of `RunCfg::MORSEL_SIZE`
    /// (one full-range walk for an accumulator-bearing or record-less
    /// frame).
    record_count: USize,
    /// Per-fiber dispatch descriptor, computed at `build` alongside
    /// `topo_order`. Each live entry slices `topo_order` for one fiber (in
    /// plan dispatch order) and carries that fiber's `morsel_local` bit, so
    /// `run` dispatches a morsel-local fiber morsel-outer (its intermediate
    /// columns stay cache-resident across the morsel) and an
    /// accumulator-bearing fiber unit-outer (the cross-record-safe form). The
    /// per-fiber bit replaces the whole-pipeline accumulator-free guard.
    fiber_dispatch: <D::Fibers as Capacity>::Array<FiberDispatch>,
    /// How many of `fiber_dispatch`'s entries are live.
    fiber_dispatch_count: USize,
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
    /// Per-unit predecessor masks (carrier-position space), copied off the
    /// plan at build. The runtime propagates the dirty seed forward over
    /// these and gates each unit by its position.
    predecessor_masks: <D::Units as Capacity>::Array<D::AdjRow>,
    /// Per-unit read access masks, copied off the plan. A unit is seeded
    /// dirty when its reads intersect the changed-store mask.
    read_masks: <D::Units as Capacity>::Array<AccessMask<D::Stores>>,
    /// Per-store change seed (Stores-list-position space). `mark_dirty`,
    /// `replace_resource`, and `replace_value` set bits here; `run` /
    /// `run_fused` consume and clear it each frame.
    store_dirty: AccessMask<D::Stores>,
    /// Cold-start flag. Every unit is dirty on the first frame after build,
    /// so the first `run` / `run_fused` executes the whole carrier; set
    /// false afterward.
    first_frame: Bool,
    /// Scheduler-owned resource bindings, built from the registered store
    /// values at `build()`. Holds only `Copy` pointers into the store's
    /// reserved columns; no destructor walk on drop.
    bindings: <Vals as BindingsFor>::Bindings,
    /// The `ColumnStorage` that backs the resource bindings. Owns the
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

impl<Cfg: RunCfg, WuVals, Vals: StoreValues + BindingsFor, CS: ColumnStorage, D: PlanDims, Stores>
    Scheduler<Cfg, WuVals, Vals, CS, D, Stores>
{
    /// Replace the existing `Resource<T>` instance in the data
    /// plane with `_new`, marking the plan dirty.
    ///
    /// `T: PlanAffecting` routes the call onto the dirty-marking
    /// path; the next `run()` recomputes the execution plan.
    /// Consumers that need a cheap value swap on a non-plan-
    /// affecting resource use `replace_value`.
    pub fn replace_resource<T: PlanAffecting, Index>(&mut self, _new: T)
    where
        Stores: Locate<T, Index>,
        Index: WitnessIndex,
    {
        // A swapped resource is a changed input, so mark its store dirty for
        // the next frame (domain-16 incremental skip seed).
        self.mark_dirty::<T, Index>();
        // The domain-22 plan-recompute seed (`plan_dirty` by PlanAffectingId)
        // and the data-plane value install are sequenced with the adapt
        // subsystem (runtime plan recompute on resource swap).
        let _ = &self.plan_dirty;
    }

    /// Cheap value-swap path for non-plan-affecting resources.
    ///
    /// `T: Replaceable` opts the type into runtime replacement
    /// without signalling plan recompute. The `Replaceable` marker
    /// is consumer-driven per Topic 8 axis B (replaceable but not
    /// plan-affecting is the typical case for app-level state).
    pub fn replace_value<T: Replaceable, Index>(&mut self, _new: T)
    where
        Stores: Locate<T, Index>,
        Index: WitnessIndex,
    {
        // A swapped value is a changed input: mark its store dirty for the
        // next frame so dependents re-run (domain-16 incremental skip). No
        // plan-recompute dirty, since the value swap is not structural. The
        // data-plane value install is sequenced with the adapt subsystem.
        self.mark_dirty::<T, Index>();
    }

    /// Mark the store named by type `T` changed for the next frame.
    ///
    /// `T` is resolved to its position in the registered `Stores` access
    /// set via the same `Locate` witness the plan projection uses, so its
    /// bit lands in the Stores-list-position space the per-unit read masks
    /// index. The next `run` / `run_fused` seeds every unit reading `T` as
    /// dirty and propagates forward, so only `T`'s transitive cone runs.
    /// `Index` infers at the call site, so `scheduler.mark_dirty::<T>()`
    /// needs no turbofish on the index. The consumer calls this for an
    /// input it mutated directly (a host-populated column or a swapped
    /// resource value tracked elsewhere); `replace_resource` and
    /// `replace_value` call it internally.
    pub fn mark_dirty<T, Index>(&mut self)
    where
        Stores: Locate<T, Index>,
        Index: WitnessIndex,
    {
        self.store_dirty = self.store_dirty.set(Index::INDEX);
    }

    /// This frame's dirty-unit mask for incremental skip (domain 16,
    /// canonical Step 9).
    ///
    /// Seeds every unit whose read set intersects the per-store change
    /// seed (or every unit, on the cold first frame), then propagates the
    /// seed forward over the predecessor masks in carrier (topological)
    /// order: a unit is dirty when directly seeded or any predecessor is
    /// dirty. Positions past the live unit count carry empty masks and stay
    /// clean. The walk over the array length runs the predecessors-before-
    /// dependents single pass because carrier position equals topological
    /// order (`build` validated it).
    fn dirty_units(&self) -> D::AdjRow {
        if self.first_frame.0 {
            // Cold frame: every unit dirty, so the first frame after build
            // executes the whole carrier.
            return D::AdjRow::default().bitnot();
        }
        let reads = self.read_masks.as_ref();
        let preds = self.predecessor_masks.as_ref();
        // Bound the seed and propagate passes to the live unit count, not the
        // full unit-capacity array length: positions past `topo_count` carry
        // empty masks and stay clean, so iterating them is pure per-frame
        // overhead on the hot path. The mask arrays are indexed by carrier
        // position (0..unit_count), and `topo_count` is that live count.
        let n = self.topo_count.0.min(reads.len()).min(preds.len());
        let mut dirty = D::AdjRow::default();
        let mut p = 0;
        while p < n {
            if reads[p].overlaps(&self.store_dirty).0 {
                // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct carrier position; tracked: #72
                dirty = dirty.with_bit_set(USize(p));
            }
            p += 1;
        }
        let mut p = 0;
        while p < n {
            if !preds[p].bitand(dirty).is_zero().0 {
                // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct carrier position; tracked: #72
                dirty = dirty.with_bit_set(USize(p));
            }
            p += 1;
        }
        dirty
    }

    /// Dispatch the retained WorkUnit carrier in carrier order, windowing the
    /// record range into morsels, then return `Cfg::Out::default()`.
    ///
    /// The retained `wu_values` carrier is walked as one type-level `RunFiber`
    /// recursion in carrier (registration) order, which `build()` validated is a
    /// topological order: a consumer registers producer-before-consumer, and an
    /// anti-topological carrier is rejected at `build()`
    /// (`BuildError::NonTopologicalRegistration`). Each `RunFiber` step projects
    /// that unit's `EngineCtx` from the bindings (resources, columns, and
    /// accumulators alike) and runs `execute`; no unit dispatches through a
    /// stored function pointer, so the whole walk monomorphises into one
    /// straight-line body that devirtualises under fat LTO. This is the flat
    /// schedule-mega dispatch (spec Approach E); the per-fiber and per-phase
    /// sub-carrier nesting is a later refinement.
    ///
    /// Drive shape: a carrier that writes no accumulator runs morsel-outer (the
    /// runtime morsel loop of `RunCfg::MORSEL_SIZE`, guarded to at least one
    /// record, wraps the whole-carrier walk so intermediate columns stay
    /// cache-resident per morsel); a carrier bearing an accumulator runs
    /// unit-outer (one full-range walk, the cross-record-safe append path),
    /// which is also the record-less-frame path (the carrier runs once over an
    /// empty morsel so a resource-only unit runs exactly once). The decision
    /// reads the per-fiber `morsel_local` bits the plan computed. The
    /// `Witnesses` parameter is the per-unit projection-index list, inferred at
    /// the call site, so `scheduler.run()` needs no turbofish.
    pub fn run<Witnesses>(&mut self) -> Cfg::Out
    where
        Cfg::Out: Default,
        WuVals: RunFiber<<Vals as BindingsFor>::Bindings, Witnesses>,
        <Vals as BindingsFor>::Bindings: ResetAccumulators,
    {
        // Schedule-once-reuse: zero every accumulator live-length at frame
        // start so this frame appends into a fresh buffer rather than
        // continuing from the prior frame's live offset. No-op for an
        // accumulator-free carrier.
        self.bindings.reset_accumulators();
        // `plan_dirty` / `plan_cache` are the domain-22 plan-recompute seed
        // and cache (set by `replace_resource`); rebuilding the plan from
        // them is the adapt subsystem's job, sequenced later. The domain-16
        // incremental-skip seed is `store_dirty`, consumed here.
        let _ = (&self.plan_dirty, &self.plan_cache);
        // `topo_order` / `topo_count` are retained plan state; the carrier
        // walk dispatches in carrier order directly, so `run` does not index
        // them.
        let _ = (&self.topo_order, &self.topo_count);
        // Per-frame incremental skip: the dirty-unit mask names which units
        // this frame must run (their input cone changed); the rest are
        // skipped, producing identical output to running them.
        let dirty = self.dirty_units();
        let msize = Cfg::MORSEL_SIZE.0.max(1);
        let total = self.record_count.0;
        // One whole-carrier drive decision for the flat Approach E body: any
        // accumulator-bearing fiber (a non-morsel-local plan descriptor) selects
        // unit-outer; otherwise morsel-outer. A record-less frame also takes the
        // single-walk path so a resource-only carrier runs exactly once.
        let descriptors = self.fiber_dispatch.as_ref();
        let fcount = self.fiber_dispatch_count.0.min(descriptors.len());
        let mut unit_outer = false;
        let mut fi = 0;
        while fi < fcount {
            if !descriptors[fi].morsel_local.0 {
                unit_outer = true;
            }
            fi += 1;
        }
        if unit_outer || total == 0 {
            // The accumulator-bearing (and record-less) path always runs every
            // unit: an accumulator is reset and re-appended each frame, so
            // skipping it would leave it reset-but-empty, which is not the same
            // output as running it. Incremental skip applies to the pure RAW
            // recompute path (morsel-outer below), not to per-frame append.
            let _ = dirty;
            self.wu_values
                .run(&self.bindings, MorselRange::new(USize::ZERO, USize(total)));
        } else {
            let mut start = 0;
            while start < total {
                let len = msize.min(total - start);
                self.wu_values.run_gated(
                    &self.bindings,
                    MorselRange::new(USize(start), USize(len)),
                    dirty,
                    USize::ZERO,
                );
                start += len;
            }
        }
        // The frame consumed the change seed; clear it and leave cold-start.
        self.store_dirty = AccessMask::empty();
        self.first_frame = Bool::FALSE;
        Cfg::Out::default()
    }

    /// Dispatch the retained carrier fused: fold its `RecordOp` work units into
    /// one `ChainWu` and walk that, keeping the chain's intermediate columns
    /// register-resident.
    ///
    /// This is the within-fiber linear fusion entry (the canonical spec's
    /// deep-single-fiber rust-pipe). It applies when the retained carrier is a
    /// linear read-after-write chain of opt-in `RecordOp` maps: `FuseCarrier`
    /// folds the carrier into the matching `OpChain` at the type level, and the
    /// fused `ChainWu` reads the chain's input column, runs the maps with every
    /// intermediate in a register, and writes only the chain's output column.
    /// Dead-store elimination under fat LTO removes the intermediate-column
    /// traffic, matching a hand-fused loop.
    ///
    /// The engine performs the fold: a consumer registers natural separate
    /// `RecordOp` work units plus their columns and calls `run_fused`; it never
    /// hand-authors the chain. The choice between this and the general per-WU
    /// `run` is an explicit entry rather than a transparent `run` auto-detection,
    /// which is not expressible on the toolchain (the fused projection witness
    /// would be an unconstrained specializing-impl parameter, and
    /// `min_specialization` does not permit specializing on the `FuseCarrier`
    /// bound). `W2` is the fused carrier's projection-witness list, inferred at
    /// the call site exactly as `run`'s `Witnesses` is, so `scheduler.run_fused()`
    /// needs no turbofish.
    ///
    /// A fusible chain writes no accumulator, so it dispatches morsel-outer (the
    /// runtime morsel loop wraps the whole-chain walk so the intermediates stay
    /// register-resident per morsel); a record-less frame runs the chain once
    /// over an empty morsel.
    pub fn run_fused<W2>(&mut self) -> Cfg::Out
    where
        Cfg::Out: Default,
        WuVals: FuseCarrier,
        WuCons<ChainWu<<WuVals as FuseCarrier>::Chain>, WuNil>:
            RunFiber<<Vals as BindingsFor>::Bindings, W2>,
    {
        let _ = (&self.plan_dirty, &self.plan_cache, &self.topo_order, &self.topo_count);
        let fused = WuCons {
            head: ChainWu::new(self.wu_values.fuse()),
            tail: WuNil,
        };
        // Incremental skip for the fused chain: the chain is one unit at
        // carrier position 0, and a linear chain's only external input is
        // its root, so the chain runs iff dirty bit 0 is set (its input
        // changed, or the cold frame). A clean frame skips the whole chain,
        // leaving its output column untouched.
        let dirty = self.dirty_units();
        let msize = Cfg::MORSEL_SIZE.0.max(1);
        let total = self.record_count.0;
        if total == 0 {
            fused.run_gated(
                &self.bindings,
                MorselRange::new(USize::ZERO, USize::ZERO),
                dirty,
                USize::ZERO,
            );
        } else {
            let mut start = 0;
            while start < total {
                let len = msize.min(total - start);
                fused.run_gated(
                    &self.bindings,
                    MorselRange::new(USize(start), USize(len)),
                    dirty,
                    USize::ZERO,
                );
                start += len;
            }
        }
        self.store_dirty = AccessMask::empty();
        self.first_frame = Bool::FALSE;
        Cfg::Out::default()
    }

    /// Borrow the resource bindings. Hidden test accessor: lets in-crate
    /// and integration tests walk the bindings nodes to confirm the
    /// moved-in resource values. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __bindings(&self) -> &<Vals as BindingsFor>::Bindings {
        &self.bindings
    }

    /// Borrow the backing store. Hidden test accessor mirroring
    /// `__bindings`: lets tests inspect reserved columns. The field is also
    /// held for its `Drop`, which frees every reserved resource column.
    /// Not part of the supported surface.
    #[doc(hidden)]
    pub fn __storage(&self) -> &CS {
        &self.storage
    }

    /// The store-backed plan locator. Hidden accessor: the dispatch consumer
    /// (and tests) read the plan columns out of `storage` through this handle.
    /// Not part of the supported surface until the dispatch reader lands.
    #[doc(hidden)]
    pub fn __plan_handle(&self) -> PlanHandle {
        self.plan_handle
    }
}

/// Default-construct an empty scheduler over the null store.
///
/// Only available for the no-store (`SvEmpty`) shape with the
/// `NullColumnStorage`: the empty bindings (`BindingNil`) owns nothing and
/// the null store reserves nothing, so no real store is needed. A
/// scheduler that owns resources is built via `build(storage)`.
impl<Cfg: RunCfg> Default for Scheduler<Cfg, WuNil, SvEmpty, NullColumnStorage> {
    fn default() -> Self {
        Self {
            _cfg: PhantomData,
            _stores: PhantomData,
            topo_order: <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(USize::ZERO),
            topo_count: USize::ZERO,
            plan_handle: PlanHandle::empty(),
            record_count: USize::ZERO,
            // The empty bundle (`WuNil`) writes no accumulator.
            fiber_dispatch: <<DefaultPlanDims as PlanDims>::Fibers as Capacity>::filled(
                FiberDispatch::default(),
            ),
            fiber_dispatch_count: USize::ZERO,
            plan_dirty: [const { AtomicBool::new(false) }; 256],
            plan_cache: PlanCache::new(),
            predecessor_masks:
                <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(
                    <DefaultPlanDims as PlanDims>::AdjRow::default(),
                ),
            read_masks: <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(
                AccessMask::empty(),
            ),
            store_dirty: AccessMask::empty(),
            first_frame: Bool::TRUE,
            bindings: crate::resource::bindings::BindingNil,
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
    >
    where
        P: BuilderInput,
        P::Dispatch: Dispatch<Wus, Stores, Platform> + RouterKind,
        <P::Dispatch as RouterKind>::Kind: Place<P>,
        WuVals: WuAppend<P>,
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
        + ContainsAll<<Wus as WorkUnitBundle>::AccumWrite>
        + AccumStoresMask<<DefaultPlanDims as PlanDims>::Stores>,
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
    ) -> notko::Outcome<Scheduler<DefaultRunCfg, WuVals, Vals, CS, DefaultPlanDims, Stores>, BuildError>
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
        let plan = match compute_plan::<Wus, Stores, BWit>(record_count) {
            notko::Outcome::Ok(p) => p,
            notko::Outcome::Err(e) => return notko::Outcome::Err(e),
        };
        let (topo_order, topo_count, fiber_dispatch, fiber_dispatch_count) =
            derive_phase_dispatch_order(&plan);
        let mut storage = storage;
        let mut next_id = USize::ZERO;
        match <Vals as DrainStores>::drain(self.store_values, &mut storage, &mut next_id, record_count) {
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
                    plan_dirty: [const { AtomicBool::new(false) }; 256],
                    plan_cache: PlanCache::new(),
                    predecessor_masks: plan.predecessor_masks,
                    read_masks: plan.read_masks,
                    store_dirty: AccessMask::empty(),
                    first_frame: Bool::TRUE,
                    bindings,
                    storage,
                    wu_values,
                })
            }
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
    ) -> notko::Outcome<Scheduler<Cfg, WuVals, Vals, CS, DefaultPlanDims, Stores>, BuildError>
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
        let plan = match compute_plan::<Wus, Stores, BWit>(record_count) {
            notko::Outcome::Ok(p) => p,
            notko::Outcome::Err(e) => return notko::Outcome::Err(e),
        };
        let (topo_order, topo_count, fiber_dispatch, fiber_dispatch_count) =
            derive_phase_dispatch_order(&plan);
        let mut storage = storage;
        let mut next_id = USize::ZERO;
        match <Vals as DrainStores>::drain(self.store_values, &mut storage, &mut next_id, record_count) {
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
                    plan_dirty: [const { AtomicBool::new(false) }; 256],
                    plan_cache: PlanCache::new(),
                    predecessor_masks: plan.predecessor_masks,
                    read_masks: plan.read_masks,
                    store_dirty: AccessMask::empty(),
                    first_frame: Bool::TRUE,
                    bindings,
                    storage,
                    wu_values,
                })
            }
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}

#[cfg(test)]
mod plan_column_offset_tests {
    use super::*;

    // A handle at a nonzero base, so the assertions discriminate "offset off
    // base" from "absolute StoreId".
    fn handle_at(base: USize) -> PlanHandle {
        PlanHandle {
            base,
            phase_count: USize::ZERO,
            trunk_count: USize::ZERO,
            fiber_count: USize::ZERO,
            unit_count: USize::ZERO,
        }
    }

    // Pin every plan column's StoreId offset off a nonzero base, and bind the
    // accessor count to `PlanColumn::COUNT`. Store and read both route through
    // `column_id`, so the offsets are a stored contract, not free to drift: a
    // wrong match arm shifts an `assert_eq!`, an absolute-vs-off-base confusion
    // drops the base, and a `COUNT` that disagrees with the column set fails the
    // length check (the span `store_plan`'s doc cites). The nonzero base is what
    // discriminates "offset off base" from "absolute StoreId".
    #[test]
    fn column_ids_are_pinned_offsets_off_base() {
        let base = USize(7); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test base literal; tracked: #72
        let h = handle_at(base);
        let at = |off: usize| StoreId(USize(base.0 + off)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: expected offset off base; tracked: #72
        let ids = [
            h.phases_id(),
            h.trunks_id(),
            h.fibers_id(),
            h.unit_meta_id(),
            h.morsel_sizes_id(),
            h.rcm_order_id(),
        ];
        assert_eq!(ids[0], at(0));
        assert_eq!(ids[1], at(1));
        assert_eq!(ids[2], at(2));
        assert_eq!(ids[3], at(3));
        assert_eq!(ids[4], at(4));
        assert_eq!(ids[5], at(5));
        assert_eq!(ids.len(), PlanColumn::COUNT);
    }
}
