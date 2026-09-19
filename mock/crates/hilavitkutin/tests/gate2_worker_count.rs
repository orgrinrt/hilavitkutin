//! The worker count `run_parallel` runs on, against executors that report
//! counts the engine's per-core state cannot hold.
//!
//! The engine sizes its per-core state by `MAX_CORES`, and runs on the
//! executor's `worker_count` clamped into `1..=MAX_CORES`. Past the ceiling
//! the extra workers are never handed a closure; before the clamp, the 257th
//! worker indexed past `worker_ctxs` and panicked.
//!
//! The zero case is provisional. The engine design leaves undecided whether
//! a zero count is refused, run inline on the caller, or handed one closure.
//! The code does the last, and the zero tests below pin that behaviour so a
//! change to it is seen, not because it is the designed answer. Before the
//! clamp the engine spawned nothing, published a frame no worker ran, and
//! returned as though the frame had run, which none of the three answers
//! allows.
//!
//! The principles bound the closures handed out at
//! `min(worker_count, parallelisable_width + 1)`. The fan-in's widest phase
//! carries two trunks, so the bound is three; the engine does not honour it
//! yet, and the test stating it is a catalogue entry.
//!
//! The fixture is the fan-in from `gate2_run_parallel.rs`: two column-disjoint
//! producers in phase 0, a combiner in phase 1 recording `Av + Bv`, so a frame
//! that did not run leaves the poisoned output column untouched.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin::thread::{MAX_CORES, runnable_worker_count};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
};
use hilavitkutin_api::platform::{MemoryProviderApi, ThreadPoolApi};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

// ---- the clamp itself ------------------------------------------------------

/// Provisional: pins the current answer to an undecided question.
#[test]
fn zero_reported_workers_run_on_one() {
    assert_eq!(runnable_worker_count(USize(0)), USize(1));
}

#[test]
fn a_count_inside_the_range_is_kept() {
    for n in [1, 2, 3, 7, 64, MAX_CORES - 1, MAX_CORES] {
        assert_eq!(runnable_worker_count(USize(n)), USize(n), "count {n}");
    }
}

#[test]
fn a_count_past_the_ceiling_runs_on_the_ceiling() {
    for n in [MAX_CORES + 1, MAX_CORES * 2, usize::MAX] {
        assert_eq!(
            runnable_worker_count(USize(n)),
            USize(MAX_CORES),
            "count {n}"
        );
    }
}

#[test]
fn the_clamp_evaluates_at_compile_time() {
    const ZERO: USize = runnable_worker_count(USize(0));
    const OVER: USize = runnable_worker_count(USize(MAX_CORES + 1));
    assert_eq!(ZERO, USize(1));
    assert_eq!(OVER, USize(MAX_CORES));
}

// ---- executors reporting what they like --------------------------------------

/// Reports `W` workers whatever it actually runs, and runs every closure it
/// is handed on a detached `std::thread`, counting them.
struct Reports<const W: usize> {
    spawns: AtomicUsize,
}

impl<const W: usize> Reports<W> {
    fn new() -> Self {
        Self {
            spawns: AtomicUsize::new(0),
        }
    }
}

impl<const W: usize> ThreadPoolApi for Reports<W> {
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.spawns.fetch_add(1, Ordering::Relaxed);
        let _ = std::thread::spawn(f);
    }

    fn worker_count(&self) -> USize {
        USize(W)
    }
}

#[test]
fn an_executor_reporting_past_max_cores_runs_the_frame_on_max_cores() {
    let pool = Reports::<{ MAX_CORES + 1 }>::new();
    let spawned = run_fan_in_and_check(&pool);
    assert_eq!(
        spawned, MAX_CORES,
        "the engine hands out one closure per worker it can hold, never the 257th"
    );
}

/// Provisional: the frame runs, which every candidate answer except refusing
/// requires, and it runs on one handed-out closure, which is the current
/// answer rather than a designed one.
#[test]
fn an_executor_reporting_zero_workers_still_runs_the_frame() {
    let pool = Reports::<0>::new();
    let spawned = run_fan_in_and_check(&pool);
    assert_eq!(
        spawned, 1,
        "provisional: a zero count runs the frame on one handed-out closure"
    );
}

