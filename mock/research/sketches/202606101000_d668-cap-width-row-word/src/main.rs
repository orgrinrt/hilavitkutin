//! Sketch (D668, GATE-1). Lift the engine's 64-node waist cap by deriving the
//! bit-matrix row word from `D::Units` capacity instead of hardcoding
//! `Bits<64, Hot, Unsigned>`.
//!
//! arvo #663 generalised `arvo_graph::waist_detect<C, B>` over the row word `B`
//! and arvo's own `tests/generic_width.rs` proves CONCRETE wide rows compile and
//! run (`BitMatrix<Bits<256, Hot, Unsigned>, Dim<129>>` fed through
//! `waist_detect`, `components`, `upward_rank`). So `Bits<N>` for N in {128, 256}
//! satisfies the `B: BitSequence + BitAccess + BitLogic + Copy + Default +
//! Identity` bound: risk cleared for concrete N.
//!
//! The OPEN question is the GENERIC width expression. `compute_waists` is generic
//! over `D::Units: Capacity`; the row word must be sized to
//! `cap_size(<C as Capacity>::CAP)` bits. `Bits` takes `const N: u16`, but
//! `cap_size` returns `usize`, so the expression is
//! `Bits<{ cap_size(<C as Capacity>::CAP) as u16 }, Hot, Unsigned>`: a
//! generic_const_exprs const expression with a usize->u16 cast, symbolic over C.
//! arvo's concrete-width tests never exercise this. The bar: this generic shape
//! compiles, the `waist_detect` bound resolves for the symbolic N, and the fn
//! instantiates for several capacities including a non-power-of-two (Dim<100>).
//!
//! Outcome recorded at the bottom.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(dead_code)]

use arvo::strategy::Identity;
use arvo::{Bits, Bool, Hot, Unsigned, USize};
use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};
use arvo_bitmask::{BitMatrix, Mask, NodeId};
use arvo_graph::waist_detect;
use arvo_tensor::{cap_size, Capacity, Dim};

/// The row word sized to a capacity's node count. This is the type alias the
/// engine's `compute_waists` would use in place of the hardcoded
/// `Bits<64, Hot, Unsigned>`.
#[allow(type_alias_bounds)]
type WaistRow<C: Capacity> = Bits<{ cap_size(<C as Capacity>::CAP) as u16 }, Hot, Unsigned>;

/// FAILED HYPOTHESIS (kept as audit, never compiled). The inline-derive row
/// word: build the bit-matrix over `WaistRow<C> = Bits<{cap_size(C::CAP) as
/// u16}>`. This does not compile (see OUTCOME): the symbolic-N container
/// projection wall. Gated off so the crate still builds to demonstrate the
/// resolution below.
#[cfg(any())]
fn compute_waists_generic<C: Capacity>(edges: &[(usize, usize)], n: usize) -> Mask<WaistRow<C>>
where
    WaistRow<C>: BitSequence + BitAccess + BitLogic + Copy + Default + Identity,
    C::Array<USize>: Copy,
    C::Array<Bool>: Copy,
{
    let cap = cap_size(<C as Capacity>::CAP);
    let mut adj: BitMatrix<WaistRow<C>, C> = BitMatrix::empty();
    for &(from, to) in edges {
        if from < cap && to < cap {
            adj.set_edge(NodeId(USize(from)), NodeId(USize(to)));
        }
    }

    // Identity topo order (node k at position k) for the prefix, sentinel tail.
    let mut topo_nodes: C::Array<NodeId> = C::filled(NodeId(USize(cap)));
    let mut k = 0;
    while k < n && k < cap {
        topo_nodes.as_mut()[k] = NodeId(USize(k));
        k += 1;
    }

    waist_detect::<C, WaistRow<C>>(&adj, &topo_nodes)
}

/// RESOLUTION (proven WORKS). Carry the row word as an associated type on a
/// trait whose impls are concrete, so `N` is pinned at the impl and the
/// container projection resolves. This is the shape the engine's `PlanDims`
/// would take: `type AdjRow: BitSequence + BitAccess + BitLogic + Copy +
/// Default + Identity`, plus `type Units: Capacity`. Each concrete `PlanDims`
/// impl sets `AdjRow` to a concrete `Bits<...>` sized to cover its `Units`.
/// `compute_waists` then uses `D::AdjRow` in place of the hardcoded
/// `Bits<64, Hot, Unsigned>`, and the cap is whatever the PlanDims impl chose.
trait RowCarrier {
    type Units: Capacity;
    type AdjRow: BitSequence + BitAccess + BitLogic + Copy + Default + Identity;
}

/// Small carrier: 64-node row (the current engine default), Units = Dim<64>.
struct Small;
impl RowCarrier for Small {
    type Units = Dim<64>;
    type AdjRow = Bits<64, Hot, Unsigned>;
}

/// Wide carrier: 256-node row over Units = Dim<256>. Proves the carrier lifts
/// the cap to the multi-limb width arvo's own generic_width tests exercise.
struct Wide;
impl RowCarrier for Wide {
    type Units = Dim<256>;
    type AdjRow = Bits<256, Hot, Unsigned>;
}

/// Non-power-of-two carrier: Units = Dim<100>, row sized to the next concrete
/// width that covers 100 nodes (128).
struct Odd;
impl RowCarrier for Odd {
    type Units = Dim<100>;
    type AdjRow = Bits<128, Hot, Unsigned>;
}

