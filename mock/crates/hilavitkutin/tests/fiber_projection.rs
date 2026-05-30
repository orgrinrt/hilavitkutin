//! Step 6+7 per-block fiber-to-trunk-component projection (Phase C C1d-3a).
//!
//! `project_fiber_components` returns a 2D `Trunk` array sized by the
//! `Capacity` TYPES projected from one `D: PlanDims`, so no
//! `generic_const_exprs` gate is needed (the dimensions are types, not
//! `Cap` const generics).

use arvo::USize;
use arvo_tensor::{cap_size, Capacity, Dim};
use hilavitkutin::plan::{
    steps, BlockPartition, DependencyGraph, PhaseBoundaries, PlanDims, Trunk, TrunkComponent,
    UnitId,
};

/// Test dimensions: eight units / sixteen edges with the per-aggregate
/// capacities the projection exercises.
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
    type ComponentsPerTrunk = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type UnitsPerFiber = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ColumnsPerFiber = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Cores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
}

type UnitDim = <TestDims as PlanDims>::Units;
const UNIT_CAP: usize = cap_size(<UnitDim as Capacity>::CAP); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-position cap projection for test loop bounds; tracked: #72
const UPF_CAP: usize = cap_size(<<TestDims as PlanDims>::UnitsPerFiber as Capacity>::CAP); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-position cap projection for test buffer; tracked: #72

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
fn two_chains_3() -> DependencyGraph<TestDims> {
    let mut g: DependencyGraph<TestDims> = DependencyGraph::new();
    g.add_edge(U0, U1);
    g.add_edge(U1, U2);
    g.add_edge(U3, U4);
    g.add_edge(U4, U5);
    let uc = g.unit_count.0;
    g.row_offsets.as_mut()[uc] = g.edge_count;
    g.unit_count = U6;
    g
}

/// Block ids [0,0,0,1,1,1]: chain A is block 0, chain B is block 1.
fn two_block_partition() -> BlockPartition<UnitDim> {
    let mut part: BlockPartition<UnitDim> = BlockPartition::new();
    part.block_count = USize(2); // lint:allow(no-bare-numeric) reason: block count; tracked: #427
    let ids = [U0, U0, U0, U1, U1, U1];
    let slots = part.block_of_unit.as_mut();
    let mut i = 0;
    while i < ids.len() {
        slots[i] = ids[i];
        i += 1;
    }
    part
}

/// One phase spanning all six units.
fn one_phase() -> PhaseBoundaries<TestDims> {
    let mut w: PhaseBoundaries<TestDims> = PhaseBoundaries::new();
    w.phase_count = USize(1); // lint:allow(no-bare-numeric) reason: single phase; tracked: #427
    w.boundaries.as_mut()[0] = U0;
    w
}

