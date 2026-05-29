//! Step 5 block-diagonal detection + trunk-count projection (Phase C C1c).

// `block_diagonalise` returns `BlockPartition<MAX_UNITS>` and
// `phase_trunk_counts` returns `[USize; cap_size(MAX_PHASES)]`; both carry
// `[(); cap_size(N)]:` bounds through the CSR adapter, so this crate enables
// generic_const_exprs to normalise them. adt_const_params is not needed (only
// Cap values via cap(N) are named, never a Cap const param).
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_tensor::{cap, cap_size};
use hilavitkutin::plan::{steps, BlockPartition, DependencyGraph, PhaseBoundaries, UnitId};

const UNITS: Cap = cap(8); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121
const EDGES: Cap = cap(16); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121
const PHASES: Cap = cap(4); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121

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
fn two_chains() -> DependencyGraph<UNITS, EDGES> {
    let mut g: DependencyGraph<UNITS, EDGES> = DependencyGraph::new();
    g.add_edge(U0, U1);
    g.add_edge(U2, U3);
    g.row_offsets[g.unit_count.0] = g.edge_count;
    g.unit_count = U4;
    g
}

/// Build a `BlockPartition` from a per-unit block-id list.
fn partition_of(block_count: USize, ids: &[USize]) -> BlockPartition<UNITS> {
    let mut part: BlockPartition<UNITS> = BlockPartition::new();
    part.block_count = block_count;
    let mut i = 0;
    while i < ids.len() && i < cap_size(UNITS) {
        part.block_of_unit[i] = ids[i];
        i += 1;
    }
    part
}

/// Identity topo order: `topo[i]` is unit `i`.
fn identity_topo() -> [UnitId; cap_size(UNITS)] {
    let mut topo = [UnitId::ZERO; cap_size(UNITS)];
    let mut i = 0;
    while i < cap_size(UNITS) {
        topo[i] = UnitId::from_index(USize(i)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
        i += 1;
    }
    topo
}

#[test]
fn block_diagonalise_finds_two_components() {
    let part = steps::block_diagonalise::<UNITS, EDGES>(&two_chains());
    // 0-1 and 2-3 are two disconnected components: two blocks. Units in the
    // same chain share a block id; units in different chains do not. A
    // degenerate impl (everything one block) gives block_count 1 and fails
    // these; the old Bool::TRUE stub does not even typecheck against this.
    assert_eq!(part.block_count, USize(2)); // lint:allow(no-bare-numeric) reason: expected block count; tracked: #427
    assert_eq!(part.block_of_unit[U0.0], part.block_of_unit[U1.0]);
    assert_eq!(part.block_of_unit[U2.0], part.block_of_unit[U3.0]);
    assert_ne!(part.block_of_unit[U0.0], part.block_of_unit[U2.0]);
}

#[test]
fn two_blocks_in_one_phase_yield_two_trunks() {
    // Four live units, block ids [0, 0, 1, 1]; one phase spanning [0, 4).
    let part = partition_of(USize(2), &[U0, U0, U1, U1]);
    let mut waists: PhaseBoundaries<PHASES> = PhaseBoundaries::new();
    waists.boundaries[0] = U0;
    waists.phase_count = USize(1); // lint:allow(no-bare-numeric) reason: single phase; tracked: #427
    let counts = steps::phase_trunk_counts::<UNITS, PHASES>(&part, &waists, &identity_topo(), U4);
    // Phase 0 holds both blocks, so two trunks. A projection that ignored the
    // partition (one trunk per phase) would give 1; the old stub left
    // trunk_count 0.
    assert_eq!(counts[0], USize(2)); // lint:allow(no-bare-numeric) reason: expected trunk count; tracked: #427
}

#[test]
fn one_block_per_phase_yields_one_trunk_each() {
    // block ids [0, 0, 1, 1]; two phases split at unit 2.
    let part = partition_of(USize(2), &[U0, U0, U1, U1]);
    let mut waists: PhaseBoundaries<PHASES> = PhaseBoundaries::new();
    waists.boundaries[0] = U0;
    waists.boundaries[1] = U2;
    waists.phase_count = USize(2); // lint:allow(no-bare-numeric) reason: two phases; tracked: #427
    let counts = steps::phase_trunk_counts::<UNITS, PHASES>(&part, &waists, &identity_topo(), U4);
    assert_eq!(counts[0], USize(1)); // lint:allow(no-bare-numeric) reason: phase 0 one block; tracked: #427
    assert_eq!(counts[1], USize(1)); // lint:allow(no-bare-numeric) reason: phase 1 one block; tracked: #427
}

#[test]
fn straddling_block_counts_in_each_phase() {
    // A single block across all four units, split into two phases. The block
    // contributes a trunk to each phase it touches.
    let part = partition_of(USize(1), &[U0, U0, U0, U0]);
    let mut waists: PhaseBoundaries<PHASES> = PhaseBoundaries::new();
    waists.boundaries[0] = U0;
    waists.boundaries[1] = U2;
    waists.phase_count = USize(2); // lint:allow(no-bare-numeric) reason: two phases; tracked: #427
    let counts = steps::phase_trunk_counts::<UNITS, PHASES>(&part, &waists, &identity_topo(), U4);
    assert_eq!(counts[0], USize(1)); // lint:allow(no-bare-numeric) reason: phase 0 touches the block; tracked: #427
    assert_eq!(counts[1], USize(1)); // lint:allow(no-bare-numeric) reason: phase 1 touches the block; tracked: #427
}
