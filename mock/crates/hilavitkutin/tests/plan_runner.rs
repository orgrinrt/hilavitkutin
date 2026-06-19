//! Smoke tests for `compute_execution_plan`: the 13-step chain runner.
//!
//! Exercises the chain end-to-end on tiny synthetic inputs to confirm
//! the surface holds together and basic invariants (empty input,
//! linear chain, multi-fiber split) produce sane plans.
//!
//! The plan dimensions arrive bundled as one `TestDims: PlanDims`, sized
//! by `Capacity` TYPES, so no `generic_const_exprs` gate is needed: the
//! dimensions are types, not `Cap` const generics.

use arvo::{Bits, Hot, Identity, USize, Unsigned};
use arvo_tensor::Dim;
use hilavitkutin::plan::{
    compute_execution_plan, steps, AccessMask, DependencyGraph, EdgeKind, PhaseConfig, PlanDims,
    PlanError, PlanInputs,
};
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
    type Cores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AccumsPerCore = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type PlanAffecting = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AdjRow = Bits<64, Hot, Unsigned>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-wide row covers the small test units; Bits width literal; tracked: #649
}

type Inputs = PlanInputs<<TestDims as PlanDims>::Units, <TestDims as PlanDims>::Stores>;

#[test]
fn empty_input_yields_empty_plan() {
    let inputs: Inputs = PlanInputs::new();
    let result = compute_execution_plan::<TestDims>(&inputs);
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
    let mut inputs: Inputs = PlanInputs::new();
    inputs.unit_count = USize(1); // lint:allow(no-bare-numeric) reason: single-unit smoke; tracked: #427
    inputs.record_count = USize(1024); // lint:allow(no-bare-numeric) reason: smoke record count; tracked: #427
    let result = compute_execution_plan::<TestDims>(&inputs);
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
    // Hand-construct a 2-node cycle via the low-level CSR graph. This
    // exercises the `topo_sort` cycle path directly, independent of how
    // the graph was produced. The end-to-end `build_dag`-driven cycle
    // path is covered by `build_dag_detects_mutual_dependency_cycle`.
    let mut g: DependencyGraph<TestDims> = DependencyGraph::new();
    g.add_edge_kind(USize(0), USize(1), EdgeKind::Read); // lint:allow(no-bare-numeric) reason: hand-crafted cycle smoke; tracked: #427
    g.add_edge_kind(USize(1), USize(0), EdgeKind::Read); // lint:allow(no-bare-numeric) reason: hand-crafted cycle smoke; tracked: #427
    // Both units have edges; unit_count advanced to 2.
    assert_eq!(g.unit_count, USize(2)); // lint:allow(no-bare-numeric) reason: invariant check; tracked: #427

    let (_topo, placed) = steps::topo_sort::<TestDims>(&g);
    // Cycle means Kahn's iteration cannot place every unit.
    assert!(placed.0 < g.unit_count.0, "expected partial placement under cycle; got placed={}", placed.0);
}

#[test]
fn size_morsels_distributes_remainder_across_first_fibers() {
    // 10 records across 3 fibers => [4, 3, 3]. Verifies the sum
    // invariant: every record is assigned somewhere. The prior
    // integer-divide-only shape returned [3, 3, 3] and silently
    // dropped record index 9.
    let sizes = steps::compute_fiber_morsel_windows::<TestDims>(USize(10), USize(3)); // lint:allow(no-bare-numeric) reason: smoke fixture; tracked: #427
    let sizes = sizes.as_ref();
    assert_eq!(sizes[0], USize(4)); // lint:allow(no-bare-numeric) reason: expected first fiber; tracked: #427
    assert_eq!(sizes[1], USize(3)); // lint:allow(no-bare-numeric) reason: expected second fiber; tracked: #427
    assert_eq!(sizes[2], USize(3)); // lint:allow(no-bare-numeric) reason: expected third fiber; tracked: #427
    let total = sizes[0].0 + sizes[1].0 + sizes[2].0;
    assert_eq!(total, 10, "sum invariant: every record must be assigned"); // lint:allow(no-bare-numeric) reason: invariant check; tracked: #427
}

#[test]
fn topo_sort_places_all_for_linear_chain() {
    // Linear A -> B chain. Verifies the cycle-detection signal is not
    // a false positive for valid DAGs.
    let mut g: DependencyGraph<TestDims> = DependencyGraph::new();
    g.add_edge_kind(USize(0), USize(1), EdgeKind::Read); // lint:allow(no-bare-numeric) reason: linear-chain smoke; tracked: #427
    // Pad the row entry for unit 1 so unit_count reaches 2.
    g.row_offsets.as_mut()[1] = g.edge_count; // lint:allow(no-bare-numeric) reason: CSR padding for trailing empty row; tracked: #427
    g.unit_count = USize(2); // lint:allow(no-bare-numeric) reason: invariant set; tracked: #427

    let (_topo, placed) = steps::topo_sort::<TestDims>(&g);
    assert_eq!(placed, USize(2)); // lint:allow(no-bare-numeric) reason: full placement expected; tracked: #427
}

