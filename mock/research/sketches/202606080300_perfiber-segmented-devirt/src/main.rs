//! Sketch (Gate-1 keystone): does a PER-FIBER-SEGMENTED dispatch devirtualise?
//!
//! Settled so far: a FLAT dispatch over a foldable order devirtualises (sketch
//! 202606071400, zero `blr`). But the canonical Gate-1 per-core program (spec
//! domain 17 :1596-1613, :1540-1586) is NOT one flat morsel-outer loop: the
//! flattener emits PER FIBER, choosing the fiber's execution strategy from its
//! properties. An accumulator-free column fiber runs MORSEL-OUTER (one morsel
//! runs its whole WU sequence before the next, cache-resident intermediates); an
//! accumulator fiber runs UNIT-OUTER (each WU completes its record range before
//! the next, cross-record-safe). The pinned test `fiber_structured_dispatch`
//! requires exactly this per-fiber distinction. A single flat morsel-outer loop
//! CANNOT express "this segment unit-outer, that one morsel-outer".
//!
//! THE OPEN QUESTION (the emit bridge): the fiber GROUPING is a runtime plan
//! computation (`group_fibers`, a graph algorithm), and D1b (sketch 202606061400)
//! proved the grouping is NOT derivable at the trait-solver level (coherence
//! wall). So can a per-fiber-SEGMENTED dispatch, whose fiber boundaries +
//! per-fiber morsel-local bits come from a FOLDABLE fn over the const access
//! masks (the same mechanism `carrier_order_dyn` uses for the order), still
//! devirtualise? I.e. does LLVM fold the foldable `group_fibers` + `topo`, unroll
//! the per-fiber and per-WU loops, and turn each `slots[order[k]]` into a direct
//! inlined call, EVEN with a real runtime morsel loop wrapping AND a mix of
//! morsel-outer / unit-outer per-fiber loop nests?
//!
//! If YES: the flattener emits the per-core body as a sequence of per-fiber
//! loop-nests with foldable bounds; per-fiber morsel-locality AND devirt both
//! hold; Gate-1 `run` implements against `carrier_order_dyn` + a foldable
//! `carrier_fibers_dyn`. If NO (the per-fiber/segment loop bounds defeat the
//! unroll, leaving an indirect `blr` gather): the foldable-fn bridge is
//! insufficient and a type-level `FiberCons` carrier (or a Step-11 escalation)
//! is required.
//!
//! Mirrors the proven 202606071400 shape (foldable order over const masks ->
//! local fn-ptr dispatch) but adds (1) a real runtime morsel loop, (2) a foldable
//! per-fiber grouping, (3) the morsel-outer / unit-outer per-fiber branch.
//! Outcome at the bottom.

#![allow(dead_code)]

use core::hint::black_box;
use core::sync::atomic::{AtomicU64, Ordering};

// Trace: each WU records its tag once per (WU, morsel) dispatch, so the observed
// sequence proves the per-fiber locality (column fiber interleaves P,C per morsel;
// accumulator fiber is A,A,A contiguous).
static TRACE: AtomicU64 = AtomicU64::new(0);
static TLEN: AtomicU64 = AtomicU64::new(0);
fn record(tag: u64) {
    let i = TLEN.fetch_add(1, Ordering::Relaxed);
    if i < 16 {
        let prev = TRACE.load(Ordering::Relaxed);
        TRACE.store(prev | (tag << (i * 4)), Ordering::Relaxed);
    }
}

const S0: u64 = 1 << 0; // input col
const S1: u64 = 1 << 1; // P's output col (C reads it) -> column chain edge P->C
const S2: u64 = 1 << 2; // accumulator store (A writes)

// Carrier in builder PREPEND order: register P, C, A -> carrier [A, C, P].
// Index 0 = A (accumulator), 1 = C (consumer), 2 = P (producer).
// Edges: P writes S1, C reads S1 => P->C. A is independent (writes S2, reads S0
// disjoint from the chain). So topo over the carrier must put P (idx 2) before
// C (idx 1); A (idx 0) is independent.
const N: usize = 3;
const READS: [u64; N] = [S0, S1, S0]; // A reads S0, C reads S1, P reads S0
const WRITES: [u64; N] = [S2, 0, S1]; // A writes S2(accum), C writes nothing, P writes S1
// Which carrier slots write an accumulator (=> their fiber is unit-outer).
const IS_ACCUM: [bool; N] = [true, false, false]; // A, C, P

#[inline(always)]
fn run_a(acc: &mut u64, _m: usize) {
    record(0xA);
    *acc = acc.wrapping_add(0x1234567);
}
#[inline(always)]
fn run_c(acc: &mut u64, _m: usize) {
    record(0xC);
    *acc = acc.rotate_left(7) ^ 0x9e3779b9;
}
const P_TAG: u64 = 0xB; // distinct trace tag for P
#[inline(always)]
fn run_p(acc: &mut u64, _m: usize) {
    record(P_TAG);
    *acc = acc.wrapping_mul(2654435761).wrapping_add(1);
}

