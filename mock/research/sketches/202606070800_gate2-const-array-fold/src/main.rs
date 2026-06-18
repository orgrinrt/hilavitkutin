//! GATE-2 mechanism sketch 3: the const-array grouping fold (gap b).
//!
//! Hypothesis: the per-unit mask arrays + grouping can be built as a const,
//! recursing the bundle cons-list, in a form the const-gated DCE walk (sketch
//! 071230) indexes, WITHOUT the generic-length-array path.
//!
//! Stage 1 (recorded outcome below) tried a generic `[u64; N]` where N = the
//! cons-list length via generic_const_exprs. It FAILED: it needs the separate
//! experimental `generic_const_items` feature AND hits E0308 (rustc will not
//! normalize the associated `Self::N` against the impl's `T::N + 1`). Wrong path.
//!
//! Stage 2 (this file): the engine never builds `[u64; N]` from a recursive
//! length; `PlanInputs` stores masks in `<CU as Capacity>::Array<AccessMask>`, a
//! FIXED-capacity array, filling the first `unit_count` slots. Mirror that: a
//! fixed `CAP` (concrete const, the unit capacity) and a const-fn fold writing
//! each unit's mask by index. CAP is concrete, so the array type is not
//! generic-length: no generic_const_items, no Self::N normalization. The grouping
//! is an associated `const [u64; CAP]` (CAP fixed), const-evaluable, indexed by
//! the const-gated walk. This is the engine-relevant shape (swap the fixed array
//! for `Capacity::Array`, already proven to compile in PlanInputs).
//!
//! Q: does the const-fn fold + associated `const GROUPING: [u64; CAP]` +
//!    `const { trunk_of::<B>(POS) == TRUNK }` gate compile and DCE?

#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

// Cons-list (mirrors hilavitkutin-api access.rs shape).
struct Empty;
struct Cons<H, T>(PhantomData<(H, T)>);

// Fixed unit capacity. Stands in for `cap_size(<D::Units as Capacity>::CAP)`,
// a concrete const per engine monomorphisation. The engine uses
// `Capacity::Array<T>` for the actual storage; a plain `[_; CAP]` models it here.
const CAP: usize = 8;

// Per-unit mask. In the engine this is the MaskProject/Locate/WitnessIndex const
// mask (gap a, mechanical, both pieces already const). Hardcoded here to isolate
// the fold.
trait UnitMask {
    const READ: u64;
    const WRITE: u64;
}

struct U0;
struct U1;
struct U2;
impl UnitMask for U0 {
    const READ: u64 = 1 << 0;
    const WRITE: u64 = 1 << 1;
}
impl UnitMask for U1 {
    const READ: u64 = 1 << 2;
    const WRITE: u64 = 1 << 3;
}
impl UnitMask for U2 {
    const READ: u64 = 1 << 1; // reads U0's output col1 -> RAW edge U0->U2
    const WRITE: u64 = 1 << 4;
}

// Const-fn fold writing each unit's masks at its carrier position into a fixed
// `[_; CAP]`. Mirrors BundleProject::project_bundle but const and over a fixed
// array. Returns the next index (so the caller learns unit_count).
const trait BundleFill {
    fn fill(reads: &mut [u64; CAP], writes: &mut [u64; CAP], idx: usize) -> usize;
}

impl const BundleFill for Empty {
    fn fill(_reads: &mut [u64; CAP], _writes: &mut [u64; CAP], idx: usize) -> usize {
        idx
    }
}

impl<H: UnitMask, T: [const] BundleFill> const BundleFill for Cons<H, T> {
    fn fill(reads: &mut [u64; CAP], writes: &mut [u64; CAP], idx: usize) -> usize {
        reads[idx] = H::READ;
        writes[idx] = H::WRITE;
        T::fill(reads, writes, idx + 1)
    }
}

