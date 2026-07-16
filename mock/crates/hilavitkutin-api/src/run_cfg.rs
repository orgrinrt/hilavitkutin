//! `RunCfg` trait + dispatch routing slot for the consumer-supplied
//! run-config type.
//!
//! Topic 1 axis 2: every consumer registers exactly one `RunCfg`
//! type via `.with(MyRunCfg)`. The typestate slot is single-
//! occupant; a second `.with(OtherRunCfg)` registration fails at
//! type-check time. The consumer's `RunCfg::Out` associated type
//! determines `Scheduler::run()`'s return shape via the dispatch-
//! routing slot.
//!
//! Topic 8 axis B: `PlanAffecting` sealed marker trait declares
//! that replacing a resource's value via `Scheduler::replace_resource`
//! must dirty the plan-cache. Default: NOT plan-affecting; consumers
//! opt in.
//!
//! Topic 5 audit-2 M7: a single `Virtual<AnomalyFired>` flag replaces
//! the previously-locked five per-anomaly Virtual markers. Per-
//! anomaly detail lives on the metrics Resources as bool fields.

use core::marker::PhantomData;

use arvo::{Identity, USize};

use crate::access::Cons;
use crate::builder_input::{BuilderInput, Dispatch};

mod sealed {
    pub trait Sealed {}
}

/// Required-field accessor: every `RunCfg` impl declares the
/// pipeline's record count. Topic 1 axis 2 supertrait bound on
/// `RunCfg`.
pub trait HasRecordCount: sealed::Sealed {
    fn record_count(&self) -> USize;
}

/// Sealed marker: indicates that replacing this resource's value
/// via `Scheduler::replace_resource` must dirty the plan-cache.
/// Default: NOT plan-affecting. Consumers opt in for resources
/// whose value shape changes the plan (e.g., the `RunCfg` resource
/// itself). Topic 8 axis B.
pub trait PlanAffecting: sealed::Sealed + BuilderInput {}

/// Consumer-supplied run-config trait. Single-slot typestate; one
/// `RunCfg` impl per Scheduler<Cfg> instantiation. Topic 1 axes
/// 1+2; Topic 4 axis H (`APPROACH_E_THRESHOLD`); Topic 7 axis C
/// (`MICRO_MORSEL_INTERVAL`, `MAX_DRIFT_RECORDS`); Topic 8 axis B
/// (`MAX_PLAN_AFFECTING_RESOURCES`).
///
/// Per `arvo-toolbox-not-policer.md`: every assoc const is a
/// consumer-tunable default on this trait, not a hardcoded
/// substrate constant. Override by writing the const explicitly in
/// the consumer's `impl RunCfg` block.
pub trait RunCfg:
    sealed::Sealed + BuilderInput<Dispatch = RunCfgDispatch<Self>> + HasRecordCount + Sized + 'static
{
    /// Successful run-output type. The `Scheduler::run() -> Self::Out`
    /// return type is parameterised by this. Typically
    /// `notko::Outcome<Summary, Error>` or `notko::Outcome<(), ()>`.
    type Out;

    /// Failure type surfaced through `Self::Out::Err`. Heavy consumers
    /// override to structured errors; simple consumers leave at `()`.
    type Err;

    /// Cap on resources marked `PlanAffecting`. Drives the dirty-
    /// bitmask width. Default 256 (32-byte bitmask, fits one cache
    /// line). Topic 8 axis B; user-bumped from 16 to 256 in the
    /// 2026-05-11 user-attention resolution. Consumer-tunable
    /// surface; per `arvo-toolbox-not-policer.md`, override by
    /// writing the const explicitly.
    const MAX_PLAN_AFFECTING_RESOURCES: USize = USize(256);

    /// Records per morsel window the single-core run-loop windows the record
    /// range into. Coarser than `MICRO_MORSEL_INTERVAL` (a morsel contains
    /// several micro-morsels). Default 256 (a multiple of the 64-record
    /// micro-morsel interval). Consumer-tunable per `arvo-toolbox-not-policer`;
    /// override by writing the const explicitly. The plan's per-fiber morsel
    /// formula (`L1_usable / sum(write_sizes)`) supersedes this default when the
    /// phase/fiber dispatch tier lands; until then it is the run-loop's window.
    const MORSEL_SIZE: USize = USize(256);

    /// Usable L1 write budget in bytes for the per-fiber morsel-window
    /// formula (domain 12): `window = (L1_USABLE / sum of write sizes)
    /// .clamp(MIN_MORSEL, MAX_MORSEL) & !3`. Default `24_576` (the 32 KB
    /// detection-fallback L1 times 0.75). Hardware detection, when the
    /// platform tier ships it, overrides the default SOURCE, not this
    /// knob. Consumer-tunable per `arvo-toolbox-not-policer`; override by
    /// writing the const explicitly.
    const L1_USABLE: USize = USize(24_576);

    /// Lower clamp of the per-fiber morsel window in records. The spec
    /// marks the floor configurable (not a design decision); default 64,
    /// one micro-morsel interval. Consumer-tunable.
    const MIN_MORSEL: USize = USize(64);

    /// Upper clamp of the per-fiber morsel window in records. Spec
    /// default 8192. Consumer-tunable.
    const MAX_MORSEL: USize = USize(8192);

    /// Records between micro-morsel inner-loop sync points. Pow2 cap.
    /// Default 64 (one cache line of f32-shaped data). Topic 7 axis C.
    const MICRO_MORSEL_INTERVAL: USize = USize(64);

    /// Max inter-fiber misalignment in records before forced realign.
    /// Pow2 cap. Default 32 (half a micro-morsel). Topic 7 axis C.
    const MAX_DRIFT_RECORDS: USize = USize(32);

    /// Record count threshold above which the plan picks the
    /// `ScheduleMega` dispatch approach. Bench-validated default
    /// `10_000` per Domain 17 L1559. Topic 4 axis H.
    const APPROACH_E_THRESHOLD: USize = USize(10_000);
}

