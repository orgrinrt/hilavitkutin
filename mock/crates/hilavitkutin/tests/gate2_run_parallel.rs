//! GATE-2 R4b: inline N-core dispatch parity (`Scheduler::run_parallel`).
//!
//! A fan-in workload: two column-disjoint producers in phase 0 (one writes
//! `Column<Av>`, one writes `Column<Bv>`, so they fall in distinct trunks), and
//! one combiner in phase 1 that reads both and records `Av + Bv`. The two RAW
//! edges (`Av` and `Bv`, both producer -> combiner) put the combiner after the
//! waist, in its own phase.
//!
//! `run_parallel(USize(2))` dispatches the canonical waist-bounded phases as
//! per-core trunk programs: phase 0 splits its two trunks across the two cores
//! (round-robin by within-phase trunk rank), then the waist barrier, then phase
//! 1's single trunk. The recorded sum is correct (`Av(i*10) + Bv(i*100) =
//! i*110`) only if both phase-0 trunks were dispatched and the phase-1 unit ran
//! after both. A dropped trunk or a broken phase order corrupts the sum.
//!
//! Red first: `Scheduler::run_parallel` does not exist before this round, so the
//! file does not compile.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use hilavitkutin::OsThreadPool;
use hilavitkutin_api::platform::ThreadPoolApi;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
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
use notko::Outcome;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

struct BumpProvider<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}

impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self {
            buf: UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
            used: Cell::new(0),
        }
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

const N: usize = 4;

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // written by Combiner, read back post-run as raw u32
struct Zv(u32);

type OneIn = Cons<Column<Inv>, Empty>;
type ColA = Cons<Column<Av>, Empty>;
type ColB = Cons<Column<Bv>, Empty>;
type ColZ = Cons<Column<Zv>, Empty>;
type ReadAB = Cons<Column<Av>, Cons<Column<Bv>, Empty>>;

// ProducerA: reads In, writes Av = In*10. Reads a host-populated input rather
// than synthesising from the each() index: that index is morsel-relative, so a
// source-from-index WU breaks once a phase is split across cores (here the two
// disjoint producers feed the fan-in, so they share one trunk that head+tail
// convergence splits). Phase 0.
struct ProducerA;
impl BuilderInput for ProducerA {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ProducerA {
    type Read = OneIn;
    type Write = ColA;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, OneIn, ColA, SnapNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for N records; Av reserved + exclusive;
            // the morsel covers reserved records (reader/writer window to absolute).
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(inp.0 * 10)) };
        });
    }
}

// ProducerB: reads In, writes Bv = In*100. Store-disjoint from A.
struct ProducerB;
impl BuilderInput for ProducerB {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ProducerB {
    type Read = OneIn;
    type Write = ColB;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, OneIn, ColB, SnapNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Bv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: as ProducerA, for Bv.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(inp.0 * 100)) };
        });
    }
}

// Combiner: reads Av + Bv, records their sum -> phase 1 (after the waist).
struct Combiner;
impl BuilderInput for Combiner {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Combiner {
    type Read = ReadAB;
    type Write = ColZ;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadAB,
        ColZ,
        SnapNil,
        ColPtrCons<Av, ColPtrCons<Bv, ColPtrNil>>,
        ColPtrCons<Zv, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: both producers (ordered before this unit by the plan's RAW
            // edges on Av and Bv, and run in an earlier phase) wrote every
            // record the morsel covers; Zv reserved + exclusive here. Writing a
            // column (windowed) rather than a static-by-index is what makes the
            // combiner correct under head+tail convergence, which splits this
            // single-trunk phase across cores with non-zero morsel starts.
            let a: Av = unsafe { ctx.reader().read::<Av, _>(i) };
            let b: Bv = unsafe { ctx.reader().read::<Bv, _>(i) };
            unsafe { ctx.writer().write::<Zv, _>(i, Zv(a.0 + b.0)) };
        });
    }
}

/// A real `ThreadPoolApi` that counts `spawn` calls, delegating to the os pool so
/// workers actually run. Proves spawn-once across frames.
struct CountingPool {
    inner: OsThreadPool,
    spawns: AtomicUsize,
}

impl ThreadPoolApi for CountingPool {
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.spawns.fetch_add(1, Ordering::Relaxed);
        self.inner.spawn(f);
    }

    fn worker_count(&self) -> USize {
        self.inner.worker_count()
    }
}