type WuFn = fn(&mut u64, usize);

// FOLDABLE topological sort over the const masks (edge i->j iff writes[i] & reads[j]).
// Lowest-index tie-break. Same shape as carrier_order_dyn / the proven 202606071400.
#[inline]
fn topo(reads: [u64; N], writes: [u64; N]) -> [usize; N] {
    let mut indeg = [0usize; N];
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            if i != j && (writes[i] & reads[j]) != 0 {
                indeg[j] += 1;
            }
            j += 1;
        }
        i += 1;
    }
    let mut order = [0usize; N];
    let mut done = [false; N];
    let mut out = 0;
    while out < N {
        let mut pick = N;
        let mut k = 0;
        while k < N {
            if !done[k] && indeg[k] == 0 {
                pick = k;
                break;
            }
            k += 1;
        }
        if pick == N {
            break;
        }
        done[pick] = true;
        order[out] = pick;
        out += 1;
        let mut j = 0;
        while j < N {
            if j != pick && !done[j] && (writes[pick] & reads[j]) != 0 {
                indeg[j] -= 1;
            }
            j += 1;
        }
    }
    order
}

// FOLDABLE fiber grouping over the order: walk the topo order, roll a new fiber
// when the dependency structure breaks the chain (here: an accumulator WU, or a
// WU that does not consume the previous WU's output, starts a new fiber). This
// is a stand-in for the shipped `group_fibers` out-degree rule, kept pure +
// foldable. Returns per-fiber (start, len, morsel_local) and the fiber count.
#[inline]
fn group_fibers(
    order: [usize; N],
    reads: [u64; N],
    writes: [u64; N],
    is_accum: [bool; N],
) -> ([usize; N], [usize; N], [bool; N], usize) {
    let mut fstart = [0usize; N];
    let mut flen = [0usize; N];
    let mut flocal = [false; N];
    let mut fc = 0;
    let mut k = 0;
    while k < N {
        // Start a new fiber at k.
        let u = order[k];
        fstart[fc] = k;
        let mut len = 1;
        let mut fiber_accum = is_accum[u];
        let mut prev_writes = writes[u];
        // Extend the fiber while the next WU consumes the running output AND
        // neither this nor the next WU is an accumulator (accumulator WUs are
        // singleton unit-outer fibers, per the cross-record contract).
        let mut nk = k + 1;
        while nk < N {
            let nu = order[nk];
            let chains = (prev_writes & reads[nu]) != 0;
            if chains && !fiber_accum && !is_accum[nu] {
                len += 1;
                prev_writes |= writes[nu];
                nk += 1;
            } else {
                break;
            }
        }
        flen[fc] = len;
        flocal[fc] = !fiber_accum; // accumulator-free fiber -> morsel-outer
        fc += 1;
        k += len;
    }
    (fstart, flen, flocal, fc)
}

// THE PER-CORE BODY: a sequence of per-fiber loop-nests. Foldable order + foldable
// grouping; a real runtime morsel loop (`morsels` is black_box'd so it cannot
// const-fold, modelling the plan's runtime record count). Morsel-outer for a
// morsel-local fiber, unit-outer for an accumulator fiber. Isolated for objdump.
#[inline(never)]
fn dispatch_per_fiber(acc: &mut u64, morsels: usize) {
    let order = topo(READS, WRITES);
    let (fstart, flen, flocal, fc) = group_fibers(order, READS, WRITES, IS_ACCUM);
    let slots: [WuFn; N] = [run_a, run_c, run_p];
    let mut fi = 0;
    while fi < fc {
        let start = fstart[fi];
        let end = start + flen[fi];
        if flocal[fi] {
            // morsel-outer: one morsel runs the fiber's whole WU sequence.
            let mut m = 0;
            while m < morsels {
                let mut k = start;
                while k < end {
                    slots[order[k]](acc, m);
                    k += 1;
                }
                m += 1;
            }
        } else {
            // unit-outer: each WU completes its record range (all morsels) first.
            let mut k = start;
            while k < end {
                let mut m = 0;
                while m < morsels {
                    slots[order[k]](acc, m);
                    m += 1;
                }
                k += 1;
            }
        }
        fi += 1;
    }
}

