//! Sketch (keystone / #340 Phase D, codegen_fiber): const-driven carrier ordering.
//!
//! The dispatch keystone wall (dual-agent consensus, `mock/research/202606070030`):
//! the builder PREPENDS, so the WU cons-list carrier is in reverse registration
//! order, which is ANTI-topological (basic producer->consumer dispatches the
//! consumer first, reading uninitialised data). The dispatch order must be a
//! COMPILE-TIME fact in TOPOLOGICAL order, but the type-level grouping/sort fold
//! needs forbidden `specialization` (D1b tier3 E0119), a proc-macro cannot see
//! `<W as WorkUnit>::Read/Write` associated types, and a RUNTIME-permuted local
//! fn-pointer slice does not devirtualise. The consensus: `codegen_fiber` must
//! assemble the carrier in topo order at build time from the access matrix.
//!
//! This sketch proves the proposed mechanism, isolated from the engine to test
//! the rustc/const-eval/const-generic capabilities directly:
//!   Tier 0: a `const fn` computes the topological order from const access masks
//!           (Kahn's). The masks are gathered from per-WU const associated
//!           constants into a const array (faithful to AccessSet exposing a const
//!           mask). LOW risk; the load-bearing question is whether the whole
//!           topo computation runs in a const context on the pinned nightly.
//!   Tier A: the domain-17 "local &[WuFn] slice" (Approach A, 1.0x). Build a
//!           stack-local fn-pointer array (contents known from the carrier) and
//!           dispatch in the CONST topo order. The open question the consensus
//!           flagged: a RUNTIME-permuted local slice does not devirtualise; does
//!           a CONST-permuted one (order is a `const [usize; N]`, dispatch loop
//!           const-unrollable) devirtualise to direct calls? objdump the isolated
//!           symbol for `blr`.
//!   Tier B: type-level `Nth<const K>` accessor + const recursion to dispatch the
//!           heterogeneous cons-list in topo order, fully inlined (the D4 fusion
//!           shape). Attempt; the risk is the DispatchNth<0>-vs-DispatchNth<K>
//!           impl overlap reintroducing the specialization wall, and `{K-1}` /
//!           `{ORDER[K]}` needing generic_const_exprs.
//!
//! Hypothesis: Tier 0 WORKS (const topo). Tier A devirtualises with a const order
//! (the canonical Approach A mechanism for codegen_fiber). Tier B either fully
//! inlines (bonus, the fusion path) or hits the specialization/GCE wall (recorded
//! as the boundary, Tier A still carries the keystone). Outcome at the bottom.

#![allow(dead_code)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::hint::black_box;
use core::sync::atomic::{AtomicU64, Ordering};

// A side-effect sink to verify dispatch ORDER (each WU appends its tag) and to
// keep the dispatch from being optimised away entirely.
static TRACE: AtomicU64 = AtomicU64::new(0);
fn record(tag: u64) {
    // shift-accumulate the dispatch order into a single word: first dispatched
    // unit lands in the high nibble. Verifiable post-run.
    let prev = TRACE.load(Ordering::Relaxed);
    TRACE.store((prev << 4) | tag, Ordering::Relaxed);
}

// =====================================================================
// Model: stores are bit positions; WUs carry const READ/WRITE masks.
// =====================================================================
const S0: u64 = 1 << 0; // producer input
const S1: u64 = 1 << 1; // A's output, B's input
const S2: u64 = 1 << 2; // B's output, C's input
const S3: u64 = 1 << 3; // C's output

trait Wu {
    const READ: u64;
    const WRITE: u64;
    const TAG: u64;
    // The monomorphised dispatch body: a distinct arithmetic op per WU so the
    // disassembly shows real work, plus an order record.
    fn run(&self, acc: &mut u64);
}

// A: S0 -> S1. B: S1 -> S2. C: S2 -> S3. Dependency chain A before B before C.
struct A;
struct B;
struct C;
impl Wu for A {
    const READ: u64 = S0;
    const WRITE: u64 = S1;
    const TAG: u64 = 0xA;
    #[inline(always)]
    fn run(&self, acc: &mut u64) {
        record(Self::TAG);
        *acc = acc.wrapping_mul(2654435761).wrapping_add(1);
    }
}
impl Wu for B {
    const READ: u64 = S1;
    const WRITE: u64 = S2;
    const TAG: u64 = 0xB;
    #[inline(always)]
    fn run(&self, acc: &mut u64) {
        record(Self::TAG);
        *acc = acc.rotate_left(7) ^ 0x9e3779b9;
    }
}
impl Wu for C {
    const READ: u64 = S2;
    const WRITE: u64 = S3;
    const TAG: u64 = 0xC;
    #[inline(always)]
    fn run(&self, acc: &mut u64) {
        record(Self::TAG);
        *acc = acc.wrapping_add(0x1234567);
    }
}

