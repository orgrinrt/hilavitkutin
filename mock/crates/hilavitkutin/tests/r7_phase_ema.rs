//! E8 adapt, per-phase EMA axis: `dispatch_trunks` folds each phase's wall-clock
//! duration into the engine-internal `phase_ema` store (single-core path).
//!
//! Per-phase EMA is engine-internal (it feeds the eventual `select_adapt_config`),
//! so these tests read it through the white-box `__phase_ema` accessor. A
//! strictly-increasing counter clock makes every phase's before/after sample
//! bracket a positive delta, so the recorded EMA is deterministic.
//!
//! Edge-case catalogue:
//!
//! - multi-phase single-core run: each phase's EMA is positive; a slot past
//!   nphases stays zero.
//! - `run_parallel`: `dispatch_trunks` is not on its path, so `phase_ema` stays
//!   zero there (the single-core invariant).
//!
//! The fan-out/fan-in fixture (P1,P2 -> Mid -> Q1,Q2 -> Sink) yields nphases > 1;
//! a width-1 chain or shallow fan-in collapses to one phase. Lives under `tests/`
//! so the bare numeric record values do not trip the src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU64, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin::OsThreadPool;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, Nanos};
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

/// Strictly increasing counter clock: every `now_ns` returns a larger value, so
/// any phase's before/after sample brackets a positive delta deterministically.
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
        Nanos::from_raw(self.c.fetch_add(1, Ordering::Relaxed) + 1)
    }
}

const N: usize = 4;

#[derive(Copy, Clone)]
struct Inv(u32);
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
#[allow(dead_code)] // written by Sink, never read back (the test reads phase_ema)
struct Sv(u32);

type OneIn = Cons<Column<Inv>, Empty>;
type ColP1 = Cons<Column<P1v>, Empty>;
type ColP2 = Cons<Column<P2v>, Empty>;
type ColMr = Cons<Column<Mv>, Empty>;
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

/// Build the multi-phase fan-out/fan-in pipeline (nphases > 1) and populate In.
macro_rules! two_waist {
    ($provider:expr, $n:expr) => {{
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
            .build(store($provider), USize($n))
            .unwrap_or_else(|_| panic!("build should succeed"));
        // Columns from head: Sv(0), Q2v(1), Q1v(2), Mv(3), P2v(4), P1v(5), In(6).
        // SAFETY: In reserved for $n records of u32; the scheduler is alive.
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
        for i in 0..$n {
            unsafe { *in_base.add(i) = i as u32 };
        }
        scheduler
    }};
}

/// Single-phase carrier (one WU, In -> P1v): nphases == 1.
macro_rules! single_phase {
    ($provider:expr) => {{
        let scheduler = Scheduler::builder()
            .clock(CounterClock::new())
            .with(Column::<Inv>::new())
            .with(Column::<P1v>::new())
            .with(P1)
            .build(store($provider), USize(N))
            .unwrap_or_else(|_| panic!("build should succeed"));
        // Columns from head: P1v(0), In(1). Populate In = i.
        // SAFETY: In reserved for N records of u32; the scheduler is alive.
        let in_base = scheduler.__bindings().__tail().__ptr().as_ptr() as *mut u32;
        for i in 0..N {
            unsafe { *in_base.add(i) = i as u32 };
        }
        scheduler
    }};
}

// --- Measurement: single-core run records each phase's EMA; higher slot zero. ---
#[test]
fn multi_phase_records_and_higher_slot_zero() {
    let provider = BumpProvider::<16384>::new();
    let mut scheduler = two_waist!(provider, N);
    let r = scheduler.run();
    assert!(matches!(r, Outcome::Ok(())));
    // nphases == 2 for this fixture (proven in the prior round); both phases
    // recorded a positive duration under the increasing clock.
    assert!(scheduler.__phase_ema(USize(0)).to_raw() > 0, "phase 0 recorded a duration");
    assert!(scheduler.__phase_ema(USize(1)).to_raw() > 0, "phase 1 recorded a duration");
    assert_eq!(
        scheduler.__phase_ema(USize(2)).to_raw(),
        0,
        "a slot past nphases is never written, so it stays zero"
    );
}

// --- Invariant: run_parallel does not call dispatch_trunks, so phase_ema stays zero. ---
#[test]
fn parallel_path_leaves_phase_ema_zero() {
    let provider = BumpProvider::<16384>::new();
    let scheduler = two_waist!(provider, N);
    let pool = OsThreadPool::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let r = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r, Outcome::Ok(())));
    assert_eq!(
        scheduler.__phase_ema(USize(0)).to_raw(),
        0,
        "the parallel path does not run dispatch_trunks, so it records no per-phase EMA"
    );
}

// --- Multi-morsel: the per-phase EMA is per-FRAME (summed across morsels). ---
#[test]
fn multi_morsel_folds_per_frame_total() {
    // Default MORSEL_SIZE is 256, so 600 records is a 3-morsel frame.
    // `dispatch_trunks` runs once per morsel and ACCUMULATES each phase's slice;
    // `run` folds the per-frame total once. The prior round folded per morsel
    // (the bug this round fixes); this case exercises the multi-morsel path the
    // single-morsel test never reached.
    let provider = BumpProvider::<65536>::new();
    let mut scheduler = two_waist!(provider, 600);
    let r = scheduler.run();
    assert!(matches!(r, Outcome::Ok(())));
    // Both phases ran in all three morsels, so each per-frame total is recorded
    // (positive) under the increasing clock; a slot past nphases stays zero.
    assert!(
        scheduler.__phase_ema(USize(0)).to_raw() > 0,
        "phase 0 recorded its per-frame total across morsels"
    );
    assert!(
        scheduler.__phase_ema(USize(1)).to_raw() > 0,
        "phase 1 recorded its per-frame total across morsels"
    );
    assert_eq!(scheduler.__phase_ema(USize(2)).to_raw(), 0, "slot past nphases stays zero");
}

// --- Single-phase carrier touches only slot 0. ---
#[test]
fn single_phase_only_slot_zero() {
    let provider = BumpProvider::<16384>::new();
    let mut scheduler = single_phase!(provider);
    let r = scheduler.run();
    assert!(matches!(r, Outcome::Ok(())));
    assert!(scheduler.__phase_ema(USize(0)).to_raw() > 0, "the single phase recorded a duration");
    assert_eq!(
        scheduler.__phase_ema(USize(1)).to_raw(),
        0,
        "a single-phase carrier never writes a higher slot"
    );
}
