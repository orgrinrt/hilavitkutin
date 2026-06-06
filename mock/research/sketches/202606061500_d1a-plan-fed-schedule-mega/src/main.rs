//! Sketch (D1a / #340, Phase D keystone bridge): plan-fed schedule-mega body.
//!
//! D1b (sketch 202606061400) settled the dispatch STRUCTURE: a type-level flat
//! per-core carrier devirtualises (zero `blr`), and the fiber/phase GROUPING is a
//! runtime plan computation (`group_fibers`) that the codegen flattener emits the
//! carrier from (canonical domain 17). The schedule-mega sketch (202606060500)
//! proved a HAND-WRITTEN carrier with a CONST morsel devirtualises end to end.
//!
//! D1a's open delta (roadmap section 9): the plan supplies RUNTIME params. Morsel
//! size is an R6 ADAPTIVE runtime parameter ("from runtime hardware detection",
//! R6 `:2442-2446`), record count is runtime (consumer data), and the shipped
//! `run<Witnesses>` today walks the plan's runtime `topo_order` array via runtime
//! indices through type-erased `FiberSlot` fn-pointer shims (the 12.6x indirect
//! anti-pattern, scheduler/mod.rs:715 `shim(ptr, ...)`). The keystone bridge must
//! replace that with the devirtualised schedule-mega body while still being fed
//! the plan's runtime params.
//!
//! The inherent tension D1b already resolved: a runtime permutation array cannot
//! become a compile-time type ordering without the codegen flattener; so the
//! dispatch ORDER is compile-time (the flattener-emitted / registration carrier,
//! validated by the plan as topo-valid), while the plan supplies the RUNTIME
//! params (morsel size, record count, phase/fiber boundary counts) that drive the
//! loop structure. The within-level RCM order is a benched ~2% refinement
//! (202606060500), not a structural blocker on one core.
//!
//! Hypothesis: the schedule-mega body still objdumps to ZERO `blr` (no indirect
//! dispatch) when morsel size and record count come from a runtime `PlanOut`
//! struct rather than const generics, over compile-time-ordered phase cons-lists.
//! The morsel constant no longer bakes as an immediate (it is a register now,
//! correctly, per R6), but devirt (the thing that matters) holds: the inline
//! RunFiberCol walk is still monomorphised straight-line code. Leeway: EXACT for
//! devirt (zero `blr`), SOME-SHAPE for the PlanOut struct. Outcome at the bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::hint::black_box;
use core::mem::MaybeUninit;

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

// The proven column-capable inline-recursive fiber walk (verbatim from
// 202606060500 / 202606052130). Resource-only accumulator pin (AccPtrNil).
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

// ---------------------------------------------------------------------
// THE PLAN OUTPUT (runtime). This is what the plan computes at build() and the
// per-core program is fed: the morsel size (R6 adaptive, from hardware probe),
// the record count (consumer data), and the per-phase fiber-boundary structure.
// In the real engine this is a slice of the stored ExecutionPlan; here a minimal
// struct with the two RUNTIME scalars the single-core schedule-mega body needs.
// The phase COUNT and per-phase WU membership are compile-time (the flattener
// emits the per-phase cons-lists, D1b); only the scalars below are runtime.
// ---------------------------------------------------------------------
#[derive(Copy, Clone)]
struct PlanOut {
    morsel_size: USize,
    record_count: USize,
}

// ---------------------------------------------------------------------
// The plan-fed schedule-mega body. IDENTICAL to 202606060500 EXCEPT the morsel
// size and record count are read from the runtime `PlanOut` (function args, not a
// const generic). The phase cons-lists stay compile-time (P0, P1). #[inline(never)]
// for a clean disasm target. The bar: zero `blr` despite the runtime morsel.
// ---------------------------------------------------------------------
#[inline(never)]
fn run_schedule_mega_planfed<A, P0, W0, P1, W1>(
    phase0: &P0,
    phase1: &P1,
    bindings: &A,
    plan: &PlanOut,
) where
    P0: RunFiberCol<A, W0>,
    P1: RunFiberCol<A, W1>,
{
    let n = plan.record_count.0;
    let msize = plan.morsel_size.0.max(1); // runtime morsel size (R6 adaptive)
    // phase 0: morsel-outer over the runtime range with the runtime chunk.
    let mut s = 0;
    while s < n {
        let len = if s + msize <= n { msize } else { n - s };
        phase0.run(bindings, MorselRange::new(USize(s), USize(len)));
        s += msize;
    }
    // phase boundary (sequence point on one core).
    let mut s = 0;
    while s < n {
        let len = if s + msize <= n { msize } else { n - s };
        phase1.run(bindings, MorselRange::new(USize(s), USize(len)));
        s += msize;
    }
}

