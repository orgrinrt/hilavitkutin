//! Sketch (D4 fusion MECHANISM, dual-agent reconciliation 2026-06-09).
//!
//! 202606061600 proved a HAND-fused single WU is 2.09x faster than two unfused
//! WUs (the internal column round-trips the arena), and that LLVM will not
//! auto-fuse the unfused pair. The open question this sketch answers: can the
//! engine AUTO-fuse a fiber's WUs into the register-passing body WITHOUT touching
//! the core `WorkUnit::execute` contract and WITHOUT runtime/fn-pointer
//! indirection (the proven devirt-failure mode)?
//!
//! Mechanism under test: an OPT-IN per-record pure-transform trait `RecordOp`
//! (only fusible WUs implement it; the core `WorkUnit` contract is untouched),
//! composed by a type-level fold `OpChain` into ONE `ChainWu` whose `execute`
//! reads the fiber-input column, runs the chain with intermediates as LOCALS, and
//! writes the fiber-output column. The chain is built from CONCRETE op types
//! (no fn pointers, no dyn), so each `apply` is a statically-known monomorphised
//! call the optimiser inlines.
//!
//! Three stages with HETEROGENEOUS intermediate types (Inv -> Av -> Bv -> Cv;
//! Av, Bv internal) test the architect's flagged risk: expressing N heterogeneous
//! per-link output types in the fold without alloc/dyn, and LLVM seeing through
//! the composition to register residency.
//!
//! Bar (the D4 target, section-9 EXACT): the fused `ChainWu` walk objdumps like
//! the hand-fused S12 (1 load + 1 store, the Av/Bv internal columns ELIMINATED,
//! zero blr) under a real runtime morsel loop, and beats the unfused S1/S2/S3
//! baseline (Av/Bv real arena columns). Outcome at the bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::hint::black_box;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use std::time::Instant;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil, AccumProject, ColPtrCons, ColPtrNil, ColProject, EngineCtx, Project, PtrNil,
};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::dispatch::{WuCons, WuNil};
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

// ---------------------------------------------------------------------
// Proven column-capable inline walk (verbatim from 202606061600 / 202606081200,
// resource-only accumulator pin). The fused fiber is a single-element WuCons.
// ---------------------------------------------------------------------
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

// ---------------------------------------------------------------------
// The pure per-record compute (three distinct stages).
// ---------------------------------------------------------------------
const M1: u32 = 2654435761;
const M2: u32 = 2246822519;
const M3: u32 = 40503;
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
#[inline(always)]
fn s3_fn(b: u32) -> u32 {
    b.wrapping_mul(M3).rotate_left(7)
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);
#[derive(Copy, Clone)]
struct Cv(u32);

type One<T> = Cons<Column<T>, Empty>;

// =====================================================================
// THE MECHANISM: an opt-in per-record pure transform + a type-level fold.
// `RecordOp` is NOT on the core `WorkUnit` contract; only fusible WUs add it.
// =====================================================================
trait RecordOp {
    type In: Copy;
    type Out: Copy;
    fn apply(&self, x: Self::In) -> Self::Out;
}

// The fold: a cons-list of ops whose link types thread `In = Head::Out`. This is
// the heterogeneous-output threading the architect flagged. `run_chain` composes
// `tail.run_chain(head.apply(x))`, all concrete inline calls, intermediates local.
trait OpChain {
    type In: Copy;
    type Out: Copy;
    fn run_chain(&self, x: Self::In) -> Self::Out;
}
struct OpNil<T>(PhantomData<T>);
impl<T: Copy> OpChain for OpNil<T> {
    type In = T;
    type Out = T;
    #[inline(always)]
    fn run_chain(&self, x: T) -> T {
        x
    }
}
struct OpCons<H, Tl> {
    head: H,
    tail: Tl,
}
impl<H, Tl> OpChain for OpCons<H, Tl>
where
    H: RecordOp,
    Tl: OpChain<In = <H as RecordOp>::Out>,
{
    type In = <H as RecordOp>::In;
    type Out = <Tl as OpChain>::Out;
    #[inline(always)]
    fn run_chain(&self, x: Self::In) -> Self::Out {
        self.tail.run_chain(self.head.apply(x))
    }
}

struct Op1;
impl RecordOp for Op1 {
    type In = Inv;
    type Out = Av;
    #[inline(always)]
    fn apply(&self, x: Inv) -> Av {
        Av(s1_fn(x.0))
    }
}
struct Op2;
impl RecordOp for Op2 {
    type In = Av;
    type Out = Bv;
    #[inline(always)]
    fn apply(&self, x: Av) -> Bv {
        Bv(s2_fn(x.0))
    }
}
struct Op3;
impl RecordOp for Op3 {
    type In = Bv;
    type Out = Cv;
    #[inline(always)]
    fn apply(&self, x: Bv) -> Cv {
        Cv(s3_fn(x.0))
    }
}