/// Substrate default RunCfg. Consumers may use this when they have
/// no special run-config needs; the `Scheduler::run() -> Outcome<(), ()>`
/// shape is the typical case.
pub struct DefaultRunCfg {
    /// Record count for the pipeline. Drives morsel sizing.
    pub record_count: USize,
}

impl DefaultRunCfg {
    /// Construct with the given record count.
    pub const fn new(record_count: USize) -> Self {
        Self { record_count }
    }
}

impl Default for DefaultRunCfg {
    fn default() -> Self {
        Self::new(USize::ZERO)
    }
}

impl sealed::Sealed for DefaultRunCfg {}

impl HasRecordCount for DefaultRunCfg {
    fn record_count(&self) -> USize {
        self.record_count
    }
}

impl BuilderInput for DefaultRunCfg {
    type Init = Self;
    type Dispatch = RunCfgDispatch<Self>;
}

impl RunCfg for DefaultRunCfg {
    type Out = notko::Outcome<(), ()>;
    type Err = ();
}

impl PlanAffecting for DefaultRunCfg {}

/// Router for the consumer-supplied run-config. Single-slot
/// typestate; the builder transitions `RunCfgSlot::Empty → Filled<C>`
/// on first `.with(C)` where `C: RunCfg`; a second `.with(OtherC)`
/// fails compile.
///
/// Implementation note: the router routes the RunCfg type into the
/// **store accumulator** so it can be read via `ctx.resource::<C>()`
/// in WUs. The single-slot typestate enforcement lives on the
/// builder typestate (Pass 6); this router only carries the type-
/// level evidence for the slot.
pub struct RunCfgDispatch<C>(PhantomData<C>);

impl<C, Wus, Stores, Platform> Dispatch<Wus, Stores, Platform> for RunCfgDispatch<C>
where
    C: 'static,
{
    type NextWus = Wus;
    type NextStores = Cons<C, Stores>;
    type NextPlatform = Platform;
}

/// Single anomaly Virtual marker. Topic 5 audit-2 M7 lock: replaces
/// the previously locked five per-anomaly Virtuals. Per-anomaly
/// detail lives on metrics Resources as bool fields.
///
/// Observer WUs check `Virtual<AnomalyFired>` then query the
/// relevant `metrics::*` Resource for which anomaly fired.
pub struct AnomalyFired;

/// Meta-Virtual marker: plan-stage phase entry.
///
/// Fired by the engine at the start of plan computation. WUs bound
/// to `OnMeta<PlanStage>` observe DAG state before any execution.
pub struct PlanStage;

/// Meta-Virtual marker: per-core dispatch programs ready.
///
/// Fired after `DispatchCodegen::build` completes. WUs bound to
/// `OnMeta<ScheduleReady>` may inspect the assembled per-core programs.
pub struct ScheduleReady;

/// Meta-Virtual marker: pass start.
///
/// Fired immediately before per-core dispatch begins each pass. WUs
/// bound to `OnMeta<PassStart>` see the pre-execution snapshot.
pub struct PassStart;

/// Meta-Virtual marker: pass end (post all phases).
///
/// Fired after every phase barrier closes. Topic 5 axis A: this is
/// the canonical observation point for `AdaptWu`, which reads per-
/// axis metrics, sets anomaly bools, and fires `Virtual<AnomalyFired>`
/// when any axis threshold trips.
pub struct ScheduleEnd;
