//! Sketch (#669 keystone, the decisive missing proof): does the TYPE-LEVEL
//! cons-list walk devirtualise UNDER A REAL RUNTIME MORSEL LOOP?
//!
//! Every prior devirt proof in this arc tested the wrong condition:
//!   - D1a (202606061500), D1b (202606061400), runtime-order (202606071400) all
//!     drove the dispatch ONCE over a single morsel range, or fully unrolled. No
//!     runtime `for morsel in 0..count` loop wrapped the walk.
//!   - The per-fiber-segmented sketch (202606080300) DID wrap a runtime morsel
//!     loop around the dispatch, and got 2 `blr` -- but it dispatched a fn-POINTER
//!     ARRAY `slots[order[k]](acc)`. The spec lists exactly that as a FAIL mode
//!     (struct-field 12.6x, `&[fn; N]` 5.8x, `run_fiber(&[WuFn])` one-fn-for-all).
//!
//! So the open question that gates the whole GATE-1 dispatch: when the carrier is
//! a TYPE-LEVEL cons-list (concrete WU types, `head.execute(); tail.run()`
//! recursion, the spec's Approach A/E "per-fiber-type trait"), and it is driven
//! inside a genuine runtime morsel loop whose trip count is opaque to the
//! optimiser (`black_box`), do the per-WU bodies still inline to direct calls
//! (zero `blr`), or does wrapping a runtime loop around the type-walk reintroduce
//! indirection the way it did for the fn-pointer array?
//!
//! Hypothesis: the type-level walk devirtualises under the morsel loop where the
//! fn-pointer array did not, because each `head.execute(&ctx)` is a call to a
//! statically-known concrete type's method (monomorphised, no pointer to hold in
//! a register across the loop), so LLVM inlines it into the loop body regardless
//! of the runtime trip count. If TRUE, the dual-agent consensus shape (append +
//! validate-topo + type-level walk, no fn-ptr shim) is fully de-risked and #669
//! reduces to a mechanical engine rewire. If FALSE, nothing devirtualises under a
//! morsel loop and the keystone is in genuine trouble (escalate).
//!
//! Faithfulness: real `EngineCtx`/`Project`/`ColProject`/`AccumProject`, real
//! `WuCons`/`WuNil`, real `Scheduler` build + bindings, real `Column`/`Accum`
//! stores. The walk traits (`RunFiberCol`, `RunTrunk`) are restated from the
//! proven D1b shape (202606061400). The ONLY new thing is the runtime morsel loop
//! wrapping the dispatch, plus the per-fiber morsel-outer-vs-unit-outer split (the
//! locked 202606051500 distinction): the accumulator-free producer/consumer fiber
//! is driven MORSEL-OUTER (the morsel loop is outside, the fiber walk inside, run
//! per morsel), the accumulator fiber is driven UNIT-OUTER (one full-range walk).
//! Outcome recorded at the bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::hint::black_box;
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, AccumProject, ColPtrCons, ColPtrNil, ColProject, EngineCtx, Project,
    PtrNil,
};
use hilavitkutin::dispatch::{WuCons, WuNil};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasAccumWriter, HasColumnReader,
    HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

// =====================================================================
// The proven type-level fiber walk (RunFiberCol, sketch 202606060730/202606061400).
// One fiber's WU cons-list, projecting each WU's EngineCtx (4 witnesses per WU),
// A-pinned by the caller. `head.execute(); tail.run()`: direct calls on concrete
// types, no fn pointers anywhere.
// =====================================================================
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
    for<'f> A: AccumProject<'f, <W as WorkUnit>::Write, WAIdx>,
    for<'f> W: WorkUnit<
        Ctx<'f> = EngineCtx<
            'f,
            <W as WorkUnit>::Read,
            <W as WorkUnit>::Write,
            <A as Project<<W as WorkUnit>::Read, RIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Read, RCIdx>>::Out,
            <A as ColProject<<W as WorkUnit>::Write, WCIdx>>::Out,
            <A as AccumProject<'f, <W as WorkUnit>::Write, WAIdx>>::Out,
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

// =====================================================================
// THE NEW THING: drive the type-level fiber under a REAL runtime morsel loop.
//
// `dispatch_morsel_outer` is the per-core-program shape for an accumulator-free
// fiber (locked 202606051500: morsel-outer = cache-resident intermediates). The
// morsel loop trip count comes from `black_box` so the optimiser cannot unroll or
// const-fold it away; this is the exact condition the fn-pointer array failed
// under (202606080300, 2 blr). `#[inline(never)]` isolates the symbol for the
// objdump asm-checklist; the inner RunFiberCol/execute calls still fold IN.
//
// objdump the monomorphised `dispatch_morsel_outer` symbol: zero `blr` is the bar.
// =====================================================================
#[inline(never)]
fn dispatch_morsel_outer<A, F, WL>(
    bindings: &A,
    fiber: &F,
    total: USize,
    morsel_size: USize,
) where
    F: RunFiberCol<A, WL>,
{
    // Runtime morsel loop. `n_morsels` is opaque (computed from black_box'd
    // inputs) so the loop is a genuine runtime loop, not an unrolled sequence.
    let total = black_box(total).0;
    let step = black_box(morsel_size).0.max(1);
    let mut start = 0usize;
    while start < total {
        let len = step.min(total - start);
        fiber.run(bindings, MorselRange::new(USize(start), USize(len)));
        start += len;
    }
}

