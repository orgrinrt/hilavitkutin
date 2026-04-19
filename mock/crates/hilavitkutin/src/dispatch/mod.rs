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
//! land as follow-ups — see BACKLOG → Engine 5a3 follow-ups.

pub mod approach;
pub mod core_dispatch;
pub mod fiber_dispatch;
pub mod morsel;
pub mod progress;
pub mod sync;
pub mod wu_fn;

pub use approach::DispatchApproach;
pub use core_dispatch::CoreDispatch;
pub use fiber_dispatch::FiberDispatch;
pub use morsel::MorselRange;
pub use progress::ProgressCounter;
pub use sync::SyncPoint;
pub use wu_fn::WuFn;

/// Pick the dispatch approach for a given record count + fiber
/// count.
///
/// Skeleton: `todo!()`. Real thresholds (10K cutover target) land
/// with benchmarks — see BACKLOG.
pub fn select_approach(record_count: u64, fiber_count: u16) -> DispatchApproach {
    let _ = (record_count, fiber_count);
    todo!("5a3: approach selection (record-count thresholds)")
}

/// Emit the monomorphised per-fiber dispatch function.
///
/// Skeleton: `todo!()`. Needs LLVM hooks or a build-time plugin
/// from hilavitkutin-build — see BACKLOG.
pub fn codegen_fiber<Ctx: 'static, const MAX_CORES: usize>() -> FiberDispatch<Ctx, MAX_CORES> {
    todo!("5a3: emit monomorphised per-fiber dispatch function")
}

/// Emit the per-core compiled pipeline.
///
/// Skeleton: `todo!()`. Encodes phases + morsel boundaries + sync
/// points + per-fiber dispatch records. Depends on `codegen_fiber`.
pub fn codegen_core<Ctx: 'static, const MAX_FIBERS: usize>() -> CoreDispatch<Ctx, MAX_FIBERS> {
    todo!("5a3: emit per-core compiled pipeline")
}