// The carrier as the builder leaves it: PREPEND of registration [A, B, C] gives
// the cons-list [C, B, A]. Carrier index 0 = C, 1 = B, 2 = A. Anti-topological:
// a flat walk runs C (carrier 0) first, reading uninitialised S2.
const N: usize = 3;
// Masks gathered from the per-WU const associated constants, in CARRIER order
// (this models a const-context gather over AccessSet const masks).
const READS: [u64; N] = [C::READ, B::READ, A::READ];
const WRITES: [u64; N] = [C::WRITE, B::WRITE, A::WRITE];

// =====================================================================
// TIER 0: const fn topological order (Kahn's) over the const mask arrays.
// Returns a permutation `order` of carrier indices: `order[k]` is the carrier
// index dispatched at topo step k. Dependency edge i -> j (i before j) iff
// WRITES[i] & READS[j] != 0 (i writes a store j reads), i != j.
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
        // lowest-index not-done with in-degree zero (deterministic tie-break).
        let mut pick = M;
        let mut k = 0;
        while k < M {
            if !done[k] && indeg[k] == 0 {
                pick = k;
                break;
            }
            k += 1;
        }
        // a DAG always has a zero-in-degree node; if not, leave remaining as 0.
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

// The dispatch order computed entirely at compile time. Expected [2, 1, 0]:
// carrier 2 = A first, carrier 1 = B, carrier 0 = C last. Topologically correct.
const ORDER: [usize; N] = topo_order::<N>(READS, WRITES);

// =====================================================================
// TIER A: domain-17 "local &[WuFn] slice" (Approach A). Build a stack-local
// fn-pointer array (contents known from the carrier) and dispatch in the CONST
// topo ORDER. Each fn is a monomorphised per-WU dispatch body.
// =====================================================================
type WuFn = fn(&mut u64);
fn run_a(acc: &mut u64) {
    A.run(acc);
}
fn run_b(acc: &mut u64) {
    B.run(acc);
}
fn run_c(acc: &mut u64) {
    C.run(acc);
}

// A1: dispatch via a runtime loop over the CONST ORDER. The loop is small and
// ORDER is const, so LLVM can unroll and constant-fold the index into direct
// calls IF a const-permuted local slice devirtualises.
#[inline(never)]
fn dispatch_tier_a1(acc: &mut u64) {
    // local fn-pointer array, contents known here (the "&[WuFn] with known
    // values" the devirt rules require), in CARRIER order.
    let slots: [WuFn; N] = [run_c, run_b, run_a];
    let mut k = 0;
    while k < N {
        slots[ORDER[k]](acc);
        k += 1;
    }
}

// Au: the sequence codegen_fiber would EMIT: the dispatch calls written out in
// const topo order, indices const-folded. This is the "flattener emits a
// monomorphised function" shape (domain 17): no runtime loop, the calls are in
// topo order at the call site. A code generator (operating on resolved types)
// emits exactly this; here it is hand-written to model the emission.
#[inline(never)]
fn dispatch_tier_au(acc: &mut u64) {
    let slots: [WuFn; N] = [run_c, run_b, run_a];
    slots[ORDER[0]](acc);
    slots[ORDER[1]](acc);
    slots[ORDER[2]](acc);
}