#[test]
fn run_parallel_threaded_fan_in_is_correct() {
    let provider = BumpProvider::<16384>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Av>::new())
        .with(Column::<Bv>::new())
        .with(Column::<Zv>::new())
        .with(ProducerA)
        .with(ProducerB)
        .with(Combiner)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Host-populate In[i] = i (absolute) and poison Zv (head column) so an
    // unwritten record is caught. Columns from head: Zv(0), Bv(1), Av(2), In(3).
    // SAFETY: both reserved for N records of u32; the scheduler is alive.
    let zv_base = scheduler.__bindings().__ptr().as_ptr() as *mut u32;
    let in_base =
        scheduler.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *mut u32;
    for i in 0..N {
        unsafe {
            *in_base.add(i) = i as u32;
            *zv_base.add(i) = u32::MAX;
        }
    }

    let pool = OsThreadPool::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let result = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(result, Outcome::Ok(())));

    // Av(i*10) + Bv(i*100) = i*110, written to Zv by the phase-1 combiner after
    // both phase-0 producer trunks finished (the waist barrier).
    let zv_base = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const u32;
    for i in 0..N {
        // SAFETY: Zv holds N reserved records; the scheduler is alive.
        let z = unsafe { *zv_base.add(i) };
        assert_eq!(
            z,
            (i as u32) * 110,
            "rec {i}: threaded run_parallel dispatched both phase-0 trunks and ran \
             the phase-1 combiner after both"
        );
    }
}

#[test]
fn run_parallel_spawns_the_pool_once_across_frames() {
    let provider = BumpProvider::<16384>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Av>::new())
        .with(Column::<Bv>::new())
        .with(Column::<Zv>::new())
        .with(ProducerA)
        .with(ProducerB)
        .with(Combiner)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Host-populate In[i] = i. Columns from head: Zv(0), Bv(1), Av(2), In(3).
    // SAFETY: In reserved for N records of u32; the scheduler is alive.
    let in_base =
        scheduler.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *mut u32;
    for i in 0..N {
        unsafe { *in_base.add(i) = i as u32 };
    }

    let pool = CountingPool { inner: OsThreadPool::new(), spawns: AtomicUsize::new(0) };
    let mut scheduler = core::pin::pin!(scheduler);
    // Two frames: spawn happens once (first call), workers park and are reused.
    let _ = scheduler.as_mut().run_parallel(&pool);
    let _ = scheduler.as_mut().run_parallel(&pool);

    assert_eq!(
        pool.spawns.load(Ordering::Relaxed),
        pool.worker_count().0,
        "the persistent pool spawns worker_count workers once, not per frame"
    );
    let zv_base = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const u32;
    for i in 0..N {
        // SAFETY: Zv holds N reserved records; the scheduler is alive.
        let z = unsafe { *zv_base.add(i) };
        assert_eq!(z, (i as u32) * 110, "rec {i} after two frames");
    }
}

// ---- 3-phase double-fan-in fixture: exercises the worker-side sense-reversing
// waist barrier REUSE within a single frame (two interior waists back-to-back,
// no frame sync between them, which is exactly the reset race the sense flip
// fixes). Shape: P1,P2 (phase 0) -> Mid fan-in (waist 1) -> Q1,Q2 fan-out ->
// Sink fan-in (waist 2). Per record i: P1=2i, P2=3i, Mid=5i, Q1=5i, Q2=10i,
// Sink=15i.


#[derive(Copy, Clone)]
struct P1v(u32);
#[derive(Copy, Clone)]
struct P2v(u32);
#[derive(Copy, Clone)]
struct Mv(u32);
#[derive(Copy, Clone)]
struct Q1v(u32);
#[derive(Copy, Clone)]
struct Q2v(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // written by Sink, read back post-run as raw u32
struct Sv(u32);

type ColP1 = Cons<Column<P1v>, Empty>;
type ColP2 = Cons<Column<P2v>, Empty>;
type ColMr = Cons<Column<Mv>, Empty>;
type ColMw = Cons<Column<Mv>, Empty>;
type ColQ1 = Cons<Column<Q1v>, Empty>;
type ColQ2 = Cons<Column<Q2v>, Empty>;
type ColS = Cons<Column<Sv>, Empty>;
type ReadP = Cons<Column<P1v>, Cons<Column<P2v>, Empty>>;
type ReadQ = Cons<Column<Q1v>, Cons<Column<Q2v>, Empty>>;

type HintT = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

struct P1;
impl BuilderInput for P1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for P1 {
    type Read = OneIn;
    type Write = ColP1;
    type Hint = HintT;
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColP1,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<P1v, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated; P1v reserved + exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<P1v, _>(i, P1v(inp.0 * 2)) };
        });
    }
}

