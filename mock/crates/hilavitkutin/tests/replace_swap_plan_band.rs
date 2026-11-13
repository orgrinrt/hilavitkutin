//! Swap spec S2, observed at the band: a plan-affecting swap runs the plan band.
//!
//! `tests/replace_swap.rs` reads the published band decision. This file reads
//! the band itself: an `OnMeta<PlanStage>` unit counts its executions, so a
//! frame that enters the band moves the count and a frame that skips it does
//! not. The same sequence runs on `run` and on `run_parallel`, where the
//! decision is consumed on the calling thread and read by each worker.
//!
//! The sequence: a cold frame (band runs), a clean frame (skipped), a frame
//! after `replace_value` (skipped, a value swap pays its cone and no band), a
//! frame after `replace_resource` (band runs), and a clean frame again (the
//! bit was consumed, so the band is skipped).
//!
//! One test function drives both paths in turn, because the count is one
//! static and two tests would race on it.
//!
//! The parallel arm swaps between two `run_parallel` frames, which is the
//! driver pattern catalogued under ticket `74e2b9918149` (a parked worker
//! keeps a `&Scheduler` across the swap's `&mut`). It runs natively; the
//! aliasing question is that ticket's and is not asserted here.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod common;

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use common::TestExecutor;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    MetaRef,
    SnapNil,
    VirtNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnWriterApi, EachApi, HasColumnWriter, HasEach};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::PlanStage;
use hilavitkutin_api::store::{Column, Resource};
use hilavitkutin_api::work_unit::{Always, HasSchedule, OnMeta, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

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

const RECORDS: usize = 8;

static PLAN_RUNS: AtomicUsize = AtomicUsize::new(0);

// Swappable scalar resource: the value route.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Val(u32);
impl hilavitkutin_api::store::Replaceable for Val {}
impl hilavitkutin_api::footprint::ResourceFootprint for Val {
    const L1_BYTES: USize = USize(0);
}

// Plan-affecting resource: the plan route.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Cfgish(u32);
impl hilavitkutin_api::run_cfg::PlanAffecting for Cfgish {}
impl hilavitkutin_api::footprint::ResourceFootprint for Cfgish {
    const L1_BYTES: USize = USize(0);
}

#[derive(Copy, Clone)]
struct Pv(u32);
#[derive(Copy, Clone)]
struct Av(u32);

type ColP = Cons<Column<Pv>, Empty>;
type ColA = Cons<Column<Av>, Empty>;

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

// The plan band's one unit. It counts executions; the declared column write
// keeps it store-anchored for the grouping, and its body writes nothing.
struct PlanWu;
impl BuilderInput for PlanWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl HasSchedule for PlanWu {
    type Sched = OnMeta<PlanStage>;
}
impl WorkUnit<OnMeta<PlanStage>> for PlanWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        ColP,
        SnapNil,
        ColPtrNil,
        ColPtrCons<Pv, ColPtrNil>,
        AccPtrNil,
        VirtNil,
        MetaRef<'frame>,
    >;
    type Hint = Hints;
    type Read = Empty;
    type Write = ColP;

    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {
        PLAN_RUNS.fetch_add(1, Ordering::Relaxed);
    }
}

// A record-writing consumer, so the carrier is record-bearing and the parallel
// arm takes the worker phase loop.
struct ConsumerWu;
impl BuilderInput for ConsumerWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ConsumerWu {
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, ColA, SnapNil, ColPtrNil, ColPtrCons<Av, ColPtrNil>>;
    type Hint = Hints;
    type Read = Empty;
    type Write = ColA;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Av reserved and exclusive; the morsel covers reserved records.
            unsafe { ctx.writer().write::<Av, _>(i, Av(7)) };
        });
    }
}

/// Run one frame and return how many times the plan band's unit ran in it.
macro_rules! frame_delta {
    ($run:expr) => {{
        let before = PLAN_RUNS.load(Ordering::Relaxed);
        assert!(matches!($run, Outcome::Ok(())));
        PLAN_RUNS.load(Ordering::Relaxed) - before
    }};
}

macro_rules! build {
    ($provider:expr) => {
        Scheduler::builder()
            .with(Resource::new(Val(5)))
            .with(Resource::new(Cfgish(1)))
            .with(Column::<Pv>::new())
            .with(Column::<Av>::new())
            .with(PlanWu)
            .with(ConsumerWu)
            .build(store($provider), USize(RECORDS))
            .unwrap_or_else(|_| panic!("build should succeed"))
    };
}

