//! Sketch (D4 auto-synthesis, GATE-1-critical). Does the engine-side type-level
//! fold of a REGISTERED `WuCons<S1, S2, S3>` carrier into the proven `OpChain`
//! compile, and does the folded chain fuse internal columns to registers
//! identically to the hand-built chain of 202606091200?
//!
//! The #664 gate registers separate WUs and calls `run()`. For it to green
//! transparently the engine must FOLD the carrier itself. The fold is two
//! NON-OVERLAPPING structural impls (`WuCons<H, WuNil>` base, `WuCons<H, WuCons<
//! H2, T>>` recursive), so it should dodge the E0119 wall that killed type-level
//! fiber GROUPING (which partitions; this only folds a single chain). The
//! heterogeneous link threading (`tail Chain: OpChain<In = Head::Out>`) is the
//! risk to clear.
//!
//! Bar: the fold compiles; `fuse()` on `WuCons<Op1, WuCons<Op2, WuCons<Op3,
//! WuNil>>>` yields the same `OpChain` as the hand-built one; the folded-chain
//! `ChainWu` walk objdumps with the internal columns ELIMINATED to registers
//! (matching 202606091200's fused 3-load/3-store, zero blr), and equals the
//! hand-built-chain walk in the same binary. Outcome at the bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::hint::black_box;
use core::marker::PhantomData;
use core::mem::MaybeUninit;

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

// Proven walk (verbatim, resource-only accumulator pin).
trait RunFiberCol<A, Witnesses> {
    fn run(&self, bindings: &A, morsel: MorselRange);
}
impl<A> RunFiberCol<A, Empty> for WuNil {
    #[inline]
    fn run(&self, _b: &A, _m: MorselRange) {}
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

// Per-record op + chain fold (from 202606091200).
trait RecordOp {
    type In: Copy;
    type Out: Copy;
    fn apply(&self, x: Self::In) -> Self::Out;
}
trait OpChain {
    type In: Copy;
    type Out: Copy;
    fn run_chain(&self, x: Self::In) -> Self::Out;
}
#[derive(Copy, Clone)]
struct OpNil<T>(PhantomData<T>);
impl<T: Copy> OpChain for OpNil<T> {
    type In = T;
    type Out = T;
    #[inline(always)]
    fn run_chain(&self, x: T) -> T {
        x
    }
}
#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
struct Op1;
impl RecordOp for Op1 {
    type In = Inv;
    type Out = Av;
    #[inline(always)]
    fn apply(&self, x: Inv) -> Av {
        Av(s1_fn(x.0))
    }
}
#[derive(Copy, Clone)]
struct Op2;
impl RecordOp for Op2 {
    type In = Av;
    type Out = Bv;
    #[inline(always)]
    fn apply(&self, x: Av) -> Bv {
        Bv(s2_fn(x.0))
    }
}
#[derive(Copy, Clone)]
struct Op3;
impl RecordOp for Op3 {
    type In = Bv;
    type Out = Cv;
    #[inline(always)]
    fn apply(&self, x: Bv) -> Cv {
        Cv(s3_fn(x.0))
    }
}

// =====================================================================
// THE NEW THING: fold a registered `WuCons` carrier of RecordOp WUs into the
// `OpChain`. Two NON-OVERLAPPING structural impls (no partitioning, so no E0119).
// The recursive impl threads the link `tail Chain: OpChain<In = Head::Out>`.
// =====================================================================
trait FuseCarrier {
    type Chain: OpChain;
    fn fuse(self) -> Self::Chain;
}
// Base: a single-WU carrier folds to a one-op chain terminated by identity.
impl<H> FuseCarrier for WuCons<H, WuNil>
where
    H: RecordOp + Copy,
{
    type Chain = OpCons<H, OpNil<<H as RecordOp>::Out>>;
    #[inline(always)]
    fn fuse(self) -> Self::Chain {
        OpCons { head: self.head, tail: OpNil(PhantomData) }
    }
}
// Recursive: head op then a non-empty tail carrier; the tail's folded chain must
// take the head's output as its input (the internal-column link).
impl<H, H2, T> FuseCarrier for WuCons<H, WuCons<H2, T>>
where
    H: RecordOp + Copy,
    WuCons<H2, T>: FuseCarrier,
    <WuCons<H2, T> as FuseCarrier>::Chain: OpChain<In = <H as RecordOp>::Out>,
{
    type Chain = OpCons<H, <WuCons<H2, T> as FuseCarrier>::Chain>;
    #[inline(always)]
    fn fuse(self) -> Self::Chain {
        OpCons { head: self.head, tail: self.tail.fuse() }
    }
}

// The fused WU, parameterised over any chain (Inv -> Cv). Same as 202606091200.
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
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            let out = self.chain.run_chain(inp);
            unsafe { ctx.writer().write::<Cv, _>(i, out) };
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
    unsafe fn deallocate(&self, _p: *mut u8, _l: USize) {}
    unsafe fn protect(&self, _p: *mut u8, _l: USize, _r: Bool, _w: Bool) {}
}
fn store<M: MemoryProviderApi>(p: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(p)
}

