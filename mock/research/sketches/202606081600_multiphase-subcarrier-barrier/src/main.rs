//! Sketch (§7-2 of roadmap r2 / GATE-1): does a MULTI-PHASE per-core program
//! devirtualise, where phases are separate type-level sub-carriers with a phase
//! barrier between, each phase its own morsel loop?
//!
//! The question r2 §3 flagged: a cons-list TYPE cannot be sliced at a runtime
//! index, so the flat schedule-mega carrier cannot be cut into "phase 0 WUs |
//! barrier | phase 1 WUs" at a runtime phase boundary. The resolution candidate:
//! phases are a TYPE-LEVEL list (like fibers in 202606061400's RunTrunk), each
//! phase a `WuCons` walked under its own morsel loop, with a phase barrier element
//! between. This composes two already-proven pieces: 202606081200 (a type-level
//! walk devirts under a runtime morsel loop) and 202606062000 (the D1c
//! generation-counter barrier objdumps zero blr + ldaddal/stlr, degenerate at one
//! arriver). The open question is whether COMPOSING them, two per-phase sub-carrier
//! walks with a real `AtomicUsize` barrier between, in one `#[inline(never)]`
//! per-core function, still objdumps zero blr (no indirection introduced at the
//! phase seam).
//!
//! Hypothesis: yes. Each phase's morsel-outer walk is the proven devirt shape; the
//! barrier is a stack `AtomicUsize` Release/Acquire pair (degenerate one-arriver at
//! single core); neither introduces an indirect call. The phase structure is a
//! compile-time type (here hand-built as two sub-carriers, the GATE-1 shape the
//! flattener emits from the plan's phase grouping), NOT a runtime slice of a flat
//! carrier. If zero blr, multi-phase dispatch is resolved for GATE-1: per-phase
//! sub-carriers walked sequentially, barrier between, no slicing needed.
//!
//! Phase boundary modelled: phase 0 produces Av (Inv -> Av, morsel-outer); the
//! barrier publishes phase-0 completion; phase 1 consumes Av (Av -> Out,
//! morsel-outer) and only runs after the barrier. At single core the barrier is a
//! degenerate one-arriver and sequential execution already orders the phases; the
//! barrier is present so the SAME code carries N-core (where it is load-bearing).
//! Correctness (Out[i] = g(f(Inv[i]))) confirms phase 1 ran after phase 0. Real
//! engine crates; RunFiberCol restated from 202606081200. Outcome at bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::hint::black_box;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccumProject, ColPtrCons, ColPtrNil, ColProject, EngineCtx, Project, PtrNil,
};
use hilavitkutin::dispatch::{WuCons, WuNil};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

// Proven type-level fiber walk (RunFiberCol), restated from 202606081200.
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

// One phase's morsel-outer drive (the proven 202606081200 shape).
#[inline]
fn run_phase<A, F, WL>(bindings: &A, phase: &F, total: usize, step: usize)
where
    F: RunFiberCol<A, WL>,
{
    let mut start = 0;
    while start < total {
        let len = step.min(total - start);
        phase.run(bindings, MorselRange::new(USize(start), USize(len)));
        start += len;
    }
}

// The D1c phase barrier (202606062000 shape): a stack AtomicUsize generation
// counter. The arriving core fetch_adds (Release-publishes its completion) and
// spins until all cores of the phase have arrived. At single core this is a
// degenerate one-arriver: arrive == expected immediately, no spin. Inlined; no
// indirect call.
#[inline]
fn phase_barrier(counter: &AtomicUsize, expected: usize) {
    let arrived = counter.fetch_add(1, Ordering::AcqRel) + 1;
    if arrived < expected {
        while counter.load(Ordering::Acquire) < expected {
            core::hint::spin_loop();
        }
    }
}

// THE PER-CORE PROGRAM (the asm-checklist target). Two PHASES as separate
// type-level sub-carriers, each walked morsel-outer, with a real AtomicUsize
// barrier between. `#[inline(never)]` isolates the symbol; the inner phase walks +
// barrier still fold IN. objdump `per_core_program`: zero blr is the bar. The
// phase structure is two distinct compile-time types (P0, P1), NOT a runtime slice
// of a flat carrier; this is what the flattener emits from the plan phase grouping.
#[inline(never)]
fn per_core_program<A, P0, P1, WL0, WL1>(
    bindings: &A,
    phase0: &P0,
    phase1: &P1,
    total: USize,
    morsel_size: USize,
    barrier: &AtomicUsize,
    cores: USize,
) where
    P0: RunFiberCol<A, WL0>,
    P1: RunFiberCol<A, WL1>,
{
    let total = black_box(total).0;
    let step = black_box(morsel_size).0.max(1);
    let expected = black_box(cores).0.max(1);
    // Phase 0: its own morsel loop.
    run_phase(bindings, phase0, total, step);
    // Phase boundary barrier (degenerate one-arriver at single core).
    phase_barrier(barrier, expected);
    // Phase 1: its own morsel loop, runs only after the barrier.
    run_phase(bindings, phase1, total, step);
}