// ---------------------------------------------------------------------
// Workload (diamond + norm), mirrors 202606060500.
// ---------------------------------------------------------------------
const M1: u32 = 2654435761;
const M2: u32 = 2246822519;
const M4: u32 = 668265263;
const SH: u32 = 13;
#[inline(always)]
fn stage1(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
fn stage2(a: u32) -> u32 {
    a.wrapping_mul(M2).wrapping_add(1)
}
#[inline(always)]
fn stage3(b: u32) -> u32 {
    (b >> SH) ^ b
}
#[inline(always)]
fn stage4(c: u32) -> u32 {
    c.wrapping_mul(M4)
}
#[inline(always)]
fn branch_x(seed: u32) -> u32 {
    stage1(seed)
}
#[inline(always)]
fn branch_y(seed: u32) -> u32 {
    stage3(stage2(seed))
}
#[inline(always)]
fn join_fn(x: u32, y: u32) -> u32 {
    stage4(x ^ y)
}
#[inline(always)]
fn norm_fn(z: u32) -> u32 {
    stage3(stage2(z))
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Xv(u32);
#[derive(Copy, Clone)]
struct Yv(u32);
#[derive(Copy, Clone)]
struct Zv(u32);
#[derive(Copy, Clone)]
struct Wv(u32);

type One<T> = Cons<Column<T>, Empty>;
type Two<A, B> = Cons<Column<A>, Cons<Column<B>, Empty>>;

struct BranchX;
impl BuilderInput for BranchX {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for BranchX {
    type Read = One<Inv>;
    type Write = One<Xv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Xv>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Xv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Xv, _>(i, Xv(branch_x(inp.0))) };
        });
    }
}
struct BranchY;
impl BuilderInput for BranchY {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for BranchY {
    type Read = One<Inv>;
    type Write = One<Yv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Inv>, One<Yv>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Yv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Yv, _>(i, Yv(branch_y(inp.0))) };
        });
    }
}
struct JoinZ;
impl BuilderInput for JoinZ {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for JoinZ {
    type Read = Two<Xv, Yv>;
    type Write = One<Zv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        Two<Xv, Yv>,
        One<Zv>,
        PtrNil,
        ColPtrCons<Xv, ColPtrCons<Yv, ColPtrNil>>,
        ColPtrCons<Zv, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let x = unsafe { ctx.reader().read::<Xv, _>(i) };
            let y = unsafe { ctx.reader().read::<Yv, _>(i) };
            unsafe { ctx.writer().write::<Zv, _>(i, Zv(join_fn(x.0, y.0))) };
        });
    }
}
struct NormW;
impl BuilderInput for NormW {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for NormW {
    type Read = One<Zv>;
    type Write = One<Wv>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> =
        EngineCtx<'frame, One<Zv>, One<Wv>, PtrNil, ColPtrCons<Zv, ColPtrNil>, ColPtrCons<Wv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let z = unsafe { ctx.reader().read::<Zv, _>(i) };
            unsafe { ctx.writer().write::<Wv, _>(i, Wv(norm_fn(z.0))) };
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

// Simulate the plan's runtime morsel-size decision (hardware probe). black_box
// stops the optimizer from const-folding it back to a baked immediate, so the
// disasm genuinely exercises a runtime morsel size.
#[inline(never)]
fn probe_plan(record_count: usize) -> PlanOut {
    let m = black_box(1024usize);
    PlanOut { morsel_size: USize(m), record_count: USize(record_count) }
}

fn main() {
    const N: usize = 1 << 17; // 131072

    let provider = HeapBump::new(8 * 1024 * 1024);
    let sched = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Xv>::new())
        .with(Column::<Yv>::new())
        .with(Column::<Zv>::new())
        .with(Column::<Wv>::new())
        .with(BranchX)
        .with(BranchY)
        .with(JoinZ)
        .with(NormW)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // Inv is the deepest column tail (prepend order: Wv,Zv,Yv,Xv,Inv head-first).
    let in_base = sched
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }
    let bindings = sched.__bindings();