struct P2;
impl BuilderInput for P2 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for P2 {
    type Read = OneIn;
    type Write = ColP2;
    type Hint = HintT;
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColP2,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<P2v, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: as P1, for P2v.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<P2v, _>(i, P2v(inp.0 * 3)) };
        });
    }
}

struct Mid;
impl BuilderInput for Mid {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Mid {
    type Read = ReadP;
    type Write = ColMw;
    type Hint = HintT;
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadP,
        ColMw,
        SnapNil,
        ColPtrCons<P1v, ColPtrCons<P2v, ColPtrNil>>,
        ColPtrCons<Mv, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: both producers ran in the prior phase (RAW edges on P1v/P2v);
            // Mv reserved + exclusive here.
            let a: P1v = unsafe { ctx.reader().read::<P1v, _>(i) };
            let b: P2v = unsafe { ctx.reader().read::<P2v, _>(i) };
            unsafe { ctx.writer().write::<Mv, _>(i, Mv(a.0 + b.0)) };
        });
    }
}

struct Q1;
impl BuilderInput for Q1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Q1 {
    type Read = ColMr;
    type Write = ColQ1;
    type Hint = HintT;
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColMr,
        ColQ1,
        SnapNil,
        ColPtrCons<Mv, ColPtrNil>,
        ColPtrCons<Q1v, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Mid ran in the prior phase (RAW on Mv); Q1v reserved + exclusive.
            let m: Mv = unsafe { ctx.reader().read::<Mv, _>(i) };
            unsafe { ctx.writer().write::<Q1v, _>(i, Q1v(m.0)) };
        });
    }
}

struct Q2;
impl BuilderInput for Q2 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Q2 {
    type Read = ColMr;
    type Write = ColQ2;
    type Hint = HintT;
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColMr,
        ColQ2,
        SnapNil,
        ColPtrCons<Mv, ColPtrNil>,
        ColPtrCons<Q2v, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: as Q1, doubling Mv into the disjoint Q2v column.
            let m: Mv = unsafe { ctx.reader().read::<Mv, _>(i) };
            unsafe { ctx.writer().write::<Q2v, _>(i, Q2v(m.0 * 2)) };
        });
    }
}

struct Sink;
impl BuilderInput for Sink {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Sink {
    type Read = ReadQ;
    type Write = ColS;
    type Hint = HintT;
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadQ,
        ColS,
        SnapNil,
        ColPtrCons<Q1v, ColPtrCons<Q2v, ColPtrNil>>,
        ColPtrCons<Sv, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Q1 and Q2 ran in the prior phase (RAW on Q1v/Q2v); Sv
            // reserved + exclusive. A column write (windowed) is correct under
            // the head+tail convergence that splits this single-trunk phase.
            let a: Q1v = unsafe { ctx.reader().read::<Q1v, _>(i) };
            let b: Q2v = unsafe { ctx.reader().read::<Q2v, _>(i) };
            unsafe { ctx.writer().write::<Sv, _>(i, Sv(a.0 + b.0)) };
        });
    }
}

// ---- 3-trunk / 2-core fixture: phase 0 holds THREE column-disjoint producers
// (three trunks). On two cores the round-robin trunk ownership wraps: core 0 owns
// within-phase ranks 0 and 2, core 1 owns rank 1, so core 0 dispatches two trunks
// in one phase. This exercises the `rank % ncores` wrap that a one-trunk-per-core
// fixture cannot. A combiner in phase 1 reads all three. Per record i:
// Av=10i, Bv=100i, Cv=1000i, Wv=1110i.

#[derive(Copy, Clone)]
struct Cv(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // written by Combiner3, read back post-run as raw u32
struct Wv(u32);

type ColC = Cons<Column<Cv>, Empty>;
type ColW = Cons<Column<Wv>, Empty>;
type ReadABC = Cons<Column<Av>, Cons<Column<Bv>, Cons<Column<Cv>, Empty>>>;

// ProducerC: reads In, writes Cv = In*1000. Store-disjoint from A and B -> a third
// trunk in phase 0.
struct ProducerC;
impl BuilderInput for ProducerC {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ProducerC {
    type Read = OneIn;
    type Write = ColC;
    type Hint = HintT;
    type Ctx<'frame> =
        EngineCtx<'frame, OneIn, ColC, SnapNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Cv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated; Cv reserved + exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Cv, _>(i, Cv(inp.0 * 1000)) };
        });
    }
}

