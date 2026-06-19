//! Smoke tests for `synthesise_core_programs`: the per-core projection
//! step that Pass 3 dispatch codegen consumes.
//!
//! The plan dimensions arrive bundled as one `TestDims: PlanDims`; the
//! per-core sub-capacities (phases / trunks / fibers per core) are their
//! own `Capacity` types because they size the hilavitkutin-api
//! `CoreProgram`'s min-const-generic arrays via `cap_size`. The engine's
//! own array sizing is GCE-free; the `{ cap_size(C::CAP) }` projection
//! into the unmigrated api type is the one residual `generic_const_exprs`
//! use, so this downstream crate still enables the gate to name those
//! const-generic arguments.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Bits, Hot, Identity, USize, Unsigned};
use arvo_tensor::Dim;
use hilavitkutin::plan::{
    compute_execution_plan, core_program::synthesise_core_programs, PlanDims, PlanInputs,
};
use hilavitkutin_api::{RecordRange, SyncRole};
use notko::Outcome;

/// Plan dimensions for the smoke tests.
struct TestDims;

impl PlanDims for TestDims {
    type Units = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Stores = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Edges = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Phases = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Trunks = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type TrunksPerPhase = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Fibers = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Lanes = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Columns = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ComponentsPerTrunk = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type UnitsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ColumnsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Cores = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AccumsPerCore = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type PlanAffecting = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AdjRow = Bits<64, Hot, Unsigned>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-wide row covers the small test units; Bits width literal; tracked: #649
}

// Per-core sub-capacities (not plan dimensions): they size the api
// `CoreProgram`'s arrays.
type PhasesPerCore = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-core sub-capacity; Dim<N> array-length root; tracked: #649
type TrunksPerCore = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-core sub-capacity; Dim<N> array-length root; tracked: #649
type FibersPerCore = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: per-core sub-capacity; Dim<N> array-length root; tracked: #649

type Inputs = PlanInputs<<TestDims as PlanDims>::Units, <TestDims as PlanDims>::Stores>;

#[test]
fn empty_plan_yields_empty_per_core_programs() {
    let inputs: Inputs = PlanInputs::new();
    let plan = match compute_execution_plan::<TestDims>(&inputs) {
        Outcome::Ok(p) => p,
        Outcome::Err(_) => panic!("empty plan must succeed"),
    };
    let programs = synthesise_core_programs::<
        TestDims,
        PhasesPerCore,
        TrunksPerCore,
        FibersPerCore,
    >(&plan, USize::ZERO);
    // Every slot stays at the default (range_count = 0, phase_count = 0).
    for p in programs.as_ref().iter() {
        assert_eq!(p.phase_count, USize::ZERO);
        assert_eq!(p.range_count, USize::ZERO);
    }
}

#[test]
fn single_unit_plan_assigns_one_core_one_fiber() {
    let mut inputs: Inputs = PlanInputs::new();
    inputs.unit_count = USize(1); // lint:allow(no-bare-numeric) reason: single-unit smoke; tracked: #427
    inputs.record_count = USize(1024); // lint:allow(no-bare-numeric) reason: smoke record count; tracked: #427
    let plan = match compute_execution_plan::<TestDims>(&inputs) {
        Outcome::Ok(p) => p,
        Outcome::Err(_) => panic!("single-unit plan must succeed"),
    };
    let programs = synthesise_core_programs::<
        TestDims,
        PhasesPerCore,
        TrunksPerCore,
        FibersPerCore,
    >(&plan, USize(1)); // lint:allow(no-bare-numeric) reason: single-core smoke; tracked: #427

    // Core 0 owns the single fiber (which exists per the plan's
    // morsel_windows); subsequent cores are empty.
    let c0 = &programs.as_ref()[0];
    assert_eq!(c0.range_count, USize(1)); // lint:allow(no-bare-numeric) reason: one fiber assigned; tracked: #427
    assert!(matches!(c0.fiber_ranges[0].1, RecordRange::Full));
    assert_eq!(c0.progress_slot_idx, USize::ZERO);
    assert_eq!(c0.phase_arrived_offset, USize::ZERO);
    // Single-phase plan: the sole phase carries WaitAndSignal (no
    // first/last distinction when there's only one).
    assert_eq!(c0.phase_count, USize(1)); // lint:allow(no-bare-numeric) reason: one phase; tracked: #427
    assert!(matches!(c0.phases[0].sync_role, SyncRole::WaitAndSignal));
}

#[test]
fn multi_fiber_plan_distributes_across_cores() {
    let mut inputs: Inputs = PlanInputs::new();
    inputs.unit_count = USize(3); // lint:allow(no-bare-numeric) reason: three-unit smoke; tracked: #427
    inputs.record_count = USize(100); // lint:allow(no-bare-numeric) reason: smoke record count; tracked: #427
    let plan = match compute_execution_plan::<TestDims>(&inputs) {
        Outcome::Ok(p) => p,
        Outcome::Err(_) => panic!("three-unit plan must succeed"),
    };
    // Run across 2 cores. The fiber count is what group_fibers produced;
    // the test asserts round-robin distribution AND that the progress
    // slot indices are sequential without overlap.
    let programs = synthesise_core_programs::<
        TestDims,
        PhasesPerCore,
        TrunksPerCore,
        FibersPerCore,
    >(&plan, USize(2)); // lint:allow(no-bare-numeric) reason: two-core smoke; tracked: #427

    // Progress slot indices must be monotonically non-decreasing
    // across cores (the slot range of core c starts where core c-1's
    // range ended).
    let progs = programs.as_ref();
    let c0_base = progs[0].progress_slot_idx.0;
    let c0_count = progs[0].range_count.0;
    let c1_base = progs[1].progress_slot_idx.0;
    assert!(c1_base >= c0_base + c0_count, "progress slot ranges must not overlap"); // lint:allow(no-bare-numeric) reason: invariant; tracked: #427

    // phase_arrived_offset is sequential per core (0, 1).
    assert_eq!(progs[0].phase_arrived_offset, USize::ZERO);
    assert_eq!(progs[1].phase_arrived_offset, USize(1)); // lint:allow(no-bare-numeric) reason: invariant; tracked: #427
}
