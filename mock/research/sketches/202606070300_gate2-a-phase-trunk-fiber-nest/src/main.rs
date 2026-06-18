//! GATE-2 Sketch A: the full phase/trunk/fiber NEST, single-core, devirt + output.
//!
//! Roadmap r3 (`202606070200_engine-roadmap-r3-gate2.md`) step G2-0c and the
//! rechart (`202606070100`). The canonical parallelism is isolated column-disjoint
//! trunks per core (spec `:741-742`, `:769`); before any core-pinning, dispatch
//! must CONSUME the trunk/waist sectioning the plan already computes, by walking a
//! nested `PhaseCons<TrunkCons<FiberCons<WuCons>>>` carrier instead of the flat
//! `WuVals` list `Scheduler::run` walks today.
//!
//! 061400 proved the trunk level (`RunTrunk` over `FiberCons`, delegating to the
//! per-fiber walk) and 081600 proved phase sub-carriers + a waist barrier, each in
//! isolation. Neither proved the full THREE-level nest, nor that its 3-deep witness
//! cons-list infers with no turbofish, nor that the nest wraps the SHIPPED engine
//! `RunFiber` (`hilavitkutin::dispatch::fiber_run`). This sketch composes all three
//! levels over the shipped `RunFiber` and asserts:
//!   (a) output equals the topological computation the flat whole-program walk
//!       produces (output-equivalence: the nest only regroups WUs into
//!       phase/trunk/fiber order, it does not change the computed values), and
//!   (b) the isolated `nest_dispatch` symbol objdumps to zero `blr` (the nest does
//!       not reintroduce an indirect call at any level).
//!
//! Workload (mirrors the canonical multi-trunk + waist shape):
//!   Phase 0, two COLUMN-DISJOINT trunks (the thing that makes them parallelisable
//!     in Sketch B): trunk X = [SX: InX -> AX], trunk Y = [SY: InY -> AY]. AX and AY
//!     are disjoint write columns, so the trunks share no write column (`:742`).
//!   Waist barrier (degenerate one-arriver at single core).
//!   Phase 1, one trunk: [SZ: AX -> CZ], consuming phase 0's output.
//!
//! Single-core: trunks within a phase run sequentially here; at N cores they would
//! be core-pinned and run concurrently (Sketch B, the keystone). The barrier is the
//! 081600 degenerate-one-arriver shape (the SHIPPED `phase_barrier_arrive` becomes
//! load-bearing in Sketch B where real concurrency needs it). Outcome at bottom.

#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
use hilavitkutin::dispatch::fiber_run::RunFiber; // the SHIPPED per-fiber walk
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
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};
use hilavitkutin_providers::ArenaColumnStorage;

// =====================================================================
// The two NEW carrier levels above the shipped per-fiber walk.
//
//   fiber   = WuCons<W, ...> / WuNil       walked by the SHIPPED RunFiber
//   trunk   = FiberCons<F, ...> / FiberNil walked by RunTrunk -> RunFiber
//   phase   = TrunkCons<T, ...> / TrunkNil walked by RunPhase  -> RunTrunk
//   pipeline= PhaseCons<P, ...> / PhaseNil walked by RunPipeline-> RunPhase
//                                          (waist barrier between phases)
//
// Each level's witness parameter is itself a cons-list whose head is the
// next-level-down witness list. The whole thing is a 3-deep nested cons-list,
// inferred with no turbofish at the `nest_dispatch` call (the inference question
// this sketch settles; 061400 proved 2-deep infers).
// =====================================================================

// ---- trunk = list of fibers ----
struct FiberCons<F, Rest> {
    fiber: F,
    rest: Rest,
}
struct FiberNil;

trait RunTrunk<A, WL> {
    fn run(&self, bindings: &A, morsel: MorselRange);
}
impl<A> RunTrunk<A, Empty> for FiberNil {
    #[inline]
    fn run(&self, _b: &A, _m: MorselRange) {}
}
impl<A, F, Rest, FW, RestWL> RunTrunk<A, Cons<FW, RestWL>> for FiberCons<F, Rest>
where
    F: RunFiber<A, FW>,
    Rest: RunTrunk<A, RestWL>,
{
    #[inline]
    fn run(&self, bindings: &A, morsel: MorselRange) {
        self.fiber.run(bindings, morsel);
        self.rest.run(bindings, morsel);
    }
}

