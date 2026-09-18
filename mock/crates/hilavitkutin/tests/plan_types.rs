//! Plan-stage type tests.
//!
//! The plan-stage types are sized by the `Capacity` type (`Dim<N>`), so
//! no `generic_const_exprs` gate is needed: the capacities are types,
//! not `Cap` const generics, and the backing arrays are plain `[T; N]`.

use arvo::{Additive, Bits, Hot, Identity, USize, Unsigned};
use arvo_tensor::Dim;
use hilavitkutin::plan::{AccessMask, DependencyGraph, PlanDims};

/// A store capacity of sixteen for the access-mask fixtures.
type C16 = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test store capacity; Dim<N> array-length root; tracked: #649

/// Eight units, sixteen edges for the dependency-graph fixture.
struct TestDims;

#[rustfmt::skip]
impl PlanDims for TestDims {
    type AccumsPerCore = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type AdjRow = Bits<64, Hot, Unsigned>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: 64-wide row covers the small test units; Bits width literal; tracked: #649
    type Columns = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ColumnsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type ComponentsPerTrunk = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Cores = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Edges = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Fibers = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Lanes = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Phases = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type PlanAffecting = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Stores = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Trunks = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type TrunksPerPhase = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type Units = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
    type UnitsPerFiber = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
}

#[test]
fn access_mask_empty_set_contains_overlaps() {
    let empty: AccessMask<C16> = AccessMask::empty();
    assert!(empty.is_empty().0);
    assert!(!empty.contains(<USize as Identity<Additive>>::IDENTITY).0);

    let m = empty
        .set(USize(3)) // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
        .set(USize(7)); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
    assert!(!m.is_empty().0);
    assert!(m.contains(USize(3)).0); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
    assert!(m.contains(USize(7)).0); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
    assert!(!m.contains(USize(4)).0); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426

    let other: AccessMask<C16> = AccessMask::empty().set(USize(7)); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
    assert!(m.overlaps(&other).0);

    let disjoint: AccessMask<C16> = AccessMask::empty()
        .set(USize(1)) // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
        .set(USize(2)); // lint:allow(no-bare-numeric) reason: slot index; tracked: #426
    assert!(!m.overlaps(&disjoint).0);
}

#[test]
fn dependency_graph_default_and_edges() {
    // CSR graph sized by TestDims: Units=8, Edges=16.
    let zero = <USize as Identity<Additive>>::IDENTITY;
    let mut g: DependencyGraph<TestDims> = DependencyGraph::new();
    assert!(!g.has_edge(zero, USize(1)).0); // lint:allow(no-bare-numeric) reason: node index; tracked: #427
    assert!(!g.has_edge(USize(3), USize(5)).0); // lint:allow(no-bare-numeric) reason: node index; tracked: #427

    // Append in ascending-from order (CSR invariant): 0 -> 1, then
    // 3 -> 5. Units 1 and 2 land as zero-out-degree implicitly.
    g.add_edge(zero, USize(1)); // lint:allow(no-bare-numeric) reason: node index; tracked: #427
    g.add_edge(USize(3), USize(5)); // lint:allow(no-bare-numeric) reason: node index; tracked: #427
    assert!(g.has_edge(zero, USize(1)).0); // lint:allow(no-bare-numeric) reason: node index; tracked: #427
    assert!(g.has_edge(USize(3), USize(5)).0); // lint:allow(no-bare-numeric) reason: node index; tracked: #427
    assert!(!g.has_edge(USize(1), zero).0); // lint:allow(no-bare-numeric) reason: node index; tracked: #427

    // Out-of-range no-ops.
    g.add_edge(USize(100), USize(200)); // lint:allow(no-bare-numeric) reason: out-of-range probe; tracked: #427
    assert!(!g.has_edge(USize(100), USize(200)).0); // lint:allow(no-bare-numeric) reason: out-of-range probe; tracked: #427
}
