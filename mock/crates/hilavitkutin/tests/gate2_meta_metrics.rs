//! E4 slice 3: the engine-to-meta bridge, read by an `OnMeta<ScheduleEnd>` hook.
//!
//! The engine owns mutable meta state in a `MetaBlock` field on the scheduler
//! (not a consumer `Resource`, which is `Copy` read-only). The engine maintains
//! `SchedulerMetrics::pass_count` per pass. An `OnMeta<ScheduleEnd>` consumer
//! hook reads it through `ctx.meta::<SchedulerMetrics>()`, a `MetaAccess`-gated
//! accessor present ONLY on a Ctx carrying a `MetaRef`. A consumer (`Always`)
//! Ctx has the default `MetaNil` meta pointer and so has no `meta` accessor: a
//! consumer cannot reach meta state (compile-time `MetaAccess` enforcement,
//! covered by the `compile_fail` doctest on the accessor in `engine_ctx.rs`).
//!
//! Two work units append to one shared `Accum<Mark>`:
//! - `ConsumerWu` (`Always`, rank 3) appends `9` (a sentinel that consumer work
//!   ran this frame).
//! - `EndWu` (`OnMeta<ScheduleEnd>`, rank 4) reads `pass_count` through the
//!   bridge accessor and appends it.
//!
//! The accumulator forces the unit-outer dispatch path (every unit runs every
//! frame) and resets per frame, so the buffer after frame N is exactly that
//! frame's appends in dispatch order. Frame 1: `[9, 1]` (consumer ran, then the
//! end hook observed `pass_count = 1`). Frame 2: `[9, 2]` (the engine advanced
//! `pass_count` to 2, the hook read the updated value). This proves the bridge:
//! engine-owned mutable meta state, maintained per pass, read by a consumer hook
//! through the gated accessor, after consumer work.
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

// Shared accumulator capacity: room for both markers in a frame.
const CAP: usize = 8;

#[derive(Copy, Clone)]
struct Mark(u32);

type AccW = Cons<Accum<Mark>, Empty>;

// Consumer Ctx: default `MetaNil` meta pointer (the 9th param defaults), so it
// has no `meta` accessor. Aliases unchanged from pre-bridge code.
type ConsumerCtx<'frame> =
    EngineCtx<'frame, Empty, AccW, SnapNil, ColPtrNil, ColPtrNil, AccPtrCons<'frame, Mark, AccPtrNil>>;

// End-hook Ctx: spells `MetaRef<'frame>` as the 9th param, so it carries a meta
// reference and gains the `meta` accessor.
type EndCtx<'frame> = EngineCtx<
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

// ConsumerWu: Always (rank 3). Appends a sentinel every frame.
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
        // SAFETY: Mark reserved (CAP) and this frame's appends stay in capacity.
        unsafe { ctx.accums().append::<Mark, _>(Mark(9)) };
    }
}

// EndWu: OnMeta<ScheduleEnd> (rank 4). Reads pass_count through the bridge
// accessor and appends it, after the consumer.
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
        // The bridge: read engine-owned meta state through the gated accessor.
        let pc = ctx.meta::<SchedulerMetrics>().pass_count.get();
        // SAFETY: Mark reserved (CAP) and this frame's appends stay in capacity.
        unsafe { ctx.accums().append::<Mark, _>(Mark(pc.0 as u32)) };
    }
}

#[test]
fn end_hook_reads_engine_owned_pass_count_through_the_bridge() {
    let provider = BumpProvider::<8192>::new();
    let mut scheduler = Scheduler::builder()
        .with(Accum::<Mark>::new())
        .with(ConsumerWu)
        .with(EndWu)
        .build(store(provider), USize(CAP))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Frame 1: the engine advances pass_count to 1, dispatches the consumer,
    // then the end hook reads pass_count = 1.
    let r1 = scheduler.run();
    assert!(matches!(r1, Outcome::Ok(())));
    let len1 = scheduler.__bindings().__len_cell().get().0;
    let base1 = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len1, 2, "frame 1: consumer and end hook both ran");
    // SAFETY: two records appended this frame into a reset buffer.
    let buf1 = unsafe { [core::ptr::read(base1.add(0)).0, core::ptr::read(base1.add(1)).0] };
    assert_eq!(
        buf1,
        [9, 1],
        "frame 1: consumer(9) -> end hook reads pass_count = 1",
    );

    // Frame 2: the engine advances pass_count to 2; the hook reads the updated
    // value through the same bridge.
    let r2 = scheduler.run();
    assert!(matches!(r2, Outcome::Ok(())));
    let len2 = scheduler.__bindings().__len_cell().get().0;
    let base2 = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len2, 2, "frame 2: consumer and end hook both ran");
    // SAFETY: two records appended this frame into a reset buffer.
    let buf2 = unsafe { [core::ptr::read(base2.add(0)).0, core::ptr::read(base2.add(1)).0] };
    assert_eq!(
        buf2,
        [9, 2],
        "frame 2: consumer(9) -> end hook reads pass_count = 2 (engine advanced it)",
    );
}
