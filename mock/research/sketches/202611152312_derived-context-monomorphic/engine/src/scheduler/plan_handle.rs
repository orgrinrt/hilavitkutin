//! The store-backed execution-plan locator, and the build-time plan machinery
//! that computes and stores it back.
//!
//! Split out of `scheduler/mod.rs` (file-size lint). No behaviour change.
//! `PlanHandle`'s fields and `PlanColumn` (with its `COUNT`) are
//! `pub(super)` rather than private: the pinned-offset test in
//! `scheduler::tests` constructs a `PlanHandle` literal and reads
//! `PlanColumn::COUNT` directly, and that test module is a sibling of this
//! one rather than a descendant, so plain module-privacy does not reach it.
//! `compute_plan`, `derive_phase_dispatch_order` and `store_plan` are
//! `pub(super)` because `SchedulerBuilder::build` / `build_with` (in
//! `scheduler::builder`, a sibling) call them.

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use hilavitkutin_api::{ColumnStorage, ColumnValue, StoreId};

use super::{BuildError, FiberDispatch, RecommendedOrder};
use crate::plan::project::{AccumStoresMask, BundleProject, StoreSizes};
use crate::plan::{
    DefaultPlanDims,
    ExecutionPlan,
    MorselBudget,
    PlanDims,
    compute_execution_plan,
    plan_inputs_from_bundle,
};

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
    pub(super) base:        USize,
    pub(super) phase_count: USize,
    pub(super) trunk_count: USize,
    pub(super) fiber_count: USize,
    pub(super) unit_count:  USize,
}

/// The closed set of store-backed plan columns. The variant's position is the
/// `StoreId` offset off a `PlanHandle`'s base.
#[derive(Copy, Clone)]
pub(super) enum PlanColumn {
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
    pub(super) const COUNT: usize = 6; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: closed-set cardinality used as a reservation span; tracked: #72
}

impl PlanHandle {
    /// The empty handle: no plan store-backed (the bare and `Default`
    /// scheduler, whose store reserves nothing).
    pub const fn empty() -> Self {
        Self {
            base:        <USize as Identity<Additive>>::IDENTITY,
            phase_count: <USize as Identity<Additive>>::IDENTITY,
            trunk_count: <USize as Identity<Additive>>::IDENTITY,
            fiber_count: <USize as Identity<Additive>>::IDENTITY,
            unit_count:  <USize as Identity<Additive>>::IDENTITY,
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
pub(super) fn compute_plan<Wus, Stores, BWit>(
    record_count: USize,
    budget: MorselBudget,
) -> notko::Outcome<ExecutionPlan<DefaultPlanDims>, BuildError>
where
    Wus: BundleProject<
            Stores,
            BWit,
            <DefaultPlanDims as PlanDims>::Units,
            <DefaultPlanDims as PlanDims>::Stores,
        >,
    Stores: AccumStoresMask<<DefaultPlanDims as PlanDims>::Stores>,
    Stores: StoreSizes<<DefaultPlanDims as PlanDims>::Stores>,
{
    let inputs = plan_inputs_from_bundle::<
        Wus,
        Stores,
        BWit,
        <DefaultPlanDims as PlanDims>::Units,
        <DefaultPlanDims as PlanDims>::Stores,
    >(record_count, budget);
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
        },
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
pub(super) fn derive_phase_dispatch_order(
    plan: &ExecutionPlan<DefaultPlanDims>,
) -> (
    <<DefaultPlanDims as PlanDims>::Units as arvo_tensor::Capacity>::Array<USize>,
    USize,
    <<DefaultPlanDims as PlanDims>::Fibers as arvo_tensor::Capacity>::Array<FiberDispatch>,
    USize,
) {
    let mut order = <<DefaultPlanDims as PlanDims>::Units as arvo_tensor::Capacity>::filled(
        <USize as Identity<Additive>>::IDENTITY,
    );
    let cap = order.as_ref().len();
    let mut descriptors = <<DefaultPlanDims as PlanDims>::Fibers as arvo_tensor::Capacity>::filled(
        FiberDispatch::default(),
    );
    let fd_cap = descriptors.as_ref().len();
    let mut fd = 0;
    let phases = plan.phases.as_ref();
    let trunks = plan.trunks.as_ref();
    let fibers = plan.fibers.as_ref();
    let morsel_windows = plan.morsel_windows.as_ref();
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
                    descriptors.as_mut()[fd] = FiberDispatch {
                        start:          USize(fib_start), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal dispatch-order cursors; tracked: #72
                        len:            USize(next - fib_start),
                        morsel_local:   fibers[f].morsel_local,
                        morsel_size:    if f < morsel_windows.len() {
                            morsel_windows[f]
                        } else {
                            <USize as Identity<Additive>>::IDENTITY
                        },
                        fiber_plan_idx: USize(f),
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
        notko::Outcome::Ok(()) => {},
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
pub(super) fn store_plan<CS: ColumnStorage>(
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
    match store_column(
        storage,
        handle.phases_id(),
        plan.phases.as_ref(),
        plan.phase_count,
    ) {
        notko::Outcome::Ok(()) => {},
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    match store_column(
        storage,
        handle.trunks_id(),
        plan.trunks.as_ref(),
        plan.trunk_count,
    ) {
        notko::Outcome::Ok(()) => {},
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    match store_column(
        storage,
        handle.fibers_id(),
        plan.fibers.as_ref(),
        plan.fiber_count,
    ) {
        notko::Outcome::Ok(()) => {},
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    match store_column(
        storage,
        handle.unit_meta_id(),
        plan.unit_meta.as_ref(),
        plan.unit_count,
    ) {
        notko::Outcome::Ok(()) => {},
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    match store_column(
        storage,
        handle.morsel_windows_id(),
        plan.morsel_windows.as_ref(),
        plan.fiber_count,
    ) {
        notko::Outcome::Ok(()) => {},
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    match store_column(
        storage,
        handle.rcm_order_id(),
        plan.rcm_order.as_ref(),
        plan.unit_count,
    ) {
        notko::Outcome::Ok(()) => {},
        notko::Outcome::Err(e) => return notko::Outcome::Err(e),
    }
    notko::Outcome::Ok(handle)
}