fn main() {
    // Tier 0: the const order is computed at compile time.
    const _: () = {
        assert!(ORDER[0] == 2); // A first (carrier index 2)
        assert!(ORDER[1] == 1); // B second
        assert!(ORDER[2] == 0); // C last
    };
    println!("Tier 0 WORKS: const topo ORDER = {ORDER:?} (expected [2, 1, 0] = A, B, C)");

    // Tier A1: runtime loop over const ORDER.
    TRACE.store(0, Ordering::Relaxed);
    let mut acc = black_box(1u64);
    dispatch_tier_a1(&mut acc);
    black_box(acc);
    let trace_a1 = TRACE.load(Ordering::Relaxed);
    assert_eq!(trace_a1, 0xABC, "Tier A1 dispatch order must be A, B, C (0xABC)");
    println!("Tier A1 WORKS: dispatched in topo order, trace = {trace_a1:#x}");

    // Tier Au: the emitted-unrolled shape (codegen_fiber output model).
    TRACE.store(0, Ordering::Relaxed);
    let mut acc = black_box(1u64);
    dispatch_tier_au(&mut acc);
    black_box(acc);
    let trace_au = TRACE.load(Ordering::Relaxed);
    assert_eq!(trace_au, 0xABC, "Tier Au dispatch order must be A, B, C (0xABC)");
    println!("Tier Au WORKS: emitted-unrolled dispatch in topo order, trace = {trace_au:#x}");

    println!(
        "WORKS: const-driven carrier. A const fn computes the topological dispatch order from \
         const access masks; an anti-topological carrier (prepend [C,B,A]) dispatches in correct \
         topo order A,B,C via the const ORDER. Run objdump on dispatch_tier_a1 / dispatch_tier_au \
         for blr (devirt)."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release fat-LTO cgu=1). The keystone
// dispatch-order mechanism for codegen_fiber is settled.
//
// Tier 0 (const topo): the `const fn topo_order` runs fully in a const context
// (a `const ORDER: [usize; N]` plus a `const _: ()` static_assert on its value),
// computing [2, 1, 0] from the const access masks. A Kahn topological sort over
// const mask arrays is a plain const fn; no GCE-extreme, no risk.
//
// Tier A1 (runtime loop over const ORDER) and Tier Au (emitted unroll): BOTH
// fully devirtualise AND inline. objdump of `dispatch_tier_a1` and
// `dispatch_tier_au`: ZERO `blr` (no indirect calls) AND ZERO `bl` (no calls at
// all) — the three per-WU bodies fold inline IN TOPO ORDER (the disassembly
// shows tag 0xa then 0xb then 0xc with each WU's real arithmetic: madd, then
// eor/ror, then add). The runtime `while k < N { slots[ORDER[k]](acc) }` form
// devirtualises identically to the hand-unrolled form, because ORDER is a
// compile-time const: LLVM unrolls the small loop and constant-folds ORDER[k]
// into the array, turning each indexed indirect call into a direct (then inlined)
// call. The dispatch trace asserts A,B,C order at runtime (0xABC) for both.
//
// WHAT THIS SETTLES (the keystone): codegen_fiber does NOT need a type-level
// grouping/sort fold (forbidden specialization), a proc-macro (cannot see
// AccessSet types), or a runtime permutation into dispatch (does not
// devirtualise). The mechanism is: const access masks (a const associated
// constant per AccessSet) -> `const fn` Kahn topo -> `const ORDER` -> dispatch
// the registration-order carrier as a LOCAL fn-pointer array in const ORDER.
// LLVM devirtualises because the order is a compile-time fact. This is the
// domain-17 "local &[WuFn] slice" (Approach A) realized with a const order. The
// consensus rejected a RUNTIME-permuted slice; a CONST-permuted one is the
// answer and it inlines as well as a direct static-order walk for small bodies.
//
// Tier A2 / Tier B (type-level const-recursion dispatch, removed from this
// final sketch): both FAIL with E0119 (the base `DispatchOrder<N>` impl overlaps
// the step `DispatchOrder<K>` impl; const where-guards do not make impls disjoint
// for coherence) plus GCE recursion blowup on `{ORDER[K]}` / `{K+1}` past N.
// This is the SAME specialization wall D1b tier3 hit. Recorded: the fully
// type-level reorder is not viable; the const-fn + local-fn-ptr-array path is.
//
// WHAT THIS DOES NOT SETTLE (smaller, for implementation): gathering the per-WU
// const access masks FROM the real `WuVals` cons-list into the `const [u64; N]`
// arrays (here hand-listed from `C::READ` etc.). The real AccessSet must expose
// a const mask, and a const fold (a recursive trait with a const fn, or const
// associated constants gathered at the build site where N and the types are
// known) must produce the arrays. This is a const-fold detail, lower risk than
// the dispatch mechanism just proven. Also: full FUSION of larger monomorphised
// dispatch bodies (D4, scratch-backed internal columns) is separate; this proves
// devirtualisation (zero blr, the #664 dispatch half) and, for small bodies,
// full inline.
// ---------------------------------------------------------------------