// `dispatch_unit_outer` is the accumulator-fiber shape (locked 202606051500:
// unit-outer = cross-record-safe). One full-range walk; the WU's own `each()`
// loops records internally. Still the type-level walk, isolated for objdump.
#[inline(never)]
fn dispatch_unit_outer<A, F, WL>(bindings: &A, fiber: &F, total: USize)
where
    F: RunFiberCol<A, WL>,
{
    let total = black_box(total);
    fiber.run(bindings, MorselRange::new(USize(0), total));
}

// =====================================================================
// Workload (same two-fiber shape as D1b).
//   Fiber 1 (accumulator-free, MORSEL-OUTER): S1 (Inv -> Av) -> Cons (Av -> Av2).
//            An intermediate column Av flows S1 -> Cons within the fiber: the
//            morsel-outer drive keeps Av cache-resident per morsel.
//   Fiber 2 (accumulator, UNIT-OUTER): Tally (Av2 -> Accum<Sum>).
// =====================================================================
const M1: u32 = 2654435761;
const M2: u32 = 40503;

#[inline(always)]
fn stage1(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
fn stage2(i: u32) -> u32 {
    i.wrapping_add(M2)
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Av2(u32);
#[derive(Copy, Clone)]
struct Sum(u32);

type One<T> = Cons<Column<T>, Empty>;
type AccW = Cons<Accum<Sum>, Empty>;

struct S1;
impl BuilderInput for S1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S1 {
    type Read = One<Inv>;
    type Write = One<Av>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Inv>,
        One<Av>,
        PtrNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Av, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Inv host-populated; Av reserved + exclusively written; morsel
            // covers only reserved records.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(stage1(inp.0))) };
        });
    }
}

struct Cons2;
impl BuilderInput for Cons2 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Cons2 {
    type Read = One<Av>;
    type Write = One<Av2>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Av>,
        One<Av2>,
        PtrNil,
        ColPtrCons<Av, ColPtrNil>,
        ColPtrCons<Av2, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Av written by S1 this morsel; Av2 reserved + exclusively written.
            let a = unsafe { ctx.reader().read::<Av, _>(i) };
            unsafe { ctx.writer().write::<Av2, _>(i, Av2(stage2(a.0))) };
        });
    }
}

struct Tally;
impl BuilderInput for Tally {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Tally {
    type Read = One<Av2>;
    type Write = AccW;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<Av2>,
        AccW,
        PtrNil,
        ColPtrCons<Av2, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Sum, AccPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let a = unsafe { ctx.reader().read::<Av2, _>(i) };
            // SAFETY: Accum<Sum> reserved for the record count; exclusive appender.
            unsafe { ctx.accums().append::<Sum, _>(Sum(a.0)) };
        });
    }
}