// ---- phase = list of trunks ----
struct TrunkCons<T, Rest> {
    trunk: T,
    rest: Rest,
}
struct TrunkNil;

trait RunPhase<A, WL> {
    fn run(&self, bindings: &A, morsel: MorselRange);
}
impl<A> RunPhase<A, Empty> for TrunkNil {
    #[inline]
    fn run(&self, _b: &A, _m: MorselRange) {}
}
impl<A, T, Rest, TW, RestWL> RunPhase<A, Cons<TW, RestWL>> for TrunkCons<T, Rest>
where
    T: RunTrunk<A, TW>,
    Rest: RunPhase<A, RestWL>,
{
    #[inline]
    fn run(&self, bindings: &A, morsel: MorselRange) {
        // Single-core: trunks run sequentially. At N cores each trunk is pinned to
        // a core and these run concurrently with zero sync (disjoint write columns).
        self.trunk.run(bindings, morsel);
        self.rest.run(bindings, morsel);
    }
}

// ---- pipeline = list of phases, waist barrier between ----
struct PhaseCons<P, Rest> {
    phase: P,
    rest: Rest,
}
struct PhaseNil;

trait RunPipeline<A, WL> {
    fn run(&self, bindings: &A, morsel: MorselRange, barrier: &AtomicUsize, expected: usize);
}
impl<A> RunPipeline<A, Empty> for PhaseNil {
    #[inline]
    fn run(&self, _b: &A, _m: MorselRange, _bar: &AtomicUsize, _exp: usize) {}
}
impl<A, P, Rest, PW, RestWL> RunPipeline<A, Cons<PW, RestWL>> for PhaseCons<P, Rest>
where
    P: RunPhase<A, PW>,
    Rest: RunPipeline<A, RestWL>,
{
    #[inline]
    fn run(&self, bindings: &A, morsel: MorselRange, barrier: &AtomicUsize, expected: usize) {
        self.phase.run(bindings, morsel);
        // Waist: all of this phase's trunks complete before the next phase begins.
        // Degenerate one-arriver at single core; the load-bearing N-core barrier is
        // the shipped phase_barrier_arrive (Sketch B).
        waist_barrier(barrier, expected);
        self.rest.run(bindings, morsel, barrier, expected);
    }
}

// 081600's degenerate-one-arriver barrier shape. At expected == 1 it never spins
// (arrived >= expected immediately); present to show the waist composes into the
// nest without breaking devirt. Inlined; no indirect call.
#[inline]
fn waist_barrier(counter: &AtomicUsize, expected: usize) {
    let arrived = counter.fetch_add(1, Ordering::AcqRel) + 1;
    if arrived < expected {
        while counter.load(Ordering::Acquire) < expected {
            core::hint::spin_loop();
        }
    }
}

// A-pinned harness (the Scheduler::run<Witnesses> shape, 061400): `A` is fixed by
// the bindings ref before the nested witness cons-list is inferred at the call.
#[inline(never)]
fn nest_dispatch<A, P, WL>(
    bindings: &A,
    pipeline: &P,
    morsel: MorselRange,
    barrier: &AtomicUsize,
    expected: USize,
) where
    P: RunPipeline<A, WL>,
{
    pipeline.run(bindings, morsel, barrier, expected.0.max(1));
}

// =====================================================================
// Workload WUs. Three pure per-record maps, column-only (no accumulators).
// =====================================================================
const M1: u32 = 2654435761;
const M2: u32 = 2246822519;
#[inline(always)]
fn fx(i: u32) -> u32 {
    i.wrapping_mul(M1)
}
#[inline(always)]
fn fy(i: u32) -> u32 {
    i.wrapping_mul(M2).wrapping_add(1)
}
#[inline(always)]
fn fz(a: u32) -> u32 {
    (a >> 13) ^ a
}

#[derive(Copy, Clone)]
struct InX(u32);
#[derive(Copy, Clone)]
struct AX(u32);
#[derive(Copy, Clone)]
struct InY(u32);
#[derive(Copy, Clone)]
struct AY(u32);
#[derive(Copy, Clone)]
struct CZ(u32);
type One<T> = Cons<Column<T>, Empty>;

