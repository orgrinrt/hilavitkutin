//! Step 5 block-diagonal detection + trunk-count projection (Phase C C1c).
//!
//! `block_diagonalise` returns `BlockPartition<TestDims::Units>` and
//! `phase_trunk_counts` returns `<TestDims::Phases as Capacity>::Array<USize>`;
//! both are now sized by the `Capacity` TYPE, so no `generic_const_exprs`
//! gate is needed (the capacity dimensions are types, not `Cap` const
//! generics).

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_tensor::{Capacity, Dim};
use hilavitkutin::plan::{steps, BlockPartition, DependencyGraph, PhaseBoundaries, PlanDims, UnitId};

/// Eight units, sixteen edges, four phases for the fixtures.
struct TestDims;

impl PlanDims for TestDims {
    type Units = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Stores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Edges = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Phases = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Trunks = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type TrunksPerPhase = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Fibers = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Lanes = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Columns = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ComponentsPerTrunk = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type UnitsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ColumnsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Cores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AccumsPerCore = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type PlanAffecting = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AdjRow = Bits<64, Hot, Unsigned>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-wide row covers the small test units; Bits width literal; tracked: #649
}

/// The unit capacity, the dimension the per-unit fixtures are sized by.
type UnitDim = <TestDims as PlanDims>::Units;
const UNIT_CAP: usize = arvo_tensor::cap_size(<UnitDim as Capacity>::CAP); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-position cap projection for test loop bounds; tracked: #72

// Unit / block indices, named once so the per-literal lint:allow lives in one
// place and the fixtures read as structure.
const U0: USize = USize(0); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U1: USize = USize(1); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U2: USize = USize(2); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U3: USize = USize(3); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U4: USize = USize(4); // lint:allow(no-bare-numeric) reason: first slack index; tracked: #427

/// Two independent chains 0->1 and 2->3 in an eight-unit cap, four units and
/// two edges live. `add_edge` appends in row-major (from non-decreasing)
/// order; the trailing sink (unit 3) gets its empty row via the
/// build_dag-style single-trailing-sink finalize.
fn two_chains() -> DependencyGraph<TestDims> {
    let mut g: DependencyGraph<TestDims> = DependencyGraph::new();
    g.add_edge(U0, U1);
    g.add_edge(U2, U3);
    let uc = g.unit_count.0;
    g.row_offsets.as_mut()[uc] = g.edge_count;
    g.unit_count = U4;
    g
}

/// Build a `BlockPartition` from a per-unit block-id list.
fn partition_of(block_count: USize, ids: &[USize]) -> BlockPartition<UnitDim> {
    let mut part: BlockPartition<UnitDim> = BlockPartition::new();
    part.block_count = block_count;
    let mut i = 0;
    while i < ids.len() && i < UNIT_CAP {
        part.block_of_unit.as_mut()[i] = ids[i];
        i += 1;
    }
    part
}

/// Identity topo order: `topo[i]` is unit `i`.
fn identity_topo() -> <UnitDim as Capacity>::Array<UnitId> {
    let mut topo = <UnitDim as Capacity>::filled(UnitId::ZERO);
    let slots = topo.as_mut();
    let mut i = 0;
    while i < UNIT_CAP {
        slots[i] = UnitId::from_index(USize(i)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
        i += 1;
    }
    topo
}

#[test]
fn block_diagonalise_finds_two_components() {
    let part = steps::block_diagonalise::<TestDims>(&two_chains());
    // 0-1 and 2-3 are two disconnected components: two blocks. Units in the
    // same chain share a block id; units in different chains do not. A
    // degenerate impl (everything one block) gives block_count 1 and fails
    // these; the old Bool::TRUE stub does not even typecheck against this.
    assert_eq!(part.block_count, USize(2)); // lint:allow(no-bare-numeric) reason: expected block count; tracked: #427
    let block_of_unit = part.block_of_unit.as_ref();
    assert_eq!(block_of_unit[U0.0], block_of_unit[U1.0]);
    assert_eq!(block_of_unit[U2.0], block_of_unit[U3.0]);
    assert_ne!(block_of_unit[U0.0], block_of_unit[U2.0]);
}

#[test]
fn two_blocks_in_one_phase_yield_two_trunks() {
    // Four live units, block ids [0, 0, 1, 1]; one phase spanning [0, 4).
    let part = partition_of(USize(2), &[U0, U0, U1, U1]);
    let mut waists: PhaseBoundaries<TestDims> = PhaseBoundaries::new();
    waists.boundaries.as_mut()[0] = U0;
    waists.phase_count = USize(1); // lint:allow(no-bare-numeric) reason: single phase; tracked: #427
    let counts = steps::phase_trunk_counts::<TestDims>(&part, &waists, &identity_topo(), U4);
    // Phase 0 holds both blocks, so two trunks. A projection that ignored the
    // partition (one trunk per phase) would give 1; the old stub left
    // trunk_count 0.
    assert_eq!(counts.as_ref()[0], USize(2)); // lint:allow(no-bare-numeric) reason: expected trunk count; tracked: #427
}

#[test]
fn one_block_per_phase_yields_one_trunk_each() {
    // block ids [0, 0, 1, 1]; two phases split at unit 2.
    let part = partition_of(USize(2), &[U0, U0, U1, U1]);
    let mut waists: PhaseBoundaries<TestDims> = PhaseBoundaries::new();
    waists.boundaries.as_mut()[0] = U0;
    waists.boundaries.as_mut()[1] = U2;
    waists.phase_count = USize(2); // lint:allow(no-bare-numeric) reason: two phases; tracked: #427
    let counts = steps::phase_trunk_counts::<TestDims>(&part, &waists, &identity_topo(), U4);
    let counts = counts.as_ref();
    assert_eq!(counts[0], USize(1)); // lint:allow(no-bare-numeric) reason: phase 0 one block; tracked: #427
    assert_eq!(counts[1], USize(1)); // lint:allow(no-bare-numeric) reason: phase 1 one block; tracked: #427
}

#[test]
fn straddling_block_counts_in_each_phase() {
    // A single block across all four units, split into two phases. The block
    // contributes a trunk to each phase it touches.
    let part = partition_of(USize(1), &[U0, U0, U0, U0]);
    let mut waists: PhaseBoundaries<TestDims> = PhaseBoundaries::new();
    waists.boundaries.as_mut()[0] = U0;
    waists.boundaries.as_mut()[1] = U2;
    waists.phase_count = USize(2); // lint:allow(no-bare-numeric) reason: two phases; tracked: #427
    let counts = steps::phase_trunk_counts::<TestDims>(&part, &waists, &identity_topo(), U4);
    let counts = counts.as_ref();
    assert_eq!(counts[0], USize(1)); // lint:allow(no-bare-numeric) reason: phase 0 touches the block; tracked: #427
    assert_eq!(counts[1], USize(1)); // lint:allow(no-bare-numeric) reason: phase 1 touches the block; tracked: #427
}