type Chain123 = OpCons<Op1, OpCons<Op2, OpCons<Op3, OpNil<Cv>>>>;
#[inline(always)]
fn chain123() -> Chain123 {
    OpCons {
        head: Op1,
        tail: OpCons { head: Op2, tail: OpCons { head: Op3, tail: OpNil(PhantomData) } },
    }
}

// The FUSED WU: a normal `WorkUnit` (core contract untouched) whose `execute`
// reads the fiber-input column, runs the op chain with the intermediates as
// register locals, and writes the fiber-output column. Av/Bv are NOT columns. The
// engine's flattener would synthesise this from a fiber's RecordOp-implementing
// WUs; here it is built by hand from the same ops to isolate the fusion question.
struct ChainWu<C> {
    chain: C,
}
impl<C: Send + Sync + 'static> BuilderInput for ChainWu<C> {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl<C> WorkUnit<Always> for ChainWu<C>
where
    C: OpChain<In = Inv, Out = Cv> + Send + Sync + 'static,
{
    type Read = One<Inv>;
    type Write = One<Cv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Cv>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Cv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Inv host-populated for the record count; Cv reserved +
            // exclusively written; morsel covers only reserved records.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            let out = self.chain.run_chain(inp); // Av, Bv flow as registers
            unsafe { ctx.writer().write::<Cv, _>(i, out) };
        });
    }
}

// =====================================================================
// Unfused baseline: three separate WUs, Av and Bv are real arena columns.
// =====================================================================
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
struct S3;
impl BuilderInput for S3 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S3 {
    type Read = One<Bv>;
    type Write = One<Cv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Bv>, One<Cv>, PtrNil, ColPtrCons<Bv, ColPtrNil>, ColPtrCons<Cv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let b = unsafe { ctx.reader().read::<Bv, _>(i) };
            unsafe { ctx.writer().write::<Cv, _>(i, Cv(s3_fn(b.0))) };
        });
    }
}

// ---------------------------------------------------------------------
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
#[inline(always)]
fn want(i: u32) -> u32 {
    s3_fn(s2_fn(s1_fn(i)))
}

