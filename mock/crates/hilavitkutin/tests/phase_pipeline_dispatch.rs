//! Phase/pipeline-nesting dispatch test (GATE-2 G2-0b, roadmap r3 stage G2-0b).
//!
//! `RunPhase<A, WL>` walks a phase (`TrunkCons` / `TrunkNil`, a list of trunks)
//! delegating each trunk to the landed `RunTrunk`; `RunPipeline<A, WL>` walks
//! the pipeline (`PhaseCons` / `PhaseNil`, a list of phases) running each phase
//! via `RunPhase` then arriving at a waist barrier before the next phase. This
//! is the outer of the `PhaseCons<TrunkCons<FiberCons<WuCons>>>` nest proven by
//! sketch `202606070300_gate2-a-phase-trunk-fiber-nest`, built on the G2-0a
//! `RunTrunk`.
//!
//! The output-equivalence contract: a `RunPipeline` walk over a two-phase
//! pipeline produces column state bit-identical to the shipped flat
//! `Scheduler::run` over the same units. The fixture mirrors sketch A: phase 0
//! has two column-disjoint trunks (`SX`: `InX -> AX`; `SY`: `InY -> AY`), phase
//! 1 has one trunk (`SZ`: `AX -> CZ`) reading phase 0's output across the waist.
//! One scheduler runs the flat walk; a second drives `RunPipeline` over a
//! hand-built nest through an `#[inline(never)]` harness with a degenerate
//! one-arriver `AtomicUsize` waist barrier, inferring the 3-deep witness
//! cons-list with no turbofish. Both paths must compute the same columns.
//!
//! Red first: `RunPhase` / `RunPipeline` / `TrunkCons` / `PhaseCons` do not
//! exist until this round lands them, so the file does not compile.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::AtomicUsize;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::dispatch::phase_run::RunPipeline;
use hilavitkutin::dispatch::{FiberCons, FiberNil, PhaseCons, PhaseNil, TrunkCons, TrunkNil, WuCons, WuNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
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
        // SAFETY: `aligned + len <= N`, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

const N: usize = 256;

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

// Phase 0, trunk X: InX -> AX.
#[derive(Copy, Clone)]
struct SX;
impl BuilderInput for SX {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SX {
    type Read = One<InX>;
    type Write = One<AX>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'f> =
        EngineCtx<'f, One<InX>, One<AX>, SnapNil, ColPtrCons<InX, ColPtrNil>, ColPtrCons<AX, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: InX host-populated; AX reserved + exclusively written; morsel-bounded.
            let v = unsafe { ctx.reader().read::<InX, _>(i) };
            unsafe { ctx.writer().write::<AX, _>(i, AX(fx(v.0))) };
        });
    }
}

// Phase 0, trunk Y: InY -> AY. Disjoint write column from trunk X.
#[derive(Copy, Clone)]
struct SY;
impl BuilderInput for SY {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SY {
    type Read = One<InY>;
    type Write = One<AY>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'f> =
        EngineCtx<'f, One<InY>, One<AY>, SnapNil, ColPtrCons<InY, ColPtrNil>, ColPtrCons<AY, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: InY host-populated; AY reserved + exclusively written; morsel-bounded.
            let v = unsafe { ctx.reader().read::<InY, _>(i) };
            unsafe { ctx.writer().write::<AY, _>(i, AY(fy(v.0))) };
        });
    }
}

// Phase 1, trunk Z: AX -> CZ. Reads phase 0's output (available after waist).
#[derive(Copy, Clone)]
struct SZ;
impl BuilderInput for SZ {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SZ {
    type Read = One<AX>;
    type Write = One<CZ>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'f> =
        EngineCtx<'f, One<AX>, One<CZ>, SnapNil, ColPtrCons<AX, ColPtrNil>, ColPtrCons<CZ, ColPtrNil>>;
    fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
        ctx.each().run(|i| {
            // SAFETY: SX (phase 0, before the waist) wrote every AX; CZ reserved + exclusive.
            let a = unsafe { ctx.reader().read::<AX, _>(i) };
            unsafe { ctx.writer().write::<CZ, _>(i, CZ(fz(a.0))) };
        });
    }
}

// Pipeline-level harness mirroring the sketch-A `nest_dispatch`: `A` is fixed by
// the bindings ref before the 3-deep witness cons-list is inferred at the call.
#[inline(never)]
fn pipeline_dispatch<A, P, WL>(
    bindings: &A,
    pipeline: &P,
    morsel: MorselRange,
    barrier: &AtomicUsize,
    expected: USize,
) where
    P: RunPipeline<A, WL>,
{
    // Epoch 1: this pipeline-equivalence harness uses Always WUs only, whose
    // gate const-folds true regardless of epoch (E4 slice 1).
    let meta = hilavitkutin::meta::MetaBlock::default();
    pipeline.run(bindings, &meta, morsel, barrier, expected, USize(1)); // lint:allow(no-bare-numeric) reason: test epoch literal; tracked: #121
}