// Const grouping: union-find over shared columns (any access overlap joins).
const fn compute_trunks(reads: [u64; CAP], writes: [u64; CAP], n: usize) -> [u64; CAP] {
    let mut parent = [0usize; CAP];
    let mut i = 0;
    while i < CAP {
        parent[i] = i;
        i += 1;
    }
    let mut a = 0;
    while a < n {
        let mut b = a + 1;
        while b < n {
            let acc_a = reads[a] | writes[a];
            let acc_b = reads[b] | writes[b];
            if (acc_a & acc_b) != 0 {
                let mut ra = a;
                while parent[ra] != ra {
                    ra = parent[ra];
                }
                let mut rb = b;
                while parent[rb] != rb {
                    rb = parent[rb];
                }
                if ra != rb {
                    parent[rb] = ra;
                }
            }
            b += 1;
        }
        a += 1;
    }
    let mut trunk = [0u64; CAP];
    let mut k = 0;
    while k < CAP {
        let mut r = k;
        while parent[r] != r {
            r = parent[r];
        }
        trunk[k] = r as u64;
        k += 1;
    }
    trunk
}

// Grouping as an associated `const [u64; CAP]` (CAP fixed: a normal associated
// const, no generic_const_items). const-evaluable, so the gate can DCE on it.
trait Grouped {
    const N: usize;
    const GROUPING: [u64; CAP];
}

impl<B: const BundleFill> Grouped for B {
    const N: usize = {
        let mut r = [0u64; CAP];
        let mut w = [0u64; CAP];
        B::fill(&mut r, &mut w, 0)
    };
    const GROUPING: [u64; CAP] = {
        let mut r = [0u64; CAP];
        let mut w = [0u64; CAP];
        let n = B::fill(&mut r, &mut w, 0);
        compute_trunks(r, w, n)
    };
}

// Const fn picking a unit's trunk (indexing lives in a const fn, not a const{}).
const fn trunk_of<B: Grouped>(pos: usize) -> u64 {
    <B as Grouped>::GROUPING[pos]
}

// The const-gated walk (DCE shape from sketch 071230): a per-TRUNK monomorphised
// walk over the bundle; non-member positions fold away. No-self associated fns so
// no cons-cell construction is needed.
trait UnitRun {
    fn run(acc: &mut u64);
}
impl UnitRun for U0 {
    fn run(acc: &mut u64) {
        *acc = acc.wrapping_mul(0x9e37_79b9);
    }
}
impl UnitRun for U1 {
    fn run(acc: &mut u64) {
        *acc = acc.wrapping_add(0x85eb_ca6b);
    }
}
impl UnitRun for U2 {
    fn run(acc: &mut u64) {
        *acc = *acc ^ (*acc >> 13);
    }
}

trait RunTrunkSel<B, const POS: usize, const TRUNK: u64> {
    fn run(acc: &mut u64);
}
impl<B, const POS: usize, const TRUNK: u64> RunTrunkSel<B, POS, TRUNK> for Empty {
    fn run(_acc: &mut u64) {}
}
impl<B: Grouped, H: UnitRun, T, const POS: usize, const TRUNK: u64> RunTrunkSel<B, POS, TRUNK>
    for Cons<H, T>
where
    T: RunTrunkSel<B, { POS + 1 }, TRUNK>,
{
    fn run(acc: &mut u64) {
        if const { trunk_of::<B>(POS) == TRUNK } {
            H::run(acc);
        }
        <T as RunTrunkSel<B, { POS + 1 }, TRUNK>>::run(acc);
    }
}

