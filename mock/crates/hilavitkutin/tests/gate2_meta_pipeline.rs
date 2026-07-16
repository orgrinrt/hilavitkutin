//! E4 slice 2: self-hosting meta pipeline lifecycle ordering through `Scheduler::run`.
//!
//! Three work units append a distinct marker to one shared `Accum<Mark>`, so the
//! accumulator buffer records the dispatch order within a frame:
//! - `PlanWu` (`OnMeta<PlanStage>`, rank 0) appends `1`.
//! - `ConsumerWu` (`Always`, rank 3) appends `2`.
//! - `EndWu` (`OnMeta<ScheduleEnd>`, rank 4) appends `3`.
//!
//! The rank-outer grouping renumber places the units in lifecycle order (plan
//! band, then consumer, then schedule-end band), so the kernel dispatches them in
//! that order and the buffer fills in band order. The accumulator forces the
//! unit-outer dispatch path (every unit runs every frame, no incremental skip),
//! and the per-frame reset zeroes the live length at frame start, so the buffer
//! after each frame is exactly the markers of the units that ran that frame, in
//! dispatch order.
//!
//! Frame 1 is plan-dirty (the plan is computed once): every band dispatches, so
//! the buffer is `[1, 2, 3]` (plan, then consumer, then schedule-end, proving
//! both that all bands ran and that schedule-end runs after the consumer). Frame
//! 2 is clean: the kernel skips the leading plan band, so the buffer is `[2, 3]`
//! (the plan-stage unit did NOT run; consumer and schedule-end run every frame,
//! still in order). This brackets both slice-2 claims: the PlanStage cadence
//! (plan-dirty only) and the lifecycle ordering.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrNil, EngineCtx, MetaRef, SnapNil, VirtNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, HasAccumWriter};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::{PlanStage, ScheduleEnd};
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

// Shared accumulator capacity: room for all three markers in a frame.
const CAP: usize = 8;

#[derive(Copy, Clone)]
struct Mark(u32);

type AccW = Cons<Accum<Mark>, Empty>;
// Consumer (`Always`) Ctx: default `MetaNil` meta pointer (no meta access).
type AccCtx<'frame> =
    EngineCtx<'frame, Empty, AccW, SnapNil, ColPtrNil, ColPtrNil, AccPtrCons<'frame, Mark, AccPtrNil>>;
// Meta (`OnMeta<V>`) Ctx: carries a `MetaRef`, since `OnMeta` units are meta
// work units (the `MetaPtrFor` mapping makes their Ctx's 9th param `MetaRef`).
// These units do not read meta state here; the meta pointer rides unused.
type MetaAccCtx<'frame> = EngineCtx<
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

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

// PlanWu: OnMeta<PlanStage> (rank 0). Appends 1 when it runs.
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
    type Write = AccW;
    type Hint = Hints;
    type Ctx<'frame> = MetaAccCtx<'frame>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // SAFETY: Mark reserved (CAP) and this frame's appends stay in capacity.
        unsafe { ctx.accums().append::<Mark, _>(Mark(1)) };
    }
}

// ConsumerWu: Always (rank 3). Appends 2 every frame.
struct ConsumerWu;
impl BuilderInput for ConsumerWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ConsumerWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = Hints;
    type Ctx<'frame> = AccCtx<'frame>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // SAFETY: Mark reserved (CAP) and this frame's appends stay in capacity.
        unsafe { ctx.accums().append::<Mark, _>(Mark(2)) };
    }
}

// EndWu: OnMeta<ScheduleEnd> (rank 4). Appends 3 after the consumer.
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
    type Ctx<'frame> = MetaAccCtx<'frame>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // SAFETY: Mark reserved (CAP) and this frame's appends stay in capacity.
        unsafe { ctx.accums().append::<Mark, _>(Mark(3)) };
    }
}

#[test]
fn meta_lifecycle_bands_dispatch_in_order_and_plan_stage_is_dirty_only() {
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Accum::<Mark>::new())
        .with(PlanWu)
        .with(ConsumerWu)
        .with(EndWu)
        .build(store(provider), USize(CAP))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Frame 1: plan-dirty (first frame). Every band dispatches, in order.
    let r1 = scheduler.run();
    assert!(matches!(r1, Outcome::Ok(())));
    let len1 = scheduler.__bindings().__len_cell().get().0;
    let base1 = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len1, 3, "frame 1: all three lifecycle bands ran");
    // SAFETY: three records appended this frame into a reset buffer.
    let buf1 = unsafe {
        [
            core::ptr::read(base1.add(0)).0,
            core::ptr::read(base1.add(1)).0,
            core::ptr::read(base1.add(2)).0,
        ]
    };
    assert_eq!(
        buf1,
        [1, 2, 3],
        "frame 1: dispatch order plan(1) -> consumer(2) -> schedule-end(3)",
    );

    // Frame 2: clean. The kernel skips the leading plan band; consumer and
    // schedule-end still run, still in order.
    let r2 = scheduler.run();
    assert!(matches!(r2, Outcome::Ok(())));
    let len2 = scheduler.__bindings().__len_cell().get().0;
    let base2 = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len2, 2, "frame 2: plan band skipped (plan-stage unit did not run)");
    // SAFETY: two records appended this frame into a reset buffer.
    let buf2 = unsafe {
        [core::ptr::read(base2.add(0)).0, core::ptr::read(base2.add(1)).0]
    };
    assert_eq!(
        buf2,
        [2, 3],
        "frame 2: consumer(2) -> schedule-end(3), plan(1) absent",
    );
}
