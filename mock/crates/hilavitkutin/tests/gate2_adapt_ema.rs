//! E8 adapt slice 1: clock-sourced pass-duration EMA through the bridge.
//!
//! The scheduler samples its clock provider at frame start and end and folds
//! the duration into `SchedulerMetrics::ema_pass_duration_ns` with the
//! canonical 1/8 weight (seed frame stores the raw sample). An
//! `OnMeta<ScheduleEnd>` hook reads the cell through the slice-3 bridge.
//! Because the fold lands after the last pass, the hook in frame N observes
//! the EMA as of frame N-1: the prediction semantics (a consumer budgets the
//! next frame from the prior frames' average).
//!
//! A scripted deterministic clock makes the fold exactly assertable. With the
//! six-value script `[1000, 1300, 2000, 2700, 3000, 3700]` the frame
//! durations are 300, 700, 700 and the EMA after each frame is 300 (seed),
//! 300 + (700 - 300) / 8 = 350, then 350 + (700 - 350) / 8 = 393. The hook
//! observes `[0, 300, 350]` across the three frames.
//!
//! Lives under `tests/` so the bare numeric script values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrNil, EngineCtx, MetaRef, PtrNil, VirtNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, HasAccumWriter};
use hilavitkutin_api::meta::SchedulerMetrics;
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, Nanos};
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

/// Scripted deterministic clock: each `now_ns` call yields the next script
/// value (the last value repeats past the end, so an over-read cannot panic
/// inside the engine).
struct ScriptClock {
    script: &'static [u64],
    cursor: AtomicUsize,
}
impl ScriptClock {
    const fn new(script: &'static [u64]) -> Self {
        Self { script, cursor: AtomicUsize::new(0) }
    }
}
impl ClockApi for ScriptClock {
    fn now_ns(&self) -> Nanos {
        let i = self.cursor.fetch_add(1, Ordering::Relaxed);
        Nanos::from_raw(self.script[i.min(self.script.len() - 1)])
    }
}

// Record capacity with headroom over the per-frame appends (appends saturate
// at the reserved capacity, which equals the record count at build).
const RECORDS: usize = 64;

#[derive(Copy, Clone)]
struct Mark(u64);

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

// ConsumerWu: Always (rank 3). Appends a sentinel every frame so the carrier
// has real consumer work between the meta bands.
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
        // SAFETY: Mark reserved (RECORDS) with headroom over this frame's appends.
        unsafe { ctx.accums().append::<Mark, _>(Mark(9)) };
    }
}

// EmaWu: OnMeta<ScheduleEnd> (rank 4). Reads the EMA cell through the bridge
// and appends its raw nanos, after the consumer.
struct EmaWu;
impl BuilderInput for EmaWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl HasSchedule for EmaWu {
    type Sched = OnMeta<ScheduleEnd>;
}
impl WorkUnit<OnMeta<ScheduleEnd>> for EmaWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = Hints;
    type Ctx<'frame> = EndCtx<'frame>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let ema = ctx.meta::<SchedulerMetrics>().ema_pass_duration_ns.get();
        // SAFETY: Mark reserved (RECORDS) with headroom over this frame's appends.
        unsafe { ctx.accums().append::<Mark, _>(Mark(ema.to_raw())) };
    }
}

