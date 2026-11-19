//! `RunCfg` trait + dispatch routing slot for the consumer-supplied
//! run-config type.
//!
//! Every consumer registers exactly one `RunCfg`
//! type via `.with(MyRunCfg)`. The typestate slot is single-
//! occupant; a second `.with(OtherRunCfg)` registration fails at
//! type-check time. The consumer's `RunCfg::Out` associated type
//! determines `Scheduler::run()`'s return shape via the dispatch-
//! routing slot.
//!
//! `PlanAffecting` is an open marker trait declaring
//! that replacing a resource's value via `Scheduler::replace_resource`
//! must dirty the plan-cache. Default: NOT plan-affecting; consumers
//! opt in.
//!
//! A single `Virtual<AnomalyFired>` flag signals anomalies. Per-
//! anomaly detail lives on the metrics Resources as bool fields.

use core::marker::PhantomData;

use arvo::{Additive, Identity, USize};

use crate::access::Cons;
use crate::accum_overflow::{AccumOverflow, Refuse};
use crate::builder_input::{BuilderInput, Dispatch};
use crate::sealed;

/// Required-field accessor: every `RunCfg` impl declares the
/// pipeline's record count. Supertrait bound on `RunCfg`.
pub trait HasRecordCount: sealed::Sealed {
    fn record_count(&self) -> USize;
}

/// Open marker on resource value types: replacing this resource's
/// value via `Scheduler::replace_resource` must dirty the plan
/// cache. Per-type consumer opt-in mirroring `Replaceable`, with
/// which it is mutually exclusive (`PlanAffecting` dominant; see the
/// negative impl at the `Replaceable` declaration). Default: NOT
/// plan-affecting. Consumers opt in for resources whose value shape
/// changes the plan (e.g., the `RunCfg` resource itself). Unsealed.
pub trait PlanAffecting {}

/// Consumer-supplied run-config trait. Single-slot typestate; one
/// `RunCfg` impl per Scheduler<Cfg> instantiation. Carries
/// `APPROACH_E_THRESHOLD`, `MICRO_MORSEL_INTERVAL`,
/// `MAX_DRIFT_RECORDS` and `MAX_PLAN_AFFECTING_RESOURCES` among
/// its tunables.
///
/// Per `arvo-toolbox-not-policer.md`: every assoc const is a
/// consumer-tunable default on this trait, not a hardcoded
/// substrate constant. Override by writing the const explicitly in
/// the consumer's `impl RunCfg` block.
pub trait RunCfg:
    sealed::Sealed + BuilderInput<Dispatch = RunCfgDispatch<Self>> + HasRecordCount + Sized + 'static
{
    /// The run's output: the overflow choice's output, and nothing else.
    type Out;

    /// What the run does when an accumulator refused an append. Every run
    /// entry returns `Self::Overflow::conclude(verdict)`, where the verdict is
    /// `Ok(())`, or `Err` with the first accumulator that refused, walked in
    /// registration order.
    type Overflow: AccumOverflow<Out = Self::Out>;

    /// Cap on resources marked `PlanAffecting`. Drives the dirty-
    /// bitmask width. Default 256 (32-byte bitmask, fits one cache
    /// line). Consumer-tunable
    /// surface; per `arvo-toolbox-not-policer.md`, override by
    /// writing the const explicitly.
    const MAX_PLAN_AFFECTING_RESOURCES: USize = USize(256);

    /// Bounded middle-tier spin budget the frame waits run before
    /// parking. The spin re-loads the wake word each iteration, so the
    /// lost-wakeup discipline is untouched; zero is park-immediately
    /// exactly. Default 0 per the re-examined wake_policy evidence
    /// (record 202607210100): across seven wait policies, three sizes,
    /// and three invocations, park is never worse and no spin policy
    /// is ever better; spin loads contend the wake words against the
    /// publisher and the done arrivals, and on aarch64 the
    /// `spin_loop` iteration is ISB at 9 to 20 ns, so a non-zero
    /// override should drop the hint or budget in wall-time rather
    /// than iterations. Consumer-tunable per
    /// `arvo-toolbox-not-policer`; the telemetry-driven `pick_tier`
    /// selection over `predicted_wait_ns` is the adapt-phase
    /// follow-up, with park-immediately as its bar.
    const WAKE_SPIN_BUDGET: USize = USize(0);

    /// Records per morsel window the single-core run-loop windows the record
    /// range into. Coarser than `MICRO_MORSEL_INTERVAL` (a morsel contains
    /// several micro-morsels). Default 256 (a multiple of the 64-record
    /// micro-morsel interval). Consumer-tunable per `arvo-toolbox-not-policer`;
    /// override by writing the const explicitly. The plan's per-fiber morsel
    /// formula (`L1_usable / sum(write_sizes)`) supersedes this default when the
    /// phase/fiber dispatch tier lands; until then it is the run-loop's window.
    const MORSEL_SIZE: USize = USize(256);

    /// Usable L1 write budget in bytes for the per-fiber morsel-window
    /// formula: `window = (L1_USABLE / sum of write sizes)
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
    /// Default 64 (one cache line of f32-shaped data).
    const MICRO_MORSEL_INTERVAL: USize = USize(64);

    /// Max inter-fiber misalignment in records before forced realign.
    /// Pow2 cap. Default 32 (half a micro-morsel).
    const MAX_DRIFT_RECORDS: USize = USize(32);

    /// Record count threshold above which the plan picks the
    /// `ScheduleMega` dispatch approach. Bench-validated default
    /// `10_000`.
    const APPROACH_E_THRESHOLD: USize = USize(10_000);
}

