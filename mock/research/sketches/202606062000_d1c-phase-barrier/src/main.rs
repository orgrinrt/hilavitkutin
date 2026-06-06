//! Sketch (D1c / #340, Phase D): phase-sync baked inside a per-core body.
//!
//! D1a (202606061500) proved a two-phase per-core body devirtualises with runtime
//! params. D1c adds the canonical phase boundary: a generation-counter barrier as
//! a real `AtomicUsize` baked into the per-core function (consolidation domain 17
//! item 5, "phase sync points (stack AtomicUsize + spin loop)"). The atomic shape
//! is proven in isolation (202605101036-progress-counter-arena); the open gap is
//! the integration into a multi-phase per-core body, and that the boundary reduces
//! to a cheap Release/Acquire pair, not a seq_cst fence or a CAS spin in the hot
//! path.
//!
//! Hypothesis: a two-phase body with a `PhaseBarrier { gen, arrived }` arrive-and-
//! wait between the phases objdumps to (a) zero `blr` (the dispatch stays
//! devirtualised), and (b) an acquire/release atomic pair at the boundary (ldar /
//! stlr or an `ldadd*`-with-acquire-release variant), NOT a `dmb ish` full fence
//! or a `ldaxr/stlxr` CAS spin on the single-core arrive path. At 1 core the
//! barrier has one arriver (always the last), so the wait path is never taken; the
//! generation bump is a Release store, the post-barrier observe an Acquire load.
//! The morsel loops bake their (here const) bounds. Leeway (section 9): SOME-SHAPE.
//! Outcome at the bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

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

// Proven column-capable inline walk (resource-only accumulator pin).
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
// The phase barrier: a generation-counter AtomicUsize arrive-and-wait. Canonical
// domain 17 item 5. At N cores: each arriver fetch_adds `arrived`; the last
// (arrived+1 == n) resets `arrived` and bumps `gen` (Release); the others spin on
// `gen` changing (Acquire) with a spin hint. At 1 core there is one arriver, which
// is always the last, so the spin path is dead; the boundary is a Release bump +
// an Acquire observe.
// ---------------------------------------------------------------------
struct PhaseBarrier {
    gen: AtomicUsize,
    arrived: AtomicUsize,
}
impl PhaseBarrier {
    const fn new() -> Self {
        Self { gen: AtomicUsize::new(0), arrived: AtomicUsize::new(0) }
    }
    #[inline]
    fn arrive_and_wait(&self, n_cores: usize) {
        let g = self.gen.load(Ordering::Relaxed);
        let prev = self.arrived.fetch_add(1, Ordering::AcqRel);
        if prev + 1 == n_cores {
            // last arriver: reset count, publish the new generation (Release).
            self.arrived.store(0, Ordering::Relaxed);
            self.gen.store(g + 1, Ordering::Release);
        } else {
            // wait for the generation to advance (Acquire), spin-hinted.
            while self.gen.load(Ordering::Acquire) == g {
                core::hint::spin_loop();
            }
        }
    }
}

// The two-phase per-core body with the barrier baked at the boundary.
// #[inline(never)] = clean disasm target.
#[inline(never)]
fn run_two_phase_barrier<const MORSEL: usize, A, P0, W0, P1, W1>(
    phase0: &P0,
    phase1: &P1,
    bindings: &A,
    n: USize,
    barrier: &PhaseBarrier,
    n_cores: usize,
) where
    P0: RunFiberCol<A, W0>,
    P1: RunFiberCol<A, W1>,
{
    let nn = n.0;
    let mut s = 0;
    while s < nn {
        let len = if s + MORSEL <= nn { MORSEL } else { nn - s };
        phase0.run(bindings, MorselRange::new(USize(s), USize(len)));
        s += MORSEL;
    }
    // THE PHASE BOUNDARY: generation-counter barrier (baked AtomicUsize).
    barrier.arrive_and_wait(n_cores);
    let mut s = 0;
    while s < nn {
        let len = if s + MORSEL <= nn { MORSEL } else { nn - s };
        phase1.run(bindings, MorselRange::new(USize(s), USize(len)));
        s += MORSEL;
    }
}