/// Poison both columns, so a record a frame did not reach is caught on the
/// consumer's and a record the plan unit's dispatch touched is caught on its
/// own. Bindings from the head: `Av`, registered last, then `Pv`.
macro_rules! poison_columns {
    ($scheduler:expr) => {{
        let consumer: *mut Av = $scheduler.__bindings().__ptr().as_ptr();
        let plan: *mut Pv = $scheduler.__bindings().__tail().__ptr().as_ptr();
        for i in 0 .. RECORDS {
            // SAFETY: both columns reserve RECORDS records; the scheduler is alive.
            unsafe {
                *consumer.add(i) = Av(u32::MAX);
                *plan.add(i) = Pv(u32::MAX);
            }
        }
    }};
}

/// Hold the plan unit's column to the poison: the unit declares the write to
/// be store-anchored and its body writes nothing, so no frame may change it.
macro_rules! assert_plan_column_untouched {
    ($scheduler:expr, $path:literal) => {{
        let base: *const Pv = $scheduler.__bindings().__tail().__ptr().as_ptr();
        for i in 0 .. RECORDS {
            // SAFETY: the column reserves RECORDS records; the scheduler is alive.
            let p = unsafe { (*base.add(i)).0 };
            assert_eq!(
                p,
                u32::MAX,
                "{}: plan rec {i} changed, and the plan unit writes nothing",
                $path
            );
        }
    }};
}

/// Hold every record of the consumer's column to what `ConsumerWu` writes.
macro_rules! assert_consumer_covered {
    ($scheduler:expr, $path:literal) => {{
        let base: *const Av = $scheduler.__bindings().__ptr().as_ptr();
        for i in 0 .. RECORDS {
            // SAFETY: the column reserves RECORDS records; the scheduler is alive.
            let a = unsafe { (*base.add(i)).0 };
            assert_eq!(
                a, 7,
                "{}: consumer rec {i} was not written by the cold frame",
                $path
            );
        }
    }};
}

#[test]
fn a_plan_affecting_swap_runs_the_plan_band_and_a_value_swap_does_not() {
    // Single core.
    {
        let mut scheduler = build!(BumpProvider::<32768>::new());
        poison_columns!(scheduler);

        let cold = frame_delta!(scheduler.run());
        assert!(cold > 0, "run: the cold frame runs the plan band");
        assert_consumer_covered!(scheduler, "run");

        assert_eq!(
            frame_delta!(scheduler.run()),
            0,
            "run: a clean frame skips the band"
        );

        scheduler.replace_value(Val(6));
        assert_eq!(
            frame_delta!(scheduler.run()),
            0,
            "run: a value swap does not run the band"
        );

        scheduler.replace_resource(Cfgish(2));
        assert_eq!(
            frame_delta!(scheduler.run()),
            cold,
            "run: a plan-affecting swap runs the band as the cold frame did"
        );

        assert_eq!(
            frame_delta!(scheduler.run()),
            0,
            "run: the bit was consumed, the next clean frame skips the band"
        );
        assert_plan_column_untouched!(scheduler, "run");
    }

    // Parallel: the decision is consumed on the calling thread and read by
    // each worker's phase loop.
    {
        let scheduler = build!(BumpProvider::<32768>::new());
        let pool = TestExecutor::new();
        let mut scheduler = core::pin::pin!(scheduler);
        poison_columns!(scheduler.as_ref());

        let cold = frame_delta!(scheduler.as_mut().run_parallel(&pool));
        assert!(cold > 0, "run_parallel: the cold frame runs the plan band");
        assert_consumer_covered!(scheduler.as_ref(), "run_parallel");

        assert_eq!(
            frame_delta!(scheduler.as_mut().run_parallel(&pool)),
            0,
            "run_parallel: a clean frame skips the band"
        );

        // SAFETY: the scheduler is not moved out of its pin; the swap writes
        // one resource slot and sets flags in place.
        unsafe { scheduler.as_mut().get_unchecked_mut() }.replace_value(Val(6));
        assert_eq!(
            frame_delta!(scheduler.as_mut().run_parallel(&pool)),
            0,
            "run_parallel: a value swap does not run the band"
        );

        // SAFETY: as above.
        unsafe { scheduler.as_mut().get_unchecked_mut() }.replace_resource(Cfgish(2));
        assert_eq!(
            frame_delta!(scheduler.as_mut().run_parallel(&pool)),
            cold,
            "run_parallel: a plan-affecting swap runs the band as the cold frame did"
        );

        assert_eq!(
            frame_delta!(scheduler.as_mut().run_parallel(&pool)),
            0,
            "run_parallel: the bit was consumed, the next clean frame skips the band"
        );
        assert_plan_column_untouched!(scheduler.as_ref(), "run_parallel");
    }
}
