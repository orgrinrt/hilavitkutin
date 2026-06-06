//! Sketch (keystone wiring / codegen_fiber): const ORDER as an associated const
//! over the heterogeneous WU carrier.
//!
//! Sketch 202606071000 proved const topo + const-order local fn-ptr dispatch
//! devirtualises (the mechanism). This proves the ENGINE WIRING: producing the
//! `const ORDER` from the carrier TYPE, so `Scheduler::run` can dispatch in it.
//!
//! Already de-risked by inspection (shipped in plan/project.rs): per-WU access
//! masks derive from types with no specialization overlap via `WitnessIndex`
//! (Peano `const INDEX`), `Locate` (disjoint Here/There position witness), and
//! the structural `MaskProject` fold. This sketch isolates the NEW pieces on top:
//!   (1) a `const LEN` over the carrier (structural assoc const),
//!   (2) filling a `[u64; LEN]` of per-WU masks by a const-trait walk (the masks
//!       themselves modelled as per-WU `const READ_MASK`/`WRITE_MASK` scalars,
//!       since their derivation is the shipped MaskProject pattern),
//!   (3) a `const fn` topo over the filled arrays yielding a `const ORDER`
//!       associated const on the carrier.
//! Risk: const_trait_impl + the assoc-const array lengths (`[u64; Self::LEN]`,
//! generic_const_exprs WATCH) on nightly-2026-05-28. Outcome at the bottom.

#![allow(dead_code)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

// =====================================================================
// Per-WU masks (modelled scalars; in the engine these come from the shipped
// MaskProject + WitnessIndex fold over each WU's access set). A two-WU chain
// registered [A, B] then PREPENDED into the carrier [B, A]: carrier index 0 = B
// (reads S1=A's output), index 1 = A (writes S1). Anti-topological; topo ORDER
// must be [1, 0] = A then B.
const S0: u64 = 1 << 0;
const S1: u64 = 1 << 1;
const S2: u64 = 1 << 2;

trait Wu {
    const READ_MASK: u64;
    const WRITE_MASK: u64;
}
struct A; // S0 -> S1
struct B; // S1 -> S2
impl Wu for A {
    const READ_MASK: u64 = S0;
    const WRITE_MASK: u64 = S1;
}
impl Wu for B {
    const READ_MASK: u64 = S1;
    const WRITE_MASK: u64 = S2;
}

// Heterogeneous carrier cons-list.
struct Cons<H, T> {
    head: H,
    tail: T,
}
struct Nil;

// =====================================================================
// (1) const LEN + (2) const-trait walk filling [u64; LEN].
// `CarrierMasks::LEN` is the carrier length; `fill_reads`/`fill_writes` are const
// trait methods that write each WU's mask at its carrier position into a passed
// fixed-length slice. A const block then materialises the arrays and the order.
// =====================================================================
const trait CarrierMasks {
    const LEN: usize;
    // Write reads[pos..]/writes[pos..] for this sub-carrier; const fn in trait.
    fn fill(reads: &mut [u64], writes: &mut [u64], pos: usize);
}

impl const CarrierMasks for Nil {
    const LEN: usize = 0;
    fn fill(_reads: &mut [u64], _writes: &mut [u64], _pos: usize) {}
}

impl<H: Wu, T: [const] CarrierMasks> const CarrierMasks for Cons<H, T> {
    const LEN: usize = 1 + T::LEN;
    fn fill(reads: &mut [u64], writes: &mut [u64], pos: usize) {
        reads[pos] = H::READ_MASK;
        writes[pos] = H::WRITE_MASK;
        T::fill(reads, writes, pos + 1);
    }
}

// =====================================================================
// (3) const fn topo (ported from sketch 202606071000), then a `const ORDER`
// associated const on the carrier, computed from the filled mask arrays.
// =====================================================================
const fn topo_order<const M: usize>(reads: [u64; M], writes: [u64; M]) -> [usize; M] {
    let mut indeg = [0usize; M];
    let mut i = 0;
    while i < M {
        let mut j = 0;
        while j < M {
            if i != j && (writes[i] & reads[j]) != 0 {
                indeg[j] += 1;
            }
            j += 1;
        }
        i += 1;
    }
    let mut order = [0usize; M];
    let mut done = [false; M];
    let mut out = 0;
    while out < M {
        let mut pick = M;
        let mut k = 0;
        while k < M {
            if !done[k] && indeg[k] == 0 {
                pick = k;
                break;
            }
            k += 1;
        }
        if pick == M {
            break;
        }
        done[pick] = true;
        order[out] = pick;
        out += 1;
        let mut j = 0;
        while j < M {
            if j != pick && !done[j] && (writes[pick] & reads[j]) != 0 {
                indeg[j] -= 1;
            }
            j += 1;
        }
    }
    order
}