// Macro to register the identical 5-column 3-unit scheduler (the bindings-walk
// order is InX, InY, AX, AY, CZ from the prepend), used for both paths.
macro_rules! build_sched {
    () => {
        Scheduler::builder()
            .with(Column::<CZ>::new())
            .with(Column::<AY>::new())
            .with(Column::<AX>::new())
            .with(Column::<InY>::new())
            .with(Column::<InX>::new())
            .with(SX)
            .with(SY)
            .with(SZ)
            .build(store(BumpProvider::<32768>::new()), USize(N))
            .unwrap_or_else(|_| panic!("engine build should succeed"))
    };
}

#[test]
fn runpipeline_two_phase_matches_flat_walk() {
    // Flat path: shipped Scheduler::run over the [SX, SY, SZ] carrier.
    let mut flat = build_sched!();
    let flat_inx = flat.__bindings().__ptr().as_ptr() as *mut InX;
    let flat_iny = flat.__bindings().__tail().__ptr().as_ptr() as *mut InY;
    for i in 0..N {
        // SAFETY: InX, InY reserved for N records; storage alive; one write each.
        unsafe { *flat_inx.add(i) = InX(i as u32) };
        unsafe { *flat_iny.add(i) = InY(i as u32) };
    }
    assert!(matches!(flat.run(), notko::Outcome::Ok(())));
    let fb = flat.__bindings();
    let flat_ax = fb.__tail().__tail().__ptr().as_ptr() as *const u32;
    let flat_ay = fb.__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    let flat_cz = fb.__tail().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: AX, AY, CZ each reserved for N records; storage alive; written every record.
    let flat_ax = unsafe { core::slice::from_raw_parts(flat_ax, N) };
    let flat_ay = unsafe { core::slice::from_raw_parts(flat_ay, N) };
    let flat_cz = unsafe { core::slice::from_raw_parts(flat_cz, N) };

    // Pipeline path: RunPipeline over the hand-built nest. Phase 0 = two
    // column-disjoint trunks (X, Y); phase 1 = trunk Z reading phase-0 AX.
    let pipe = build_sched!();
    let pipe_inx = pipe.__bindings().__ptr().as_ptr() as *mut InX;
    let pipe_iny = pipe.__bindings().__tail().__ptr().as_ptr() as *mut InY;
    for i in 0..N {
        // SAFETY: InX, InY reserved for N records; storage alive; one write each.
        unsafe { *pipe_inx.add(i) = InX(i as u32) };
        unsafe { *pipe_iny.add(i) = InY(i as u32) };
    }
    let trunk_x = FiberCons { fiber: WuCons { head: SX, tail: WuNil }, rest: FiberNil };
    let trunk_y = FiberCons { fiber: WuCons { head: SY, tail: WuNil }, rest: FiberNil };
    let phase0 = TrunkCons { trunk: trunk_x, rest: TrunkCons { trunk: trunk_y, rest: TrunkNil } };
    let trunk_z = FiberCons { fiber: WuCons { head: SZ, tail: WuNil }, rest: FiberNil };
    let phase1 = TrunkCons { trunk: trunk_z, rest: TrunkNil };
    let pipeline = PhaseCons { phase: phase0, rest: PhaseCons { phase: phase1, rest: PhaseNil } };
    let barrier = AtomicUsize::new(0);
    pipeline_dispatch(
        pipe.__bindings(),
        &pipeline,
        MorselRange::new(USize(0), USize(N)),
        &barrier,
        USize(1),
    );
    let pb = pipe.__bindings();
    let pipe_ax = pb.__tail().__tail().__ptr().as_ptr() as *const u32;
    let pipe_ay = pb.__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    let pipe_cz = pb.__tail().__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
    // SAFETY: AX, AY, CZ each reserved for N records; storage alive; written every record.
    let pipe_ax = unsafe { core::slice::from_raw_parts(pipe_ax, N) };
    let pipe_ay = unsafe { core::slice::from_raw_parts(pipe_ay, N) };
    let pipe_cz = unsafe { core::slice::from_raw_parts(pipe_cz, N) };

    for i in 0..N {
        assert_eq!(flat_ax[i], fx(i as u32), "flat AX[{i}]");
        assert_eq!(flat_ay[i], fy(i as u32), "flat AY[{i}]");
        assert_eq!(flat_cz[i], fz(fx(i as u32)), "flat CZ[{i}]");
        assert_eq!(pipe_ax[i], flat_ax[i], "RunPipeline AX[{i}] vs flat");
        assert_eq!(pipe_ay[i], flat_ay[i], "RunPipeline AY[{i}] vs flat");
        assert_eq!(pipe_cz[i], flat_cz[i], "RunPipeline CZ[{i}] vs flat");
    }
}
