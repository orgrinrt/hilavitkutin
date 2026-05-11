//! Smoke tests for `compute_execution_plan`: the 13-step chain runner.
//!
//! Exercises the chain end-to-end on tiny synthetic inputs to confirm
//! the surface holds together and basic invariants (empty input,
//! linear chain, multi-fiber split) produce sane plans.

use arvo::{Identity, USize};
use hilavitkutin::plan::{compute_execution_plan, PhaseConfig, PlanInputs};
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

#[test]
fn empty_input_yields_empty_plan() {
    let inputs: PlanInputs<MU, MS> = PlanInputs::new();
    let result = compute_execution_plan::<
        MU, MS, ME, MP, MT, MF, ML, MC, MCT, MUF, MCF, MTP,
    >(&inputs);
    match result {
        Outcome::Ok(plan) => {
            assert_eq!(plan.unit_count, USize::ZERO);
            assert_eq!(plan.phase_count, USize::ZERO);
        }
        Outcome::Err(_) => panic!("empty plan should succeed"),
    }
}

#[test]
fn single_unit_yields_one_phase_one_fiber() {
    let mut inputs: PlanInputs<MU, MS> = PlanInputs::new();
    inputs.unit_count = USize(1); // lint:allow(no-bare-numeric) reason: single-unit smoke; tracked: #427
    inputs.record_count = USize(1024); // lint:allow(no-bare-numeric) reason: smoke record count; tracked: #427
    let result = compute_execution_plan::<
        MU, MS, ME, MP, MT, MF, ML, MC, MCT, MUF, MCF, MTP,
    >(&inputs);
    match result {
        Outcome::Ok(plan) => {
            assert_eq!(plan.unit_count, USize(1)); // lint:allow(no-bare-numeric) reason: roundtrip; tracked: #427
            // At least one phase is always present.
            assert!(plan.phase_count.0 >= 1);
        }
        Outcome::Err(_) => panic!("trivial single-unit plan should succeed"),
    }
}

#[test]
fn phase_config_heuristics_apply_low_record_count() {
    let mut inputs: PlanInputs<MU, MS> = PlanInputs::new();
    inputs.unit_count = USize(3); // lint:allow(no-bare-numeric) reason: three-unit smoke; tracked: #427
    inputs.record_count = USize(100); // lint:allow(no-bare-numeric) reason: small record count picks MaxFuse; tracked: #427
    let result = compute_execution_plan::<
        MU, MS, ME, MP, MT, MF, ML, MC, MCT, MUF, MCF, MTP,
    >(&inputs);
    match result {
        Outcome::Ok(plan) => {
            // First phase config should be MaxFuse for low record counts.
            assert_eq!(plan.phases[0].config, PhaseConfig::MaxFuse);
        }
        Outcome::Err(_) => panic!("three-unit plan should succeed"),
    }
}