/// Interleaved topo `[0, 3, 1, 4, 2, 5]`: blocks alternate. A global
/// fiber walk over this order rolls a fiber at unit 2 (out-degree 0) and
/// would split chain B into two fibers; per-block formation keeps B as a
/// single fiber. This interleaving is what makes the projection's
/// nesting non-trivial to get right.
fn interleaved_topo() -> <UnitDim as Capacity>::Array<UnitId> {
    let mut topo = <UnitDim as Capacity>::filled(UnitId::ZERO);
    let order = [U0, U3, U1, U4, U2, U5];
    let slots = topo.as_mut();
    let mut i = 0;
    while i < order.len() {
        slots[i] = UnitId::from_index(order[i]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
        i += 1;
    }
    topo
}

/// Collect a trunk's single Fiber component's units into a fixed buffer.
/// Asserts the trunk holds exactly one Fiber, returns (count, units).
fn single_fiber_units(trunk: &Trunk<TestDims>) -> (usize, [usize; UPF_CAP]) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test buffer of raw indices sized by the units-per-fiber cap; tracked: #72
    assert_eq!(trunk.component_count, USize(1), "trunk should hold exactly one fiber"); // lint:allow(no-bare-numeric) reason: expected component count; tracked: #427
    let mut out = [0usize; UPF_CAP]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test buffer of raw indices; tracked: #72
    let mut count = 0;
    if let TrunkComponent::Fiber(f) = &trunk.components.as_ref()[0] {
        let uc = f.unit_count.0;
        let units = f.units.as_ref();
        let mut u = 0;
        while u < uc {
            out[count] = units[u].index().0;
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
    let trunks = steps::project_fiber_components::<TestDims>(&g, &part, &waists, &topo, U6);

    // Block A (first-seen in topo) is trunk 0, block B is trunk 1.
    let phase0 = trunks.as_ref()[0].as_ref();
    let (a_count, a_units) = single_fiber_units(&phase0[0]);
    let (b_count, b_units) = single_fiber_units(&phase0[1]);

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

/// Two directed triangles {0,1,2} and {3,4,5} joined by one bridge edge
/// 2->3: a single connected block of six units. Edges appended in
/// non-decreasing `from` order; unit 5 is the trailing sink. The
/// undirected Laplacian's min cut is the bridge.
fn two_triangles_bridged() -> DependencyGraph<TestDims> {
    let mut g: DependencyGraph<TestDims> = DependencyGraph::new();
    g.add_edge(U0, U1);
    g.add_edge(U0, U2);
    g.add_edge(U1, U2);
    g.add_edge(U2, U3); // the weak bridge between the triangles
    g.add_edge(U3, U4);
    g.add_edge(U3, U5);
    g.add_edge(U4, U5);
    let uc = g.unit_count.0;
    g.row_offsets.as_mut()[uc] = g.edge_count;
    g.unit_count = U6;
    g
}

#[test]
fn wide_block_routes_to_spectral_cut() {
    let g = two_triangles_bridged();
    // One connected block (the bridge links the triangles); BlockPartition
    // defaults every unit to block 0, so block_count = 1 suffices.
    let mut part: BlockPartition<UnitDim> = BlockPartition::new();
    part.block_count = USize(1); // lint:allow(no-bare-numeric) reason: single connected block; tracked: #427
    let waists = one_phase();
    // Identity topo [0,1,2,3,4,5], a valid order for this DAG.
    let mut topo = <UnitDim as Capacity>::filled(UnitId::ZERO);
    {
        let slots = topo.as_mut();
        let mut i = 0;
        while i < 6 {
            slots[i] = UnitId::from_index(USize(i)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: USize-construct from internal index; tracked: #72
            i += 1;
        }
    }
    let trunks = steps::project_fiber_components::<TestDims>(&g, &part, &waists, &topo, U6);

    // Six units exceed the >5 gate, so the single block forms fibers
    // spectrally. The Fiedler cut separates the triangles, and every
    // k-way sub-split stays within a triangle, so no fiber straddles the
    // bridge. The greedy former (walking topo, rolling on out-degree)
    // would instead produce a {1,2,3} fiber spanning the bridge; that
    // straddle is the discriminator a width-gate-off flip would fail.
    let trunk = &trunks.as_ref()[0].as_ref()[0];
    assert!(trunk.component_count.0 >= 2, "the wide block is partitioned into multiple fibers");
    let comps = trunk.components.as_ref();
    let mut c = 0;
    while c < trunk.component_count.0 {
        if let TrunkComponent::Fiber(f) = &comps[c] {
            let uc = f.unit_count.0;
            assert!(uc > 0, "every emitted fiber is non-empty");
            let units = f.units.as_ref();
            let lo = units[0].index().0 < 3;
            let mut u = 0;
            while u < uc {
                let ui = units[u].index().0;
                assert_eq!(ui < 3, lo, "a spectral fiber must not straddle the bridge");
                u += 1;
            }
        } else {
            panic!("component should be a Fiber");
        }
        c += 1;
    }
}

// Silence the unused-const warning for the unit-cap projection, which is
// kept for symmetry with the other plan tests' fixtures.
const _: usize = UNIT_CAP;
