//! Dispatch codegen (domain 17).
//!
//! Turns the plan-stage `ExecutionPlan` (5a2 output) into
//! executable code: per-fiber monomorphised dispatch functions,
//! per-core compiled pipelines, progress counters.
//!
//! This module is the *skeleton* for 5a3: public surface is
//! complete; every code-emit function (`select_approach`,
//! `codegen_fiber`, `codegen_core`) stubs to `todo!()`. The
//! real LLVM / ExpandedLto wiring + rust-pipe emission pattern
//! land as follow-ups: see BACKLOG → Engine 5a3 follow-ups.

pub mod approach;
pub mod core_dispatch;
pub mod fiber_dispatch;
pub mod morsel;
pub mod progress;
pub mod standard;
pub mod sync;
pub mod wu_fn;

use arvo::USize;
pub use hilavitkutin_api::dispatch_codegen::StandardCodegen;

pub use approach::DispatchApproach;
pub use core_dispatch::CoreDispatch;
pub use fiber_dispatch::FiberDispatch;
pub use morsel::MorselRange;
pub use progress::ProgressCounter;
pub use sync::SyncPoint;
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
/// Skeleton: `todo!()`. Needs LLVM hooks or a build-time plugin
/// from hilavitkutin-build: see BACKLOG.
pub fn codegen_fiber<Ctx: 'static, const MAX_CORES: usize>() -> FiberDispatch<Ctx, MAX_CORES> {
    todo!("5a3: emit monomorphised per-fiber dispatch function")
}

/// Emit the per-core compiled pipeline.
///
/// Skeleton: `todo!()`. Encodes phases + morsel boundaries + sync
/// points + per-fiber dispatch records. Depends on `codegen_fiber`.
pub fn codegen_core<Ctx: 'static, const MAX_FIBERS: usize>() -> CoreDispatch<Ctx, MAX_FIBERS> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    todo!("5a3: emit per-core compiled pipeline")
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
        let result = select_approach(USize(9_999), USize(1)); // lint:allow(no-bare-numeric) reason: boundary fixture; tracked: #72
        assert_eq!(result, DispatchApproach::TrunkMega);
    }

    #[test]
    fn zero_fiber_count_picks_trunk_mega() {
        // Zero fibers falls through `<= SINGLE_FIBER_CUTOVER` to
        // TrunkMega.
        let result = select_approach(USize(1_000), USize(0)); // lint:allow(no-bare-numeric) reason: zero-fiber fixture; tracked: #72
        assert_eq!(result, DispatchApproach::TrunkMega);
    }
}