const N: usize = 1 << 17;
const MORSEL: usize = 1024;
#[inline(always)]
fn want(i: u32) -> u32 {
    s3_fn(s2_fn(s1_fn(i)))
}

// Build a fused scheduler (In + Cv columns; the chain WU) and run it.
#[inline(never)]
fn run_fused<C>(chain: C) -> Vec<u32>
where
    C: OpChain<In = Inv, Out = Cv> + Send + Sync + 'static + Copy,
{
    let p = HeapBump::new(16 * 1024 * 1024);
    let sched = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Cv>::new())
        .with(ChainWu { chain })
        .build(store(p), USize(N))
        .unwrap_or_else(|_| panic!("build fused"));
    let inp = sched.__bindings().__tail().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *inp.add(i) = Inv(i as u32) };
    }
    let fiber = WuCons { head: ChainWu { chain }, tail: WuNil };
    run_fiber_morsel_outer(&fiber, sched.__bindings(), USize(N), USize(MORSEL));
    let cv = sched.__bindings().__ptr().as_ptr() as *const u32;
    unsafe { core::slice::from_raw_parts(cv, N) }.to_vec()
}

fn main() {
    // FOLD the registered carrier of RecordOp WUs into the chain. This is the
    // engine-synthesis step: `WuCons<Op1, WuCons<Op2, WuCons<Op3, WuNil>>>` (the
    // exact cons-list shape the scheduler builder retains) -> the OpChain.
    let carrier = WuCons {
        head: Op1,
        tail: WuCons { head: Op2, tail: WuCons { head: Op3, tail: WuNil } },
    };
    let folded = carrier.fuse();
    // STATIC type-equality proof: the fold's output binds to the hand-built chain
    // type. Compiles only if `fuse()` produced exactly `OpCons<Op1, OpCons<Op2,
    // OpCons<Op3, OpNil<Cv>>>>`.
    let hand: OpCons<Op1, OpCons<Op2, OpCons<Op3, OpNil<Cv>>>> = folded;

    let from_folded = run_fused(folded);
    let from_hand = run_fused(hand);
    for i in 0..N {
        let w = want(i as u32);
        assert_eq!(from_folded[i], w, "folded Cv[{i}]");
        assert_eq!(from_hand[i], w, "hand Cv[{i}]");
    }
    black_box(&folded);
    println!(
        "WORKS: carrier.fuse() folded WuCons<Op1,Op2,Op3> -> OpChain (statically the hand-built \
         type); both dispatch correctly for {N} recs. objdump run_fused for register residency \
         (Av/Bv eliminated, matching 202606091200's fused 3-load/3-store)."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS / DECISIVE (nightly-2026-05-28, release fat-LTO cgu=1, arvo dev
// HEAD ff514a7). The engine CAN auto-synthesise the fused chain from a registered
// carrier; #664 can green transparently (consumer registers separate RecordOp
// WUs, engine folds them).
//
//   - COMPILES: the two non-overlapping `FuseCarrier` impls (`WuCons<H, WuNil>`
//     base, `WuCons<H, WuCons<H2, T>>` recursive) with the heterogeneous link
//     bound `tail Chain: OpChain<In = Head::Out>` type-check with NO E0119. Folding
//     a single chain (no partitioning) dodges the coherence wall that killed
//     type-level fiber GROUPING (roadmap r2 D1b).
//   - STATIC TYPE-EQUALITY: `let hand: OpCons<Op1, OpCons<Op2, OpCons<Op3,
//     OpNil<Cv>>>> = carrier.fuse();` compiles, so `fuse()` produces EXACTLY the
//     hand-built chain type. Confirmed by objdump: only ONE `run_fiber_morsel_outer`
//     monomorphisation exists (folded and hand-built are the same type -> one
//     codegen).
//   - REGISTER RESIDENCY: that mono is 72 instrs, 0 blr, 3 loads / 3 stores --
//     byte-identical to 202606091200's hand-fused ChainWu. The internal columns
//     Av/Bv are ELIMINATED to registers (binding carries only Inv + Cv); the
//     3 loads / 3 stores are Inv in + Cv out + auto-vectorisation.
//   - CORRECT: both folded and hand chains produce Cv[i] = s3(s2(s1(i))) for all N.
//
// DECISIVE for the D4 BUILD: the engine's flattener can fold a fiber's registered
// RecordOp-implementing WU carrier (`WuCons<S1, S2, S3>`, the exact cons-list the
// scheduler builder retains) into the fused `OpChain` at the type level, with the
// internal columns becoming registers, NO core `WorkUnit::execute` contract change,
// NO E0119, NO fn-pointer indirection. Combined with 202606091200 (the chain fuses
// to registers), D4 auto-fusion is fully de-risked. The build wires: WUs add an
// opt-in `RecordOp` impl; `run()` (single-fiber whole-carrier case for GATE-1)
// folds the carrier via `FuseCarrier` and dispatches the fused `ChainWu`; the
// internal columns are elided from reservation (C7 Internal classification).
// Residual is implementation, not feasibility.
// ---------------------------------------------------------------------
