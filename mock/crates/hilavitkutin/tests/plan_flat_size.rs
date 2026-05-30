//! Regression: the execution plan must stay small enough to live on the
//! stack.
//!
//! The nested const-array shape measured 11 MB at `DefaultPlanDims`,
//! dominated by the dense `Phases x TrunksPerPhase x ComponentsPerTrunk`
//! fiber nesting (32768 reserved fiber slots). That overflowed
//! `compute_execution_plan`'s stack frame and blocked the
//! topological-dispatch slice. The CSR flatten collapses the nesting onto
//! the plan-wide `D::Fibers` pool, so the plan fits on the stack.

use arvo::USize;
use hilavitkutin::plan::{
    compute_execution_plan, DefaultPlanDims, ExecutionPlan, PlanDims, PlanInputs,
};
use notko::Outcome;

/// 256 KiB ceiling. The flattened plan is far under this (flat pools sized
/// by the plan-wide caps); the nested shape was ~11 MB.
const MAX_PLAN_BYTES: usize = 262144; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: size_of byte ceiling, value-position const for the regression bound; tracked: #72

#[test]
fn execution_plan_fits_on_the_stack() {
    let size = core::mem::size_of::<ExecutionPlan<DefaultPlanDims>>(); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: size_of returns usize; regression bound; tracked: #72
    assert!(
        size < MAX_PLAN_BYTES,
        "ExecutionPlan<DefaultPlanDims> is {size} bytes, expected < {MAX_PLAN_BYTES}; the flat CSR pools should keep it small",
    );
}

/// The runner builds a plan at the default dimensions without overflowing
/// the stack (the nested plan's 11 MB frame did) and the flat pools
/// recover the unit assignment end to end.
#[test]
fn compute_execution_plan_runs_at_default_dims() {
    let mut inputs: PlanInputs<
        <DefaultPlanDims as PlanDims>::Units,
        <DefaultPlanDims as PlanDims>::Stores,
    > = PlanInputs::new();
    inputs.unit_count = USize(3); // lint:allow(no-bare-numeric) reason: small roundtrip; tracked: #427
    inputs.record_count = USize(1024); // lint:allow(no-bare-numeric) reason: roundtrip record count; tracked: #427
    match compute_execution_plan::<DefaultPlanDims>(&inputs) {
        Outcome::Ok(plan) => {
            assert_eq!(plan.unit_count, USize(3), "unit count roundtrips"); // lint:allow(no-bare-numeric) reason: roundtrip; tracked: #427
            assert!(plan.phase_count.0 >= 1, "at least one phase");
            assert!(plan.fiber_count.0 >= 1, "the flat fibers pool is populated");
            assert!(plan.trunk_count.0 >= 1, "the flat trunks pool is populated");
        }
        Outcome::Err(_) => panic!("three-unit plan at default dims should succeed"),
    }
}