const M1: u32 = 2654435761;
const M2: u32 = 2246822519;
#[inline(always)]
fn s1_fn(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
fn s2_fn(a: u32) -> u32 {
    a.wrapping_mul(M2).wrapping_add(1)
}

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);

type One<T> = Cons<Column<T>, Empty>;

struct P0wu;
impl BuilderInput for P0wu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for P0wu {
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
struct P1wu;
impl BuilderInput for P1wu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for P1wu {
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

const N: usize = 1 << 16;
const MORSEL: usize = 1024;
static BARRIER: PhaseBarrier = PhaseBarrier::new();

fn main() {
    let provider = HeapBump::new(4 * 1024 * 1024);
    let sched = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Av>::new())
        .with(Column::<Bv>::new())
        .with(P0wu)
        .with(P1wu)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build"));
    let in_base = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *mut Inv;
    for i in 0..N {
        unsafe { *in_base.add(i) = Inv(i as u32) };
    }
    let p0 = WuCons { head: P0wu, tail: WuNil };
    let p1 = WuCons { head: P1wu, tail: WuNil };
    // One core: the barrier's single arriver is always last; spin path dead.
    run_two_phase_barrier::<MORSEL, _, _, _, _, _>(&p0, &p1, sched.__bindings(), USize(N), &BARRIER, 1);

    let bv = sched.__bindings().__ptr().as_ptr() as *const u32;
    let bvs = unsafe { core::slice::from_raw_parts(bv, N) };
    for i in 0..N {
        assert_eq!(bvs[i], s2_fn(s1_fn(i as u32)), "Bv[{i}]");
    }
    // gen advanced once (one phase boundary crossed).
    assert_eq!(BARRIER.gen.load(Ordering::Relaxed), 1, "one barrier crossing");

    println!(
        "WORKS: two-phase per-core body with a generation-counter AtomicUsize barrier baked at the \
         boundary ran {N} records, Bv correct, gen advanced once."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28).
//
// The two-phase per-core body with the generation-counter AtomicUsize barrier ran
// 65536 records correct (Bv = s2(s1(i))), gen advanced once. objdump of
// `run_two_phase_barrier`:
//   - blr: 0 (dispatch stays devirtualised through the barrier).
//   - the barrier is exactly TWO atomics: `ldaddal x11, x10, [x10]` (the
//     fetch_add(AcqRel) on `arrived`) + `stlr x9, [x8]` (the Release publish of
//     `gen`). That is the cheap acquire/release pair the spec wants.
//   - NO `dmb` full fence, NO `ldaxr`/`stlxr` CAS spin loop. The single-core
//     arrive path (always-last-arriver) takes the Release-bump branch; the
//     Acquire-spin wait path is dead and emits no CAS.
//
// WHAT THIS SETTLES (D1c): the canonical phase sync point (domain 17 item 5)
// bakes into the per-core function as a generation-counter AtomicUsize barrier
// that reduces to a Release/Acquire atomic pair at the boundary, with the
// dispatch staying devirtualised. At 1 core it is degenerate (one arriver, no
// spin); the SAME code carries the real N-core barrier (the ldaddal counts
// arrivers, the stlr publishes the generation the waiters Acquire-load). No
// `dmb`, no CAS spin in the single-core hot path.
//
// WHAT THIS DOES NOT SETTLE: the actual N-core multi-thread wake (futex / park
// tier selection) lives in the pool mainloop (E2, bench-proven model; E3 barrier
// generation-bit fix). This sketch proves the per-core-body INTEGRATION of the
// barrier primitive + its cheap asm, not the multi-thread wake, which is E-phase.
// ---------------------------------------------------------------------