// Trunk X, fiber 1: InX -> AX.
#[derive(Copy, Clone)]
struct SX;
impl BuilderInput for SX {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SX {
    type Read = One<InX>;
    type Write = One<AX>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'f> =
        EngineCtx<'f, One<InX>, One<AX>, PtrNil, ColPtrCons<InX, ColPtrNil>, ColPtrCons<AX, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: InX host-populated; AX reserved + exclusively written; morsel-bounded.
            let v = unsafe { ctx.reader().read::<InX, _>(i) };
            unsafe { ctx.writer().write::<AX, _>(i, AX(fx(v.0))) };
        });
    }
}

// Trunk Y, fiber 1: InY -> AY. Disjoint write column from trunk X.
#[derive(Copy, Clone)]
struct SY;
impl BuilderInput for SY {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SY {
    type Read = One<InY>;
    type Write = One<AY>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'f> =
        EngineCtx<'f, One<InY>, One<AY>, PtrNil, ColPtrCons<InY, ColPtrNil>, ColPtrCons<AY, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            let v = unsafe { ctx.reader().read::<InY, _>(i) };
            unsafe { ctx.writer().write::<AY, _>(i, AY(fy(v.0))) };
        });
    }
}

// Phase 1 trunk, fiber 1: AX -> CZ. Reads phase 0's output (available after waist).
#[derive(Copy, Clone)]
struct SZ;
impl BuilderInput for SZ {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SZ {
    type Read = One<AX>;
    type Write = One<CZ>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'f> =
        EngineCtx<'f, One<AX>, One<CZ>, PtrNil, ColPtrCons<AX, ColPtrNil>, ColPtrCons<CZ, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            let a = unsafe { ctx.reader().read::<AX, _>(i) };
            unsafe { ctx.writer().write::<CZ, _>(i, CZ(fz(a.0))) };
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
    // Register columns CZ, AY, AX, InY, InX (prepend order: InX is bindings head).
    // Then units SX, SY, SZ: the flat carrier [SX, SY, SZ] is topological (SZ reads
    // AX which SX writes), which `build` validates. The nested pipeline below
    // regroups these same three WUs into phase/trunk/fiber order; the computed
    // values are identical, which is the output-equivalence claim.
    let sched = Scheduler::builder()
        .with(Column::<CZ>::new())
        .with(Column::<AY>::new())
        .with(Column::<AX>::new())
        .with(Column::<InY>::new())
        .with(Column::<InX>::new())
        .with(SX)
        .with(SY)
        .with(SZ)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("engine build should succeed"));

    // Host-populate InX[i] = i (bindings head) and InY[i] = i (its tail, after AX).
    // bindings prepend-reverse order: InX -> InY -> AX -> AY -> CZ.
    let inx_base = sched.__bindings().__ptr().as_ptr() as *mut InX;
    for i in 0..N {
        // SAFETY: InX reserved for N records; storage alive; one write each.
        unsafe { *inx_base.add(i) = InX(i as u32) };
    }
    let iny_base = sched.__bindings().__tail().__ptr().as_ptr() as *mut InY;
    for i in 0..N {
        // SAFETY: InY reserved for N records; storage alive; one write each.
        unsafe { *iny_base.add(i) = InY(i as u32) };
    }

    // Build the nested pipeline value.
    //   Phase 0: trunk X [SX] and trunk Y [SY], column-disjoint.
    //   Phase 1: trunk Z [SZ].
    let trunk_x = FiberCons { fiber: WuCons { head: SX, tail: WuNil }, rest: FiberNil };
    let trunk_y = FiberCons { fiber: WuCons { head: SY, tail: WuNil }, rest: FiberNil };
    let phase0 = TrunkCons { trunk: trunk_x, rest: TrunkCons { trunk: trunk_y, rest: TrunkNil } };
    let trunk_z = FiberCons { fiber: WuCons { head: SZ, tail: WuNil }, rest: FiberNil };
    let phase1 = TrunkCons { trunk: trunk_z, rest: TrunkNil };
    let pipeline = PhaseCons { phase: phase0, rest: PhaseCons { phase: phase1, rest: PhaseNil } };

