//! E4 run_parallel meta parity: unit-outer path, once-per-frame ordered meta bands.
//!
//! An accumulator-bearing carrier under `run_parallel`: an `OnMeta<PassStart>`
//! unit that counts its executions in a test atomic, an `Always` consumer that
//! appends a marker per dispatch over its record slice, and an
//! `OnMeta<ScheduleEnd>` hook that reads `pass_count` through the slice-3
//! bridge and appends it. The meta bands must run exactly once per frame on the
//! designated thread (the main thread, around the publish/await window), NOT
//! once per participating core, and the epilogue append must land after the
//! merged consumer markers.
//!
//! Red first: before this round the unit-outer worker dispatches the whole
//! carrier (meta units included) once per participating core, so the pass-start
//! count is the core count, and the epilogue appends land inside per-core
//! regions instead of after the merge.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrNil, EngineCtx, MetaRef, SnapNil, VirtNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin::OsThreadPool;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, HasAccumWriter};
use hilavitkutin_api::meta::SchedulerMetrics;
use hilavitkutin_api::platform::{MemoryProviderApi, ThreadPoolApi};
use hilavitkutin_api::run_cfg::{PassStart, ScheduleEnd};
use hilavitkutin_api::store::Accum;
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
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// The accumulator's reserved capacity equals the record count at build, and
// appends saturate at it (the engine's soundness guard), so the record count
// provides the headroom for the per-core consumer markers plus the epilogue
// append.
const RECORDS: usize = 64;

static PASS_STARTS: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone)]
struct Mark(u32);

type AccW = Cons<Accum<Mark>, Empty>;

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

// Consumer Ctx: default MetaNil meta pointer.
type ConsumerCtx<'frame> =
    EngineCtx<'frame, Empty, AccW, SnapNil, ColPtrNil, ColPtrNil, AccPtrCons<'frame, Mark, AccPtrNil>>;

// Meta Ctx: MetaRef as the 9th param (forced for OnMeta schedules).
type MetaCtx<'frame> = EngineCtx<
    'frame,
    Empty,
    AccW,
    SnapNil,
    ColPtrNil,
    ColPtrNil,
    AccPtrCons<'frame, Mark, AccPtrNil>,
    VirtNil,
    MetaRef<'frame>,
>;

// StartWu: OnMeta<PassStart> (leading band). Counts executions; appends nothing
// (a leading-band accumulator append is unsupported on the parallel unit-outer
// path per the round's documented constraint).
struct StartWu;
impl BuilderInput for StartWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl HasSchedule for StartWu {
    type Sched = OnMeta<PassStart>;
}
impl WorkUnit<OnMeta<PassStart>> for StartWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = Hints;
    type Ctx<'frame> = MetaCtx<'frame>;
    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {
        PASS_STARTS.fetch_add(1, Ordering::Relaxed);
    }
}

// ConsumerWu: Always (consumer band). Appends a marker once per dispatch over
// its record slice (once per participating core under the unit-outer split).
struct ConsumerWu;
impl BuilderInput for ConsumerWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ConsumerWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = Hints;
    type Ctx<'frame> = ConsumerCtx<'frame>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // SAFETY: Mark reserved (RECORDS); appends stay within the per-core region.
        unsafe { ctx.accums().append::<Mark, _>(Mark(9)) };
    }
}

// EndWu: OnMeta<ScheduleEnd> (trailing band). Reads pass_count through the
// bridge and appends it after the merged consumer markers.
struct EndWu;
impl BuilderInput for EndWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl HasSchedule for EndWu {
    type Sched = OnMeta<ScheduleEnd>;
}
impl WorkUnit<OnMeta<ScheduleEnd>> for EndWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = Hints;
    type Ctx<'frame> = MetaCtx<'frame>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let pc = ctx.meta::<SchedulerMetrics>().pass_count.get();
        // SAFETY: Mark reserved (RECORDS); the epilogue append lands at the merged
        // live end.
        unsafe { ctx.accums().append::<Mark, _>(Mark(pc.0 as u32)) };
    }
}

