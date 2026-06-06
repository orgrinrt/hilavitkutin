//! `compute_execution_plan` rejects a `PlanDims` whose phase or trunk
//! capacity exceeds the fixed-width id types' addressable range (#641).
//!
//! `PhaseId` wraps `Uint<5>` (addresses 32) and `TrunkId` wraps `Uint<6>`
//! (addresses 64). `DefaultPlanDims` aligns its capacities to those
//! widths, so it can never overflow; a consumer who overrides `PlanDims`
//! with a wider phase/trunk capacity would otherwise get silently wrapped
//! ids on the high slots. The guard is a property of the dims type, so it
//! fires even for an empty plan, which is what these tests drive.

use arvo::{Bits, Hot, Identity, USize, Unsigned};
use arvo_tensor::Dim;
use hilavitkutin::plan::{compute_execution_plan, DefaultPlanDims, PlanDims, PlanError, PlanInputs};
use notko::Outcome;

/// Dims whose phase capacity (33) is one past what `PhaseId` can name (32).
/// Every other dimension is a sane small size; only `Phases` is over-wide.
struct OverWidePhasesDims;

impl PlanDims for OverWidePhasesDims {
    type Units = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Stores = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Edges = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Phases = Dim<33>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: one past PhaseId::ADDRESSABLE, the case under test; tracked: #641
    type Trunks = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type TrunksPerPhase = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Fibers = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Lanes = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Columns = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ComponentsPerTrunk = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type UnitsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ColumnsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Cores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AdjRow = Bits<64, Hot, Unsigned>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-wide row covers the small test units; Bits width literal; tracked: #649
}

/// Dims whose trunk capacity (65) is one past what `TrunkId` can name (64),
/// with the phase capacity kept inside `PhaseId`'s range so the phase guard
/// passes and the trunk guard is the one exercised.
struct OverWideTrunksDims;

impl PlanDims for OverWideTrunksDims {
    type Units = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Stores = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Edges = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Phases = Dim<32>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: aligned phase capacity so the phase guard passes; tracked: #641
    type Trunks = Dim<65>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: one past TrunkId::ADDRESSABLE, the case under test; tracked: #641
    type TrunksPerPhase = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Fibers = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Lanes = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Columns = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ComponentsPerTrunk = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type UnitsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ColumnsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Cores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AdjRow = Bits<64, Hot, Unsigned>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-wide row covers the small test units; Bits width literal; tracked: #649
}

#[test]
fn over_wide_phase_capacity_is_rejected() {
    let inputs: PlanInputs<
        <OverWidePhasesDims as PlanDims>::Units,
        <OverWidePhasesDims as PlanDims>::Stores,
    > = PlanInputs::new();
    // The guard is a property of the dims, so even an empty plan is rejected.
    match compute_execution_plan::<OverWidePhasesDims>(&inputs) {
        Outcome::Err(PlanError::PhaseCapacityExceedsIdWidth) => {}
        other => panic!("expected PhaseCapacityExceedsIdWidth, got {:?}", other),
    }
}

#[test]
fn over_wide_trunk_capacity_is_rejected() {
    let inputs: PlanInputs<
        <OverWideTrunksDims as PlanDims>::Units,
        <OverWideTrunksDims as PlanDims>::Stores,
    > = PlanInputs::new();
    match compute_execution_plan::<OverWideTrunksDims>(&inputs) {
        Outcome::Err(PlanError::TrunkCapacityExceedsIdWidth) => {}
        other => panic!("expected TrunkCapacityExceedsIdWidth, got {:?}", other),
    }
}

#[test]
fn aligned_default_dims_is_not_rejected() {
    // DefaultPlanDims aligns Phases=32 / Trunks=64 to the id widths, so the
    // guard must not false-positive: an empty plan still succeeds.
    let inputs: PlanInputs<
        <DefaultPlanDims as PlanDims>::Units,
        <DefaultPlanDims as PlanDims>::Stores,
    > = PlanInputs::new();
    match compute_execution_plan::<DefaultPlanDims>(&inputs) {
        Outcome::Ok(plan) => assert_eq!(plan.unit_count, USize::ZERO),
        Outcome::Err(e) => panic!("aligned default dims should not be rejected, got {:?}", e),
    }
}