/// Mirror of the engine's `compute_waists` core via the associated-type carrier.
/// `R::AdjRow` is concrete at every instantiation, so the container projection
/// resolves and this compiles for every carrier.
fn compute_waists_via_carrier<R: RowCarrier>(
    edges: &[(usize, usize)],
    n: usize,
) -> Mask<R::AdjRow>
where
    <R::Units as Capacity>::Array<USize>: Copy,
    <R::Units as Capacity>::Array<Bool>: Copy,
{
    let cap = cap_size(<R::Units as Capacity>::CAP);
    let mut adj: BitMatrix<R::AdjRow, R::Units> = BitMatrix::empty();
    for &(from, to) in edges {
        if from < cap && to < cap {
            adj.set_edge(NodeId(USize(from)), NodeId(USize(to)));
        }
    }
    let mut topo_nodes: <R::Units as Capacity>::Array<NodeId> =
        <R::Units as Capacity>::filled(NodeId(USize(cap)));
    let mut k = 0;
    while k < n && k < cap {
        topo_nodes.as_mut()[k] = NodeId(USize(k));
        k += 1;
    }
    waist_detect::<R::Units, R::AdjRow>(&adj, &topo_nodes)
}

fn main() {
    // 0 -> 1 -> 2, 0 -> 3 -> 2: node 2 is a width-1 waist at depth 2.
    let edges = [(0usize, 1usize), (1, 2), (0, 3), (3, 2)];

    let small = compute_waists_via_carrier::<Small>(&edges, 4);
    let odd = compute_waists_via_carrier::<Odd>(&edges, 4);
    let wide = compute_waists_via_carrier::<Wide>(&edges, 4);

    let bits = [
        small.contains(USize(2)).0,
        odd.contains(USize(2)).0,
        wide.contains(USize(2)).0,
    ];
    core::hint::black_box(&bits);
}

// OUTCOME: FAILS WITH E0277 (`Bits<{ cap_size(<C as Capacity>::CAP) as u16 }>:
// Sized` not satisfied) + E0599 (method `contains` unavailable for the same
// reason). 19 errors, all the same root cause.
//
// Root cause: arvo's `Bits<const N: u16, S, Sign>` resolves its storage
// container by const-tag dispatch, `Picker: Project<N_TAG, Sign, BYTES, S>`
// (arvo-strategy container.rs). The trait solver discharges that `Project`
// bound (hence `Bits<N>: Sized`) only for a CONCRETE N. The width expression
// `{ cap_size(<C as Capacity>::CAP) as u16 }` stays a symbolic generic_const_expr
// over `C`, never reduces to a literal during bound-checking, so
// `Picker: Project<{that expr}, ...>` cannot be proven and `Bits<{expr}>` is not
// even `Sized`. Adding the cast does not help; the wall is the symbolic-N
// container projection, not the cast.
//
// This is why arvo's own `tests/generic_width.rs` passes CONCRETE widths
// (`Bits<256>`, `Bits<128>`): with a literal N the `Project` bound resolves. The
// "derive the row word from the capacity inline" shape is NOT expressible while
// `Bits` dispatches its container per-concrete-N.
//
// Resolution direction (for the round / design call): the row word must be
// carried by a type whose N is already CONCRETE at the use site, not computed
// from a symbolic capacity. Two candidate shapes:
//   A. arvo-side: `Capacity` (or `Dim<N>`) gains an associated row-word type,
//      e.g. `type AdjRow: BitSequence + BitAccess + BitLogic + Copy + Default +
//      Identity`, each concrete `Dim<N>` impl setting it to its concrete
//      `Bits<...>`. Fix-the-stack-upstream; every arvo-graph consumer benefits.
//   B. engine-side: `PlanDims` gains `type WaistRow: <those bounds>`, each
//      concrete PlanDims impl pinning a concrete `Bits<...>` sized to its
//      `Units`. compute_waists uses `D::WaistRow`. No arvo change.
// Both pin N at a concrete impl, sidestepping the symbolic-N projection wall.
// A is the cleaner substrate fix (the capacity is what knows its own node count);
// B is contained to the engine.
//
// RESOLUTION PROVEN: option B compiles and runs clean (the `RowCarrier` trait +
// `Small`/`Wide`/`Odd` impls + `compute_waists_via_carrier` above; `cargo run
// --release` exits 0). `R::AdjRow` is concrete at each impl so the container
// projection resolves; `BitMatrix<R::AdjRow, R::Units>` and
// `waist_detect::<R::Units, R::AdjRow>` typecheck for 64-, 128-, and 256-wide
// rows over Dim<64>/Dim<100>/Dim<256>.
//
// NOTE on option A: arvo cannot map `Dim<N>` -> `Bits<{N}>` generically either,
// because `impl<const N: usize> Capacity for Dim<N>` is itself generic over N,
// so `type AdjRow = Bits<{N as u16}>` would hit the SAME symbolic-N projection
// wall in the impl. A truly capacity-generic row needs an array-of-words row
// representation (Sized for any symbolic N) in arvo-bitmask, a larger separate
// substrate effort. So the engine-side `PlanDims::AdjRow` (option B) is the
// realistic fix: each concrete PlanDims impl names a concrete row word covering
// its Units, matching how arvo's own generic_width tests hardcode concrete
// widths. The cap is lifted onto D (D's author picks AdjRow); the
// AdjRow-covers-Units relation is a documented PlanDims contract, not type-
// enforced, which is forced by the toolchain and consistent with arvo's pattern.

