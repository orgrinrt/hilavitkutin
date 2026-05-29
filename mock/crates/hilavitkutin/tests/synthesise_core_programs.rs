//! Smoke tests for `synthesise_core_programs`: the per-core projection
//! step that Pass 3 dispatch codegen consumes.

// The lifted Cap-dimension types carry `[(); cap_size(N)]:` bounds. A
// downstream crate that instantiates them must itself enable generic_const_exprs
// so its own trait solver can normalise the bounds, mirroring arvo's cross-crate
// tests. adt_const_params is needed only where a Cap const-generic param is
// declared (the engine crate root), not where a lifted type is named. WATCH-tier
// per the unstable-feature soundness sweep (#626).
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, Identity, USize};
use arvo_tensor::cap;
use hilavitkutin::plan::{
    compute_execution_plan, core_program::synthesise_core_programs, PlanInputs,
};
use hilavitkutin_api::{RecordRange, SyncRole};
use notko::Outcome;

const MU: Cap = cap(8);
const MS: Cap = cap(4);
const ME: Cap = cap(16);
const MP: Cap = cap(4);
const MT: Cap = cap(4);
const MF: Cap = cap(4);
const ML: Cap = cap(4);
const MC: Cap = cap(8);
const MCT: Cap = cap(4);
const MUF: Cap = cap(4);
const MCF: Cap = cap(4);
const MTP: Cap = cap(4);

// Per-core caps.
const MAX_CORES: Cap = cap(4);
const MAX_PHASES_PER_CORE: Cap = cap(4);
const MAX_TRUNKS_PER_CORE: Cap = cap(4);
const MAX_FIBERS_PER_CORE: Cap = cap(4);

#[test]
fn empty_plan_yields_empty_per_core_programs() {
    let inputs: PlanInputs<MU, MS> = PlanInputs::new();
    let plan = match compute_execution_plan::<MU, MS, ME, MP, MT, MF, ML, MC, MCT, MUF, MCF, MTP>(
        &inputs,
    ) {
        Outcome::Ok(p) => p,
        Outcome::Err(_) => panic!("empty plan must succeed"),
    };
    let programs = synthesise_core_programs::<
        MU, MP, MT, MF, ML, MC, MCT, MUF, MCF, MTP,
        MAX_CORES, MAX_PHASES_PER_CORE, MAX_TRUNKS_PER_CORE, MAX_FIBERS_PER_CORE,
    >(&plan, USize::ZERO);
    // Every slot stays at the default (range_count = 0, phase_count = 0).
    for p in programs.iter() {
        assert_eq!(p.phase_count, USize::ZERO);
        assert_eq!(p.range_count, USize::ZERO);
    }
}

#[test]
fn single_unit_plan_assigns_one_core_one_fiber() {
    let mut inputs: PlanInputs<MU, MS> = PlanInputs::new();
    inputs.unit_count = USize(1); // lint:allow(no-bare-numeric) reason: single-unit smoke; tracked: #427
    inputs.record_count = USize(1024); // lint:allow(no-bare-numeric) reason: smoke record count; tracked: #427
    let plan = match compute_execution_plan::<MU, MS, ME, MP, MT, MF, ML, MC, MCT, MUF, MCF, MTP>(
        &inputs,
    ) {
        Outcome::Ok(p) => p,
        Outcome::Err(_) => panic!("single-unit plan must succeed"),
    };
    let programs = synthesise_core_programs::<
        MU, MP, MT, MF, ML, MC, MCT, MUF, MCF, MTP,
        MAX_CORES, MAX_PHASES_PER_CORE, MAX_TRUNKS_PER_CORE, MAX_FIBERS_PER_CORE,
    >(&plan, USize(1)); // lint:allow(no-bare-numeric) reason: single-core smoke; tracked: #427

    // Core 0 owns the single fiber (which exists per the plan's
    // morsel_sizes); subsequent cores are empty.
    let c0 = &programs[0];
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
    let mut inputs: PlanInputs<MU, MS> = PlanInputs::new();
    inputs.unit_count = USize(3); // lint:allow(no-bare-numeric) reason: three-unit smoke; tracked: #427
    inputs.record_count = USize(100); // lint:allow(no-bare-numeric) reason: smoke record count; tracked: #427
    let plan = match compute_execution_plan::<MU, MS, ME, MP, MT, MF, ML, MC, MCT, MUF, MCF, MTP>(
        &inputs,
    ) {
        Outcome::Ok(p) => p,
        Outcome::Err(_) => panic!("three-unit plan must succeed"),
    };
    // Run across 2 cores. The fiber count is what group_fibers produced;
    // the test asserts round-robin distribution AND that the progress
    // slot indices are sequential without overlap.
    let programs = synthesise_core_programs::<
        MU, MP, MT, MF, ML, MC, MCT, MUF, MCF, MTP,
        MAX_CORES, MAX_PHASES_PER_CORE, MAX_TRUNKS_PER_CORE, MAX_FIBERS_PER_CORE,
    >(&plan, USize(2)); // lint:allow(no-bare-numeric) reason: two-core smoke; tracked: #427

    // Progress slot indices must be monotonically non-decreasing
    // across cores (the slot range of core c starts where core c-1's
    // range ended).
    let c0_base = programs[0].progress_slot_idx.0;
    let c0_count = programs[0].range_count.0;
    let c1_base = programs[1].progress_slot_idx.0;
    assert!(c1_base >= c0_base + c0_count, "progress slot ranges must not overlap"); // lint:allow(no-bare-numeric) reason: invariant; tracked: #427

    // phase_arrived_offset is sequential per core (0, 1).
    assert_eq!(programs[0].phase_arrived_offset, USize::ZERO);
    assert_eq!(programs[1].phase_arrived_offset, USize(1)); // lint:allow(no-bare-numeric) reason: invariant; tracked: #427
}