fn main() {
    TRACE.store(0, Ordering::Relaxed);
    TLEN.store(0, Ordering::Relaxed);
    let mut acc = black_box(1u64);
    let morsels = black_box(3usize); // runtime record/morsel count (cannot const-fold)
    dispatch_per_fiber(&mut acc, morsels);
    black_box(acc);
    let trace = TRACE.load(Ordering::Relaxed);
    let tlen = TLEN.load(Ordering::Relaxed);
    // Expected: fiber order is [P,C fiber] then [A fiber] OR [A fiber] then [P,C
    // fiber] (lowest-index tie-break: A is carrier idx 0, so topo order = [A, P, C]
    // -> group: A is accumulator (singleton unit-outer fiber) first, then P->C
    // morsel-local fiber). So dispatch: A,A,A (unit-outer, 3 morsels) then per
    // morsel P,C: P,C,P,C,P,C. Trace (nibbles, first=low): A A A P C P C P C.
    // tag A=0xA, P=0xB, C=0xC.
    println!(
        "tlen={tlen} trace={trace:#x} (expect 9 dispatches: A,A,A then P,C x3). \
         Now objdump dispatch_per_fiber for blr to decide devirtualisation."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: FAILS WITH 2 `blr` (nightly-2026-05-28, release fat-LTO cgu=1).
//
// Correctness HOLDS: trace 0xcbcbcbaaa = A,A,A (accumulator fiber, unit-outer, 3
// morsels) then P,C,P,C,P,C (column fiber, morsel-outer) -> per-fiber locality is
// expressed correctly by the sequence-of-per-fiber-loop-nests body.
//
// Devirt FAILS: `dispatch_per_fiber` objdumps to 1 `bl` + 2 `blr`. The single-WU
// accumulator fiber (len 1, unit-outer) unrolled to a DIRECT call (`bl run_a`).
// The 2-WU column fiber (morsel-outer) did NOT: LLVM held its two fn pointers in
// registers (x22 = run_p, x23 = run_c) and dispatched them INDIRECTLY (`blr x22`,
// `blr x23`) once per morsel. The foldable order + foldable grouping fold fine
// (no runtime index math survives), but the `slots[order[k]]` FN-POINTER-ARRAY
// dispatch under the runtime morsel loop does NOT collapse to direct calls when a
// fiber has >1 WU: the inner WU loop is not unrolled inside the runtime morsel
// loop, so the array index stays a runtime gather = indirect.
//
// WHY 202606071400 (flat) worked but this does not: that sketch had NO runtime
// morsel loop around the dispatch, so the WHOLE walk fully unrolled and the array
// indexing constant-folded away. Here the real runtime morsel loop (the plan's
// record count, which the engine MUST have) keeps the multi-WU fiber's array
// dispatch rolled, and a rolled fn-pointer-array index is an indirect call. This
// is exactly the spec's 5.8x `&[fn;N]` / struct-field anti-pattern family
// (domain 17 :1507-1545): a fn-pointer slice indexed in a loop does not
// devirtualise.
//
// WHAT THIS SETTLES (the bridge mechanism): the foldable-order + fn-pointer-array
// approach (the `carrier_order_dyn` + `FiberSlot` direction) is NOT the per-fiber
// devirt vehicle. The PROVEN devirt vehicle is the TYPE-LEVEL cons-list walk
// (D2/202606051601/202606060730: `head.execute(&ctx)` recursion over concrete
// `WuCons<W, Tail>` types, each a direct/inlined call regardless of any wrapping
// runtime loop, because the WU TYPE is statically known at each recursion step).
// A type-level walk needs the fiber's WU sequence to be a compile-time TYPE
// (the `FiberCons<WuCons<...>, ...>` carrier, D1b Tier 1, proven zero `blr` when
// hand-written). So per-fiber devirt REQUIRES a compile-time fiber-membership
// carrier; the runtime grouping cannot drive a fn-pointer-array dispatch and get
// devirt.
//
// THE NARROWED OPEN QUESTION (next sketch / op call): how is the compile-time
// `FiberCons` carrier CONSTRUCTED? D1b proved it cannot be type-level-DERIVED from
// the access sets (coherence wall). It cannot be built from a runtime value (Rust
// types are static). So the fiber membership must be a compile-time fact of the
// REGISTRATION structure. Candidates (the next thing to settle, touches the
// consumer-facing builder API = an irreversible-ish design call): (1) the consumer
// declares fibers explicitly in the builder (`.fiber((WuA, WuB)).fiber((WuC,))`);
// (2) a `Kit`/macro emits the `FiberCons` grouping; (3) the builder groups by a
// compile-time-expressible rule (store-disjointness via the proven `SharesStore`
// #[marker], which D1b proved resolves POSITIVE even though the full partition
// fails) into contiguous runs. group_fibers stays as the runtime validator +
// morsel-sizer. This is the Gate-1 keystone's true remaining piece (#669); it is
// NOT the foldable-array shape this sketch ruled out.
// ---------------------------------------------------------------------