/// Substrate default RunCfg. Consumers may use this when they have
/// no special run-config needs. `O` is the overflow choice:
/// `DefaultRunCfg` (with `Refuse`) makes `Scheduler::run()` return
/// `Outcome<(), AccumRefused>`, and `DefaultRunCfg<Assert>` makes it
/// return `()` and panic when an accumulator refused an append.
pub struct DefaultRunCfg<O: AccumOverflow = Refuse> {
    /// Record count for the pipeline. Drives morsel sizing.
    pub record_count: USize,
    _overflow:        PhantomData<O>,
}

impl<O: AccumOverflow> DefaultRunCfg<O> {
    /// Construct with the given record count.
    pub const fn new(record_count: USize) -> Self {
        Self {
            record_count,
            _overflow: PhantomData,
        }
    }
}

impl<O: AccumOverflow> Default for DefaultRunCfg<O> {
    fn default() -> Self {
        Self::new(<USize as Identity<Additive>>::IDENTITY)
    }
}

impl<O: AccumOverflow> sealed::Sealed for DefaultRunCfg<O> {}

impl<O: AccumOverflow> HasRecordCount for DefaultRunCfg<O> {
    fn record_count(&self) -> USize {
        self.record_count
    }
}

impl<O: AccumOverflow> BuilderInput for DefaultRunCfg<O> {
    type Dispatch = RunCfgDispatch<Self>;
    type Init = Self;
}

impl<O: AccumOverflow> RunCfg for DefaultRunCfg<O> {
    type Out = <O as AccumOverflow>::Out;
    type Overflow = O;
}

impl<O: AccumOverflow> PlanAffecting for DefaultRunCfg<O> {}

/// Router for the consumer-supplied run-config. Single-slot
/// typestate; the builder transitions `RunCfgSlot::Empty → Filled<C>`
/// on first `.with(C)` where `C: RunCfg`; a second `.with(OtherC)`
/// fails compile.
///
/// Implementation note: the router routes the RunCfg type into the
/// **store accumulator** so it can be read via `ctx.resource::<C>()`
/// in WUs. The single-slot typestate enforcement lives on the
/// builder typestate; this router only carries the type-
/// level evidence for the slot.
pub struct RunCfgDispatch<C>(PhantomData<C>);

impl<C, Wus, Stores, Platform> Dispatch<Wus, Stores, Platform> for RunCfgDispatch<C>
where
    C: 'static,
{
    type NextPlatform = Platform;
    type NextStores = Cons<C, Stores>;
    type NextWus = Wus;
}

/// Single anomaly Virtual marker. Per-anomaly detail lives on
/// metrics Resources as bool fields.
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
/// Fired after every phase barrier closes. This is
/// the canonical observation point for `AdaptWu`, which reads per-
/// axis metrics, sets anomaly bools, and fires `Virtual<AnomalyFired>`
/// when any axis threshold trips.
pub struct ScheduleEnd;