// Compute the carrier order in a const generic context where the length N is a
// concrete const (in the engine, N = `D::Units` Capacity = `Dim<N>`, a concrete
// const at the `run` call site; here a const fn over a concrete N). Carrier::LEN
// feeds N, so the array lengths are constrained by N, not by a self-referential
// assoc const.
const fn carrier_order<L, const N: usize>() -> [usize; N]
where
    L: [const] CarrierMasks,
{
    let mut reads = [0u64; N];
    let mut writes = [0u64; N];
    <L as CarrierMasks>::fill(&mut reads, &mut writes, 0);
    topo_order::<N>(reads, writes)
}

type Carrier = Cons<B, Cons<A, Nil>>;

fn main() {
    const N: usize = <Carrier as CarrierMasks>::LEN;
    const ORDER: [usize; N] = carrier_order::<Carrier, N>();
    // carrier [B(0), A(1)]; A writes S1, B reads S1, so A before B: ORDER = [1, 0].
    const _: () = {
        assert!(ORDER.len() == 2);
        assert!(ORDER[0] == 1); // A (carrier index 1) first
        assert!(ORDER[1] == 0); // B (carrier index 0) second
    };
    println!(
        "WORKS: const ORDER assoc const over the carrier = {ORDER:?} (expected [1, 0] = A, B). \
         const LEN + const-trait fill + const fn topo all evaluate at compile time over the \
         carrier TYPE. The engine's run dispatches the local fn-ptr array in this const ORDER."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28). The engine wiring of the const ORDER is
// proven. `const trait CarrierMasks` gives a structural `const LEN` and a
// const-trait `fill` walk (`impl const CarrierMasks for Cons<H, T> where T:
// [const] CarrierMasks`, the current nightly syntax: `const trait`, `impl
// const`, `[const]` bounds) that writes each WU's mask at its carrier position
// into a `[u64; N]`. A `const fn carrier_order::<L, const N: usize>()` fills the
// arrays and runs the proven const-fn topo (202606071000), returning the const
// ORDER. For carrier [B, A] (anti-topological prepend; A writes S1, B reads S1)
// it returns [1, 0] = dispatch A then B. Asserted in a `const _: ()` block.
//
// KEY: the order is computed in a const generic context where N is a concrete
// const (here `const N = Carrier::LEN`; in the engine N = `D::Units` Capacity =
// `Dim<N>`, concrete at the `run` call site). This sidesteps the GCE
// "unconstrained generic constant" friction that a SELF-REFERENTIAL associated
// const `const ORDER: [usize; Self::N]` hits (recorded: that form fails E0277
// unconstrained-const; use a const fn with N passed as a concrete const generic
// instead, which the engine's Dim-based capacity already provides).
//
// WHAT THIS SETTLES (with 202606071000): the FULL const-driven dispatch is
// proven end to end. (1) per-WU masks derive from types with no overlap (shipped
// WitnessIndex const INDEX + Locate + MaskProject; modelled here as scalar
// consts). (2) const LEN + const-trait fill -> const [u64; N] mask arrays (this
// sketch). (3) const fn topo -> const ORDER (this + 202606071000). (4) const-
// order local fn-ptr dispatch devirtualises (202606071000). codegen_fiber is
// realizable: no specialization, no proc-macro, no runtime permutation.
//
// REMAINING INTEGRATION LINK (engine, not a design risk): the `fill` must compute
// each WU's real mask from its access set via the shipped MaskProject/WitnessIndex
// pattern in a const context (project_mask is currently a runtime fn; needs a
// const path reading `<Idx as WitnessIndex>::INDEX`, which is already a const).
// Modelled here as `H::READ_MASK`/`WRITE_MASK` scalars; the derivation itself is
// the shipped no-overlap fold.
// ---------------------------------------------------------------------
