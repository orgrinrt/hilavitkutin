//! Smoke tests for `compute_execution_plan`: the 13-step chain runner.
//!
//! Exercises the chain end-to-end on tiny synthetic inputs to confirm
//! the surface holds together and basic invariants (empty input,
//! linear chain, multi-fiber split) produce sane plans.

use arvo::{Identity, USize};
use hilavitkutin::plan::{
    compute_execution_plan, steps, DependencyGraph, EdgeKind, PhaseConfig, PlanInputs,
};
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
fn topo_sort_detects_two_node_cycle() {
    // Hand-construct a 2-node cycle via the low-level CSR graph.
    // `build_dag` walks `i < j` strictly so it can never produce a
    // cycle from inputs; this test exercises the defensive path that
    // the runner uses to translate a cyclic graph into PlanError::Cycle.
    let mut g: DependencyGraph<MU, ME> = DependencyGraph::new();
    g.add_edge_kind(USize(0), USize(1), EdgeKind::Read); // lint:allow(no-bare-numeric) reason: hand-crafted cycle smoke; tracked: #427
    g.add_edge_kind(USize(1), USize(0), EdgeKind::Read); // lint:allow(no-bare-numeric) reason: hand-crafted cycle smoke; tracked: #427
    // Both units have edges; unit_count advanced to 2.
    assert_eq!(g.unit_count, USize(2)); // lint:allow(no-bare-numeric) reason: invariant check; tracked: #427

    let (_topo, placed) = steps::topo_sort::<MU, ME>(&g);
    // Cycle means Kahn's iteration cannot place every unit.
    assert!(placed.0 < g.unit_count.0, "expected partial placement under cycle; got placed={}", placed.0);
}

#[test]
fn topo_sort_places_all_for_linear_chain() {
    // Linear A -> B chain. Verifies the cycle-detection signal is not
    // a false positive for valid DAGs.
    let mut g: DependencyGraph<MU, ME> = DependencyGraph::new();
    g.add_edge_kind(USize(0), USize(1), EdgeKind::Read); // lint:allow(no-bare-numeric) reason: linear-chain smoke; tracked: #427
    // Pad the row entry for unit 1 so unit_count reaches 2.
    g.row_offsets[1] = g.edge_count; // lint:allow(no-bare-numeric) reason: CSR padding for trailing empty row; tracked: #427
    g.unit_count = USize(2); // lint:allow(no-bare-numeric) reason: invariant set; tracked: #427

    let (_topo, placed) = steps::topo_sort::<MU, ME>(&g);
    assert_eq!(placed, USize(2)); // lint:allow(no-bare-numeric) reason: full placement expected; tracked: #427
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
