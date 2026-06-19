//! P1 R3b: the change_class axis numerator (`change_seen_count`) is exposed
//! through the engine-owned `MetaBlock` so an `OnMeta<ScheduleEnd>` hook can read
//! it and divide by `pass_count` for the input-change rate. The engine increments
//! `SchedulerMetrics::change_seen_count` in the single-core `run` fold for every
//! frame whose `store_dirty` was non-empty.
//!
//! `store_dirty`'s only public trigger is `replace_resource<T: PlanAffecting>`,
//! and `PlanAffecting` is sealed, so an external test cannot mark the store dirty
//! through the supported surface. This is a white-box test: it uses the hidden
//! `__mark_store_dirty` accessor to set the dirty mask directly, then asserts the
//! hook observes the count through the bridge. Mirrors
//! `r3c_throughput_record_count.rs`.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrNil, EngineCtx, MetaRef, PtrNil, VirtNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, HasAccumWriter};
use hilavitkutin_api::meta::SchedulerMetrics;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::ScheduleEnd;
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

const RECORDS: usize = 64;

#[derive(Copy, Clone)]
struct Mark(u32);

type AccW = Cons<Accum<Mark>, Empty>;

type ConsumerCtx<'frame> =
    EngineCtx<'frame, Empty, AccW, PtrNil, ColPtrNil, ColPtrNil, AccPtrCons<'frame, Mark, AccPtrNil>>;

type EndCtx<'frame> = EngineCtx<
    'frame,
    Empty,
    AccW,
    PtrNil,
    ColPtrNil,
    ColPtrNil,
    AccPtrCons<'frame, Mark, AccPtrNil>,
    VirtNil,
    MetaRef<'frame>,
>;

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

// Always consumer (rank 3): sentinel so the pipeline has a consumer band.
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
        // SAFETY: Mark reserved (RECORDS); one append per frame stays in capacity.
        unsafe { ctx.accums().append::<Mark, _>(Mark(9)) };
    }
}

// OnMeta<ScheduleEnd> hook: read the change_class numerator through the bridge.
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
    type Ctx<'frame> = EndCtx<'frame>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let c = ctx.meta::<SchedulerMetrics>().change_seen_count.get();
        // SAFETY: Mark reserved (RECORDS); one append per frame stays in capacity.
        unsafe { ctx.accums().append::<Mark, _>(Mark(c.0 as u32)) };
    }
}

#[test]
fn end_hook_reads_change_seen_count_through_the_bridge() {
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Accum::<Mark>::new())
        .with(ConsumerWu)
        .with(EndWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // The engine increments change_seen_count at frame end (after the OnMeta
    // dispatch), so a hook observes frame N-1's value, like the pass-duration EMA.
    // Frame 1: mark the store dirty before run. The end hook reads the seed (0);
    // the fold then increments to 1 (stores were dirty this frame).
    scheduler.__mark_store_dirty();
    let r1 = scheduler.run();
    assert!(matches!(r1, Outcome::Ok(())));
    let len1 = scheduler.__bindings().__len_cell().get().0;
    let base1 = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len1, 2, "frame 1: consumer and end hook both ran");
    // SAFETY: two records appended this frame into a reset buffer.
    let buf1 = unsafe { [core::ptr::read(base1.add(0)).0, core::ptr::read(base1.add(1)).0] };
    assert_eq!(buf1, [9, 0], "frame 1: end hook reads the seed change count (0)");

    // Frame 2: no mark, so store_dirty is empty. The end hook reads frame 1's
    // count (1); the fold does not increment (no change this frame).
    let r2 = scheduler.run();
    assert!(matches!(r2, Outcome::Ok(())));
    let len2 = scheduler.__bindings().__len_cell().get().0;
    let base2 = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len2, 2, "frame 2: consumer and end hook both ran");
    // SAFETY: two records appended this frame into a reset buffer.
    let buf2 = unsafe { [core::ptr::read(base2.add(0)).0, core::ptr::read(base2.add(1)).0] };
    assert_eq!(
        buf2,
        [9, 1],
        "frame 2: end hook reads frame 1's change count (1) through the bridge",
    );

    // Frame 3: still no mark. The count stays 1 (frame 2 saw no change).
    let r3 = scheduler.run();
    assert!(matches!(r3, Outcome::Ok(())));
    let base3 = scheduler.__bindings().__ptr().as_ptr();
    // SAFETY: two records appended this frame into a reset buffer.
    let buf3 = unsafe { [core::ptr::read(base3.add(0)).0, core::ptr::read(base3.add(1)).0] };
    assert_eq!(buf3, [9, 1], "frame 3: count unchanged, no dirty frame since");
}

#[test]
fn parallel_end_hook_reads_change_seen_count() {
    use hilavitkutin::OsThreadPool;
    use hilavitkutin_api::platform::ThreadPoolApi;

    let provider = BumpProvider::<16384>::new();
    let scheduler = Scheduler::builder()
        .with(Accum::<Mark>::new())
        .with(ConsumerWu)
        .with(EndWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let pool = OsThreadPool::new();
    let ncores = pool.worker_count().0.max(1);
    let per = (RECORDS + ncores - 1) / ncores;
    let participating = (0..ncores).filter(|c| c * per < RECORDS).count();
    assert!(participating + 1 <= RECORDS, "fixture headroom over per-frame appends");

    // Mark the store dirty once before the first parallel frame. The fold runs
    // on the main thread after every worker re-parks, so the same capture and
    // increment as the single-core path applies. The end hook (epilogue, last
    // record) reads frame N-1's count.
    scheduler.__mark_store_dirty();
    let mut scheduler = core::pin::pin!(scheduler);

    // Frame 1: store was dirty. The hook reads the seed (0); the fold increments.
    let r1 = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r1, Outcome::Ok(())));
    let len1 = scheduler.__bindings().__len_cell().get().0;
    let base1 = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len1, participating + 1, "frame 1: per-core consumer marks plus the epilogue");
    // SAFETY: len1 records appended this frame; the epilogue append is last.
    let c1 = unsafe { core::ptr::read(base1.add(len1 - 1)).0 };
    assert_eq!(c1, 0, "frame 1: end hook reads the seed change count (0)");

    // Frame 2: store_dirty was reset after frame 1, so this frame is clean. The
    // hook reads frame 1's count (1) through the bridge on the parallel path.
    let r2 = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r2, Outcome::Ok(())));
    let len2 = scheduler.__bindings().__len_cell().get().0;
    let base2 = scheduler.__bindings().__ptr().as_ptr();
    // SAFETY: len2 records appended this frame; the epilogue append is last.
    let c2 = unsafe { core::ptr::read(base2.add(len2 - 1)).0 };
    assert_eq!(c2, 1, "frame 2: run_parallel counted frame 1's change through the bridge");
}
