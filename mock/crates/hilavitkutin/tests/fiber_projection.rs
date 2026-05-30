//! Step 6+7 per-block fiber-to-trunk-component projection (Phase C C1d-3a).

// `project_fiber_components` returns a 2D `Trunk` array carrying
// `[(); cap_size(N)]:` bounds through the CSR-derived plan dims, so this
// crate enables generic_const_exprs to normalise them. adt_const_params
// is not needed (only Cap values via cap(N) are named, never a Cap const
// param).
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_tensor::{cap, cap_size};
use hilavitkutin::plan::{
    steps, BlockPartition, DependencyGraph, PhaseBoundaries, TrunkComponent, UnitId,
};

const UNITS: Cap = cap(8); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121
const EDGES: Cap = cap(16); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121
const FIBERS: Cap = cap(8); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121
const PHASES: Cap = cap(4); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121
const TRUNKS: Cap = cap(4); // lint:allow(no-bare-numeric) reason: trunks-per-phase cap; tracked: #121
const COMPS: Cap = cap(8); // lint:allow(no-bare-numeric) reason: components-per-trunk cap; tracked: #121
const UPF: Cap = cap(8); // lint:allow(no-bare-numeric) reason: units-per-fiber cap; tracked: #121
const CPF: Cap = cap(8); // lint:allow(no-bare-numeric) reason: columns-per-fiber cap; tracked: #121

const U0: USize = USize(0); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U1: USize = USize(1); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U2: USize = USize(2); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U3: USize = USize(3); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U4: USize = USize(4); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U5: USize = USize(5); // lint:allow(no-bare-numeric) reason: unit index; tracked: #427
const U6: USize = USize(6); // lint:allow(no-bare-numeric) reason: live unit count; tracked: #427

/// Two disconnected chains A = 0->1->2 and B = 3->4->5 in an eight-unit
/// cap, six units and four edges live. `add_edge` advances the frontier,
/// so units 2 and 5 land as empty (out-degree 0) sink rows. Out-degrees
/// are then [1, 1, 0, 1, 1, 0].
fn two_chains_3() -> DependencyGraph<UNITS, EDGES> {
    let mut g: DependencyGraph<UNITS, EDGES> = DependencyGraph::new();
    g.add_edge(U0, U1);
    g.add_edge(U1, U2);
    g.add_edge(U3, U4);
    g.add_edge(U4, U5);
    g.row_offsets[g.unit_count.0] = g.edge_count;
    g.unit_count = U6;
    g
}

/// Block ids [0,0,0,1,1,1]: chain A is block 0, chain B is block 1.
fn two_block_partition() -> BlockPartition<UNITS> {
    let mut part: BlockPartition<UNITS> = BlockPartition::new();
    part.block_count = USize(2); // lint:allow(no-bare-numeric) reason: block count; tracked: #427
    let ids = [U0, U0, U0, U1, U1, U1];
    let mut i = 0;
    while i < ids.len() {
        part.block_of_unit[i] = ids[i];
        i += 1;
    }
    part
}

/// One phase spanning all six units.
fn one_phase() -> PhaseBoundaries<PHASES> {
    let mut w: PhaseBoundaries<PHASES> = PhaseBoundaries::new();
    w.phase_count = USize(1); // lint:allow(no-bare-numeric) reason: single phase; tracked: #427
    w.boundaries[0] = U0;
    w
}

/// Interleaved topo `[0, 3, 1, 4, 2, 5]`: blocks alternate. A global
/// fiber walk over this order rolls a fiber at unit 2 (out-degree 0) and
/// would split chain B into two fibers; per-block formation keeps B as a
/// single fiber. This interleaving is what makes the projection's
/// nesting non-trivial to get right.
fn interleaved_topo() -> [UnitId; cap_size(UNITS)] {
    let mut topo = [UnitId::ZERO; cap_size(UNITS)];
    let order = [U0, U3, U1, U4, U2, U5];
    let mut i = 0;
    while i < order.len() {
        topo[i] = UnitId::from_index(order[i]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
        i += 1;
    }
    topo
}

/// Collect a trunk's single Fiber component's units into a fixed buffer.
/// Asserts the trunk holds exactly one Fiber, returns (count, units).
fn single_fiber_units(
    trunk: &hilavitkutin::plan::Trunk<COMPS, UPF, CPF>,
) -> (usize, [usize; cap_size(UPF)]) {
    assert_eq!(trunk.component_count, USize(1), "trunk should hold exactly one fiber"); // lint:allow(no-bare-numeric) reason: expected component count; tracked: #427
    let mut out = [0usize; cap_size(UPF)];
    let mut count = 0;
    if let TrunkComponent::Fiber(f) = &trunk.components[0] {
        let uc = f.unit_count.0;
        let mut u = 0;
        while u < uc {
            out[count] = f.units[u].index().0;
            count += 1;
            u += 1;
        }
    } else {
        panic!("component 0 should be a Fiber");
    }
    (count, out)
}

#[test]
fn per_block_projection_keeps_each_chain_one_fiber() {
    let g = two_chains_3();
    let part = two_block_partition();
    let waists = one_phase();
    let topo = interleaved_topo();
    let trunks = steps::project_fiber_components::<
        UNITS, EDGES, FIBERS, PHASES, TRUNKS, COMPS, UPF, CPF,
    >(&g, &part, &waists, &topo, U6);

    // Block A (first-seen in topo) is trunk 0, block B is trunk 1.
    let (a_count, a_units) = single_fiber_units(&trunks[0][0]);
    let (b_count, b_units) = single_fiber_units(&trunks[0][1]);

    // Chain A: one fiber holding exactly {0, 1, 2}; chain B: one fiber
    // holding exactly {3, 4, 5}. Under a global former on the interleaved
    // topo, chain B would split into {3,4} and {5} (component_count 2),
    // failing `single_fiber_units`'s one-fiber assertion on trunk 1.
    assert_eq!(a_count, 3, "chain A is one three-unit fiber");
    assert_eq!(b_count, 3, "chain B is one three-unit fiber");
    let mut a_seen = [false; 6];
    let mut b_seen = [false; 6];
    let mut i = 0;
    while i < 3 {
        assert!(a_units[i] < 3, "chain A fiber must not straddle into block B");
        assert!(b_units[i] >= 3 && b_units[i] < 6, "chain B fiber must stay in block B");
        a_seen[a_units[i]] = true;
        b_seen[b_units[i]] = true;
        i += 1;
    }
    assert!(a_seen[0] && a_seen[1] && a_seen[2], "chain A fiber is exactly {{0,1,2}}");
    assert!(b_seen[3] && b_seen[4] && b_seen[5], "chain B fiber is exactly {{3,4,5}}");
}