// Combiner3: reads Av + Bv + Cv, records their sum -> phase 1 (after the waist).
struct Combiner3;
impl BuilderInput for Combiner3 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Combiner3 {
    type Read = ReadABC;
    type Write = ColW;
    type Hint = HintT;
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadABC,
        ColW,
        SnapNil,
        ColPtrCons<Av, ColPtrCons<Bv, ColPtrCons<Cv, ColPtrNil>>>,
        ColPtrCons<Wv, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: all three producers ran in phase 0 (RAW edges on Av/Bv/Cv);
            // Wv reserved + exclusive here.
            let a: Av = unsafe { ctx.reader().read::<Av, _>(i) };
            let b: Bv = unsafe { ctx.reader().read::<Bv, _>(i) };
            let c: Cv = unsafe { ctx.reader().read::<Cv, _>(i) };
            unsafe { ctx.writer().write::<Wv, _>(i, Wv(a.0 + b.0 + c.0)) };
        });
    }
}

#[test]
fn run_parallel_three_trunks_two_cores_matches_single_core() {
    let provider = BumpProvider::<32768>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<Av>::new())
        .with(Column::<Bv>::new())
        .with(Column::<Cv>::new())
        .with(Column::<Wv>::new())
        .with(ProducerA)
        .with(ProducerB)
        .with(ProducerC)
        .with(Combiner3)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Host-populate In[i] = i and poison Wv (head column). Columns from head:
    // Wv(0), Cv(1), Bv(2), Av(3), In(4).
    // SAFETY: both reserved for N records of u32; the scheduler is alive.
    let wv_base = scheduler.__bindings().__ptr().as_ptr() as *mut u32;
    let in_base = scheduler
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *mut u32;
    for i in 0..N {
        unsafe {
            *in_base.add(i) = i as u32;
            *wv_base.add(i) = u32::MAX;
        }
    }

    // Force two cores so phase 0's three trunks distribute round-robin: core 0
    // gets ranks 0 and 2, core 1 gets rank 1. A dropped trunk (a rank-wrap bug
    // where core 0 fails to run its second trunk) corrupts the sum.
    let pool = OsThreadPool::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let result = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(result, Outcome::Ok(())));

    // Wv = Av(10i) + Bv(100i) + Cv(1000i) = 1110i, correct only if all three
    // phase-0 trunks were dispatched (including both trunks core 0 owns) and the
    // phase-1 combiner ran after the waist.
    let wv_base = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const u32;
    for i in 0..N {
        // SAFETY: Wv holds N reserved records; the scheduler is alive.
        let w = unsafe { *wv_base.add(i) };
        assert_eq!(
            w,
            (i as u32) * 1110,
            "rec {i}: all three phase-0 trunks dispatched (core 0 ran two) and the \
             phase-1 combiner ran after the waist"
        );
    }
}

#[test]
fn run_parallel_threaded_two_waists_reuses_barrier() {
    let provider = BumpProvider::<32768>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::new())
        .with(Column::<P1v>::new())
        .with(Column::<P2v>::new())
        .with(Column::<Mv>::new())
        .with(Column::<Q1v>::new())
        .with(Column::<Q2v>::new())
        .with(Column::<Sv>::new())
        .with(P1)
        .with(P2)
        .with(Mid)
        .with(Q1)
        .with(Q2)
        .with(Sink)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Host-populate In[i] = i and poison Sv (head column). Columns from head:
    // Sv(0), Q2v(1), Q1v(2), Mv(3), P2v(4), P1v(5), In(6).
    // SAFETY: both reserved for N records of u32; the scheduler is alive.
    let sv_base = scheduler.__bindings().__ptr().as_ptr() as *mut u32;
    let in_base = scheduler
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *mut u32;
    for i in 0..N {
        unsafe {
            *in_base.add(i) = i as u32;
            *sv_base.add(i) = u32::MAX;
        }
    }

    let pool = OsThreadPool::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let result = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(result, Outcome::Ok(())));

    // Sv = Q1(5i) + Q2(10i) = 15i, correct only if both interior waist barriers
    // ordered their phases AND the barrier was reused correctly for the second
    // waist (the sense flip; a stale-count reset would deadlock or corrupt here).
    let sv_base = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const u32;
    for i in 0..N {
        // SAFETY: Sv holds N reserved records; the scheduler is alive.
        let s = unsafe { *sv_base.add(i) };
        assert_eq!(
            s,
            (i as u32) * 15,
            "rec {i}: two-waist frame ran P1,P2 -> Mid -> Q1,Q2 -> Sink in phase order \
             with the worker-side barrier reused across both waists"
        );
    }
}