    let barrier = AtomicUsize::new(0);
    // Drive the full nest through the isolated symbol. The 3-deep witness cons-list
    // is inferred with no turbofish (the inference question this sketch settles).
    nest_dispatch(
        sched.__bindings(),
        &pipeline,
        MorselRange::new(USize(0), USize(N)),
        &barrier,
        USize(1),
    );

    // Verify output-equivalence: the nest computed exactly what the flat topological
    // walk over [SX, SY, SZ] would. AX = fx(InX), AY = fy(InY), CZ = fz(AX).
    let ax_base = sched.__bindings().__tail().__tail().__ptr().as_ptr() as *const u32;
    let ay_base = sched.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    let cz_base =
        sched.__bindings().__tail().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: AX, AY, CZ each reserved for N records; storage alive; written every record.
    let ax = unsafe { core::slice::from_raw_parts(ax_base, N) };
    let ay = unsafe { core::slice::from_raw_parts(ay_base, N) };
    let cz = unsafe { core::slice::from_raw_parts(cz_base, N) };
    for i in 0..N {
        assert_eq!(ax[i], fx(i as u32), "AX[{i}] (phase 0, trunk X)");
        assert_eq!(ay[i], fy(i as u32), "AY[{i}] (phase 0, trunk Y)");
        assert_eq!(cz[i], fz(fx(i as u32)), "CZ[{i}] (phase 1, trunk Z, reads phase-0 AX)");
    }

    println!(
        "WORKS: drove {N} records through a PhaseCons<TrunkCons<FiberCons<WuCons>>> nest. \
         Phase 0 ran two column-disjoint trunks (X: InX->AX, Y: InY->AY), waist barrier \
         (degenerate 1-arriver), phase 1 ran trunk Z (AX->CZ). All columns correct = \
         output-equivalent to the flat topological walk. 3-deep witness cons-list inferred \
         with no turbofish. objdump nest_dispatch for zero blr."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release, fat LTO, cgu=1).
//
// Ran 256 records through the full PhaseCons<TrunkCons<FiberCons<WuCons>>> nest.
// Phase 0 ran two column-disjoint trunks (X: InX->AX, Y: InY->AY), waist barrier
// (degenerate one-arriver at single core), phase 1 ran trunk Z (AX->CZ). All three
// output columns correct: AX[i]=fx(i), AY[i]=fy(i), CZ[i]=fz(fx(i)) for all i,
// i.e. bit-identical to the topological computation the flat whole-program walk
// produces. The nest only regroups the same three WUs into phase/trunk/fiber
// order; it does not change the values (output-equivalence, the G2-0 oracle).
//
// 3-deep witness inference: the nested witness cons-list
// Cons<Cons<Cons<(quad), ...>, ...>, ...> inferred with NO turbofish at the
// nest_dispatch call. 061400 proved 2-deep (trunk over fibers) infers; this
// settles the full phase->trunk->fiber 3-deep nest.
//
// Devirt: objdump of the isolated `nest_dispatch` mono = 590 instructions, ZERO
// blr (indirect call), ZERO br (indirect branch), ZERO bl (direct call). All four
// walk levels (RunPipeline -> RunPhase -> RunTrunk -> shipped RunFiber -> execute)
// plus the waist barrier fold into one straight-line body. The nest reintroduces
// no indirection at any level.
//
// SETTLES (roadmap r3 G2-0c, Sketch A): dispatch can consume the trunk/waist
// sectioning by walking a type-level PhaseCons<TrunkCons<FiberCons<WuCons>>> nest
// built over the SHIPPED RunFiber, output-equivalent to the flat walk, devirt
// preserved. The build-time NEST shape is proven; per-frame record ranges / morsel
// bounds stay runtime plan params fed in as MorselRange (here whole-range at one
// core), not baked into the type (the r3 "two senses of compile-time" note). The
// real round builds this from the plan's PhaseBoundaries + BlockPartition + fiber
// grouping (grouping is plan-computed; 061400 Tier 3 proved full type-level
// grouping derivation needs forbidden specialization, so the carrier TYPE carries
// the plan's result, hybrid). Sketch B proves the N-core step: these column-
// disjoint trunks run concurrently, one per core, zero sync, waist barrier between
// phases.
// ---------------------------------------------------------------------
