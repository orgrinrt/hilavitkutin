//! Sketch (keystone unblock / GCE wall): does a RUNTIME-computed dispatch order
//! still devirtualise?
//!
//! The run rewire hit a GCE wall: a const `[USize; N]` order (`carrier_order`)
//! cannot be evaluated as a const-generic in the generic `Scheduler::run` without
//! E0275 overflow, and the GCE-safe `<C as Capacity>::Array` is not `const`. So
//! the const-order path needs a per-core-program type (codegen_core) to carry the
//! order. BEFORE building that heavier mechanism, settle the cheaper question
//! (bench, not theory): if the order is computed by a NON-const fn (over masks
//! that are themselves compile-time constants) right before the dispatch loop,
//! does LLVM const-fold it at -O3 + fat-LTO and devirtualise the indexed local
//! fn-pointer dispatch anyway?
//!
//! If YES: the engine drops the const requirement, computes the order with a
//! non-const fn over `<C as Capacity>::Array` (GCE-safe, no const-generic N, no
//! Scheduler struct surgery, no codegen_core), and devirt survives. If NO: the
//! const-order-on-a-per-core-program-type path is required.
//!
//! This mirrors the proven sketch 202606071000 (const order -> zero blr) but with
//! the order produced by a RUNTIME call `topo(READS, WRITES)` instead of a `const`.
//! Compare the two disassemblies. Outcome at the bottom.

#![allow(dead_code)]

use core::hint::black_box;
use core::sync::atomic::{AtomicU64, Ordering};

static TRACE: AtomicU64 = AtomicU64::new(0);
fn record(tag: u64) {
    let prev = TRACE.load(Ordering::Relaxed);
    TRACE.store((prev << 4) | tag, Ordering::Relaxed);
}

const S0: u64 = 1 << 0;
const S1: u64 = 1 << 1;
const S2: u64 = 1 << 2;
const S3: u64 = 1 << 3;

// Anti-topological carrier [C, B, A] (prepend of registered A, B, C). A: S0->S1,
// B: S1->S2, C: S2->S3. Topo order must dispatch A (idx 2), B (1), C (0).
const N: usize = 3;
const READS: [u64; N] = [S2, S1, S0]; // C, B, A
const WRITES: [u64; N] = [S3, S2, S1]; // C, B, A

#[inline(always)]
fn run_a(acc: &mut u64) {
    record(0xA);
    *acc = acc.wrapping_mul(2654435761).wrapping_add(1);
}
#[inline(always)]
fn run_b(acc: &mut u64) {
    record(0xB);
    *acc = acc.rotate_left(7) ^ 0x9e3779b9;
}
#[inline(always)]
fn run_c(acc: &mut u64) {
    record(0xC);
    *acc = acc.wrapping_add(0x1234567);
}

type WuFn = fn(&mut u64);

// NON-const Kahn topological sort (the engine would run this over
// `<C as Capacity>::Array` masks; here plain arrays). Kept `#[inline]` so LLVM
// can fold it when its inputs are compile-time constants.
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

// The dispatch: compute the order at RUNTIME from the const masks, then dispatch
// the local fn-pointer slots in that order. Isolated symbol for objdump.
#[inline(never)]
fn dispatch_runtime_order(acc: &mut u64) {
    let order = topo(READS, WRITES);
    let slots: [WuFn; N] = [run_c, run_b, run_a];
    let mut k = 0;
    while k < N {
        slots[order[k]](acc);
        k += 1;
    }
}

fn main() {
    TRACE.store(0, Ordering::Relaxed);
    let mut acc = black_box(1u64);
    dispatch_runtime_order(&mut acc);
    black_box(acc);
    let trace = TRACE.load(Ordering::Relaxed);
    assert_eq!(trace, 0xABC, "runtime-computed order must dispatch A, B, C");
    println!(
        "WORKS (correctness): runtime-computed topo order dispatched A, B, C (trace {trace:#x}). \
         Now objdump dispatch_runtime_order for blr to decide devirtualisation."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release fat-LTO cgu=1). A RUNTIME-computed
// dispatch order devirtualises identically to a const one.
//
// `dispatch_runtime_order` objdumps to ZERO `blr` (no indirect calls) AND ZERO
// `bl` (no calls at all): LLVM const-folded the non-const `topo(READS, WRITES)`
// call (its inputs are compile-time constants), turning the runtime-permuted
// indexed dispatch into the SAME flat inlined body the const-order sketch
// 202606071000 produced. The three WU bodies fold in topo order (tag 0xa ->
// `madd` for A, 0xb -> `eor ... ror` for B, 0xc -> `add 0x1234567` for C). The
// runtime trace asserts A,B,C (0xABC).
//
// WHAT THIS SETTLES (the GCE-wall unblock): the dispatch order does NOT need to
// be a `const [USize; N]` (the const-generic that overflows GCE well-formedness
// in the generic `Scheduler::run`), nor a per-core-program type (`codegen_core`),
// nor a `Scheduler` struct const-N param. A NON-const fn computing the order over
// `<C as Capacity>::Array` masks (a GCE-safe associated type, no const-generic
// `N`) and dispatching the local fn-pointer slots in it devirtualises at -O3 +
// fat-LTO, because the masks are post-monomorphisation constants (from the
// type-driven `MaskProject` fold) and LLVM folds the pure topo + the indexed
// calls. op-persona's "runtime permutation doesn't devirt" rejection of this
// (Option A) conflated a runtime-FIELD-read shim (the 12.6x path) with a
// runtime-COMPUTED-LOCAL order; the objdump distinguishes them. Bench-decided.
//
// ENGINE SHAPE: add a non-const `carrier_order_dyn<Bundle, Stores, Witnesses, U:
// Capacity, CS: Capacity>() -> <U as Capacity>::Array<USize>` (fills
// `<U as Capacity>::filled` mask arrays via the existing `CarrierMasks::fill`,
// runs an empty-aware Kahn `topo_into` over the slices, returns the order
// `Capacity::Array`); `Scheduler::run` calls it (U = `D::Units`, CS =
// `D::Stores`) and dispatches the `CollectFiber` slots `order[k]` for k in
// `0..topo_count` (whole-program morsel-outer). No const-generic `N`, so no GCE
// overflow. The const `topo_order`/`carrier_order` stay for the unit tests.
//
// WHAT THIS DOES NOT SETTLE: the engine integration's exact devirt (objdump the
// real monomorphised `run`); larger non-ZST or many-unit bodies may inline less
// fully (devirt = zero `blr` is the bar; full inline is a fusion-D4 concern).
// ---------------------------------------------------------------------