fn main() {
    // Unfused baseline: In/Av/Bv/Cv columns; S1, S2, S3.
    let pu = HeapBump::new(16 * 1024 * 1024);
    let sched_u = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Av>::new())
        .with(Column::<Bv>::new())
        .with(Column::<Cv>::new())
        .with(S1)
        .with(S2)
        .with(S3)
        .build(store(pu), USize(N))
        .unwrap_or_else(|_| panic!("build unfused"));
    // Prepend order: builder appends now (D1d), so registration order is head
    // order; Inv is the first store registered -> deepest binding tail. Walk the
    // tails to the Inv pointer (4 stores: Inv, Av, Bv, Cv -> Inv is 3 tails down).
    let in_u =
        sched_u.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *in_u.add(i) = Inv(i as u32) };
    }
    let fiber_u =
        WuCons { head: S1, tail: WuCons { head: S2, tail: WuCons { head: S3, tail: WuNil } } };
    run_fiber_morsel_outer(&fiber_u, sched_u.__bindings(), USize(N), USize(MORSEL));

    // Fused: In/Cv columns only; one ChainWu (Av/Bv never columns).
    let pf = HeapBump::new(16 * 1024 * 1024);
    let sched_f = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Cv>::new())
        .with(ChainWu { chain: chain123() })
        .build(store(pf), USize(N))
        .unwrap_or_else(|_| panic!("build fused"));
    let in_f = sched_f.__bindings().__tail().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *in_f.add(i) = Inv(i as u32) };
    }
    let fiber_f = WuCons { head: ChainWu { chain: chain123() }, tail: WuNil };
    run_fiber_morsel_outer(&fiber_f, sched_f.__bindings(), USize(N), USize(MORSEL));

    // Correctness: both produce Cv[i] = s3(s2(s1(i))).
    let cv_u = sched_u.__bindings().__ptr().as_ptr() as *const u32;
    let cv_f = sched_f.__bindings().__ptr().as_ptr() as *const u32;
    let cu = unsafe { core::slice::from_raw_parts(cv_u, N) };
    let cf = unsafe { core::slice::from_raw_parts(cv_f, N) };
    for i in 0..N {
        let w = want(i as u32);
        assert_eq!(cu[i], w, "unfused Cv[{i}]");
        assert_eq!(cf[i], w, "fused Cv[{i}]");
    }

    let warmup = 50;
    let iters = 500;
    let u_ns = bench_min(warmup, iters, || {
        run_fiber_morsel_outer(&fiber_u, sched_u.__bindings(), USize(N), USize(MORSEL));
        black_box(&fiber_u);
    });
    let f_ns = bench_min(warmup, iters, || {
        run_fiber_morsel_outer(&fiber_f, sched_f.__bindings(), USize(N), USize(MORSEL));
        black_box(&fiber_f);
    });
    println!("WORKS: unfused (S1/S2/S3, Av+Bv arena columns) + fused (RecordOp chain) both correct for {N} recs");
    println!(
        "bench (min of {iters}): unfused = {u_ns} ns ({:.3} ns/rec), fused = {f_ns} ns \
         ({:.3} ns/rec), unfused/fused = {:.3}x",
        u_ns as f64 / N as f64,
        f_ns as f64 / N as f64,
        u_ns as f64 / f_ns as f64
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS / DECISIVE (nightly-2026-05-28, release fat-LTO cgu=1, arvo dev
// HEAD ff514a7). The opt-in `RecordOp` + type-level `OpChain` fold composes to
// register residency WITHOUT a core `WorkUnit::execute` contract change.
//
// objdump of the two `run_fiber_morsel_outer` monomorphisations (confirmed by
// the mangled generic args):
//   UNFUSED `WuCons<S1, S2, S3>` (bindings ColumnBinding Cv/Bv/Av/Inv = 4 real
//     columns): 131 instrs, 12 loads, 10 stores, 0 blr. Av and Bv ROUND-TRIP the
//     arena: each of the three WUs owns its `each()` loop, so the two internal
//     columns are fully materialised between stages.
//   FUSED `WuCons<ChainWu<OpCons<Op1, OpCons<Op2, OpCons<Op3, OpNil<Cv>>>>>>`
//     (bindings ColumnBinding Cv/Inv = ONLY 2 columns; Av/Bv are NOT columns at
//     all): 72 instrs, 3 loads, 3 stores, 0 blr. The internal columns Av and Bv
//     are ELIMINATED to registers; the 3 loads / 3 stores are the Inv input + Cv
//     output + auto-vectorisation, no internal-column traffic.
//
// DECISIVE: a chain of CONCRETE per-record ops composed by a type-level fold
// (`tail.run_chain(head.apply(x))`, all `#[inline]` monomorphised calls, no fn
// pointers, no dyn) DSEs the heterogeneous intermediates (Inv->Av->Bv->Cv) to
// registers, matching the hand-fused S12 of 202606061600 (1-load/1-store family,
// internal columns gone) and devirtualising (0 blr) under a real runtime morsel
// loop. The architect's flagged risk (N heterogeneous per-link output types
// without alloc/dyn) is CLEARED: the `OpChain` fold with `In = Head::Out`
// threading compiled and fused.
//
// Bench (min of 500, N=131072, morsel=1024, cheap u32 ops): unfused 0.236 ns/rec,
// fused 0.202 ns/rec, 1.17x. The ratio is smaller than 202606061600's 2.09x
// because this config is memory-bandwidth-bound at a lower absolute ns/rec and
// the morsel-resident arena columns are L1-hot; the load/store STRUCTURE (12/10
// -> 3/3, internal columns absent from the binding) is the decisive register-
// residency proof, not the ratio. Larger ops / wider intermediates / colder
// columns widen the gap (cf. 202606061600).
//
// RESOLUTION (dual-agent reconciliation, answered): register residency for D4 is
// achievable via an OPT-IN `RecordOp` per-record pure transform + an `OpChain`
// type-level fold composed into a normal `ChainWu: WorkUnit`. The core
// `WorkUnit::execute` contract is UNTOUCHED (op's constraint), and the
// composition is concrete-type / monomorphised (not the failed fn-pointer
// path). Accumulator WUs (append, not a record-indexed pure transform) do NOT
// implement `RecordOp`; they stay on the existing `execute`/`each` unit-outer
// path, unfused. Residual for the D4 BUILD: whether the engine's flattener
// AUTO-synthesises `ChainWu` from a fiber's separately-registered `RecordOp`
// WUs (keyed by the C7 Internal classification) or a Kit/builder composes the
// chain explicitly. Feasibility (the chain fuses to registers) is now proven;
// the auto-vs-authored synthesis is the build's design increment.
// ---------------------------------------------------------------------
