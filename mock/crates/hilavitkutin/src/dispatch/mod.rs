//! Dispatch codegen (domain 17).
//!
//! Turns the plan-stage `ExecutionPlan` (5a2 output) into
//! executable code: per-fiber monomorphised dispatch functions,
//! per-core compiled pipelines, progress counters.
//!
//! `select_approach` picks the dispatch shape from record and
//! fiber counts. `codegen_fiber` and `codegen_core` return empty
//! records so the engine runs end to end; the LLVM and ExpandedLto
//! wiring that fills them is a BACKLOG item.

pub mod approach;
pub mod core_dispatch;
pub mod core_mask;
pub mod engine_ctx;
pub mod fiber_dispatch;
pub mod fiber_run;
pub mod fusion;
pub mod morsel;
pub mod order;
pub mod phase_run;
pub mod progress;
pub mod standard;
pub mod sync;
pub mod trunk_dispatch;
pub mod trunk_gate;
pub mod trunk_run;
pub mod wu_fn;

pub use approach::DispatchApproach;
use arvo::USize;
use arvo_tensor::Capacity;
pub use core_dispatch::CoreDispatch;
pub use engine_ctx::{
    AccumBundleOf,
    ColBundleOf,
    CtxFor,
    EngineCtx,
    ResourceBundleOf,
    VirtBundleOf,
};
pub use fiber_dispatch::FiberDispatch;
pub use fiber_run::RunFiber;
pub use hilavitkutin_api::dispatch_codegen::StandardCodegen;
// The value-carrying WorkUnit, fiber, trunk, and phase lists live in the api
// crate so the builder can construct them; re-export them here next to the
// dispatch machinery that consumes them.
pub use hilavitkutin_api::work_unit_values::{
    FiberCons,
    FiberNil,
    PhaseCons,
    PhaseNil,
    TrunkCons,
    TrunkNil,
    WuCons,
    WuNil,
};
pub use morsel::MorselRange;
pub use phase_run::{RunPhase, RunPipeline};
pub use progress::ProgressCounter;
pub use sync::SyncPoint;
pub use trunk_dispatch::RunTrunkDispatch;
pub use trunk_gate::RunGatedTrunk;
pub use trunk_run::RunTrunk;
pub use wu_fn::WuFn;

/// Record count at or above which `select_approach` picks
/// `ScheduleMega`. Matches the `<10K target` named in the
/// `DispatchApproach` doc comments. Benchmark-tuned refinement
/// lands per the `select_approach benchmark-tuned thresholds`
/// follow-up entry in `BACKLOG.md.tmpl`.
const SCHEDULE_MEGA_THRESHOLD: USize = USize(10_000); // lint:allow(no-bare-numeric) reason: declaration site for the typed threshold; tracked: #72

/// Fiber-count tiebreaker between `TrunkMega` and `IndirectPerFiber`
/// in the small-record-count path. `fiber_count <= SINGLE_FIBER_CUTOVER`
/// picks `TrunkMega`; greater picks `IndirectPerFiber`. Benchmark-
/// tuned refinement lands per the same follow-up entry.
const SINGLE_FIBER_CUTOVER: USize = USize(1); // lint:allow(no-bare-numeric) reason: declaration site for the typed tiebreaker; tracked: #72

/// Pick the dispatch approach for a given record count + fiber
/// count.
///
/// Three-branch heuristic: large record counts pick `ScheduleMega`
/// for LLVM's whole-pipeline optimisation window; small record
/// counts with one fiber pick `TrunkMega`; small record counts
/// with many fibers pick `IndirectPerFiber`. The two cutovers live
/// as typed `USize` constants (`SCHEDULE_MEGA_THRESHOLD`,
/// `SINGLE_FIBER_CUTOVER`); the function body compares
/// `USize`-to-`USize` directly via `PartialOrd`, so the bare-primitive
/// literal lives at one declaration site instead of every call
/// site. Benchmark-tuned thresholds (both the size threshold AND
/// the fiber-count cutover) land in the
/// `select_approach benchmark-tuned thresholds` follow-up entry in
/// `BACKLOG.md.tmpl`.
pub fn select_approach(record_count: USize, fiber_count: USize) -> DispatchApproach {
    if record_count >= SCHEDULE_MEGA_THRESHOLD {
        DispatchApproach::ScheduleMega
    } else if fiber_count <= SINGLE_FIBER_CUTOVER {
        DispatchApproach::TrunkMega
    } else {
        DispatchApproach::IndirectPerFiber
    }
}

/// Emit the monomorphised per-fiber dispatch function.
///
/// Skeleton-return stub: delegates to `FiberDispatch::new()`,
/// which builds the empty record with `body: Maybe::Isnt` and
/// zero-init metadata. The full LLVM-driven monomorphisation
/// lands per `codegen_fiber + codegen_core LLVM-driven
/// monomorphisation` in `BACKLOG.md.tmpl`; until then this stub
/// allows the engine call chain to compile and execute (returning
/// a typed-correct, body-empty record) without panic.
pub fn codegen_fiber<Ctx: 'static, C: Capacity>() -> FiberDispatch<Ctx, C> {
    // FIXME: returns an empty record; the monomorphised body needs the LLVM-driven codegen in BACKLOG.
    FiberDispatch::new()
}

