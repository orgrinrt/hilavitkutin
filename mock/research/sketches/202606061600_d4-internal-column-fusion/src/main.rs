//! Sketch (D4 / #340, Phase D): internal-column fusion to registers (#664 gate).
//!
//! The perf gate #664 is red because every intermediate column round-trips the
//! arena. D4's premise (roadmap section 9): with C7's Input/Output/Internal
//! classification, an INTERNAL column (written by one WU, read by the next within
//! a fiber, read nowhere else) should be kept in registers across the fiber
//! chain, not written to and re-read from the arena. The bar: EXACT register
//! residency.
//!
//! The decisive experiment, two variants of the same fiber (In -> Av -> Bv, Av
//! internal), both real engine columns + EngineCtx + the proven RunFiberCol walk:
//!   A (UNFUSED): two WUs, S1 (In->Av) and S2 (Av->Bv). Av is a real arena
//!     column. This is the current shape: each WU owns its `each()` morsel loop,
//!     so S1 writes all of Av to the arena, then S2 reads all of Av from the
//!     arena. Av round-trips.
//!   C (FUSED): one WU, S12 (In->Bv), computing av as a LOCAL. Av is not a column
//!     at all. This is what fiber fusion of an internal column should emit: the
//!     intermediate flows as a register value, never touching the arena.
//!
//! Hypothesis: A round-trips Av through the arena (the #664 gap); C has ZERO Av
//! memory traffic (the intermediate is a register), and both devirtualise (zero
//! `blr`). LLVM does NOT auto-fuse A into C across the two separate each-loops and
//! the arena pointer, so the elimination is an ENGINE codegen decision (fuse a
//! fiber's internal-column-linked WUs / pass the intermediate as a value), keyed
//! by C7's Internal classification, not a toolchain capability we lack. If A
//! already shows no Av round-trip, fusion is free and C is unnecessary. The build
//! + objdump is the test. Outcome at the bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::hint::black_box;
use core::mem::MaybeUninit;
use std::time::Instant;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil, AccumProject, ColPtrCons, ColPtrNil, ColProject, EngineCtx, Project, PtrNil,
};
use hilavitkutin::dispatch::fiber_walk::{WuCons, WuNil};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

// Proven column-capable inline walk (verbatim, resource-only accumulator pin).
trait RunFiberCol<A, Witnesses> {
    fn run(&self, bindings: &A, morsel: MorselRange);
}
impl<A> RunFiberCol<A, Empty> for WuNil {
    #[inline]
    fn run(&self, _bindings: &A, _morsel: MorselRange) {}
}
impl<A, W, Tail, RIdx, RCIdx, WCIdx, WAIdx, WTail>
    RunFiberCol<A, Cons<(RIdx, RCIdx, WCIdx, WAIdx), WTail>> for WuCons<W, Tail>
where
    W: WorkUnit,
    A: Project<<W as WorkUnit>::Read, RIdx>,
    A: ColProject<<W as WorkUnit>::Read, RCIdx>,
    A: ColProject<<W as WorkUnit>::Write, WCIdx>,
    for<'f> A: AccumProject<'f, <W as WorkUnit>::Write, WAIdx, Out = AccPtrNil>,
    for<'f> W: WorkUnit<
        Ctx<'f> = EngineCtx<
            'f,
            <W as WorkUnit>::Read,
            <W as WorkUnit>::Write,
            <A as Project<<W as WorkUnit>::Read, RIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Read, RCIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Write, WCIdx>>::Out,
            AccPtrNil,
        >,
    >,
    Tail: RunFiberCol<A, WTail>,
{
    #[inline]
    fn run(&self, bindings: &A, morsel: MorselRange) {
        let ctx: <W as WorkUnit>::Ctx<'_> =
            EngineCtx::project::<A, A, RIdx, RCIdx, WCIdx, WAIdx>(bindings, bindings, morsel);
        self.head.execute(&ctx);
        self.tail.run(bindings, morsel);
    }
}