#[inline(never)]
fn run_one_trunk<B: Grouped, const TRUNK: u64>(acc: &mut u64)
where
    B: RunTrunkSel<B, 0, TRUNK>,
{
    <B as RunTrunkSel<B, 0, TRUNK>>::run(acc);
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn gate_trunk0(acc: &mut u64) {
    run_one_trunk::<Cons<U0, Cons<U1, Cons<U2, Empty>>>, 0>(acc);
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn gate_trunk1(acc: &mut u64) {
    run_one_trunk::<Cons<U0, Cons<U1, Cons<U2, Empty>>>, 1>(acc);
}

fn main() {
    type Bundle = Cons<U0, Cons<U1, Cons<U2, Empty>>>;

    // Q (fold): the const fold produced the right unit count + grouping.
    assert_eq!(<Bundle as Grouped>::N, 3);
    let grouping = <Bundle as Grouped>::GROUPING;
    // U0,U2 share col1 -> same trunk; U1 distinct.
    assert_eq!(grouping[0], grouping[2]);
    assert_ne!(grouping[0], grouping[1]);

    // Q (gate): const gate over the generically-built grouping.
    const T0: u64 = trunk_of::<Bundle>(0);
    const T1: u64 = trunk_of::<Bundle>(1);
    const T2: u64 = trunk_of::<Bundle>(2);
    assert_eq!(T0, T2);
    assert_ne!(T0, T1);

    // run the two trunks; each mono should DCE to its members.
    let mut acc_t0 = 0xdead_beefu64;
    run_one_trunk::<Bundle, 0>(&mut acc_t0); // trunk 0 = {U0, U2}
    let mut acc_t1 = 0xdead_beefu64;
    run_one_trunk::<Bundle, 1>(&mut acc_t1); // trunk 1 = {U1}

    println!(
        "WORKS: N={}, GROUPING(first 3)={:?}, T0={} T1={} T2={}, acc_t0={:#x} acc_t1={:#x}",
        <Bundle as Grouped>::N,
        &grouping[..3],
        T0,
        T1,
        T2,
        acc_t0,
        acc_t1
    );
}

// ---------------------------------------------------------------------------
// OUTCOME (2026-06-07): WORKS.
//
// Stage 1 (generic `[u64; N]`, N = cons-list length, via generic_const_exprs):
//   FAILED. Needs the separate experimental `generic_const_items` feature AND
//   hits E0308 (rustc will not normalize associated `Self::N` against the impl's
//   `T::N + 1`). Wrong path; do not pursue generic-length mask arrays.
//
// Stage 2 (this file, fixed `CAP` array + const-fn fold): WORKS.
//   - The const-fn fold (`const trait BundleFill`, `impl const`) recurses the
//     bundle cons-list and fills a fixed `[u64; CAP]`, returning the unit count.
//     N=3 derived; GROUPING=[0,1,0] (U0,U2 share col1 -> trunk 0; U1 -> trunk 1).
//   - The grouping is an associated `const GROUPING: [u64; CAP]` (CAP concrete,
//     NOT generic-length: a plain associated const, no generic_const_items).
//   - The gate `const { trunk_of::<B>(POS) == TRUNK }` const-evaluates the
//     grouping and DCEs the walk. objdump of the two `run_one_trunk` monos:
//       TRUNK0 = movk #0x9e37 + mul (U0) and eor x8,x8,x8,lsr #13 (U2), ret;
//               NO U1 add #0x85eb.
//       TRUNK1 = movk #0x85eb (U1), ret; NO U0 mul, NO U2 eor.
//     Member-only per-trunk programs, zero blr. Same DCE as sketch 071230, now
//     driven by the const-FOLD grouping rather than a hardcoded literal.
//
// Features: const_trait_impl + generic_const_exprs (both WATCH-allowed). NO
// generic_const_items, NO proc-macro, NO build.rs, NO LLVM pass.
//
// Engine mapping (gap b CLOSED, gap a mechanical):
//   - Replace fixed `[u64; CAP]` with `<CU as Capacity>::Array<...>` (the engine's
//     existing fixed-capacity generic array; PlanInputs uses it, already compiles).
//     CAP = cap_size(<D::Units as Capacity>::CAP).
//   - Replace `UnitMask::{READ,WRITE}` with the engine's existing `MaskProject`
//     const trait + `Locate`/`WitnessIndex` (already const) for each unit's mask
//     over the global Stores numbering (gap a), threaded per cons cell in the fold.
//   - `compute_trunks` here is union-find; the real grouping is read-after-write
//     phases + within-phase column-disjoint trunks (sketch 071330 const fns), all
//     const-fn over the mask array.
//   - `BundleFill` maps to a const version of the existing `BundleProject` walk.
// ---------------------------------------------------------------------------
