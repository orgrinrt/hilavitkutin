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

pub use assignment::{CoreAssignment, NO_TRUNK};
pub use class::CoreClass;
pub use convergence::Convergence;
pub use handle::ThreadHandle;
pub use pool::ThreadPool;
pub use wake::WakeStrategy;

/// Map plan trunks onto concrete cores.
///
/// Skeleton: `todo!()`. Real body walks the plan's trunk set,
/// groups trunks per phase, and pins them to P-cores per
/// `CoreClass`. Lands in Session 2B (HILA-RUNTIME-C4) of the
/// runtime megaround.
pub fn assign_cores<
    const MAX_UNITS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_PHASES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_TRUNKS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_FIBERS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_LANES: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COLUMNS: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COMPONENTS_PER_TRUNK: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_UNITS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_COLUMNS_PER_FIBER: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    const MAX_TRUNKS_PER_PHASE: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
>(
    core_count: USize,
    plan: &crate::plan::ExecutionPlan<
        MAX_UNITS,
        MAX_PHASES,
        MAX_TRUNKS,
        MAX_FIBERS,
        MAX_LANES,
        MAX_COLUMNS,
        MAX_COMPONENTS_PER_TRUNK,
        MAX_UNITS_PER_FIBER,
        MAX_COLUMNS_PER_FIBER,
        MAX_TRUNKS_PER_PHASE,
    >,
) -> CoreAssignment<MAX_LANES> {
    let _ = (core_count, plan);
    todo!("session 2B (HILA-RUNTIME-C4): map plan trunks onto concrete cores")
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
/// `T: Executor` once the trait ships in a follow-up round;
/// see BACKLOG.
pub fn steal_fallback<T>(executor: &T, fiber_id: crate::plan::FiberId) {
    let _ = (executor, fiber_id);
    todo!("5a4: work-stealing fallback against an Executor override")
}