    // Compile-time phase cons-lists (the flattener output, D1b). Registration
    // order; the plan validates it as topo-valid (JoinZ after its inputs).
    let p0 = WuCons {
        head: BranchX,
        tail: WuCons { head: BranchY, tail: WuCons { head: JoinZ, tail: WuNil } },
    };
    let p1 = WuCons { head: NormW, tail: WuNil };

    // The plan output: RUNTIME morsel size + record count.
    let plan = probe_plan(N);

    run_schedule_mega_planfed(&p0, &p1, bindings, &plan);

    // Verify.
    let wv_base = sched.__bindings().__ptr().as_ptr() as *const u32;
    let zv_base = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
    let wv = unsafe { core::slice::from_raw_parts(wv_base, N) };
    let zv = unsafe { core::slice::from_raw_parts(zv_base, N) };
    for i in 0..N {
        let z = join_fn(branch_x(i as u32), branch_y(i as u32));
        assert_eq!(zv[i], z, "Zv[{i}] mismatch (phase 0)");
        assert_eq!(wv[i], norm_fn(z), "Wv[{i}] mismatch (phase 1)");
    }

    println!(
        "WORKS: plan-fed schedule-mega ran {N} records with a RUNTIME morsel size \
         (PlanOut.morsel_size, not const) over compile-time phase cons-lists; Zv+Wv correct"
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release fat-LTO cgu=1).
//
// Type-check + run: the plan-fed schedule-mega body compiled with only the two
// RunFiberCol bounds (witnesses inferred), ran 131072 records correct (Zv =
// join(branch_x, branch_y), Wv = norm(Zv)) with morsel size + record count read
// from a runtime `PlanOut` struct (probe_plan returns a black_box'd 1024 so the
// optimizer cannot const-fold it back to an immediate).
//
// Devirt (objdump of `run_schedule_mega_planfed`, 350 instrs):
//   - ZERO `blr` (indirect calls). PASS, the bar.
//   - ZERO `bl` (any call): the whole two-phase body + both runtime-bounded morsel
//     loops + the inline walk are one straight-line body.
//   - 59 vector ops (.4s/.16b/dup): the per-record body STILL auto-vectorizes; the
//     runtime morsel size did NOT block vectorization.
//   - 0 surviving fiber_shim / CollectFiber / run_fiber_col symbols in the binary.
//
// WHAT THIS SETTLES (D1a, the keystone bridge): the per-core schedule-mega body
// devirtualises end to end when FED THE PLAN'S RUNTIME PARAMS (morsel size, the
// R6 adaptive param; record count, consumer data), over compile-time-ordered
// phase cons-lists. The morsel no longer bakes as an immediate (it is a register,
// correctly, per R6 "adaptive at plan time"), and devirt + vectorization both
// survive. This replaces the shipped `run<Witnesses>` runtime-order-via-FiberSlot-
// shim walk (scheduler/mod.rs:715, the 12.6x indirect anti-pattern) with the
// devirtualised body, fed by the plan.
//
// WHAT THIS CONFIRMS ABOUT THE BRIDGE SHAPE (with D1b): the dispatch ORDER is
// compile-time (the flattener-emitted / registration carrier, plan-validated as
// topo-valid); the plan supplies RUNTIME params, not a runtime permutation of the
// type-level order. Turning a runtime permutation array into a compile-time type
// ordering is the codegen flattener's job (D1b, domain 17), not a runtime index
// walk (which would reintroduce the shim indirection). The within-level RCM order
// is the benched ~2% refinement (202606060500), accepted as registration order on
// one core or emitted by the flattener.
//
// WHAT THIS DOES NOT SETTLE: the multi-CORE per-core program (each core's flat
// carrier = its assigned trunk's fibers, trunk-to-core assignment runtime/spectral)
// is the Gate-2 extension; the genuine cross-record phase barrier (reduction/scan)
// needs the non-nil AccumProject tie (proven separately, 202606060730). Both are
// downstream of this single-core bridge, not blocked by it.
// ---------------------------------------------------------------------
