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
pub mod barrier;
pub mod class;
pub mod convergence;
pub mod frame;
pub mod handle;
pub mod hybrid;
pub mod parking;
pub mod pool;

use arvo::USize;
use arvo_tensor::{cap_size, Capacity};

use crate::plan::PlanDims;

pub use assignment::{CoreAssignment, NO_TRUNK};
pub use barrier::{phase_barrier_arrive, phase_barrier_observe, phase_barrier_reset, BarrierArrival};
pub use frame::{
    await_exit, frame_await, frame_await_done, frame_done_arrive, frame_exit_arrive, frame_publish,
    request_shutdown,
};
pub use class::{classify_cores, CoreClass, MAX_CORES};
pub use convergence::Convergence;
pub use handle::ThreadHandle;
pub use hybrid::HybridExecutor;
pub use parking::{
    atomic_wait, atomic_wake_all, pick_tier, predicted_wait_ns_load, predicted_wait_ns_store, spin,
    spin_budget_for, ParkTier,
};
pub use hilavitkutin_api::platform::WakeStrategy;
pub use pool::{ThreadPool, ThreadPoolBuilder};

/// Map plan trunks onto concrete cores.
///
/// Base round-robin assignment: each phase reuses the same per-core
/// trunk mapping, so the parallel-dispatch width is bounded by the
/// phase with the most trunks. `width = min(max_trunks_per_phase,
/// core_count, MAX_LANES)`. `trunk_index[i] = USize(i)` for
/// `i in 0..width`; `NO_TRUNK` for the rest. `assigned_count = width`.
///
/// `fiber_assignments` stays `FiberId::ZERO` and
/// `morsel_size_multiplier` stays `USize(100)` (1.0x). Both wait
/// for follow-up slices that populate fiber-to-trunk maps and
/// `CoreClass`-aware weighting. See `CoreClass-aware assign_cores
/// follow-up` in `BACKLOG.md.tmpl` for the heterogeneous-core path.
pub fn assign_cores<D: PlanDims>(
    core_count: USize,
    plan: &crate::plan::ExecutionPlan<D>,
) -> CoreAssignment<D::Lanes> {
    let mut assignment: CoreAssignment<D::Lanes> = CoreAssignment::new();

    // Find the phase with the most trunks; that bounds the
    // parallel-dispatch width.
    let mut max_trunks_needed: usize = 0; // lint:allow(no-bare-numeric) reason: loop accumulator; tracked: #72
    let phase_count = plan.phase_count.0;
    let phases = plan.phases.as_ref();
    let mut p: usize = 0; // lint:allow(no-bare-numeric) reason: loop index; tracked: #72
    while p < phase_count {
        let tc = phases[p].trunk_count.0;
        if tc > max_trunks_needed {
            max_trunks_needed = tc;
        }
        p += 1; // lint:allow(no-bare-numeric) reason: loop increment; tracked: #72
    }

    // Clamp width against core count and the per-pipeline lane capacity.
    let mut width = max_trunks_needed;
    if core_count.0 < width {
        width = core_count.0;
    }
    if cap_size(<D::Lanes as Capacity>::CAP) < width {
        width = cap_size(<D::Lanes as Capacity>::CAP);
    }

    // Populate the round-robin slots; the rest stay NO_TRUNK from
    // CoreAssignment::new().
    let mut i: usize = 0; // lint:allow(no-bare-numeric) reason: loop index; tracked: #72
    while i < width {
        assignment.trunk_index.as_mut()[i] = USize(i);
        i += 1; // lint:allow(no-bare-numeric) reason: loop increment; tracked: #72
    }
    assignment.assigned_count = USize(width);

    assignment
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

#[cfg(test)]
mod assign_cores_tests {
    use super::*;
    use crate::plan::ExecutionPlan;
    use arvo::strategy::Identity;
    use arvo::{Bits, Hot, Unsigned};
    use arvo_tensor::{cap_size, Capacity, Dim};

    // Per-pipeline capacities sized small for the tests. The body's
    // logic does not depend on the capacity values, only on the
    // runtime phase_count / trunk_count fields. The lane capacity
    // (`D::Lanes`) bounds the assignment array.
    struct TestDims;

    impl PlanDims for TestDims {
        type Units = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type Stores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type Edges = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type Phases = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type Trunks = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type TrunksPerPhase = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type Fibers = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type Lanes = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type Columns = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type ComponentsPerTrunk = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type UnitsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type ColumnsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type Cores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity budget literal; Dim<N> array-length root; tracked: #649
        type AdjRow = Bits<64, Hot, Unsigned>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-wide row covers Dim<8> units; Bits width literal; tracked: #649
    }

    const LANES: usize = cap_size(<<TestDims as PlanDims>::Lanes as Capacity>::CAP); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-position cap projection for test loop bounds; tracked: #72

    type TestPlan = ExecutionPlan<TestDims>;

    fn plan_with_phase_trunks(per_phase: &[usize]) -> TestPlan { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test helper takes raw usize counts to mirror runtime field shape; tracked: #72
        let mut plan = TestPlan::new();
        let mut i: usize = 0; // lint:allow(no-bare-numeric) reason: loop index; tracked: #72
        while i < per_phase.len() {
            plan.phases.as_mut()[i].trunk_count = USize(per_phase[i]);
            i += 1; // lint:allow(no-bare-numeric) reason: loop increment; tracked: #72
        }
        plan.phase_count = USize(per_phase.len());
        plan
    }

    #[test]
    fn empty_plan_returns_default_assignment() {
        let plan = TestPlan::new();
        let result = assign_cores(USize(4), &plan); // lint:allow(no-bare-numeric) reason: test core count; tracked: #72
        assert_eq!(result.assigned_count, USize::ZERO);
        let trunk_index = result.trunk_index.as_ref();
        let mut i: usize = 0; // lint:allow(no-bare-numeric) reason: loop index; tracked: #72
        while i < LANES {
            assert_eq!(trunk_index[i], NO_TRUNK);
            i += 1; // lint:allow(no-bare-numeric) reason: loop increment; tracked: #72
        }
    }

    #[test]
    fn single_phase_single_trunk_4_cores() {
        let plan = plan_with_phase_trunks(&[1]);
        let result = assign_cores(USize(4), &plan); // lint:allow(no-bare-numeric) reason: test core count; tracked: #72
        assert_eq!(result.assigned_count, USize(1)); // lint:allow(no-bare-numeric) reason: width=1 expected; tracked: #72
        let trunk_index = result.trunk_index.as_ref();
        assert_eq!(trunk_index[0], USize(0)); // lint:allow(no-bare-numeric) reason: slot 0 trunk 0; tracked: #72
        let mut i: usize = 1; // lint:allow(no-bare-numeric) reason: loop start; tracked: #72
        while i < LANES {
            assert_eq!(trunk_index[i], NO_TRUNK);
            i += 1; // lint:allow(no-bare-numeric) reason: loop increment; tracked: #72
        }
    }

    #[test]
    fn core_count_zero_returns_empty_assignment() {
        // Boundary case: zero cores with a non-empty plan produces
        // an empty assignment via the core_count clamp.
        let plan = plan_with_phase_trunks(&[3]); // lint:allow(no-bare-numeric) reason: fixture trunk count; tracked: #72
        let result = assign_cores(USize(0), &plan); // lint:allow(no-bare-numeric) reason: zero-core fixture; tracked: #72
        assert_eq!(result.assigned_count, USize::ZERO);
        let trunk_index = result.trunk_index.as_ref();
        let mut i: usize = 0; // lint:allow(no-bare-numeric) reason: loop index; tracked: #72
        while i < LANES {
            assert_eq!(trunk_index[i], NO_TRUNK);
            i += 1; // lint:allow(no-bare-numeric) reason: loop increment; tracked: #72
        }
    }

    #[test]
    fn three_phases_max_3_trunks_4_cores() {
        let plan = plan_with_phase_trunks(&[2, 3, 1]); // lint:allow(no-bare-numeric) reason: fixture trunk counts; tracked: #72
        let result = assign_cores(USize(4), &plan); // lint:allow(no-bare-numeric) reason: test core count; tracked: #72
        // max_trunks_needed = 3 (middle phase), bounded by 4 cores
        // and 8 lanes, so width = 3.
        assert_eq!(result.assigned_count, USize(3)); // lint:allow(no-bare-numeric) reason: width=3 expected; tracked: #72
        let trunk_index = result.trunk_index.as_ref();
        assert_eq!(trunk_index[0], USize(0)); // lint:allow(no-bare-numeric) reason: slot 0 trunk 0; tracked: #72
        assert_eq!(trunk_index[1], USize(1)); // lint:allow(no-bare-numeric) reason: slot 1 trunk 1; tracked: #72
        assert_eq!(trunk_index[2], USize(2)); // lint:allow(no-bare-numeric) reason: slot 2 trunk 2; tracked: #72
        let mut i: usize = 3; // lint:allow(no-bare-numeric) reason: loop start; tracked: #72
        while i < LANES {
            assert_eq!(trunk_index[i], NO_TRUNK);
            i += 1; // lint:allow(no-bare-numeric) reason: loop increment; tracked: #72
        }
    }
}