/// Emit the per-core compiled pipeline.
///
/// Skeleton-return stub: delegates to `CoreDispatch::new()`, which
/// builds an array of `FiberDispatch::new()` records plus zero-init
/// phases and morsel boundaries. The full per-core compilation
/// (fusing the morsel loop + arena progress + S3 fence + micro-morsel
/// sync per Topic 6 axis E) lands per the same BACKLOG entry as
/// `codegen_fiber`.
pub fn codegen_core<Ctx: 'static, C: Capacity>() -> CoreDispatch<Ctx, C> {
    // FIXME: returns an empty record; the fused per-core pipeline needs the same BACKLOG codegen as `codegen_fiber`.
    CoreDispatch::new()
}

#[cfg(test)]
mod select_approach_tests {
    use super::*;

    #[test]
    fn large_record_count_picks_schedule_mega() {
        let result = select_approach(USize(50_000), USize(8)); // lint:allow(no-bare-numeric) reason: test fixture; tracked: #72
        assert_eq!(result, DispatchApproach::ScheduleMega);
    }

    #[test]
    fn small_records_single_fiber_picks_trunk_mega() {
        let result = select_approach(USize(1_000), USize(1)); // lint:allow(no-bare-numeric) reason: test fixture; tracked: #72
        assert_eq!(result, DispatchApproach::TrunkMega);
    }

    #[test]
    fn small_records_many_fibers_picks_indirect_per_fiber() {
        let result = select_approach(USize(1_000), USize(8)); // lint:allow(no-bare-numeric) reason: test fixture; tracked: #72
        assert_eq!(result, DispatchApproach::IndirectPerFiber);
    }

    #[test]
    fn threshold_boundary_picks_schedule_mega() {
        // SCHEDULE_MEGA_THRESHOLD is inclusive for ScheduleMega per
        // the `>=` semantics.
        let result = select_approach(SCHEDULE_MEGA_THRESHOLD, USize(8)); // lint:allow(no-bare-numeric) reason: many-fiber fixture; tracked: #72
        assert_eq!(result, DispatchApproach::ScheduleMega);
    }

    #[test]
    fn threshold_boundary_below_picks_single_fiber_path() {
        // One below SCHEDULE_MEGA_THRESHOLD with single-fiber falls
        // through to TrunkMega; pins the boundary from the small-
        // record-count side.
        let below = USize(SCHEDULE_MEGA_THRESHOLD.0 - 1); // lint:allow(no-bare-numeric) reason: one below the threshold; tracked: #72
        let one = USize(1); // lint:allow(no-bare-numeric) reason: single-fiber fixture; tracked: #72
        let result = select_approach(below, one);
        assert_eq!(result, DispatchApproach::TrunkMega);
    }

    #[test]
    fn threshold_boundary_below_many_fibers_picks_indirect_per_fiber() {
        // The same boundary from the many-fiber side: below the threshold the
        // record count stops deciding, and the fiber count picks.
        let below = USize(SCHEDULE_MEGA_THRESHOLD.0 - 1); // lint:allow(no-bare-numeric) reason: one below the threshold; tracked: #72
        let many = USize(8); // lint:allow(no-bare-numeric) reason: many-fiber fixture; tracked: #72
        let result = select_approach(below, many);
        assert_eq!(result, DispatchApproach::IndirectPerFiber);
    }

    #[test]
    fn large_record_count_single_fiber_picks_schedule_mega() {
        // The record count is checked first: at the threshold one fiber does
        // not pull the pick down to TrunkMega.
        let one = USize(1); // lint:allow(no-bare-numeric) reason: single-fiber fixture; tracked: #72
        let result = select_approach(SCHEDULE_MEGA_THRESHOLD, one);
        assert_eq!(result, DispatchApproach::ScheduleMega);
    }

    #[test]
    fn fiber_cutover_boundary_picks_indirect_per_fiber() {
        // One fiber past SINGLE_FIBER_CUTOVER is the first count that leaves
        // TrunkMega.
        let past = USize(SINGLE_FIBER_CUTOVER.0 + 1); // lint:allow(no-bare-numeric) reason: one past the cutover; tracked: #72
        let records = USize(1_000); // lint:allow(no-bare-numeric) reason: small-record fixture; tracked: #72
        let result = select_approach(records, past);
        assert_eq!(result, DispatchApproach::IndirectPerFiber);
    }

    #[test]
    fn zero_fiber_count_picks_trunk_mega() {
        // Zero fibers falls through `<= SINGLE_FIBER_CUTOVER` to
        // TrunkMega.
        let result = select_approach(USize(1_000), USize(0)); // lint:allow(no-bare-numeric) reason: zero-fiber fixture; tracked: #72
        assert_eq!(result, DispatchApproach::TrunkMega);
    }
}
