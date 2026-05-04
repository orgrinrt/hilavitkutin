//! Threading primitives (domain 20).
//!
//! Pre-allocated pool, hybrid wake strategy, heterogeneous-core
//! awareness, per-core role assignments, head+tail convergence.
//!
//! This module is the *skeleton* for 5a4: public surface is
//! complete; every coordination function (`assign_cores`,
//! `classify_cores`, `steal_fallback`) stubs to `todo!()`. Real
//! OS thread spawning is gated on a future `threading-std`
//! feature (not introduced this round): see BACKLOG → Engine
//! 5a4 follow-ups.

pub mod assignment;
pub mod class;
pub mod convergence;
pub mod handle;
pub mod pool;
pub mod wake;

use arvo::USize;

pub use assignment::CoreAssignment;
pub use class::CoreClass;
pub use convergence::Convergence;
pub use handle::ThreadHandle;
pub use pool::ThreadPool;
pub use wake::WakeStrategy;

/// Map plan lane assignments onto concrete cores.
///
/// Skeleton: `todo!()`. Real body walks the plan's lane set +
/// groups them into trunks + pins trunks to P-cores: see
/// BACKLOG.
pub fn assign_cores<const MAX_CORES: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    core_count: USize,
    plan: &crate::scheduler::ExecutionPlan<MAX_CORES>,
) -> CoreAssignment<MAX_CORES> {
    let _ = (core_count, plan);
    todo!("5a4: map plan lane assignments onto concrete cores")
}

/// Classify cores by performance/efficiency class.
///
/// Skeleton: `todo!()`. Real body runs heterogeneous-core
/// detection (CPUID leaf 0x1A on x86, sysfs on Linux, IOKit on
/// macOS): see BACKLOG. Returns a fixed-size array of 256
/// classes (documented upper bound); the const-generic
/// generalisation is a follow-up.
pub fn classify_cores(total_cores: USize, p_cores: USize) -> [CoreClass; 256] {
    let _ = (total_cores, p_cores);
    todo!("5a4: heterogeneous-core detection + classification")
}

/// Work-stealing fallback against a consumer-provided Executor.
///
/// Skeleton: `todo!()`. Real signature will constrain
/// `T: Executor` once the trait ships in a follow-up round , 
/// see BACKLOG.
pub fn steal_fallback<T>(executor: &T, fiber_id: crate::plan::FiberId) {
    let _ = (executor, fiber_id);
    todo!("5a4: work-stealing fallback against an Executor override")
}