#[test]
fn phase_config_heuristics_apply_low_record_count() {
    let mut inputs: Inputs = PlanInputs::new();
    inputs.unit_count = USize(3); // lint:allow(no-bare-numeric) reason: three-unit smoke; tracked: #427
    inputs.record_count = USize(100); // lint:allow(no-bare-numeric) reason: small record count picks MaxFuse; tracked: #427
    let result = compute_execution_plan::<TestDims>(&inputs);
    match result {
        Outcome::Ok(plan) => {
            // First phase config should be MaxFuse for low record counts.
            assert_eq!(plan.phases.as_ref()[0].config, PhaseConfig::MaxFuse);
        }
        Outcome::Err(_) => panic!("three-unit plan should succeed"),
    }
}

#[test]
fn build_dag_orders_writer_before_reader_regardless_of_registration() {
    // Unit 0 reads store 0; unit 1 writes store 0. The reader sits at the
    // lower input index, the writer at the higher: a genuine RAW
    // dependency that forward-only edge detection (i < j) drops entirely.
    // Order-independent RAW places the writer (unit 1) before the reader
    // (unit 0) in the topological dispatch order recorded on unit_meta.
    let mut inputs: Inputs = PlanInputs::new();
    inputs.unit_count = USize(2); // lint:allow(no-bare-numeric) reason: two-unit fixture; tracked: #339
    inputs.record_count = USize(64); // lint:allow(no-bare-numeric) reason: smoke record count; tracked: #339
    // unit 0 reads store 0.
    inputs.reads.as_mut()[0] = AccessMask::empty().set(USize(0)); // lint:allow(no-bare-numeric) reason: store/unit index fixture; tracked: #339
    inputs.access.as_mut()[0] = AccessMask::empty().set(USize(0)); // lint:allow(no-bare-numeric) reason: store/unit index fixture; tracked: #339
    // unit 1 writes store 0.
    inputs.writes.as_mut()[1] = AccessMask::empty().set(USize(0)); // lint:allow(no-bare-numeric) reason: store/unit index fixture; tracked: #339
    inputs.access.as_mut()[1] = AccessMask::empty().set(USize(0)); // lint:allow(no-bare-numeric) reason: store/unit index fixture; tracked: #339
    let result = compute_execution_plan::<TestDims>(&inputs);
    match result {
        Outcome::Ok(plan) => {
            assert_eq!(plan.unit_count, USize(2)); // lint:allow(no-bare-numeric) reason: roundtrip; tracked: #339
            assert_eq!(
                plan.unit_meta.as_ref()[0].id.index(),
                USize(1), // lint:allow(no-bare-numeric) reason: expected writer unit-id; tracked: #339
                "writer (unit 1) dispatches first under order-independent RAW"
            );
            assert_eq!(
                plan.unit_meta.as_ref()[1].id.index(),
                USize(0), // lint:allow(no-bare-numeric) reason: expected reader unit-id; tracked: #339
                "reader (unit 0) dispatches after its writer"
            );
        }
        Outcome::Err(_) => panic!("two-unit RAW chain should plan successfully"),
    }
}

#[test]
fn build_dag_detects_mutual_dependency_cycle() {
    // Unit 0 reads store 0 and writes store 1; unit 1 reads store 1 and
    // writes store 0. Each reads what the other writes: a cyclic data
    // dependency. Order-independent RAW produces edges in both directions
    // (writer-before-reader each way), which topo_sort cannot linearise;
    // the runner reports PlanError::Cycle. The forward-only graph produced
    // only one forward edge and reported success.
    let mut inputs: Inputs = PlanInputs::new();
    inputs.unit_count = USize(2); // lint:allow(no-bare-numeric) reason: two-unit fixture; tracked: #339
    inputs.record_count = USize(64); // lint:allow(no-bare-numeric) reason: smoke record count; tracked: #339
    inputs.reads.as_mut()[0] = AccessMask::empty().set(USize(0)); // lint:allow(no-bare-numeric) reason: store/unit index fixture; tracked: #339
    inputs.writes.as_mut()[0] = AccessMask::empty().set(USize(1)); // lint:allow(no-bare-numeric) reason: store/unit index fixture; tracked: #339
    inputs.reads.as_mut()[1] = AccessMask::empty().set(USize(1)); // lint:allow(no-bare-numeric) reason: store/unit index fixture; tracked: #339
    inputs.writes.as_mut()[1] = AccessMask::empty().set(USize(0)); // lint:allow(no-bare-numeric) reason: store/unit index fixture; tracked: #339
    // `access` is left zero: the runner short-circuits at topo_sort
    // (cycle) before any access-consuming step runs, and build_dag reads
    // `reads`/`writes` directly.
    let result = compute_execution_plan::<TestDims>(&inputs);
    assert!(
        matches!(result, Outcome::Err(PlanError::Cycle)),
        "mutual read/write dependency must be rejected as a cycle"
    );
}
