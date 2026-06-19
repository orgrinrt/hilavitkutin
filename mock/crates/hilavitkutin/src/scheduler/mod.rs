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
use core::marker::{PhantomData, PhantomPinned};
use core::cell::Cell;
use core::pin::Pin;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};

use arvo::strategy::Identity;
use arvo::Bool;
use arvo::USize;
use arvo_bitmask::{BitAccess, BitLogic, BitSequence};
use arvo_tensor::{cap_size, Capacity, ConstCapacity};
use crate::plan::project::{AccumStoresMask, BundleProject, Locate, WitnessIndex};
use crate::plan::{
    compute_execution_plan, plan_inputs_from_bundle, AccessMask, DefaultPlanDims, ExecutionPlan,
    PlanDims,
};
use hilavitkutin_api::access::{AccessSet, ContainsAll, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, Dispatch};
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, Nanos};
use hilavitkutin_api::run_cfg::{DefaultRunCfg, PlanAffecting, RunCfg};
use hilavitkutin_api::store::Replaceable;
use hilavitkutin_api::store_values::{Place, RouterKind, StoreValues, SvEmpty};
use hilavitkutin_api::work_unit::WorkUnitBundle;
use hilavitkutin_api::work_unit_values::{WuAppend, WuCons, WuNil};
use hilavitkutin_api::{ColumnStorage, ColumnValue, StoreId, UnitId};

use crate::dispatch::core_mask::{grouping_arrays, phase_mask, phase_trunk_count};
use crate::thread::barrier::waist_barrier;
use crate::thread::class::MAX_CORES;
use crate::thread::frame::{
    await_exit, frame_await, frame_await_done, frame_done_arrive, frame_exit_arrive, frame_publish,
    request_shutdown,
};
use hilavitkutin_api::platform::{PoolFrame, ThreadPoolApi};
use crate::dispatch::fiber_run::RunFiber;
use crate::dispatch::fusion::{ChainWu, FuseCarrier};
use crate::dispatch::morsel::MorselRange;
use crate::dispatch::engine_ctx::Here;
use crate::dispatch::trunk_dispatch::RunTrunkDispatch;
use crate::dispatch::trunk_gate::RunGatedTrunk;
use crate::meta::{fold_ema, MetaBlock};
use crate::plan::grouping::{
    consumer_mask, consumer_phase_end, phase_count, plan_phase_count, pre_consumer_phase_count,
    BundleMasks, GATE2_MAX_ACCUMS, GATE2_MAX_UNITS,
};

pub mod plan;

pub use plan::PlanCache;

use crate::resource::bindings::{
    BindingsFor, CollectAccumLive, DrainStores, MergeAccums, RebaseBindings, ResetAccumulators,
};

/// The default empty store-value list, used as the `Vals` default for a
/// bare `Scheduler` type.
pub use hilavitkutin_api::store_values::SvEmpty as DefaultStoreValues;

