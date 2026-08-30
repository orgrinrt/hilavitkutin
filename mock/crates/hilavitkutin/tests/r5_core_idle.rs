//! E8 adapt, core-idle axis: per-core barrier-park idle through the waist
//! barrier into `SchedulerMetrics::idle_ns`.
//!
//! The waist barrier times its own follower park (a worker that finishes a
//! phase early and waits at the waist for slower cores). `run_parallel` reduces
//! the per-core `idle_accumulator` to the worst-core idle at `ScheduleEnd` and
//! resets it. A consumer reads `idle_ns` through an `OnMeta<ScheduleEnd>` hook;
//! these tests read it through the white-box `__idle_ns` accessor because an
//! accumulator readback would force the unit-outer no-barrier path (zero idle).
//!
//! Edge-case catalogue (the workspace TDD discipline: every edge case is a
//! test):
//!
//! - single-core `run`: no waist barrier crossed, idle stays zero (invariant).
//! - accumulator unit-outer parallel: the carrier bypasses the barrier, idle
//!   stays zero (invariant).
//! - imbalanced multi-core column pipeline: at least one follower parks, idle is
//!   positive (the measurement case). A strictly increasing counter clock makes
//!   any park record a positive delta, so the assertion is deterministic, not
//!   timing-dependent; guarded for single-core hosts where no follower parks.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU64, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin::OsThreadPool;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasAccumWriter, HasColumnReader,
    HasColumnWriter, HasEach,
};
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, Nanos, ThreadPoolApi};
use hilavitkutin_api::store::{Accum, Column};
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
        // SAFETY: aligned + len <= N, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

/// Strictly increasing counter clock: every `now_ns` call returns a larger
/// value than the previous one. This makes any follower park record a positive
/// idle delta (the two `now()` calls bracket at least one increment), so the
/// idle measurement is deterministic rather than dependent on real wall-clock.
struct CounterClock {
    c: AtomicU64,
}
impl CounterClock {
    fn new() -> Self {
        Self { c: AtomicU64::new(0) }
    }
}
impl ClockApi for CounterClock {
    fn now_ns(&self) -> Nanos {
        // fetch_add returns the prior value; +1 keeps the first reading nonzero.
        Nanos::from_raw(self.c.fetch_add(1, Ordering::Relaxed) + 1)
    }
}

const N: usize = 4;
// Accumulator capacity headroom over the per-frame appends (appends saturate at
// the reserved capacity, which equals the record count at build).
const RECORDS: usize = 64;

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // written by Combiner, never read back (the test reads idle, not Zv)
struct Zv(u32);

type OneIn = Cons<Column<Inv>, Empty>;
type ColA = Cons<Column<Av>, Empty>;
type ColB = Cons<Column<Bv>, Empty>;
type ColZ = Cons<Column<Zv>, Empty>;
type ReadAB = Cons<Column<Av>, Cons<Column<Bv>, Empty>>;

type HintT = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

// ProducerA: In -> Av (phase 0, one trunk). ProducerB: In -> Bv (phase 0, a
// disjoint trunk). Combiner: Av + Bv -> Zv (phase 1, after the waist). The two
// phase-0 trunks plus the phase-1 combiner give exactly one interior waist, the
// park point the idle axis measures.
struct ProducerA;
impl BuilderInput for ProducerA {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ProducerA {
    type Read = OneIn;
    type Write = ColA;
    type Hint = HintT;
    type Ctx<'frame> =
        EngineCtx<'frame, OneIn, ColA, SnapNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for N records; Av reserved + exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(inp.0 * 10)) };
        });
    }
}

struct ProducerB;
impl BuilderInput for ProducerB {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ProducerB {
    type Read = OneIn;
    type Write = ColB;
    type Hint = HintT;
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

struct Combiner;
impl BuilderInput for Combiner {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for Combiner {
    type Read = ReadAB;
    type Write = ColZ;
    type Hint = HintT;
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
            // SAFETY: both producers ran in phase 0 (RAW edges on Av/Bv); Zv
            // reserved + exclusive here.
            let a: Av = unsafe { ctx.reader().read::<Av, _>(i) };
            let b: Bv = unsafe { ctx.reader().read::<Bv, _>(i) };
            unsafe { ctx.writer().write::<Zv, _>(i, Zv(a.0 + b.0)) };
        });
    }
}

// Width-2 fan-out / fan-in fixture (P1,P2 -> Mid -> Q1,Q2 -> Sink). A width-1
// chain or a shallow fan-in collapses to one phase (the grouping orders the
// trunks within a single phase), but a level with two parallel trunks feeding a
// dependent unit forces a real interior waist, which is the park point the
// core-idle axis measures. This is the same shape the engine's barrier-reuse
// test uses; it yields nphases > 1.
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
#[allow(dead_code)] // written by Sink, never read back (the test reads idle, not Sv)
struct Sv(u32);

type ColP1 = Cons<Column<P1v>, Empty>;
type ColP2 = Cons<Column<P2v>, Empty>;
type ColMr = Cons<Column<Mv>, Empty>;
type ColQ1 = Cons<Column<Q1v>, Empty>;
type ColQ2 = Cons<Column<Q2v>, Empty>;
type ColS = Cons<Column<Sv>, Empty>;
type ReadP = Cons<Column<P1v>, Cons<Column<P2v>, Empty>>;
type ReadQ = Cons<Column<Q1v>, Cons<Column<Q2v>, Empty>>;