struct BumpProvider<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}
impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self { buf: UnsafeCell::new([const { MaybeUninit::uninit() }; N]), used: Cell::new(0) }
    }
}
unsafe impl<const N: usize> Send for BumpProvider<N> {}
unsafe impl<const N: usize> Sync for BumpProvider<N> {}
impl<const N: usize> MemoryProviderApi for BumpProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = self.buf.get() as *mut u8;
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > N {
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

const N: usize = 256;

fn main() {
    let provider = BumpProvider::<262144>::new();
    // Register stores then units. Prepend order makes Inv the bindings head.
    let sched = Scheduler::builder()
        .with(Accum::<Sum>::new())
        .with(Column::<Av2>::new())
        .with(Column::<Av>::new())
        .with(Column::<Inv>::new())
        .with(S1)
        .with(Cons2)
        .with(Tally)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // Host-populate Inv[i] = i (bindings head).
    let in_base = sched.__bindings().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        // SAFETY: Inv reserved for N records; storage alive; each slot written once.
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }

    // Fiber 1 (accumulator-free): S1 -> Cons2. Driven MORSEL-OUTER under the real
    // runtime morsel loop. morsel_size opaque so the loop genuinely iterates.
    let fiber1 = WuCons { head: S1, tail: WuCons { head: Cons2, tail: WuNil } };
    dispatch_morsel_outer(sched.__bindings(), &fiber1, USize(N), USize(32));

    // Fiber 2 (accumulator): Tally. Driven UNIT-OUTER (one full-range walk).
    let fiber2 = WuCons { head: Tally, tail: WuNil };
    dispatch_unit_outer(sched.__bindings(), &fiber2, USize(N));

    // Verify: Av = stage1(i), Av2 = stage2(stage1(i)), Sum[i] = Av2[i].
    let av_base = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
    let av2_base = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: Av, Av2 reserved for N records; storage alive; written every record.
    let av = unsafe { core::slice::from_raw_parts(av_base, N) };
    let av2 = unsafe { core::slice::from_raw_parts(av2_base, N) };
    for i in 0..N {
        assert_eq!(av[i], stage1(i as u32), "Av[{i}] (fiber1 S1, morsel-outer)");
        assert_eq!(av2[i], stage2(stage1(i as u32)), "Av2[{i}] (fiber1 Cons2, morsel-outer)");
    }
    let sum_binding = sched.__bindings().__tail().__tail().__tail();
    let sum_len = sum_binding.__len_cell().get().0;
    assert_eq!(sum_len, N, "accum live length should be N (fiber2 Tally, unit-outer)");
    let sum_base = sum_binding.__ptr().as_ptr() as *const u32;
    // SAFETY: Sum reserved for N records; storage alive; Tally appended N values.
    let sums = unsafe { core::slice::from_raw_parts(sum_base, N) };
    for i in 0..N {
        assert_eq!(sums[i], stage2(stage1(i as u32)), "Sum[{i}] (fiber2 Tally)");
    }

    println!(
        "ran {N} records: fiber1 (S1->Cons2) morsel-outer under a runtime morsel loop, fiber2 \
         (Tally) unit-outer; all columns + accumulator correct. objdump dispatch_morsel_outer / \
         dispatch_unit_outer for zero blr."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS, on nightly-2026-05-28 (release, fat LTO, cgu=1).
//
// Ran 256 records; Av, Av2 columns and the Sum accumulator all correct.
//
// objdump of the two isolated dispatch symbols:
//   dispatch_morsel_outer: 140 instructions, ZERO `blr`, ZERO `bl`. The S1
//     (multiply by M1=0x9e3779b1) and Cons2 (add M2) WU bodies fuse into the
//     morsel loop AND auto-vectorise: `ldp q2, q3` 128-bit vector loads, four
//     `add.4s vN, vN, v0` NEON lanes, `stp q2, q3` vector stores, with a scalar
//     `b.ne` tail. The morsel loop is a genuine runtime loop (black_box'd trip
//     count, `subs`/`b.ne`) and the walk still devirtualised + vectorised.
//   dispatch_unit_outer: ZERO `blr`, ZERO `bl`. The Tally accumulator body
//     (read Av2, append Sum) inlines to a scalar `ldr`/`str`/`add` loop.
//
// DECISIVE: the TYPE-LEVEL cons-list walk devirtualises under a REAL runtime
// morsel loop, which is exactly the condition the fn-POINTER ARRAY failed under
// (sketch 202606080300: `slots[order[k]](acc)` in a runtime morsel loop -> 2
// `blr`). The difference is the mechanism, not the loop: a fn-ptr array holds the
// pointers in registers across iterations (indirect call); the type-level walk
// has no pointer to hold (each `head.execute()` is a statically-known concrete
// type's method, inlined + fused into the loop body). This matches domain 17:
// Approach A "per-fiber-type trait" 1.0x vs the `&[fn;N]`/struct-field/
// `run_fiber(&[WuFn])` FAIL modes (5.8x-12.6x).
//
// This was the one missing devirt proof: D1a (202606061500), D1b (202606061400),
// runtime-order (202606071400) all dispatched ONCE / unrolled with no runtime
// morsel loop. With the morsel loop now proven, the dual-agent consensus shape
// (append + validate-topo registration order + type-level walk, delete the
// FiberSlot fn-ptr shim) is fully de-risked. Per-fiber morsel-outer (fiber1) and
// unit-outer (fiber2) BOTH devirt as separate type-level walks.
//
// REMAINING (mechanism, not feasibility): how does `run` obtain the per-fiber /
// per-phase TYPE-LEVEL sub-walks? A cons-list type cannot be sliced at a runtime
// index, so either (a) GATE-1 ships the whole-program flat schedule-mega (spec
// Approach E, >10K records): one carrier, one morsel-outer walk, accumulators
// separated by phase barriers; or (b) a nested FiberCons (Approach A, <10K) whose
// fiber TYPES are constructed by consumer-declared grouping (this sketch builds
// fiber1/fiber2 by hand exactly that way). Both devirt (proven here + D1b). The
// flat-vs-nested split is spec-driven by record count (select_approach) and
// bench-decidable; it is NOT a feasibility blocker.
// ---------------------------------------------------------------------