// The lifecycle band shape of this exact carrier, asserted against the const
// grouping directly: ranks 2/3/4 renumber into one phase per band, the leading
// pre-consumer block is one phase, and the consumer band ends before the
// epilogue. The dispatch tests below ride these bounds; this pins them.
#[test]
fn carrier_band_bounds() {
    use hilavitkutin::dispatch::engine_ctx::Here;
    use hilavitkutin::plan::grouping::{
        consumer_phase_end, phase_count, phase_of, plan_phase_count, pre_consumer_phase_count,
        UnitAccess,
    };
    use hilavitkutin::plan::{DefaultPlanDims, PlanDims};
    type Stores = Cons<Accum<Mark>, Empty>;
    type CS = <DefaultPlanDims as PlanDims>::Stores;
    type CU = <DefaultPlanDims as PlanDims>::Units;
    type Adj = <DefaultPlanDims as PlanDims>::AdjRow;
    type W = (Empty, Cons<Here, Empty>);
    type Wit = Cons<W, Cons<W, Cons<W, Empty>>>;
    type Units = Cons<StartWu, Cons<ConsumerWu, Cons<EndWu, Empty>>>;
    assert_eq!(<StartWu as UnitAccess>::RANK.0, 2, "start rank");
    assert_eq!(<ConsumerWu as UnitAccess>::RANK.0, 3, "consumer rank");
    assert_eq!(<EndWu as UnitAccess>::RANK.0, 4, "end rank");
    assert_eq!(phase_of::<Units, Stores, Wit, CU, CS, Adj>(USize(0)).0, 0, "start phase");
    assert_eq!(phase_of::<Units, Stores, Wit, CU, CS, Adj>(USize(1)).0, 1, "consumer phase");
    assert_eq!(phase_of::<Units, Stores, Wit, CU, CS, Adj>(USize(2)).0, 2, "end phase");
    assert_eq!(phase_count::<Units, Stores, Wit, CU, CS, Adj>().0, 3, "nphases");
    assert_eq!(plan_phase_count::<Units, Stores, Wit, CU, CS, Adj>().0, 0, "plan phases");
    assert_eq!(pre_consumer_phase_count::<Units, Stores, Wit, CU, CS, Adj>().0, 1, "pre");
    assert_eq!(consumer_phase_end::<Units, Stores, Wit, CU, CS, Adj>().0, 2, "cend");
}

#[test]
fn unit_outer_meta_bands_run_once_per_frame_ordered() {
    PASS_STARTS.store(0, Ordering::Relaxed);
    let provider = BumpProvider::<16384>::new();
    let scheduler = Scheduler::builder()
        .with(Accum::<Mark>::new())
        .with(StartWu)
        .with(ConsumerWu)
        .with(EndWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let pool = OsThreadPool::new();
    let ncores = pool.worker_count().0.max(1);
    // Participating cores under the ceil record split (mirrors the worker math).
    let per = (RECORDS + ncores - 1) / ncores;
    let participating = (0..ncores).filter(|c| c * per < RECORDS).count();
    assert!(participating >= 1);
    assert!(
        participating + 1 <= RECORDS,
        "fixture headroom: the reserved capacity (= record count) covers the \
         per-core markers plus the epilogue append",
    );

    let mut scheduler = core::pin::pin!(scheduler);

    // Frame 1.
    let r1 = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r1, Outcome::Ok(())));
    assert_eq!(
        PASS_STARTS.load(Ordering::Relaxed),
        1,
        "frame 1: the pass-start meta band ran exactly once, not once per core",
    );
    let len1 = scheduler.as_ref().__bindings().__len_cell().get().0;
    let base1 = scheduler.as_ref().__bindings().__ptr().as_ptr();
    assert_eq!(
        len1,
        participating + 1,
        "frame 1: merged consumer markers plus one epilogue append",
    );
    for k in 0..participating {
        // SAFETY: len1 records appended into the merged prefix.
        let m = unsafe { core::ptr::read(base1.add(k)).0 };
        assert_eq!(m, 9, "frame 1 slot {k}: merged consumer marker");
    }
    // SAFETY: the epilogue record is the last live one.
    let tail1 = unsafe { core::ptr::read(base1.add(participating)).0 };
    assert_eq!(tail1, 1, "frame 1: epilogue hook read pass_count = 1 after the merge");

    // Frame 2.
    let r2 = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r2, Outcome::Ok(())));
    assert_eq!(
        PASS_STARTS.load(Ordering::Relaxed),
        2,
        "frame 2: one more pass-start execution, once per frame",
    );
    let len2 = scheduler.as_ref().__bindings().__len_cell().get().0;
    let base2 = scheduler.as_ref().__bindings().__ptr().as_ptr();
    assert_eq!(len2, participating + 1, "frame 2: reset buffer, same shape");
    for k in 0..participating {
        // SAFETY: len2 records appended into the merged prefix.
        let m = unsafe { core::ptr::read(base2.add(k)).0 };
        assert_eq!(m, 9, "frame 2 slot {k}: merged consumer marker");
    }
    // SAFETY: the epilogue record is the last live one.
    let tail2 = unsafe { core::ptr::read(base2.add(participating)).0 };
    assert_eq!(tail2, 2, "frame 2: epilogue hook read the advanced pass_count = 2");
}