/// Compile-time guard that a `PlanDims`'s `Units` capacity fits the
/// `GATE2_MAX_UNITS` ceiling that still sizes `run_parallel`'s
/// `gate2_phase` / `gate2_trunk` scratch (#690 lifts those onto `Units`).
/// Forcing `<D as UnitsFitGate2>::ASSERT_UNITS_FIT` evaluates the assertion at
/// monomorphisation, failing the build with a clear message rather than letting
/// an over-wide `Units` index past the fixed arrays at runtime. A named assoc
/// const is used rather than an inline `const {}` because the latter is an
/// anonymous generic constant the `generic_const_exprs` grammar rejects.
trait UnitsFitGate2 {
    const ASSERT_UNITS_FIT: ();
}
impl<D: PlanDims> UnitsFitGate2 for D {
    const ASSERT_UNITS_FIT: () = assert!(
        cap_size(<<D as PlanDims>::Units as Capacity>::CAP) <= GATE2_MAX_UNITS,
        "PlanDims::Units capacity exceeds GATE2_MAX_UNITS: run_parallel's gate2_phase/gate2_trunk scratch is sized by GATE2_MAX_UNITS; reduce Units or wait for the #690 lift onto Units",
    );
}

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
    MorselWindows,
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
            PlanColumn::MorselWindows => 4,
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
    /// `StoreId` of the per-fiber morsel-windows column.
    pub fn morsel_windows_id(&self) -> StoreId {
        self.column_id(PlanColumn::MorselWindows)
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
    /// Live fiber count (records in the fibers and morsel-windows columns).
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
/// per-fiber morsel windows, and RCM renumber pools (one column per `PlanColumn`
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
    match store_column(storage, handle.morsel_windows_id(), plan.morsel_windows.as_ref(), plan.fiber_count) {
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
    Clk = DefaultClock,
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
    /// fibers, per-unit metadata, per-fiber morsel windows, the RCM renumber),
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
    // The plan-affecting dirty bitset, sized by the `PlanDims::PlanAffecting`
    // capacity type (the GCE-free lift of the former hardcoded `[AtomicBool;
    // 256]`). `DefaultPlanDims::PlanAffecting = Dim<256>` keeps the default
    // width; a consumer tunes it via its `PlanDims` impl. The capacity is a
    // type, so no `cap_size` expression sits in array-length position and
    // `generic_const_exprs` never runs over it.
    plan_dirty: <D::PlanAffecting as Capacity>::Array<AtomicBool>,
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
    /// `Cell` for interior mutability: `run_parallel` rewrites the seed between
    /// frames while parked workers hold a live shared reference to the
    /// scheduler, so the write must not go through a plain field.
    store_dirty: Cell<AccessMask<D::Stores>>,
    /// Cold-start flag. Every unit is dirty on the first frame after build,
    /// so the first `run` / `run_fused` executes the whole carrier; set
    /// false afterward. `AtomicBool` (Relaxed, ordered by the frame barriers)
    /// so the between-frame write goes through a shared reference.
    first_frame: AtomicBool,
    /// E4 slice 1: the virtual-fire epoch. Incremented once per pass before
    /// dispatch; a producer's `fire<V>` stamps its `Virtual<V>` cell with the
    /// current value, and an `On<V>` consumer's gate opens when the cell equals
    /// it. Per-pass increment is the domain-10 epoch-reset (spec :709-713): last
    /// pass's stamp no longer equals this pass's epoch, so a stale fire gates
    /// shut without an explicit clear. `AtomicUsize` (Relaxed, ordered by the
    /// frame barriers) wraps effectively-never.
    virtual_epoch: AtomicUsize,
    /// E4 slice 3: engine-owned meta state (the self-hosting meta pipeline's
    /// mutable resources). Not a `Store` (consumer stores are `Copy` read-only),
    /// written directly by the engine each pass; an `OnMeta` work unit reads it
    /// through the `MetaAccess`-gated Ctx accessor. `SchedulerMetrics::pass_count`
    /// advances once per pass.
    meta_block: MetaBlock,
    /// E8 adapt: the clock provider sampled at frame start and end for the
    /// pass-duration EMA. Carried from the builder's clock slot;
    /// `DefaultClock` unless overridden via `SchedulerBuilder::clock`.
    clock: Clk,
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
    /// GATE-2 persistent-pool sync words (frame seq/done/exited + shutdown +
    /// phase barrier). `'static` with dangling progress_slots and `<1, 1>` arrays
    /// (the C/P-sized adapt arrays are unused until the adapt subsystem ships;
    /// the sync words are scalars). Pinned (see `_pin`), so the spawned workers'
    /// raw pointers into it stay valid for the scheduler's life.
    pool: PoolFrame<'static, MAX_CORES, 1>,
    /// Per-worker contexts the spawned-once workers read through a raw pointer.
    /// Populated at the first `run_parallel`; stable because the scheduler is
    /// pinned once threaded.
    worker_ctxs: [WorkerCtx; MAX_CORES],
    /// Whether the persistent pool has been spawned (first `run_parallel`).
    spawned: Bool,
    /// Const-grouping result, computed once at the first `run_parallel` and read
    /// by every worker: per-unit waist-bounded phase and within-phase trunk, the
    /// live unit count, the phase count, and the active core count.
    gate2_phase: [USize; GATE2_MAX_UNITS],
    gate2_trunk: [USize; GATE2_MAX_UNITS],
    gate2_n: USize,
    gate2_nphases: USize,
    gate2_ncores: USize,
    /// Per-core accumulator live counts published by workers on the threaded
    /// unit-outer accumulator path (GATE-2 deviation 9). Flat `[core *
    /// GATE2_MAX_ACCUMS + accum]`; worker `c` stores its per-accumulator live
    /// length (Relaxed) before `frame_done_arrive`, the main thread loads them
    /// after `frame_await_done` (acquire via the done counter) and feeds the
    /// `merge_accums` compaction. Sized by `MAX_CORES * GATE2_MAX_ACCUMS`.
    gate2_accum_live: [AtomicUsize; MAX_CORES * GATE2_MAX_ACCUMS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-(core,accum) atomic publish array; tracked: #121
    /// Engine-internal per-phase duration EMA (domain-22 adapt). The single-core
    /// `dispatch_trunks` loop folds each phase's wall-clock duration here with the
    /// 1/8 EMA. `Cell` interior mutability is sound because only the main thread
    /// writes it (workers never call `dispatch_trunks`), the same discipline as
    /// `store_dirty`. Phases are bounded by units, so `GATE2_MAX_UNITS` bounds it.
    /// Feeds the eventual `select_adapt_config`; not consumer-exposed.
    phase_ema: [Cell<Nanos>; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase EMA store sized by the unit cap; tracked: #121
    /// Per-frame per-phase duration accumulator (raw nanos). `dispatch_trunks`
    /// runs once per morsel, so it SUMS each morsel's phase-slice duration here;
    /// `run` folds the per-frame total into `phase_ema` once at frame end (so the
    /// EMA is per-frame, not per-morsel) and zeroes it. Same single-writer
    /// discipline as `phase_ema` / `store_dirty`.
    phase_accum: [Cell<Nanos>; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-frame per-phase duration accumulator; tracked: #121
    /// Phase-imbalance reconfigure trigger (domain-22 adapt). `select_adapt_config`
    /// sets it each frame when one active phase's EMA dominates the least active
    /// phase's by more than `BALANCE_FACTOR`. The actuation that acts on it is a
    /// follow-up; this is the decision half. Single-writer (main thread in `run`).
    adapt_reconfigure: Cell<Bool>,
    /// Marks the scheduler `!Unpin`: once a worker holds a raw pointer into it,
    /// moving it would dangle that pointer, so `run_parallel` takes `Pin`.
    _pin: PhantomPinned,
}

/// Per-worker context for the GATE-2 persistent pool. Holds a type-erased
/// back-pointer to the owning `Scheduler` (the monomorphic `worker_main` casts it
/// back to the concrete type) plus the worker's core id. Stored inline in the
/// scheduler at a pinned, stable address; the spawned worker closure captures one
/// `*const WorkerCtx` (pointer-sized, so the `OsThreadPool::spawn` smuggle fits).
struct WorkerCtx {
    sched: *const (),
    core_id: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: core index, smuggled through the pthread arg; tracked: #121
}

/// Send wrapper so the one-pointer worker closure satisfies `F: Send`. SAFETY:
/// the pointee is pinned scheduler-owned storage that outlives every worker (Drop
/// joins via `await_exit` before teardown); workers touch disjoint write columns.
#[derive(Copy, Clone)]
struct SendCtxPtr(*const WorkerCtx);
// SAFETY: see above.
unsafe impl Send for SendCtxPtr {}

/// Build an empty `PoolFrame<'static, MAX_CORES, 1>` for the scheduler's pool:
/// all sync words zero, dangling progress_slots (the frame protocol never reads
/// them). The core dimension is `MAX_CORES` so the per-core `idle_accumulator` /
/// `park_count` arrays are genuinely per-core (the waist barrier fills
/// `idle_accumulator[core]` for the core-idle adapt axis). The phase dimension
/// stays 1: `predicted_wait_ns` is per-phase and not yet driven, so it needs no
/// real phase cap here.
fn empty_pool_frame() -> PoolFrame<'static, MAX_CORES, 1> {
    PoolFrame {
        shutdown: AtomicBool::new(false),
        phase_arrived: AtomicU32::new(0), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        barrier_sense: AtomicU32::new(0), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        seq: AtomicU32::new(0),           // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        done: AtomicU32::new(0),          // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        exited: AtomicU32::new(0),        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        predicted_wait_ns: [AtomicU32::new(0)], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic init constant; tracked: #121
        idle_accumulator: [const { AtomicU64::new(0) }; MAX_CORES], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-core atomic init; tracked: #121
        park_count: [const { AtomicU64::new(0) }; MAX_CORES], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-core atomic init; tracked: #121
        progress_slots: NonNull::dangling(),
        progress_slot_count: USize::ZERO,
        _arena: PhantomData,
    }
}

/// Persistent-pool worker mainloop (GATE-2 R4c). Spawned once per core at the
/// first `run_parallel`; parks on the frame `seq` between phases, runs its core's
/// dispatch for the published phase, and arrives at the frame done-barrier. The
/// `ctx` back-pointer is cast to the concrete `Scheduler` here (the monomorphic
/// turbofish at the spawn site supplies the types). Phase is derived from the seq
/// value: the main thread publishes one phase per `seq` bump, so
/// `phase = (seq - 1) % nphases`.
fn worker_main<Cfg, WuVals, Vals, CS, D, Stores, Clk, Witnesses, GW>(ctx: *const WorkerCtx)
where
    Cfg: RunCfg,
    Vals: StoreValues + BindingsFor,
    CS: ColumnStorage,
    D: PlanDims,
    Clk: ClockApi,
    WuVals: RunFiber<<Vals as BindingsFor>::Bindings, Witnesses>,
    WuVals: RunTrunkDispatch<
        WuVals,
        <Vals as BindingsFor>::Bindings,
        Witnesses,
        GW,
        Stores,
        <D as PlanDims>::Units,
        <D as PlanDims>::Stores,
        <D as PlanDims>::AdjRow,
        0, // lint:allow(no-bare-numeric) reason: const-generic entry position; tracked: #121
    >,
    WuVals: BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
    <D as PlanDims>::Units: ConstCapacity,
    <D as PlanDims>::AdjRow: BitAccess + Identity,
    <Vals as BindingsFor>::Bindings: RebaseBindings + CollectAccumLive,
{
    // SAFETY: `ctx` points into the pinned scheduler's `worker_ctxs`, valid until
    // `await_exit` runs at Drop (which joins before teardown).
    let core_id = unsafe { (*ctx).core_id };
    let sched = unsafe { (*ctx).sched } as *const Scheduler<Cfg, WuVals, Vals, CS, D, Stores, Clk>;
    // SAFETY: the pinned scheduler outlives every worker. This shared
    // reference is held live across every park, so the main thread must never
    // write any scheduler field through a plain `*mut` while it is alive: the
    // between-frame mutated, worker-visible fields (`first_frame`,
    // `virtual_epoch`, `store_dirty`, the `meta_block` cells) all carry
    // interior mutability, so the main thread writes through a shared
    // reference and never invalidates this borrow under the aliasing model.
    let s = unsafe { &*sched };
    let ncores = s.gate2_ncores;
    let nphases = s.gate2_nphases.0.max(1); // lint:allow(no-bare-numeric) reason: avoid modulo by zero; tracked: #121
    // E4 parity: leading plan-band phase count, skipped on a clean frame
    // (mirrors single-core dispatch_trunks).
    let plan_phases = plan_phase_count::<
        WuVals,
        Stores,
        GW,
        <D as PlanDims>::Units,
        <D as PlanDims>::Stores,
        <D as PlanDims>::AdjRow,
    >()
    .0; // lint:allow(no-bare-numeric) reason: phase loop offset; tracked: #121
    let total = s.record_count;
    let msize = USize(Cfg::MORSEL_SIZE.0.max(1)); // lint:allow(no-bare-numeric) reason: morsel length guard; tracked: #121
    let mut last = USize::ZERO;
    loop {
        last = frame_await(&s.pool, last);
        if s.pool.shutdown.load(Ordering::Relaxed) {
            frame_exit_arrive(&s.pool, ncores);
            return;
        }
        if s.carrier_unit_outer().0 {
            // Deviation 9 threaded accumulator path: an accumulator-bearing
            // carrier runs unit-outer (each unit completes its full record
            // range). Each core takes its head+tail record slice `[lo, hi)`,
            // dispatches the whole carrier ONCE over a per-core bindings copy
            // whose accumulators are offset into the core's region with fresh
            // cells, then publishes its per-accumulator live counts for the
            // main-thread merge. No phase loop or waist barrier: cores are
            // independent over disjoint record ranges, joined only by the merge.
            worker_accum_unit_outer::<Cfg, WuVals, Vals, CS, D, Stores, Clk, Witnesses, GW>(
                s,
                USize(core_id),
                ncores,
                total,
            );
            frame_done_arrive(&s.pool, ncores);
            continue;
        }
        // One wake per frame: the worker runs ALL waist-bounded phases hot,
        // crossing each interior waist via the worker-side sense-reversing
        // barrier (the canonical worker-side sync; the main thread no longer
        // round-trips per phase). Phase order is the array order 0..nphases.
        // On a clean (not plan-dirty) frame the loop starts past the leading
        // plan band; `first_frame` is written between frames while every
        // worker is parked, so the read is stable under the publish/await
        // happens-before. All workers compute the same start, so the interior
        // waist-barrier counts stay matched.
        let mut p = if s.first_frame.load(Ordering::Relaxed) { 0 } else { plan_phases }; // lint:allow(no-bare-numeric) reason: phase loop start; tracked: #121
        while p < nphases {
            s.run_core_phase::<Witnesses, GW>(
                &s.gate2_phase,
                &s.gate2_trunk,
                s.gate2_n,
                USize(core_id),
                USize(p),
                ncores,
                total,
                msize,
            );
            if p + 1 < nphases {
                // every worker participates in each waist, even one that owned
                // no trunk this phase, so `expected` is the full core count. The
                // barrier times this core's follower park into the idle
                // accumulator using the scheduler's clock.
                waist_barrier(&s.pool, USize(core_id), ncores, || s.clock.now_ns());
            }
            p += 1; // lint:allow(no-bare-numeric) reason: phase loop step; tracked: #121
        }
        frame_done_arrive(&s.pool, ncores);
    }
}

/// One core's unit-outer accumulator dispatch over its head+tail record slice
/// (GATE-2 deviation 9). Builds a per-core bindings copy with every accumulator
/// offset to the slice start (fresh live cells, slice-sized cap), dispatches the
/// whole carrier once over `[lo, hi)`, and publishes the per-accumulator live
/// counts into the scheduler's `gate2_accum_live` row for this core (Relaxed; the
/// `frame_done_arrive` Release that follows publishes them to the merge). The
/// core's row is zeroed first so a non-participating core (surplus, or a
/// record-less frame's non-zero cores) contributes zeros to the merge.
fn worker_accum_unit_outer<Cfg, WuVals, Vals, CS, D, Stores, Clk, Witnesses, GW>(
    s: &Scheduler<Cfg, WuVals, Vals, CS, D, Stores, Clk>,
    core: USize,
    ncores: USize,
    total: USize,
) where
    Cfg: RunCfg,
    Vals: StoreValues + BindingsFor,
    CS: ColumnStorage,
    D: PlanDims,
    Clk: ClockApi,
    WuVals: RunFiber<<Vals as BindingsFor>::Bindings, Witnesses>,
    WuVals: BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
    <D as PlanDims>::Units: ConstCapacity,
    <D as PlanDims>::AdjRow: BitAccess + Identity,
    <Vals as BindingsFor>::Bindings: RebaseBindings + CollectAccumLive,
{
    let total0 = total.0; // lint:allow(no-bare-numeric) reason: frame record count; tracked: #121
    let ncores0 = ncores.0.max(1); // lint:allow(no-bare-numeric) reason: avoid div by zero; tracked: #121
    // Zero this core's publish row up front (every slot, so the merge reads clean
    // values regardless of this frame's accumulator count).
    let mut z = 0; // lint:allow(no-bare-numeric) reason: publish-row index; tracked: #121
    while z < GATE2_MAX_ACCUMS {
        s.gate2_accum_live[core.0 * GATE2_MAX_ACCUMS + z].store(0, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-(core,accum) publish slot reset; tracked: #121
        z += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
    // Head+tail record slice for this core (mirrors run_core_phase's split).
    let per = (total0 + ncores0 - 1) / ncores0; // lint:allow(no-bare-numeric) reason: ceil record slice; tracked: #121
    let lo = (core.0 * per).min(total0); // lint:allow(no-bare-numeric) reason: slice start; tracked: #121
    let hi = (lo + per).min(total0); // lint:allow(no-bare-numeric) reason: slice end; tracked: #121
    // A record-less frame runs the carrier once on core 0 only (a resource-only
    // unit must run exactly once, not once per core). With records, a surplus
    // core (`lo == hi`) appends nothing and is skipped.
    let run_this = if total0 == 0 { core.0 == 0 } else { lo < hi }; // lint:allow(no-bare-numeric) reason: participation guard; tracked: #121
    if !run_this {
        return;
    }
    let region = hi - lo; // lint:allow(no-bare-numeric) reason: slice length; tracked: #121
    let per_core = s.bindings.rebase_accums(USize(lo), USize(region));
    // E4 parity: meta units do not ride the per-core slice walk (the designated
    // thread dispatches them once per frame around the publish/await window).
    // A no-meta carrier keeps the ungated whole-carrier walk; the band counts
    // are const, so this branch folds at compile time.
    let pre = pre_consumer_phase_count::<
        WuVals,
        Stores,
        GW,
        <D as PlanDims>::Units,
        <D as PlanDims>::Stores,
        <D as PlanDims>::AdjRow,
    >();
    let cend = consumer_phase_end::<
        WuVals,
        Stores,
        GW,
        <D as PlanDims>::Units,
        <D as PlanDims>::Stores,
        <D as PlanDims>::AdjRow,
    >();
    let nphases = phase_count::<
        WuVals,
        Stores,
        GW,
        <D as PlanDims>::Units,
        <D as PlanDims>::Stores,
        <D as PlanDims>::AdjRow,
    >();
    if pre.0 == 0 && cend.0 == nphases.0 {
        s.wu_values.run(&per_core, &s.meta_block, MorselRange::new(USize(lo), USize(region)), USize(s.virtual_epoch.load(Ordering::Relaxed)));
    } else {
        let cmask = consumer_mask::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >();
        s.wu_values.run_gated(
            &per_core,
            &s.meta_block,
            MorselRange::new(USize(lo), USize(region)),
            cmask,
            USize::ZERO,
            USize(s.virtual_epoch.load(Ordering::Relaxed)),
        );
    }
    // Publish this core's per-accumulator live counts.
    let mut live = [USize::ZERO; GATE2_MAX_ACCUMS];
    let mut idx = USize::ZERO;
    per_core.collect_accum_live(&mut live, &mut idx);
    let mut a = 0; // lint:allow(no-bare-numeric) reason: accum index; tracked: #121
    while a < idx.0 {
        s.gate2_accum_live[core.0 * GATE2_MAX_ACCUMS + a].store(live[a].0, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: publish per-(core,accum) live count; tracked: #121
        a += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
    }
}

impl<Cfg: RunCfg, WuVals, Vals: StoreValues + BindingsFor, CS: ColumnStorage, D: PlanDims, Stores, Clk>
    Drop for Scheduler<Cfg, WuVals, Vals, CS, D, Stores, Clk>
{
    fn drop(&mut self) {
        if self.spawned.0 {
            // Signal shutdown and wait every spawned worker to leave its mainloop
            // before the inline pool (which the workers read) tears down. This is
            // the join with no thread-join: the exit-counter barrier.
            request_shutdown(&self.pool);
            await_exit(&self.pool, self.gate2_ncores);
        }
    }
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

/// Null clock: `now_ns` always returns zero.
///
/// The unconditional fallback `Clk` for builds without the os tier: the
/// pass-duration EMA stays zero until a real clock is supplied via the
/// builder's `clock(...)` slot (the no_os DI path). With the default
/// `platform-os` feature the builder starts on `OsClock` instead, so this
/// type is only ever the live clock when a no_os consumer leaves the slot
/// untouched.
pub struct NullClock;

impl NullClock {
    /// Construct the null clock.
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for NullClock {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ClockApi for NullClock {
    #[inline]
    fn now_ns(&self) -> Nanos {
        Nanos::from_raw(0) // lint:allow(no-bare-numeric) reason: null clock zero reading; tracked: #121
    }
}

/// The builder's starting clock: the os-tier monotonic clock when the
/// default `platform-os` feature is on, the null clock otherwise (no_os
/// consumers supply their own via the builder's `clock(...)` slot).
#[cfg(feature = "platform-os")]
pub type DefaultClock = crate::platform::OsClock;
/// The builder's starting clock (no_os fallback; see the platform-os arm).
#[cfg(not(feature = "platform-os"))]
pub type DefaultClock = NullClock;

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
    pub const fn builder() -> SchedulerBuilder<Empty, Empty, Empty, SvEmpty, WuNil, DefaultClock> {
        SchedulerBuilder {
            store_values: SvEmpty,
            wu_values: WuNil,
            clock: DefaultClock::new(),
            _phantom: PhantomData,
        }
    }
}

impl<Cfg: RunCfg, WuVals, Vals: StoreValues + BindingsFor, CS: ColumnStorage, D: PlanDims, Stores, Clk: ClockApi>
    Scheduler<Cfg, WuVals, Vals, CS, D, Stores, Clk>
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
        self.store_dirty.set(self.store_dirty.get().set(Index::INDEX));
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
        if self.first_frame.load(Ordering::Relaxed) {
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
            if reads[p].overlaps(&self.store_dirty.get()).0 {
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
    pub fn run<Witnesses, GW>(&mut self) -> Cfg::Out // lint:allow(no-bare-numeric) reason: const-generic dispatch entry position; tracked: #121
    where
        Cfg::Out: Default,
        WuVals: RunTrunkDispatch<
            WuVals,
            <Vals as BindingsFor>::Bindings,
            Witnesses,
            GW,
            Stores,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
            0, // lint:allow(no-bare-numeric) reason: const-generic entry position; tracked: #121
        >,
        WuVals: BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
        <D as PlanDims>::Units: ConstCapacity,
        <D as PlanDims>::AdjRow: BitAccess + Identity,
        <Vals as BindingsFor>::Bindings: ResetAccumulators,
    {
        // E8 adapt: sample the frame start; the cold-start state is the EMA
        // seed flag (the first frame stores its raw duration).
        let frame_start = self.clock.now_ns();
        let ema_seed = Bool(self.first_frame.load(Ordering::Relaxed));
        // Schedule-once-reuse: zero every accumulator live-length at frame
        // start so this frame appends into a fresh buffer rather than
        // continuing from the prior frame's live offset. No-op for an
        // accumulator-free carrier.
        self.bindings.reset_accumulators();
        // E4 slice 1: advance the virtual epoch once per pass. A fire this pass
        // stamps cells with the new value; last pass's stamps no longer match, so
        // a stale fire gates its `On<V>` consumer shut (epoch-based reset).
        self.virtual_epoch.fetch_add(1, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-pass epoch successor; tracked: #121
        self.meta_block.metrics.pass_count.set(USize(self.meta_block.metrics.pass_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: per-pass meta pass_count; tracked: #121
        let epoch = USize(self.virtual_epoch.load(Ordering::Relaxed));
        // E4 slice 2 (self-hosting meta pipeline): a plan-dirty frame runs the
        // leading plan band (`OnMeta<PlanStage>` units recompute the plan); a clean
        // frame skips it. The first frame is always plan-dirty (the plan is computed
        // once); the `replace_resource`-driven `plan_dirty` bit-array re-dirty is
        // the domain-22 recompute, sequenced with the adapt subsystem (slice 3). For
        // a carrier with no plan-stage meta unit, `plan_phase_count` is zero, so this
        // is a no-op and dispatch is byte-identical to before.
        let plan_dirty = Bool(self.first_frame.load(Ordering::Relaxed));
        // `plan_dirty` array / `plan_cache` are the domain-22 plan-recompute seed
        // and cache (set by `replace_resource`); rebuilding the plan from them is the
        // adapt subsystem's job, sequenced later. The domain-16 incremental-skip seed
        // is `store_dirty`, consumed here.
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
            // Per-trunk dispatch over the whole range, all members (all-ones
            // dirty): output-equivalent to the flat unit-outer walk, every trunk
            // an independently monomorphised program.
            let all = <D as PlanDims>::AdjRow::default().bitnot();
            self.dispatch_trunks::<Witnesses, GW, _>(
                MorselRange::new(USize::ZERO, USize(total)),
                all,
                epoch,
                plan_dirty,
            );
        } else {
            let mut start = 0;
            while start < total {
                let len = msize.min(total - start);
                // Per-trunk dispatch over this morsel, skipping clean members
                // (incremental skip preserved): same output as the flat gated
                // morsel walk, in phase / trunk order.
                self.dispatch_trunks::<Witnesses, GW, _>(
                    MorselRange::new(USize(start), USize(len)),
                    dirty,
                    epoch,
                    plan_dirty,
                );
                start += len;
            }
        }
        // Capture the change_class signal before the seed is consumed: a
        // non-empty store_dirty means an input change was seen this frame.
        let stores_changed = !self.store_dirty.get().is_empty().0;
        // The frame consumed the change seed; clear it and leave cold-start.
        self.store_dirty.set(AccessMask::empty());
        self.first_frame.store(false, Ordering::Relaxed);
        // E8 adapt: fold this frame's duration into the pass-duration EMA.
        // Between frames, so the write needs no synchronisation.
        let m = &self.meta_block.metrics;
        m.ema_pass_duration_ns
            .set(fold_ema(m.ema_pass_duration_ns.get(), self.clock.now_ns() - frame_start, ema_seed));
        m.last_record_count.set(self.record_count);
        if stores_changed {
            m.change_seen_count.set(USize(m.change_seen_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: increment by one frame; tracked: #121
        }
        // E8 adapt, per-phase EMA: fold each phase's per-frame total (summed
        // across this frame's morsels by `dispatch_trunks`) into its EMA with the
        // same seed, then zero the accumulator for the next frame. Per-frame, so
        // a multi-morsel frame folds once, not once per morsel.
        let nph = phase_count::<WuVals, Stores, GW, <D as PlanDims>::Units, <D as PlanDims>::Stores, <D as PlanDims>::AdjRow>().0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: phase-fold bound; tracked: #121
        let mut pe = 0; // lint:allow(no-bare-numeric) reason: phase-fold index; tracked: #121
        while pe < nph && pe < self.phase_ema.len() {
            let acc = self.phase_accum[pe].get();
            self.phase_ema[pe].set(fold_ema(self.phase_ema[pe].get(), acc, ema_seed));
            self.phase_accum[pe].set(Nanos::from_raw(0)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-frame accumulator reset; tracked: #121
            pe += 1; // lint:allow(no-bare-numeric) reason: phase-fold step; tracked: #121
        }
        // E8 adapt tuning: read the just-folded per-phase EMA and set the
        // reconfigure trigger when the frame's phases are imbalanced.
        self.select_adapt_config();
        Cfg::Out::default()
    }

    /// Dispatch only the units of one phase and trunk (GATE-2, round 2a).
    ///
    /// Walks the carrier through `RunGatedTrunk`, running just the members of
    /// `(PHASE, TRUNK)` over the whole record range; every other carrier
    /// position folds away, so this monomorphisation is that trunk's member-only
    /// program. The per-trunk entry the round-2b dispatcher loops across every
    /// `(phase, trunk)` in phase order, and the unit each core runs at G2-N.
    /// `Witnesses` is the per-unit projection list and `GW` the grouping witness
    /// list, both inferred at the call site.
    pub fn run_one_trunk<Witnesses, GW, const TRUNK: usize>(&mut self) // lint:allow(no-bare-numeric) reason: const-generic trunk selector; tracked: #121
    where
        WuVals: RunGatedTrunk<
            WuVals,
            <Vals as BindingsFor>::Bindings,
            Witnesses,
            GW,
            Stores,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
            TRUNK,
            Here, // the walk starts at carrier position zero (Peano Here)
        >,
    {
        let total = self.record_count.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: morsel length; tracked: #121
        self.virtual_epoch.fetch_add(1, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-pass epoch successor; tracked: #121
        self.meta_block.metrics.pass_count.set(USize(self.meta_block.metrics.pass_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: per-pass meta pass_count; tracked: #121
        let epoch = USize(self.virtual_epoch.load(Ordering::Relaxed));
        // No-skip entry: every member of the trunk runs (all-ones dirty mask).
        let all = <D as PlanDims>::AdjRow::default().bitnot();
        self.wu_values
            .run_trunk(&self.bindings, &self.meta_block, MorselRange::new(USize::ZERO, USize(total)), all, epoch);
    }

    /// Dispatch every trunk in phase order over `morsel`, single-core (round 2b).
    ///
    /// The outer driver: for each phase pass `0..phase_count` it walks the
    /// carrier through `trunk_dispatch::RunTrunkDispatch`, dispatching each
    /// trunk-root's per-trunk mono whose compile-time phase equals the pass. Each
    /// trunk's members run in carrier (RCM-reordered topological) order; phases
    /// run in waist order; so the result is output-equivalent to the flat
    /// `RunFiber` walk, while every trunk is an independently monomorphised
    /// program (the unit a core runs at G2-N). Whole-range, no-skip entry (every
    /// member runs); the morsel-windowed, dirty-skipping form drives the
    /// incremental `run` path. `Witnesses` (per-unit projection list) and `GW`
    /// (grouping witness list) infer at the call site.
    pub fn run_all_trunks<Witnesses, GW>(&mut self)
    where
        WuVals: RunTrunkDispatch<
            WuVals,
            <Vals as BindingsFor>::Bindings,
            Witnesses,
            GW,
            Stores,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
            0, // the walk starts at carrier position zero // lint:allow(no-bare-numeric) reason: const-generic entry position; tracked: #121
        >,
        WuVals: BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
        <D as PlanDims>::Units: ConstCapacity,
        <D as PlanDims>::AdjRow: BitAccess + Identity,
    {
        let total = self.record_count.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: morsel length; tracked: #121
        self.virtual_epoch.fetch_add(1, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-pass epoch successor; tracked: #121
        self.meta_block.metrics.pass_count.set(USize(self.meta_block.metrics.pass_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: per-pass meta pass_count; tracked: #121
        let epoch = USize(self.virtual_epoch.load(Ordering::Relaxed));
        // No-skip: every member runs (all-ones dirty mask).
        let all = <D as PlanDims>::AdjRow::default().bitnot();
        // No-skip entry: run every band including the plan band (plan_dirty=true).
        self.dispatch_trunks::<Witnesses, GW, _>(MorselRange::new(USize::ZERO, USize(total)), all, epoch, Bool::TRUE);
    }

    /// Phase-loop core of the per-trunk dispatch: for each phase pass walk the
    /// carrier through `RunTrunkDispatch` over `morsel`, skipping members clear in
    /// `dirty`. `run_all_trunks` (whole-range, all-ones) and the incremental
    /// `run` path (per-morsel, real dirty) both delegate here.
    fn dispatch_trunks<Witnesses, GW, M: BitAccess>(
        &self,
        morsel: MorselRange,
        dirty: M,
        epoch: USize,
        plan_dirty: Bool,
    ) where
        WuVals: RunTrunkDispatch<
            WuVals,
            <Vals as BindingsFor>::Bindings,
            Witnesses,
            GW,
            Stores,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
            0, // lint:allow(no-bare-numeric) reason: const-generic entry position; tracked: #121
        >,
        WuVals: BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
        <D as PlanDims>::Units: ConstCapacity,
        <D as PlanDims>::AdjRow: BitAccess + Identity,
    {
        // Phase-loop bound = the const grouping's phase count, the same axis the
        // dispatcher's per-trunk phase gate reads, so every trunk-root fires in
        // exactly one pass.
        let nphases = phase_count::<WuVals, Stores, GW, <D as PlanDims>::Units, <D as PlanDims>::Stores, <D as PlanDims>::AdjRow>().0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: phase-loop bound; tracked: #121
        // E4 slice 2 (self-hosting meta pipeline): the rank-outer grouping places
        // `OnMeta<PlanStage>` units in the leading plan band (phases
        // `0..plan_phase_count`). On a clean frame (not plan-dirty) the kernel skips
        // that band, so plan-stage meta units run only when the plan is recomputed;
        // the schedule-ready / pass-start / consumer / schedule-end bands always
        // dispatch. This is the kernel's lifecycle sequencing: the band order is the
        // canonical PlanStage < ScheduleReady < PassStart < consumer < ScheduleEnd.
        let start = if plan_dirty.0 {
            0 // lint:allow(no-bare-numeric) reason: plan-dirty frame runs the plan band; tracked: #121
        } else {
            plan_phase_count::<WuVals, Stores, GW, <D as PlanDims>::Units, <D as PlanDims>::Stores, <D as PlanDims>::AdjRow>().0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: skip the leading plan band on a clean frame; tracked: #121
        };
        // E8 adapt, per-phase timing: `dispatch_trunks` runs once per morsel, so
        // ADD each phase's per-morsel duration into the per-frame accumulator;
        // `run` folds the per-frame total into `phase_ema` once at frame end (so
        // the EMA is per-frame, not per-morsel). Engine-internal; feeds the
        // eventual `select_adapt_config`.
        let mut p = start; // lint:allow(no-bare-numeric) reason: phase-pass index; tracked: #121
        while p < nphases {
            let t0 = self.clock.now_ns().to_raw(); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: raw nanos for the duration delta; tracked: #121
            self.wu_values
                .dispatch(&self.wu_values, USize(p), &self.meta_block, &self.bindings, morsel, dirty, epoch);
            let dur = self.clock.now_ns().to_raw().saturating_sub(t0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: monotonic phase-slice delta; tracked: #121
            if p < self.phase_accum.len() {
                let slot = &self.phase_accum[p];
                slot.set(Nanos::from_raw(slot.get().to_raw().saturating_add(dur))); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-frame phase-duration sum; tracked: #121
            }
            p += 1; // lint:allow(no-bare-numeric) reason: phase-pass step; tracked: #121
        }
    }

    /// Dispatch the carrier as per-core trunk programs joined by waist barriers
    /// (GATE-2 N-core dispatch, inline single-threaded form).
    ///
    /// op's runtime-mask mechanism: the canonical waist-bounded phase axis (R2)
    /// and per-phase round-robin trunk-to-core ownership (R4a `core_mask`)
    /// select, for each `(core, phase)`, the carrier positions that core owns in
    /// that phase. Each selection is a `run_gated` walk over the flat carrier, so
    /// unit bodies devirtualise exactly as the single-core walk does; only the
    /// per-unit ownership test is a runtime branch. This inline form runs the
    /// per-core programs sequentially (one thread sweeps every core's program per
    /// phase), with the waist barrier between phases collapsing to the phase loop
    /// boundary. It is output-equivalent to `run` for a pure read-after-write
    /// carrier: phases run in waist order, so a phase-`p+1` reader sees every
    /// record a phase-`p` writer produced; trunks within a phase touch disjoint
    /// columns, so their order is immaterial; each trunk's units run in carrier
    /// (topological) order. Single-core (`ncores == 1`) is the degenerate case
    /// with one core owning every trunk per phase, not a separate path.
    ///
    /// Scope (R4b-inline): the accumulator (unit-outer, cross-record) carrier
    /// that `run` routes specially is out of scope here; it lands with the
    /// threaded executor step, which replaces the sequential core sweep with the
    /// spawned-once pool plus the column-disjoint borrow split, leaving the
    /// partition this method proves unchanged. The `phase` / `trunk` arrays are
    /// recomputed per call in this form; the threaded form lifts them to a
    /// build-time precompute (schedule-once-reuse).
    ///
    /// `Witnesses` is the per-unit projection list (for the carrier walk) and
    /// `GW` the grouping witness list (for the const grouping that fills the
    /// `phase` / `trunk` arrays), both inferred at the call site.
    pub fn run_parallel<Witnesses, GW, P>(self: Pin<&mut Self>, pool: &P) -> Cfg::Out
    where
        Cfg::Out: Default,
        WuVals: RunFiber<<Vals as BindingsFor>::Bindings, Witnesses>,
        WuVals: RunTrunkDispatch<
            WuVals,
            <Vals as BindingsFor>::Bindings,
            Witnesses,
            GW,
            Stores,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
            0, // lint:allow(no-bare-numeric) reason: const-generic entry position; tracked: #121
        >,
        WuVals: BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
        <D as PlanDims>::Units: ConstCapacity,
        <D as PlanDims>::AdjRow: BitAccess + Identity,
        <Vals as BindingsFor>::Bindings:
            ResetAccumulators + RebaseBindings + CollectAccumLive + MergeAccums,
        P: ThreadPoolApi,
    {
        // Cross-cap guard (#690): the grouping producer is sized by `D::Units`,
        // but the gate2_phase / gate2_trunk scratch is still sized by
        // GATE2_MAX_UNITS. A `D` whose `Units` capacity exceeds that ceiling
        // would index past those arrays. Forcing this trait's assoc const fails
        // the build per monomorphisation with a clear message rather than an
        // out-of-bounds panic. Removed when #690 lifts the parallel scratch onto
        // `Units`. (A named assoc const, not an inline `const {}`, because the
        // latter is an anon generic constant the GCE grammar rejects.)
        let () = <D as UnitsFitGate2>::ASSERT_UNITS_FIT;
        // SAFETY: the scheduler is pinned (the receiver is `Pin<&mut Self>`), so
        // its address is fixed for the workers' raw pointers. Take a raw pointer
        // so no live `&mut` aliases the workers' `*const Self`; the `&mut`
        // reborrows below are confined to moments when every worker is parked
        // (before the first publish and between frames), matching the proven
        // sketch discipline (202606071930).
        let me: *mut Self = unsafe { self.get_unchecked_mut() };

        // First call: compute the const grouping into the gate2_* fields and spawn
        // the persistent pool once. Workers park immediately on `seq == 0`.
        let already = unsafe { (*me).spawned.0 };
        if !already {
            let mut phase = [USize::ZERO; GATE2_MAX_UNITS];
            let mut trunk = [USize::ZERO; GATE2_MAX_UNITS];
            let n = grouping_arrays::<
                WuVals,
                Stores,
                GW,
                <D as PlanDims>::Units,
                <D as PlanDims>::Stores,
                <D as PlanDims>::AdjRow,
            >(&mut phase, &mut trunk);
            let count = n.0; // lint:allow(no-bare-numeric) reason: live unit count; tracked: #121
            let mut nphases = 0; // lint:allow(no-bare-numeric) reason: phase count accumulator; tracked: #121
            let mut u = 0; // lint:allow(no-bare-numeric) reason: unit index; tracked: #121
            while u < count {
                if phase[u].0 + 1 > nphases {
                    nphases = phase[u].0 + 1; // lint:allow(no-bare-numeric) reason: phase successor; tracked: #121
                }
                u += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
            }
            let ncores = pool.worker_count();
            // SAFETY: no workers running yet; exclusive setup of pinned fields.
            unsafe {
                (*me).gate2_phase = phase;
                (*me).gate2_trunk = trunk;
                (*me).gate2_n = n;
                (*me).gate2_nphases = USize(nphases);
                (*me).gate2_ncores = ncores;
            }
            let mut c = 0; // lint:allow(no-bare-numeric) reason: core index; tracked: #121
            while c < ncores.0 {
                // SAFETY: pinned, stable address; the ctx outlives every worker
                // (Drop joins via await_exit before teardown).
                unsafe {
                    (*me).worker_ctxs[c] = WorkerCtx { sched: me as *const (), core_id: c };
                }
                let cp = SendCtxPtr(unsafe { &(*me).worker_ctxs[c] as *const WorkerCtx });
                pool.spawn(move || {
                    let cp = cp; // capture the Send wrapper whole, not the raw field
                    worker_main::<Cfg, WuVals, Vals, CS, D, Stores, Clk, Witnesses, GW>(cp.0);
                });
                c += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
            }
            // SAFETY: setup complete; mark spawned.
            unsafe {
                (*me).spawned = Bool::TRUE;
            }
        }

        // E8 adapt: sample the frame start on the main thread; the cold-start
        // state is the EMA seed flag. SAFETY: every worker is parked between
        // frames; exclusive field reads.
        let frame_start = unsafe { (*me).clock.now_ns() };
        let ema_seed = Bool(unsafe { (*me).first_frame.load(Ordering::Relaxed) });
        // Frame start: zero accumulators (every worker is parked).
        // SAFETY: between frames, no worker is dereferencing the bindings.
        unsafe {
            (*me).bindings.reset_accumulators();
        }
        // E4 slice 1: advance the virtual epoch once per frame, before the
        // publish, while every worker is parked. Workers read it after the
        // publish under the frame happens-before; a stale fire from last frame no
        // longer matches this frame's epoch (epoch-based reset).
        // SAFETY: every worker is parked between frames; exclusive field write.
        unsafe {
            (*me).virtual_epoch.fetch_add(1, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-frame epoch successor; tracked: #121
            (*me).meta_block.metrics.pass_count.set(USize((*me).meta_block.metrics.pass_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: per-frame meta pass_count; tracked: #121
        }
        // E4 parity, unit-outer path: the main thread is the designated core for
        // the meta bands, with the frame publish/await pair as the two ordering
        // barriers. The leading bands (plan, skipped on a clean frame, then the
        // remaining pre-consumer bands) dispatch here, before the publish, so
        // every worker's consumer slice work happens-after them; the trailing
        // bands dispatch after the await plus merge below. `core = 0, ncores =
        // 1` makes this thread own every trunk in the dispatched phases. The
        // band ranges are const and empty for a no-meta carrier.
        let unit_outer = unsafe { (*me).carrier_unit_outer() }.0;
        let pre_phases = pre_consumer_phase_count::<
            WuVals,
            Stores,
            GW,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
        >()
        .0; // lint:allow(no-bare-numeric) reason: leading-band loop bound; tracked: #121
        if unit_outer && pre_phases > 0 {
            let start = if unsafe { (*me).first_frame.load(Ordering::Relaxed) } {
                0 // lint:allow(no-bare-numeric) reason: plan-dirty frame runs the plan band; tracked: #121
            } else {
                plan_phase_count::<
                    WuVals,
                    Stores,
                    GW,
                    <D as PlanDims>::Units,
                    <D as PlanDims>::Stores,
                    <D as PlanDims>::AdjRow,
                >()
                .0 // lint:allow(no-bare-numeric) reason: clean frame skips the plan band; tracked: #121
            };
            let all = <<D as PlanDims>::AdjRow as Identity>::ZERO.bitnot();
            let epoch = USize(unsafe { (*me).virtual_epoch.load(Ordering::Relaxed) });
            let mut p = start;
            while p < pre_phases {
                let mut rank = USize::ZERO;
                // SAFETY: every worker is parked (pre-publish); exclusive frame
                // access to the bindings and the meta block.
                unsafe {
                    (*me).wu_values.dispatch_core(
                        &(*me).wu_values,
                        USize(p),
                        USize::ZERO,
                        USize(1), // lint:allow(no-bare-numeric) reason: designated thread owns every trunk; tracked: #121
                        &mut rank,
                        &(*me).bindings,
                        &(*me).meta_block,
                        MorselRange::new(USize::ZERO, USize::ZERO),
                        all,
                        epoch,
                    );
                }
                p += 1; // lint:allow(no-bare-numeric) reason: phase step; tracked: #121
            }
        }
        let ncores = unsafe { (*me).gate2_ncores };
        // One publish/await per frame. Workers run every waist-bounded phase hot
        // and cross each interior waist via the worker-side sense-reversing
        // `waist_barrier`; the main thread no longer round-trips per phase. It
        // publishes the frame once and waits for every worker to finish all
        // phases (the per-frame waist barrier is now worker-side, not here).
        // SAFETY: `pool` is a pinned field at a stable address; the frame
        // helpers only touch its atomics.
        let pool_frame = unsafe { &(*me).pool };
        frame_publish(pool_frame);
        frame_await_done(pool_frame, ncores);
        // Deviation 9: for the unit-outer accumulator carrier, each worker
        // appended into its own per-core region of the reserved buffer and
        // published its per-accumulator live counts. Forward-compact those
        // regions into each accumulator's `[0, sum)` prefix and set the binding
        // live length, so downstream readers see the same contiguous prefix
        // single-core `run()` would produce. The `frame_await_done` Acquire
        // paired with the workers' `frame_done_arrive` Release publishes the live
        // counts; load them Relaxed under that happens-before.
        // SAFETY: all workers re-parked; exclusive access to the bindings + array.
        if unsafe { (*me).carrier_unit_outer() }.0 {
            let total0 = unsafe { (*me).record_count.0 }; // lint:allow(no-bare-numeric) reason: frame record count; tracked: #121
            let ncores0 = ncores.0.max(1); // lint:allow(no-bare-numeric) reason: avoid div by zero; tracked: #121
            let per = (total0 + ncores0 - 1) / ncores0; // lint:allow(no-bare-numeric) reason: ceil record slice; tracked: #121
            let mut live = [USize::ZERO; MAX_CORES * GATE2_MAX_ACCUMS];
            let mut c = 0; // lint:allow(no-bare-numeric) reason: core index; tracked: #121
            while c < ncores0 {
                let mut a = 0; // lint:allow(no-bare-numeric) reason: accum index; tracked: #121
                while a < GATE2_MAX_ACCUMS {
                    let slot = c * GATE2_MAX_ACCUMS + a; // lint:allow(no-bare-numeric) reason: flat (core,accum) index; tracked: #121
                    live[slot] = USize(unsafe { (*me).gate2_accum_live[slot].load(Ordering::Relaxed) }); // lint:allow(no-bare-numeric) reason: load published live count; tracked: #121
                    a += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
                }
                c += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
            }
            let mut accum_idx = USize::ZERO;
            unsafe {
                (*me).bindings.merge_accums(
                    USize(per),
                    ncores,
                    USize(total0),
                    &live,
                    USize(GATE2_MAX_ACCUMS),
                    &mut accum_idx,
                );
            }
        }
        // E4 parity, unit-outer path: the trailing meta bands (the schedule-end
        // epilogue) dispatch on the main thread after the await plus merge, so
        // they happen-after all consumer work and an epilogue hook's appends
        // land after the merged consumer data (single-core buffer order).
        if unit_outer {
            let cend = consumer_phase_end::<
                WuVals,
                Stores,
                GW,
                <D as PlanDims>::Units,
                <D as PlanDims>::Stores,
                <D as PlanDims>::AdjRow,
            >()
            .0; // lint:allow(no-bare-numeric) reason: trailing-band loop start; tracked: #121
            let nphases = unsafe { (*me).gate2_nphases.0 }; // lint:allow(no-bare-numeric) reason: trailing-band loop bound; tracked: #121
            let all = <<D as PlanDims>::AdjRow as Identity>::ZERO.bitnot();
            let epoch = USize(unsafe { (*me).virtual_epoch.load(Ordering::Relaxed) });
            let mut p = cend;
            while p < nphases {
                let mut rank = USize::ZERO;
                // SAFETY: every worker re-parked (post-await); exclusive frame
                // access to the bindings and the meta block.
                unsafe {
                    (*me).wu_values.dispatch_core(
                        &(*me).wu_values,
                        USize(p),
                        USize::ZERO,
                        USize(1), // lint:allow(no-bare-numeric) reason: designated thread owns every trunk; tracked: #121
                        &mut rank,
                        &(*me).bindings,
                        &(*me).meta_block,
                        MorselRange::new(USize::ZERO, USize::ZERO),
                        all,
                        epoch,
                    );
                }
                p += 1; // lint:allow(no-bare-numeric) reason: phase step; tracked: #121
            }
        }
        // Capture the change_class signal before the seed is consumed: a
        // non-empty store_dirty means an input change was seen this frame. The
        // increment runs here on the main thread after every worker re-parks, so
        // it shares the single-core fold's discipline. A consumer's own append on
        // a worker thread that over-runs an accumulator panics there (the append
        // path's capacity assert); a worker panic can stall the join rather than
        // abort cleanly, which is the accepted failure mode for that contract
        // violation (the over-capacity should_panic test runs single-core).
        // SAFETY: all phases done, every worker re-parked.
        let stores_changed = unsafe { !(*me).store_dirty.get().is_empty().0 };
        // The frame consumed the change seed; clear it and leave cold-start.
        // SAFETY: all phases done, every worker re-parked.
        unsafe {
            (*me).store_dirty.set(AccessMask::empty());
            (*me).first_frame.store(false, Ordering::Relaxed);
        }
        // E8 adapt: fold this frame's duration into the pass-duration EMA on
        // the main thread, after the await plus merge plus trailing bands.
        // SAFETY: every worker re-parked; between-frames write, same
        // discipline as virtual_epoch and pass_count.
        unsafe {
            let m = &(*me).meta_block.metrics;
            m.ema_pass_duration_ns.set(fold_ema(
                m.ema_pass_duration_ns.get(),
                (*me).clock.now_ns() - frame_start,
                ema_seed,
            ));
            m.last_record_count.set((*me).record_count);
            if stores_changed {
                m.change_seen_count.set(USize(m.change_seen_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: increment by one frame; tracked: #121
            }
            // E8 adapt, core-idle axis: reduce this frame's per-core barrier
            // idle (filled by the waist barrier follower parks) to the worst
            // core, then zero the accumulators for the next frame. Worst-core,
            // not sum, because the adapt trigger is "is some core starved".
            // Bounded by the slot count, never the worker count (which can
            // exceed MAX_CORES).
            let acc = &(*me).pool.idle_accumulator;
            let mut worst = 0u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: raw nanos max reduction; tracked: #121
            let mut c = 0; // lint:allow(no-bare-numeric) reason: slot index; tracked: #121
            while c < acc.len() {
                let v = acc[c].swap(0, Ordering::AcqRel); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: read-and-reset accumulator; tracked: #121
                if v > worst {
                    worst = v;
                }
                c += 1; // lint:allow(no-bare-numeric) reason: slot index step; tracked: #121
            }
            m.idle_ns.set(Nanos::from_raw(worst));
        }
        Cfg::Out::default()
    }

    /// One core's dispatch for one waist-bounded phase: compute that core's
    /// per-phase unit mask (the trunks it owns this phase, by round-robin) and
    /// walk the carrier gated by it, morsel-outer (or a single empty-morsel walk
    /// for a record-less frame so a resource-only unit runs once).
    ///
    /// The shared per-(core,phase) primitive: the single-threaded `run_parallel`
    /// sweep calls it phase-outer / core-inner, and the threaded worker mainloop
    /// (round B2) calls it per phase for its own core with `phase_barrier_arrive`
    /// between phases. Reads only `wu_values` + `bindings`; the trunks a core
    /// owns this phase are column-disjoint from sibling cores' trunks, so
    /// concurrent calls for distinct cores in the same phase touch no shared
    /// column.
    ///
    /// Whether the carrier is unit-outer (accumulator-bearing): any fiber whose
    /// `morsel_local` bit is false. Mirrors the decision `run` makes. An
    /// accumulator fiber stays unit-outer (each unit completes its full record
    /// range), so the threaded path routes the whole carrier through the per-core
    /// bindings rebase (deviation 9) rather than the morsel-local phase walk.
    fn carrier_unit_outer(&self) -> Bool {
        let descriptors = self.fiber_dispatch.as_ref();
        let fcount = self.fiber_dispatch_count.0.min(descriptors.len()); // lint:allow(no-bare-numeric) reason: fiber descriptor count; tracked: #121
        let mut unit_outer = false; // lint:allow(no-bare-numeric) reason: local accumulator flag; tracked: #121
        let mut fi = 0; // lint:allow(no-bare-numeric) reason: fiber index; tracked: #121
        while fi < fcount {
            if !descriptors[fi].morsel_local.0 {
                unit_outer = true; // lint:allow(no-bare-numeric) reason: local flag set; tracked: #121
            }
            fi += 1; // lint:allow(no-bare-numeric) reason: index step; tracked: #121
        }
        Bool(unit_outer)
    }

    fn run_core_phase<Witnesses, GW>(
        &self,
        phase: &[USize],
        trunk: &[USize],
        n: USize,
        core: USize,
        p: USize,
        ncores: USize,
        total: USize,
        msize: USize,
    ) where
        WuVals: RunFiber<<Vals as BindingsFor>::Bindings, Witnesses>,
        WuVals: RunTrunkDispatch<
            WuVals,
            <Vals as BindingsFor>::Bindings,
            Witnesses,
            GW,
            Stores,
            <D as PlanDims>::Units,
            <D as PlanDims>::Stores,
            <D as PlanDims>::AdjRow,
            0, // lint:allow(no-bare-numeric) reason: const-generic entry position; tracked: #121
        >,
        WuVals: BundleMasks<Stores, GW, <D as PlanDims>::Stores>,
        <D as PlanDims>::Units: ConstCapacity,
        <D as PlanDims>::AdjRow: BitAccess + Identity,
    {
        // E4 slice 1: the worker reads the per-frame epoch set by `run_parallel`
        // before the publish (stable for the frame under the publish/await
        // happens-before).
        let epoch = USize(self.virtual_epoch.load(Ordering::Relaxed));
        let total = total.0; // lint:allow(no-bare-numeric) reason: frame record count; tracked: #121
        let tphase = phase_trunk_count(phase, trunk, n, p);
        // All-ones dirty: run_parallel dispatches the pure-RAW path (no
        // incremental skip), so every owned member runs.
        let all = <<D as PlanDims>::AdjRow as Identity>::ZERO.bitnot();
        if tphase.0 == 1 && ncores.0 > 1 && total > 0 {
            // Head+tail convergence (spec :770): a single-trunk waist-bounded
            // phase is the serial bottleneck. Ownership there is by record slice,
            // not by trunk, so all cores walk the same one trunk (the whole-phase
            // mask) over a disjoint ceil-sized record slice, the union covering
            // [0, total) with no gap or overlap (surplus cores get lo == hi and
            // do nothing). Stays on the runtime-mask run_gated path; the per-trunk
            // dispatch_core walk cannot express a record-range split.
            let per = (total + ncores.0 - 1) / ncores.0; // lint:allow(no-bare-numeric) reason: ceil record slice; tracked: #121
            let lo = (core.0 * per).min(total); // lint:allow(no-bare-numeric) reason: slice start; tracked: #121
            let hi = (lo + per).min(total); // lint:allow(no-bare-numeric) reason: slice end; tracked: #121
            let mask = phase_mask::<<D as PlanDims>::AdjRow>(phase, n, p);
            let msize = msize.0; // lint:allow(no-bare-numeric) reason: morsel length; tracked: #121
            let mut start = lo; // lint:allow(no-bare-numeric) reason: morsel start; tracked: #121
            while start < hi {
                let len = msize.min(hi - start);
                self.wu_values.run_gated(
                    &self.bindings,
                    &self.meta_block,
                    MorselRange::new(USize(start), USize(len)),
                    mask,
                    USize::ZERO,
                    epoch,
                );
                start += len; // lint:allow(no-bare-numeric) reason: morsel step; tracked: #121
            }
        } else if total == 0 {
            // Record-less frame: one empty-morsel dispatch_core so a resource-only
            // trunk this core owns runs exactly once.
            let mut rank = USize::ZERO;
            self.wu_values.dispatch_core(
                &self.wu_values,
                p,
                core,
                ncores,
                &mut rank,
                &self.bindings,
                &self.meta_block,
                MorselRange::new(USize::ZERO, USize::ZERO),
                all,
                epoch,
            );
        } else {
            // Ordinary trunk-rank ownership over the full range: per morsel,
            // dispatch_core fires the trunks this core owns as compiled per-trunk
            // monos (one runtime ownership branch per trunk-root, not per unit).
            let msize = msize.0; // lint:allow(no-bare-numeric) reason: morsel length; tracked: #121
            let mut start = 0; // lint:allow(no-bare-numeric) reason: morsel start; tracked: #121
            while start < total {
                let len = msize.min(total - start);
                let mut rank = USize::ZERO;
                self.wu_values.dispatch_core(
                    &self.wu_values,
                    p,
                    core,
                    ncores,
                    &mut rank,
                    &self.bindings,
                    &self.meta_block,
                    MorselRange::new(USize(start), USize(len)),
                    all,
                    epoch,
                );
                start += len; // lint:allow(no-bare-numeric) reason: morsel step; tracked: #121
            }
        }
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
        // E8 adapt: sample the frame start; the cold-start state is the EMA
        // seed flag (the first frame stores its raw duration).
        let frame_start = self.clock.now_ns();
        let ema_seed = Bool(self.first_frame.load(Ordering::Relaxed));
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
        self.virtual_epoch.fetch_add(1, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: per-pass epoch successor; tracked: #121
        self.meta_block.metrics.pass_count.set(USize(self.meta_block.metrics.pass_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: per-pass meta pass_count; tracked: #121
        let epoch = USize(self.virtual_epoch.load(Ordering::Relaxed));
        let msize = Cfg::MORSEL_SIZE.0.max(1);
        let total = self.record_count.0;
        if total == 0 {
            fused.run_gated(
                &self.bindings,
                &self.meta_block,
                MorselRange::new(USize::ZERO, USize::ZERO),
                dirty,
                USize::ZERO,
                epoch,
            );
        } else {
            let mut start = 0;
            while start < total {
                let len = msize.min(total - start);
                fused.run_gated(
                    &self.bindings,
                    &self.meta_block,
                    MorselRange::new(USize(start), USize(len)),
                    dirty,
                    USize::ZERO,
                    epoch,
                );
                start += len;
            }
        }
        // Capture the change_class signal before the seed is consumed.
        let stores_changed = !self.store_dirty.get().is_empty().0;
        self.store_dirty.set(AccessMask::empty());
        self.first_frame.store(false, Ordering::Relaxed);
        // E8 adapt: fold this frame's duration into the pass-duration EMA.
        // Between frames, so the write needs no synchronisation.
        let m = &self.meta_block.metrics;
        m.ema_pass_duration_ns
            .set(fold_ema(m.ema_pass_duration_ns.get(), self.clock.now_ns() - frame_start, ema_seed));
        m.last_record_count.set(self.record_count);
        if stores_changed {
            m.change_seen_count.set(USize(m.change_seen_count.get().0 + 1)); // lint:allow(no-bare-numeric) reason: increment by one frame; tracked: #121
        }
        Cfg::Out::default()
    }

    /// Borrow the resource bindings. Hidden test accessor: lets in-crate
    /// and integration tests walk the bindings nodes to confirm the
    /// moved-in resource values. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __bindings(&self) -> &<Vals as BindingsFor>::Bindings {
        &self.bindings
    }

    /// Set `store_dirty` to a non-empty mask. Hidden test accessor: the only
    /// public trigger for `store_dirty` is `replace_resource<T: PlanAffecting>`,
    /// and `PlanAffecting` is sealed, so a white-box test for the change_class
    /// signal sets the dirty mask directly. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __mark_store_dirty(&self) {
        self.store_dirty.set(AccessMask::empty().set(USize(0))); // lint:allow(no-bare-numeric) reason: store index zero; tracked: #121
    }

    /// Read the core-idle adapt metric (`SchedulerMetrics::idle_ns`) after a
    /// frame. Hidden test accessor: an accumulator-bearing carrier takes the
    /// unit-outer no-barrier path (zero idle by design), so a barrier-driven
    /// signal cannot be read back through an accumulator append. Not part of the
    /// supported surface; consumers read it through an `OnMeta<ScheduleEnd>` hook.
    #[doc(hidden)]
    pub fn __idle_ns(&self) -> Nanos {
        self.meta_block.metrics.idle_ns.get()
    }

    /// Read the engine-internal per-phase duration EMA for phase `p`. Hidden
    /// test accessor: per-phase EMA is engine-internal (it feeds the eventual
    /// `select_adapt_config`), with no `OnMeta` consumer read, so a white-box
    /// test asserts the recorded per-phase durations directly.
    #[doc(hidden)]
    pub fn __phase_ema(&self, p: USize) -> Nanos {
        self.phase_ema[p.0].get()
    }

    /// E8 adapt tuning decision (domain-22, R5): scan the per-phase EMA and set
    /// the phase-imbalance reconfigure trigger when one active phase dominates the
    /// least active phase. Active phases are the slots with a nonzero EMA; the
    /// trigger fires only with at least two active phases and `max > FACTOR * min`.
    /// Pure read of engine-internal state; the actuation that acts on the trigger
    /// is a follow-up. `BALANCE_FACTOR` is a tunable default (consumer-tunable per
    /// the caps-are-defaults discipline).
    fn select_adapt_config(&self) {
        const BALANCE_FACTOR: u64 = 2; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: imbalance ratio default; tracked: #121
        let mut active = 0usize; // lint:allow(no-bare-numeric) reason: active-phase counter; tracked: #121
        let mut max = 0u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: raw-nanos max over active phases; tracked: #121
        let mut min = u64::MAX; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: raw-nanos min over active phases; tracked: #121
        let mut p = 0usize; // lint:allow(no-bare-numeric) reason: phase-scan index; tracked: #121
        while p < self.phase_ema.len() {
            let e = self.phase_ema[p].get().to_raw();
            if e > 0 {
                active += 1; // lint:allow(no-bare-numeric) reason: count active phases; tracked: #121
                if e > max {
                    max = e;
                }
                if e < min {
                    min = e;
                }
            }
            p += 1; // lint:allow(no-bare-numeric) reason: phase-scan step; tracked: #121
        }
        let imbalanced = active >= 2 && max > min.saturating_mul(BALANCE_FACTOR); // lint:allow(no-bare-numeric) reason: imbalance predicate; tracked: #121
        self.adapt_reconfigure.set(Bool(imbalanced));
    }

    /// Read the phase-imbalance reconfigure trigger. Hidden test accessor:
    /// engine-internal, no consumer read yet (actuation is a follow-up).
    #[doc(hidden)]
    pub fn __adapt_reconfigure(&self) -> Bool {
        self.adapt_reconfigure.get()
    }

    /// Set a per-phase EMA slot directly. Hidden test accessor: lets a test drive
    /// `select_adapt_config` with a chosen balance state without depending on
    /// wall-clock timing.
    #[doc(hidden)]
    pub fn __set_phase_ema(&self, p: USize, ns: Nanos) {
        self.phase_ema[p.0].set(ns);
    }

    /// Run the adapt tuning decision. Hidden test accessor for the decision logic.
    #[doc(hidden)]
    pub fn __select_adapt_config(&self) {
        self.select_adapt_config();
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
            plan_dirty: <<DefaultPlanDims as PlanDims>::PlanAffecting as Capacity>::from_fn(|_| AtomicBool::new(false)),
            plan_cache: PlanCache::new(),
            predecessor_masks:
                <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(
                    <DefaultPlanDims as PlanDims>::AdjRow::default(),
                ),
            read_masks: <<DefaultPlanDims as PlanDims>::Units as Capacity>::filled(
                AccessMask::empty(),
            ),
            store_dirty: Cell::new(AccessMask::empty()),
            first_frame: AtomicBool::new(true),
            virtual_epoch: AtomicUsize::new(0),
            meta_block: MetaBlock::default(),
            clock: DefaultClock::new(),
            bindings: crate::resource::bindings::BindingNil,
            storage: NullColumnStorage,
            wu_values: WuNil,
            pool: empty_pool_frame(),
            worker_ctxs: [const { WorkerCtx { sched: core::ptr::null(), core_id: 0 } }; MAX_CORES], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: worker ctx init, core index; tracked: #121
            spawned: Bool::FALSE,
            gate2_phase: [USize::ZERO; GATE2_MAX_UNITS],
            gate2_trunk: [USize::ZERO; GATE2_MAX_UNITS],
            gate2_n: USize::ZERO,
            gate2_nphases: USize::ZERO,
            gate2_ncores: USize::ZERO,
            gate2_accum_live: [const { core::sync::atomic::AtomicUsize::new(0) }; MAX_CORES * GATE2_MAX_ACCUMS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic publish array init; tracked: #121
            phase_ema: [const { Cell::new(Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase EMA zero-init; tracked: #121
            phase_accum: [const { Cell::new(Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase accumulator zero-init; tracked: #121
            adapt_reconfigure: Cell::new(Bool::FALSE),
            _pin: PhantomPinned,
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
    wu_values: WuVals,
    clock: Clk,
    _phantom: PhantomData<(Wus, Stores, Platform)>,
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
        let (store_values, wu_values) =
            <<P::Dispatch as RouterKind>::Kind as Place<P>>::place(
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

impl<Wus, Stores, Platform, Vals, WuVals, Clk> SchedulerBuilder<Wus, Stores, Platform, Vals, WuVals, Clk>
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
    ) -> notko::Outcome<Scheduler<DefaultRunCfg, WuVals, Vals, CS, DefaultPlanDims, Stores, Clk>, BuildError>
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
                    plan_dirty: <<DefaultPlanDims as PlanDims>::PlanAffecting as Capacity>::from_fn(|_| AtomicBool::new(false)),
                    plan_cache: PlanCache::new(),
                    predecessor_masks: plan.predecessor_masks,
                    read_masks: plan.read_masks,
                    store_dirty: Cell::new(AccessMask::empty()),
                    first_frame: AtomicBool::new(true),
                    virtual_epoch: AtomicUsize::new(0),
                    meta_block: MetaBlock::default(),
                    clock: self.clock,
                    bindings,
                    storage,
                    wu_values,
                    pool: empty_pool_frame(),
                    worker_ctxs: [const { WorkerCtx { sched: core::ptr::null(), core_id: 0 } }; MAX_CORES], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: worker ctx init, core index; tracked: #121
                    spawned: Bool::FALSE,
                    gate2_phase: [USize::ZERO; GATE2_MAX_UNITS],
                    gate2_trunk: [USize::ZERO; GATE2_MAX_UNITS],
                    gate2_n: USize::ZERO,
                    gate2_nphases: USize::ZERO,
                    gate2_ncores: USize::ZERO,
            gate2_accum_live: [const { core::sync::atomic::AtomicUsize::new(0) }; MAX_CORES * GATE2_MAX_ACCUMS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic publish array init; tracked: #121
            phase_ema: [const { Cell::new(Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase EMA zero-init; tracked: #121
            phase_accum: [const { Cell::new(Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase accumulator zero-init; tracked: #121
            adapt_reconfigure: Cell::new(Bool::FALSE),
                    _pin: PhantomPinned,
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
                    plan_dirty: <<DefaultPlanDims as PlanDims>::PlanAffecting as Capacity>::from_fn(|_| AtomicBool::new(false)),
                    plan_cache: PlanCache::new(),
                    predecessor_masks: plan.predecessor_masks,
                    read_masks: plan.read_masks,
                    store_dirty: Cell::new(AccessMask::empty()),
                    first_frame: AtomicBool::new(true),
                    virtual_epoch: AtomicUsize::new(0),
                    meta_block: MetaBlock::default(),
                    clock: self.clock,
                    bindings,
                    storage,
                    wu_values,
                    pool: empty_pool_frame(),
                    worker_ctxs: [const { WorkerCtx { sched: core::ptr::null(), core_id: 0 } }; MAX_CORES], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: worker ctx init, core index; tracked: #121
                    spawned: Bool::FALSE,
                    gate2_phase: [USize::ZERO; GATE2_MAX_UNITS],
                    gate2_trunk: [USize::ZERO; GATE2_MAX_UNITS],
                    gate2_n: USize::ZERO,
                    gate2_nphases: USize::ZERO,
                    gate2_ncores: USize::ZERO,
            gate2_accum_live: [const { core::sync::atomic::AtomicUsize::new(0) }; MAX_CORES * GATE2_MAX_ACCUMS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic publish array init; tracked: #121
            phase_ema: [const { Cell::new(Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase EMA zero-init; tracked: #121
            phase_accum: [const { Cell::new(Nanos::from_raw(0)) }; GATE2_MAX_UNITS], // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-phase accumulator zero-init; tracked: #121
            adapt_reconfigure: Cell::new(Bool::FALSE),
                    _pin: PhantomPinned,
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
            h.morsel_windows_id(),
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
