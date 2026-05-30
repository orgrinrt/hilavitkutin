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
pub mod engine_ctx;
pub mod fiber_dispatch;
pub mod fiber_walk;
pub mod morsel;
pub mod progress;
pub mod standard;
pub mod sync;
pub mod wu_fn;

use arvo::USize;
use arvo_tensor::Capacity;
pub use hilavitkutin_api::dispatch_codegen::StandardCodegen;

pub use approach::DispatchApproach;
pub use core_dispatch::CoreDispatch;
pub use engine_ctx::EngineCtx;
pub use fiber_dispatch::FiberDispatch;
pub use fiber_walk::{run_fiber_walk, RunFiber, WuCons, WuNil};
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
/// Skeleton-return stub: delegates to `FiberDispatch::new()`,
/// which builds the empty record with `body: Maybe::Isnt` and
/// zero-init metadata. The full LLVM-driven monomorphisation
/// lands per `codegen_fiber + codegen_core LLVM-driven
/// monomorphisation` in `BACKLOG.md.tmpl`; until then this stub
/// allows the engine call chain to compile and execute (returning
/// a typed-correct, body-empty record) without panic.
pub fn codegen_fiber<Ctx: 'static, C: Capacity>() -> FiberDispatch<Ctx, C> {
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
    CoreDispatch::new()
}

#[cfg(test)]
mod codegen_stub_tests {
    use super::*;
    use notko::Maybe;
    use crate::plan::FiberId;
    use arvo::strategy::Identity;
    use arvo_tensor::Dim;

    // A fixed capacity of four for the codegen stub records.
    type C4 = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity literal; Dim<N> array-length root; tracked: #649

    #[test]
    fn codegen_fiber_returns_empty_skeleton() {
        use crate::plan::PhaseId;
        let result: FiberDispatch<(), C4> = codegen_fiber::<(), C4>();
        assert!(matches!(result.body, Maybe::Isnt));
        assert_eq!(result.fiber_id, FiberId::ZERO);
        assert_eq!(result.phase, PhaseId::ZERO);
        assert_eq!(result.morsel_range.start, USize::ZERO);
        assert_eq!(result.morsel_range.len, USize::ZERO);
        assert_eq!(result.sync_point_count, USize::ZERO);
    }

    #[test]
    fn codegen_core_returns_empty_skeleton() {
        let result: CoreDispatch<(), C4> = codegen_core::<(), C4>();
        assert_eq!(result.fiber_count, USize::ZERO);
        assert_eq!(result.phase_count, USize::ZERO);
        assert_eq!(result.boundary_count, USize::ZERO);
        assert_eq!(result.sync_point_count, USize::ZERO);
        // Element check: every fiber slot is itself an empty skeleton.
        assert!(matches!(result.fibers.as_ref()[0].body, Maybe::Isnt)); // lint:allow(no-bare-numeric) reason: element-zero check; tracked: #72
        assert_eq!(result.fibers.as_ref()[0].sync_point_count, USize::ZERO); // lint:allow(no-bare-numeric) reason: element-zero check; tracked: #72
    }
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
