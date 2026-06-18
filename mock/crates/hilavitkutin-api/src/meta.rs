//! Self-hosting meta pipeline surface (T5 §Q9).
//!
//! The scheduler schedules itself: the four lifecycle markers in `run_cfg`
//! (`PlanStage` / `ScheduleReady` / `PassStart` / `ScheduleEnd`) are fired as
//! virtuals by the engine kernel, and meta work units gate on them via the
//! `OnMeta<V>` schedule (in `work_unit`). This module carries the const lifecycle
//! classifier the grouping reads, plus the four meta resource markers and the
//! `MetaAccess` gate.
//!
//! `MetaVirtual` assigns each lifecycle marker a const RANK, the lifecycle
//! ordinal that orders the kernel: PlanStage < ScheduleReady < PassStart <
//! consumer < ScheduleEnd. The grouping makes the rank the outer phase key, so a
//! meta work unit lands in the phase band for its lifecycle point. See
//! `mock/research/sketches/202606082000_e4-slice2-lifecycle-classify` and
//! `202606082200_e4-slice2-rank-phase-renumber`.

use core::cell::Cell;

use arvo::USize;

use crate::platform::Nanos;
use crate::run_cfg::{PassStart, PlanStage, ScheduleEnd, ScheduleReady};

/// Lifecycle rank: plan-stage meta work units run first (rank 0), then
/// schedule-ready (1), then pass-start (2), then consumers (3), then the
/// schedule-end epilogue (4). The kernel fires each lifecycle virtual at the
/// band boundary so a meta work unit's gate is open exactly at its point.
pub const RANK_PLAN_STAGE: USize = USize(0); // lint:allow(no-bare-numeric) reason: lifecycle rank ordinal; tracked: #121
/// Schedule-ready band: after plan-stage work units complete.
pub const RANK_SCHEDULE_READY: USize = USize(1); // lint:allow(no-bare-numeric) reason: lifecycle rank ordinal; tracked: #121
/// Pass-start band: at the top of each pass, before consumer work.
pub const RANK_PASS_START: USize = USize(2); // lint:allow(no-bare-numeric) reason: lifecycle rank ordinal; tracked: #121
/// Consumer band: ordinary `Always` / `On<V>` work units.
pub const RANK_CONSUMER: USize = USize(3); // lint:allow(no-bare-numeric) reason: lifecycle rank ordinal; tracked: #121
/// Schedule-end epilogue band: after all consumer work.
pub const RANK_SCHEDULE_END: USize = USize(4); // lint:allow(no-bare-numeric) reason: lifecycle rank ordinal; tracked: #121

/// Classifies a meta lifecycle marker by its const lifecycle rank.
///
/// Implemented only on the four closed-set lifecycle markers (the engine owns
/// the set). `OnMeta<V>`'s `Lifecycle` impl reads `<V as MetaVirtual>::RANK`;
/// consumer virtuals never implement this, which is correct because only
/// `OnMeta<V>` reads it.
pub trait MetaVirtual {
    /// The lifecycle ordinal that places this marker's band among the phases.
    const RANK: USize;
}

impl MetaVirtual for PlanStage {
    const RANK: USize = RANK_PLAN_STAGE;
}
impl MetaVirtual for ScheduleReady {
    const RANK: USize = RANK_SCHEDULE_READY;
}
impl MetaVirtual for PassStart {
    const RANK: USize = RANK_PASS_START;
}
impl MetaVirtual for ScheduleEnd {
    const RANK: USize = RANK_SCHEDULE_END;
}

mod sealed {
    pub trait Sealed {}
}

/// Sealed marker on the four meta resource types.
///
/// Restricts the meta resources to meta work units. The compile-time `Context`
/// bound that enforces "consumer work units cannot reach the meta resources" is
/// a follow-up (slice 3); this round lands the marker and the resource types.
pub trait MetaAccess: sealed::Sealed {}

/// Meta resource: the dependency graph the scheduler analyses.
pub struct Dag;
/// Meta resource: the fiber / trunk / phase assignments.
pub struct ExecutionPlan;
/// Meta resource: the per-core lane assignments.
pub struct LaneAssignment;
/// Meta resource: scheduler self-observation state (domain 22).
///
/// Engine-owned mutable meta state, NOT a consumer `Resource` (consumer
/// resources are `Copy` and read-only after init, so they cannot carry mutable
/// per-pass state). It lives in the engine's `MetaBlock`, written directly by
/// the scheduler, and is read by an `OnMeta` work unit through the
/// `MetaAccess`-gated Ctx accessor. `pass_count` advances once per pass;
/// `ema_pass_duration_ns` folds at frame end (the domain-22 frame time
/// prediction surface); the remaining canonical fields (active units) land
/// with their data sources.
pub struct SchedulerMetrics {
    /// Passes the scheduler has run, advanced once per pass before dispatch.
    pub pass_count: Cell<USize>,
    /// Exponential moving average of the frame duration in nanoseconds,
    /// weight 1/8. The seed frame stores its raw duration; the engine folds
    /// at frame end (workers parked), so a hook reading it during frame N
    /// observes the average as of frame N-1.
    pub ema_pass_duration_ns: Cell<Nanos>,
}

impl Default for SchedulerMetrics {
    #[inline]
    fn default() -> Self {
        Self {
            pass_count: Cell::new(USize(0)), // lint:allow(no-bare-numeric) reason: zero pass count; tracked: #121
            ema_pass_duration_ns: Cell::new(Nanos::from_raw(0)), // lint:allow(no-bare-numeric) reason: zero-duration seed; tracked: #121
        }
    }
}

impl sealed::Sealed for Dag {}
impl sealed::Sealed for ExecutionPlan {}
impl sealed::Sealed for LaneAssignment {}
impl sealed::Sealed for SchedulerMetrics {}

impl MetaAccess for Dag {}
impl MetaAccess for ExecutionPlan {}
impl MetaAccess for LaneAssignment {}
impl MetaAccess for SchedulerMetrics {}
