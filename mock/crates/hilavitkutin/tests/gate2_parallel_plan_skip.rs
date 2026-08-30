//! E4 run_parallel meta parity: phase-loop path, plan-band skip on clean frames.
//!
//! A record-bearing, accumulator-free carrier under `run_parallel`: an
//! `OnMeta<PlanStage>` unit that counts its executions in a test atomic, plus a
//! record-writing consumer (which keeps the carrier on the worker phase-loop
//! path). Frame 1 is plan-dirty (cold start), so the plan band runs; frame 2 is
//! clean, so the worker phase loop must start past the plan band and the count
//! stays unchanged. Mirrors single-core `dispatch_trunks`.
//!
//! The plan unit's per-frame multiplicity on this record-bearing path is the
//! documented per-morsel limitation (single-core and parallel alike) and is not
//! asserted against; the assertion is the frame-2 delta of zero.
//!
//! Red first: before this round the worker phase loop always starts at phase 0,
//! so the plan band re-runs on the clean frame.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, MetaRef, SnapNil, VirtNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin::OsThreadPool;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnWriterApi, EachApi, HasColumnWriter, HasEach};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::PlanStage;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, HasSchedule, OnMeta, WorkUnit};
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

const N: usize = 8;

static PLAN_RUNS: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone)]
struct Pv(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)] // written by the consumer, read back post-run as raw u32
struct Av(u32);

type ColP = Cons<Column<Pv>, Empty>;
type ColA = Cons<Column<Av>, Empty>;

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

// PlanWu: OnMeta<PlanStage>, plan band. Counts executions; the declared column
// write keeps it store-anchored for the grouping, its body writes nothing.
struct PlanWu;
impl BuilderInput for PlanWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl HasSchedule for PlanWu {
    type Sched = OnMeta<PlanStage>;
}
impl WorkUnit<OnMeta<PlanStage>> for PlanWu {
    type Read = Empty;
    type Write = ColP;
    type Hint = Hints;
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
    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {
        PLAN_RUNS.fetch_add(1, Ordering::Relaxed);
    }
}

// ConsumerWu: Always, writes a record column so the carrier is record-bearing
// (morsel-local) and takes the worker phase-loop path.
struct ConsumerWu;
impl BuilderInput for ConsumerWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ConsumerWu {
    type Read = Empty;
    type Write = ColA;
    type Hint = Hints;
    type Ctx<'frame> = EngineCtx<'frame, Empty, ColA, SnapNil, ColPtrNil, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Av reserved + exclusive; the morsel covers reserved records.
            unsafe { ctx.writer().write::<Av, _>(i, Av(7)) };
        });
    }
}

#[test]
fn worker_phase_loop_skips_plan_band_on_clean_frame() {
    PLAN_RUNS.store(0, Ordering::Relaxed);
    let provider = BumpProvider::<16384>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Pv>::new())
        .with(Column::<Av>::new())
        .with(PlanWu)
        .with(ConsumerWu)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let pool = OsThreadPool::new();
    let mut scheduler = core::pin::pin!(scheduler);

    // Frame 1 (cold start = plan-dirty): the plan band runs.
    let r1 = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r1, Outcome::Ok(())));
    let after_frame1 = PLAN_RUNS.load(Ordering::Relaxed);
    assert!(after_frame1 > 0, "frame 1: the plan band ran on the plan-dirty frame");

    // Frame 2 (clean): the worker phase loop starts past the plan band.
    let r2 = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r2, Outcome::Ok(())));
    let after_frame2 = PLAN_RUNS.load(Ordering::Relaxed);
    assert_eq!(
        after_frame2, after_frame1,
        "frame 2: the plan band was skipped on the clean frame",
    );
}