const M1: u32 = 2654435761;
const ADD: u32 = 12345;
#[inline(always)]
fn f(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
fn g(a: u32) -> u32 {
    a.wrapping_add(ADD)
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Out(u32);
type One<T> = Cons<Column<T>, Empty>;

// Phase 0: Producer, Inv -> Av.
struct Producer;
impl BuilderInput for Producer {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Producer {
    type Read = One<Inv>;
    type Write = One<Av>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'f> =
        EngineCtx<'f, One<Inv>, One<Av>, PtrNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: Inv host-populated; Av reserved + exclusively written; morsel-bounded.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(f(inp.0))) };
        });
    }
}

// Phase 1: Consumer, Av -> Out. Reads what phase 0 produced.
struct Consumer;
impl BuilderInput for Consumer {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Consumer {
    type Read = One<Av>;
    type Write = One<Out>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'f> =
        EngineCtx<'f, One<Av>, One<Out>, PtrNil, ColPtrCons<Av, ColPtrNil>, ColPtrCons<Out, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: Av written by phase 0 (complete before this phase via the
            // barrier / sequential single-core ordering); Out reserved + exclusive.
            let a = unsafe { ctx.reader().read::<Av, _>(i) };
            unsafe { ctx.writer().write::<Out, _>(i, Out(g(a.0))) };
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
    unsafe fn deallocate(&self, _p: *mut u8, _l: USize) {}
    unsafe fn protect(&self, _p: *mut u8, _l: USize, _r: Bool, _w: Bool) {}
}
fn store<M: MemoryProviderApi>(p: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(p)
}

const N: usize = 256;

fn main() {
    let provider = BumpProvider::<262144>::new();
    let sched = Scheduler::builder()
        .with(Column::<Out>::new())
        .with(Column::<Av>::new())
        .with(Column::<Inv>::new())
        .with(Producer)
        .with(Consumer)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    let in_base = sched.__bindings().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        // SAFETY: Inv reserved for N records; storage alive; one write each.
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }

    // Two phases as separate type-level sub-carriers.
    let phase0 = WuCons { head: Producer, tail: WuNil };
    let phase1 = WuCons { head: Consumer, tail: WuNil };
    // Stack barrier, single core (one arriver, degenerate).
    let barrier = AtomicUsize::new(0);

    per_core_program(
        sched.__bindings(),
        &phase0,
        &phase1,
        USize(N),
        USize(32),
        &barrier,
        USize(1),
    );

    // Verify phase 1 ran after phase 0: Out[i] = g(f(Inv[i])).
    let av_base = sched.__bindings().__tail().__ptr().as_ptr() as *const u32;
    let out_base = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: Av, Out reserved for N records; storage alive; written every record.
    let av = unsafe { core::slice::from_raw_parts(av_base, N) };
    let out = unsafe { core::slice::from_raw_parts(out_base, N) };
    for i in 0..N {
        assert_eq!(av[i], f(i as u32), "Av[{i}] (phase 0)");
        assert_eq!(out[i], g(f(i as u32)), "Out[{i}] (phase 1, after barrier)");
    }
    println!(
        "ran {N} records through a 2-phase per-core program: phase 0 (Producer) morsel-outer, \
         AtomicUsize barrier (1-core degenerate), phase 1 (Consumer) morsel-outer; cross-phase \
         data correct. objdump per_core_program for zero blr."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS, on nightly-2026-05-28 (release, fat LTO, cgu=1).
//
// Ran 256 records; Av (phase 0) and Out (phase 1) both correct: Out[i] =
// g(f(Inv[i])), confirming phase 1 ran after phase 0.
//
// objdump per_core_program: 173 instrs, ZERO blr, ZERO bl. The two per-phase
// sub-carrier walks (each the proven 202606081200 morsel-outer type-level walk)
// and the AtomicUsize phase barrier between them all fold into a flat body with no
// indirect call.
//
// RESOLVES r2 §3's multi-phase / "can't slice a cons-list type at a runtime phase
// index" question. The per-core program runs phase 0 (its own morsel loop) ->
// barrier -> phase 1 (its own morsel loop) by carrying the phases as SEPARATE
// compile-time type-level sub-carriers (here P0 = WuCons<Producer>, P1 =
// WuCons<Consumer>), NOT by slicing a flat carrier at a runtime boundary. This is
// the RunTrunk shape (202606061400) at phase granularity composed with the D1c
// barrier (202606062000); both were proven, and composing them keeps zero blr.
//
// So multi-phase dispatch for GATE-1 = a type-level list of phases (PhaseCons-
// shaped, the flattener emits it from the plan's phase grouping the same way it
// emits fiber grouping: from append+validate registration structure, not a runtime
// slice). For a single-phase pipeline (no waist) D1a's one flat carrier suffices;
// multi-phase nests into per-phase sub-carriers with barriers between. The barrier
// is degenerate (one arriver) at single core and load-bearing at N-core; the same
// code carries both. No per-record indirection at the phase seam.
// ---------------------------------------------------------------------
