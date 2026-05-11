//! Smoke tests for `synthesise_core_programs`: the per-core projection
//! step that Pass 3 dispatch codegen consumes.

use arvo::{Identity, USize};
use hilavitkutin::plan::{
    compute_execution_plan, core_program::synthesise_core_programs, PlanInputs,
};
use hilavitkutin_api::{RecordRange, SyncRole};
use notko::Outcome;

const MU: usize = 8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MS: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const ME: usize = 16; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MP: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MT: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MF: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const ML: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MC: usize = 8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MCT: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MUF: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MCF: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MTP: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121

// Per-core caps.
const MAX_CORES: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MAX_PHASES_PER_CORE: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MAX_TRUNKS_PER_CORE: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
const MAX_FIBERS_PER_CORE: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121

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