#[test]
fn single_core_scripted_ema_fold() {
    static SCRIPT: [u64; 6] = [1000, 1300, 2000, 2700, 3000, 3700];
    let provider = BumpProvider::<16384>::new();
    let mut scheduler = Scheduler::builder()
        .clock(ScriptClock::new(&SCRIPT))
        .with(Accum::<Mark>::new())
        .with(ConsumerWu)
        .with(EmaWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Frame 1: the hook reads the cell before any fold has landed.
    let r1 = scheduler.run();
    assert!(matches!(r1, Outcome::Ok(())));
    let len1 = scheduler.__bindings().__len_cell().get().0;
    let base1 = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len1, 2, "frame 1: consumer and ema hook both ran");
    // SAFETY: two records appended this frame into a reset buffer.
    let ema1 = unsafe { core::ptr::read(base1.add(1)).0 };
    assert_eq!(ema1, 0, "frame 1: no fold has landed yet");

    // Frame 2: the seed fold from frame 1 (duration 300) is visible.
    let r2 = scheduler.run();
    assert!(matches!(r2, Outcome::Ok(())));
    let base2 = scheduler.__bindings().__ptr().as_ptr();
    // SAFETY: two records appended this frame into a reset buffer.
    let ema2 = unsafe { core::ptr::read(base2.add(1)).0 };
    assert_eq!(ema2, 300, "frame 2: seed frame stored the raw duration 300");

    // Frame 3: frame 2's duration 700 folded once: 300 + (700 - 300) / 8.
    let r3 = scheduler.run();
    assert!(matches!(r3, Outcome::Ok(())));
    let base3 = scheduler.__bindings().__ptr().as_ptr();
    // SAFETY: two records appended this frame into a reset buffer.
    let ema3 = unsafe { core::ptr::read(base3.add(1)).0 };
    assert_eq!(ema3, 350, "frame 3: one 1/8 fold toward 700");
}

#[test]
fn parallel_scripted_ema_seed() {
    use hilavitkutin::OsThreadPool;
    use hilavitkutin_api::platform::ThreadPoolApi;

    static SCRIPT: [u64; 4] = [1000, 1300, 2000, 2700];
    let provider = BumpProvider::<16384>::new();
    let scheduler = Scheduler::builder()
        .clock(ScriptClock::new(&SCRIPT))
        .with(Accum::<Mark>::new())
        .with(ConsumerWu)
        .with(EmaWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let pool = OsThreadPool::new();
    let ncores = pool.worker_count().0.max(1);
    let per = (RECORDS + ncores - 1) / ncores;
    let participating = (0..ncores).filter(|c| c * per < RECORDS).count();
    assert!(participating + 1 <= RECORDS, "fixture headroom over per-frame appends");

    let mut scheduler = core::pin::pin!(scheduler);

    // Frame 1: the epilogue hook reads the unseeded cell.
    let r1 = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r1, Outcome::Ok(())));
    let len1 = scheduler.__bindings().__len_cell().get().0;
    let base1 = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len1, participating + 1, "frame 1: per-core consumer marks plus the epilogue");
    // SAFETY: len1 records appended this frame; the epilogue append is last.
    let ema1 = unsafe { core::ptr::read(base1.add(len1 - 1)).0 };
    assert_eq!(ema1, 0, "frame 1: no fold has landed yet");

    // Frame 2: frame 1's seed (duration 300) is visible through the bridge.
    let r2 = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r2, Outcome::Ok(())));
    let len2 = scheduler.__bindings().__len_cell().get().0;
    let base2 = scheduler.__bindings().__ptr().as_ptr();
    // SAFETY: len2 records appended this frame; the epilogue append is last.
    let ema2 = unsafe { core::ptr::read(base2.add(len2 - 1)).0 };
    assert_eq!(ema2, 300, "frame 2: the parallel path seeded the raw duration 300");
}

#[cfg(feature = "platform-os")]
#[test]
fn os_clock_default_nonzero() {
    let provider = BumpProvider::<16384>::new();
    let mut scheduler = Scheduler::builder()
        .with(Accum::<Mark>::new())
        .with(ConsumerWu)
        .with(EmaWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let r1 = scheduler.run();
    assert!(matches!(r1, Outcome::Ok(())));
    let r2 = scheduler.run();
    assert!(matches!(r2, Outcome::Ok(())));
    let len2 = scheduler.__bindings().__len_cell().get().0;
    let base2 = scheduler.__bindings().__ptr().as_ptr();
    // SAFETY: this frame's appends end with the epilogue read.
    let ema2 = unsafe { core::ptr::read(base2.add(len2 - 1)).0 };
    assert!(ema2 > 0, "default os clock produced a nonzero monotonic duration");
}