// morsel-outer driver for one fiber (the morsel-local path the engine takes for a
// no-accumulator fiber). #[inline(never)] = clean disasm target per variant.
#[inline(never)]
fn run_fiber_morsel_outer<A, F, W>(fiber: &F, bindings: &A, n: USize, msize: USize)
where
    F: RunFiberCol<A, W>,
{
    let n = n.0;
    let m = msize.0.max(1);
    let mut s = 0;
    while s < n {
        let len = if s + m <= n { m } else { n - s };
        fiber.run(bindings, MorselRange::new(USize(s), USize(len)));
        s += m;
    }
}

const M1: u32 = 2654435761;
const M2: u32 = 2246822519;
const SH: u32 = 13;
#[inline(always)]
fn s1_fn(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
fn s2_fn(a: u32) -> u32 {
    let b = a.wrapping_mul(M2).wrapping_add(1);
    (b >> SH) ^ b
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);

type One<T> = Cons<Column<T>, Empty>;

// ---- Variant A: unfused, Av is a real arena column ----
struct S1;
impl BuilderInput for S1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S1 {
    type Read = One<Inv>;
    type Write = One<Av>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Av>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(s1_fn(inp.0))) };
        });
    }
}
struct S2;
impl BuilderInput for S2 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S2 {
    type Read = One<Av>;
    type Write = One<Bv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Av>, One<Bv>, PtrNil, ColPtrCons<Av, ColPtrNil>, ColPtrCons<Bv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let a = unsafe { ctx.reader().read::<Av, _>(i) };
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(s2_fn(a.0))) };
        });
    }
}

// ---- Variant C: fused, av is a LOCAL (no Av column) ----
struct S12;
impl BuilderInput for S12 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S12 {
    type Read = One<Inv>;
    type Write = One<Bv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Bv>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Bv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            let av = s1_fn(inp.0); // INTERNAL value: a local, never a column
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(s2_fn(av))) };
        });
    }
}