/// The fan-in's widest phase: `ProducerA` and `ProducerB` write disjoint
/// columns, so phase 0 carries two trunks (`gate2_run_parallel.rs` relies on
/// the same split).
const FAN_IN_WIDTH: usize = 2;

/// Where the executor's count is the smaller term of the bound, the engine
/// already hands out exactly that many.
#[test]
fn an_executor_narrower_than_the_plan_gets_its_own_count() {
    let pool = Reports::<FAN_IN_WIDTH>::new();
    let spawned = run_fan_in_and_check(&pool);
    assert_eq!(spawned, FAN_IN_WIDTH.min(FAN_IN_WIDTH + 1));
}

#[test]
#[ignore = "catalogue: run_parallel hands out one closure per clamped worker, not min(worker_count, parallelisable_width + 1) as the principles bound it; tracked: FIXME in scheduler/run_parallel.rs"]
fn an_executor_wider_than_the_plan_gets_width_plus_one() {
    const REPORTED: usize = 8;
    let pool = Reports::<REPORTED>::new();
    let spawned = run_fan_in_and_check(&pool);
    assert_eq!(
        spawned,
        REPORTED.min(FAN_IN_WIDTH + 1),
        "the engine hands out one closure per trunk of the widest phase plus the convergence \
         worker, and no more"
    );
}

#[test]
fn an_executor_reporting_one_worker_runs_the_frame_on_one() {
    let pool = Reports::<1>::new();
    let spawned = run_fan_in_and_check(&pool);
    assert_eq!(spawned, 1);
}

/// Build the fan-in, run two frames on `pool`, check every record of both,
/// and return how many closures the pool was handed.
fn run_fan_in_and_check<const W: usize>(pool: &Reports<W>) -> usize {
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

    // Columns from head: Zv(0), Bv(1), Av(2), In(3).
    // SAFETY: both reserved for N records of u32; the scheduler is alive.
    let zv_base = scheduler.__bindings().__ptr().as_ptr() as *mut u32;
    let in_base = scheduler
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *mut u32;
    for i in 0 .. N {
        unsafe {
            *in_base.add(i) = i as u32;
        }
    }

    let mut scheduler = core::pin::pin!(scheduler);
    for frame in 0 .. 2 {
        for i in 0 .. N {
            // SAFETY: Zv reserved for N records; every worker is parked
            // between frames.
            unsafe { *zv_base.add(i) = u32::MAX };
        }
        let result = scheduler.as_mut().run_parallel(pool);
        assert!(matches!(result, Outcome::Ok(())));
        let zv = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const u32;
        for i in 0 .. N {
            // SAFETY: Zv holds N reserved records; the scheduler is alive.
            let z = unsafe { *zv.add(i) };
            assert_eq!(z, (i as u32) * 110, "frame {frame}, record {i}");
        }
    }
    pool.spawns.load(Ordering::Relaxed)
}

// ---- the fan-in fixture ----------------------------------------------------

/// The arena store at its default capacity, named so the builder can infer it.
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

struct BumpProvider<const N: usize> {
    buf:  UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}

impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self {
            buf:  UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
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
        let aligned = used.div_ceil(align) * align;
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
type HintT = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

struct ProducerA;
impl BuilderInput for ProducerA {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ProducerA {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColA,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Av, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColA;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for N records; Av reserved + exclusive.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(inp.0 * 10)) };
        });
    }
}

struct ProducerB;
impl BuilderInput for ProducerB {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ProducerB {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColB,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Bv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColB;

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
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Combiner {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadAB,
        ColZ,
        SnapNil,
        ColPtrCons<Av, ColPtrCons<Bv, ColPtrNil>>,
        ColPtrCons<Zv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = ReadAB;
    type Write = ColZ;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: both producers ran in the earlier phase and wrote every
            // record the morsel covers; Zv reserved + exclusive here.
            let a: Av = unsafe { ctx.reader().read::<Av, _>(i) };
            let b: Bv = unsafe { ctx.reader().read::<Bv, _>(i) };
            unsafe { ctx.writer().write::<Zv, _>(i, Zv(a.0 + b.0)) };
        });
    }
}