struct P1;
impl BuilderInput for P1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for P1 {
    type Read = OneIn;
    type Write = ColP1;
    type Hint = HintT;
    type Ctx<'frame> =
        EngineCtx<'frame, OneIn, ColP1, SnapNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<P1v, ColPtrNil>>;
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
    type Ctx<'frame> =
        EngineCtx<'frame, OneIn, ColP2, SnapNil, ColPtrCons<Inv, ColPtrNil>, ColPtrCons<P2v, ColPtrNil>>;
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
    type Write = ColMr;
    type Hint = HintT;
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadP,
        ColMr,
        SnapNil,
        ColPtrCons<P1v, ColPtrCons<P2v, ColPtrNil>>,
        ColPtrCons<Mv, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: both producers ran in the prior phase (RAW on P1v/P2v); Mv reserved.
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
    type Ctx<'frame> =
        EngineCtx<'frame, ColMr, ColQ1, SnapNil, ColPtrCons<Mv, ColPtrNil>, ColPtrCons<Q1v, ColPtrNil>>;
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
    type Ctx<'frame> =
        EngineCtx<'frame, ColMr, ColQ2, SnapNil, ColPtrCons<Mv, ColPtrNil>, ColPtrCons<Q2v, ColPtrNil>>;
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
            // SAFETY: Q1 and Q2 ran in the prior phase (RAW on Q1v/Q2v); Sv reserved.
            let a: Q1v = unsafe { ctx.reader().read::<Q1v, _>(i) };
            let b: Q2v = unsafe { ctx.reader().read::<Q2v, _>(i) };
            unsafe { ctx.writer().write::<Sv, _>(i, Sv(a.0 + b.0)) };
        });
    }
}

/// Macro: build the multi-phase fan-out/fan-in pipeline and populate In. Yields
/// nphases > 1, so the worker phase loop crosses an interior waist barrier.
macro_rules! two_waist {
    ($provider:expr) => {{
        let scheduler = Scheduler::builder()
            .clock(CounterClock::new())
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
            .build(store($provider), USize(N))
            .unwrap_or_else(|_| panic!("build should succeed"));
        // Columns from head: Sv(0), Q2v(1), Q1v(2), Mv(3), P2v(4), P1v(5), In(6).
        // SAFETY: In reserved for N records of u32; the scheduler is alive.
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
            unsafe { *in_base.add(i) = i as u32 };
        }
        scheduler
    }};
}

/// Macro: build the two-phase fan-in pipeline and host-populate `In[i] = i`.
/// A macro rather than a fn so the concrete `Scheduler` type stays inferred
/// (returning it from a fn would need to spell every opaque generic param).
macro_rules! fan_in {
    ($provider:expr) => {{
        let scheduler = Scheduler::builder()
            .clock(CounterClock::new())
            .with(Column::<Inv>::new())
            .with(Column::<Av>::new())
            .with(Column::<Bv>::new())
            .with(Column::<Zv>::new())
            .with(ProducerA)
            .with(ProducerB)
            .with(Combiner)
            .build(store($provider), USize(N))
            .unwrap_or_else(|_| panic!("build should succeed"));
        // Columns from head: Zv(0), Bv(1), Av(2), In(3). Populate In = i.
        // SAFETY: In reserved for N records of u32; the scheduler is alive.
        let in_base =
            scheduler.__bindings().__tail().__tail().__tail().__ptr().as_ptr() as *mut u32;
        for i in 0..N {
            unsafe { *in_base.add(i) = i as u32 };
        }
        scheduler
    }};
}

// --- Invariant: single-core `run` crosses no waist barrier, so idle is zero. ---
#[test]
fn single_core_run_no_idle() {
    let provider = BumpProvider::<16384>::new();
    let mut scheduler = fan_in!(provider);
    let r = scheduler.run();
    assert!(matches!(r, Outcome::Ok(())));
    assert_eq!(
        scheduler.__idle_ns().to_raw(),
        0,
        "single-core run takes no waist-barrier park, so it records no idle"
    );
}

// --- Invariant: an accumulator carrier takes the unit-outer no-barrier path. ---
#[derive(Copy, Clone)]
struct Mark(u64);
type AccW = Cons<Accum<Mark>, Empty>;

struct AccumWu;
impl BuilderInput for AccumWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for AccumWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = HintT;
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, AccW, SnapNil, ColPtrNil, ColPtrNil, AccPtrCons<'frame, Mark, AccPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // SAFETY: Mark reserved (RECORDS) with headroom over this frame's appends.
        unsafe { ctx.accums().append::<Mark, _>(Mark(7)) };
    }
}

#[test]
fn accum_unit_outer_no_idle() {
    let provider = BumpProvider::<16384>::new();
    let scheduler = Scheduler::builder()
        .clock(CounterClock::new())
        .with(Accum::<Mark>::new())
        .with(AccumWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let pool = OsThreadPool::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let r = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r, Outcome::Ok(())));
    assert_eq!(
        scheduler.__idle_ns().to_raw(),
        0,
        "an accumulator carrier runs unit-outer with no waist barrier, so it records no idle"
    );
}

// --- Measurement: an imbalanced multi-core column pipeline parks a follower. ---
#[test]
fn imbalanced_parallel_records_idle() {
    let provider = BumpProvider::<16384>::new();
    let scheduler = two_waist!(provider);

    let pool = OsThreadPool::new();
    let ncores = pool.worker_count().0.max(1);
    let mut scheduler = core::pin::pin!(scheduler);
    let r = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r, Outcome::Ok(())));

    let idle = scheduler.__idle_ns().to_raw();
    if ncores > 1 {
        // The phase-0 / phase-1 waist parks at least one follower; under the
        // strictly increasing counter clock that park records a positive delta.
        assert!(
            idle > 0,
            "a multi-core waist crossing parks a follower, which records nonzero idle (ncores={ncores})"
        );
    } else {
        // Single worker: the barrier takes the last-arriver path (expected == 1)
        // and never parks, so the single-core invariant applies.
        assert_eq!(idle, 0, "a single worker never parks at a waist, so it records no idle");
    }
}