struct HeapBump {
    buf: UnsafeCell<Box<[MaybeUninit<u8>]>>,
    cap: usize,
    used: Cell<usize>,
}
impl HeapBump {
    fn new(bytes: usize) -> Self {
        let v: Box<[MaybeUninit<u8>]> = (0..bytes).map(|_| MaybeUninit::uninit()).collect();
        Self { buf: UnsafeCell::new(v), cap: bytes, used: Cell::new(0) }
    }
}
unsafe impl Send for HeapBump {}
unsafe impl Sync for HeapBump {}
impl MemoryProviderApi for HeapBump {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = unsafe { (*self.buf.get()).as_mut_ptr() as *mut u8 };
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > self.cap {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

fn bench_min<F: FnMut()>(warmup: usize, iters: usize, mut f: F) -> u128 {
    for _ in 0..warmup {
        f();
    }
    let mut best = u128::MAX;
    for _ in 0..iters {
        let t = Instant::now();
        f();
        let ns = t.elapsed().as_nanos();
        if ns < best {
            best = ns;
        }
    }
    best
}

const N: usize = 1 << 17;
const MORSEL: usize = 1024;

fn main() {
    // Variant A scheduler: In, Av, Bv columns; S1, S2.
    let pa = HeapBump::new(8 * 1024 * 1024);
    let sched_a = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Av>::new())
        .with(Column::<Bv>::new())
        .with(S1)
        .with(S2)
        .build(store(pa), USize(N))
        .unwrap_or_else(|_| panic!("build A"));
    // In is the deepest tail (prepend: Bv, Av, In head-first -> In two tails down).
    let in_a = sched_a.__bindings().__tail().__tail().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *in_a.add(i) = Inv(i as u32) };
    }
    let fiber_a = WuCons { head: S1, tail: WuCons { head: S2, tail: WuNil } };
    run_fiber_morsel_outer(&fiber_a, sched_a.__bindings(), USize(N), USize(MORSEL));

    // Variant C scheduler: In, Bv columns; S12 (no Av column at all).
    let pc = HeapBump::new(8 * 1024 * 1024);
    let sched_c = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Bv>::new())
        .with(S12)
        .build(store(pc), USize(N))
        .unwrap_or_else(|_| panic!("build C"));
    let in_c = sched_c.__bindings().__tail().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *in_c.add(i) = Inv(i as u32) };
    }
    let fiber_c = WuCons { head: S12, tail: WuNil };
    run_fiber_morsel_outer(&fiber_c, sched_c.__bindings(), USize(N), USize(MORSEL));

    // Correctness: both produce Bv[i] = s2(s1(i)).
    let bv_a = sched_a.__bindings().__ptr().as_ptr() as *const u32;
    let bv_c = sched_c.__bindings().__ptr().as_ptr() as *const u32;
    let ba = unsafe { core::slice::from_raw_parts(bv_a, N) };
    let bc = unsafe { core::slice::from_raw_parts(bv_c, N) };
    for i in 0..N {
        let want = s2_fn(s1_fn(i as u32));
        assert_eq!(ba[i], want, "A Bv[{i}]");
        assert_eq!(bc[i], want, "C Bv[{i}]");
    }

    let warmup = 50;
    let iters = 500;
    let a_ns = bench_min(warmup, iters, || {
        run_fiber_morsel_outer(&fiber_a, sched_a.__bindings(), USize(N), USize(MORSEL));
        black_box(&fiber_a);
    });
    let c_ns = bench_min(warmup, iters, || {
        run_fiber_morsel_outer(&fiber_c, sched_c.__bindings(), USize(N), USize(MORSEL));
        black_box(&fiber_c);
    });
    println!("WORKS: A (unfused, Av arena column) + C (fused, av local) both correct for {N} recs");
    println!(
        "bench (min of {iters}): A unfused = {a_ns} ns ({:.3} ns/rec), C fused = {c_ns} ns \
         ({:.3} ns/rec), A/C ratio = {:.3}",
        a_ns as f64 / N as f64,
        c_ns as f64 / N as f64,
        a_ns as f64 / c_ns as f64
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS / DECISIVE (nightly-2026-05-28, release fat-LTO cgu=1).
//
// objdump of the two `run_fiber_morsel_outer` monomorphizations:
//   A (unfused, Av is a real arena column, S1->S2): 2 stores, 3 loads, 22 vec
//     ops, 0 blr. The internal column Av ROUND-TRIPS the arena: S1 stores all of
//     Av, S2 re-loads it. Each WU owns its `each()` loop (WU-outer), so the
//     intermediate is fully materialised to the arena between the two stages.
//   C (fused, av is a LOCAL, S12): 1 store (Bv), 1 load (Inv), 17 vec ops, 0 blr.
//     Av is GONE: no store, no load for it. The intermediate is a register.
//
// Bench (min of 500, N=131072, morsel=1024): A unfused = 0.173 ns/rec, C fused =
// 0.083 ns/rec, A/C = 2.09x. The fused form is 2.09x faster, and 2.09x sits
// squarely in the #664 perf-gate red band (~2.1x-4.6x, memory-bandwidth
// signature). This internal-column round-trip IS a primary driver of the gate.
//
// WHAT THIS SETTLES (D4, the #664 enabler): register residency of an internal
// column is achievable (variant C) and worth ~2x. The mechanism is FUSING a
// fiber's internal-column-linked WUs so the intermediate flows as a value, never
// becoming an arena column. LLVM does NOT do this automatically (variant A still
// round-trips across the two separate each-loops + the arena pointer), so the
// fusion is an ENGINE codegen / plan decision, keyed by C7's Internal
// classification (Internal column between two WUs in one fiber -> fuse them, drop
// the column). This matches canonical domain 17 ("DSE + stores-at-end: the
// flattener emits the optimal loop body", :1609) and is consistent with D1b's
// flattener-emits resolution: the flattener emits the fused record-outer body for
// internal columns. No toolchain blocker; the bar (register residency) is met.
//
// WHAT THIS DOES NOT SETTLE: the mechanical auto-fusion of two consumer-authored
// WUs (S1, S2) into one fused body (S12) at codegen, without the consumer
// hand-merging. The sketch proves the TARGET (C) is correct, devirtualised, and
// 2x faster, and the GAP (A) is the cost; producing C from S1+S2 is the
// flattener's emit job (same family as D1b's carrier emission). That codegen is
// an implementation task, not an open feasibility question: the target shape is
// proven to compile + devirtualise + win.
// ---------------------------------------------------------------------
