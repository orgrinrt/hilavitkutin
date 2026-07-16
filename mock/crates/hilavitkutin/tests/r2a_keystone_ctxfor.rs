//! P1 R2a keystone: a nine-`Resource` Read access set + an `Accum`/`Virtual`
//! Write on an `OnMeta<ScheduleEnd>` WorkUnit whose `Ctx` is the merged `CtxFor`
//! alias (not hand-spelled), compiles and dispatches on a real scheduler frame,
//! and its `execute` reads a resource through the projected Ctx and fires a
//! virtual. This is the keystone the P1 adapt arc depends on: `AdaptWu` carries
//! nine metrics `Resource`s plus one anomaly `Virtual`. The OnMeta dispatch band
//! itself is proven (`gate2_meta_metrics.rs` EndWu); this isolates the question
//! the granularity pass flagged as the risk: does the projection machinery close
//! all nine index-witness chains at once via `CtxFor` under the real solver.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::CtxFor;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi, HasAccumWriter, HasResourceProvider, HasVirtualFirer, ResourceProviderApi,
    VirtualFirerApi,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::ScheduleEnd;
use hilavitkutin_api::store::{Accum, Resource, Virtual};
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

// Nine distinct resource markers (stand in for the nine adapt metrics
// Resources). R0 carries the value the execute reads back.
#[derive(Copy, Clone)]
struct R0(u32);
#[derive(Copy, Clone)]
struct R1(u32);
#[derive(Copy, Clone)]
struct R2(u32);
#[derive(Copy, Clone)]
struct R3(u32);
#[derive(Copy, Clone)]
struct R4(u32);
#[derive(Copy, Clone)]
struct R5(u32);
#[derive(Copy, Clone)]
struct R6(u32);
#[derive(Copy, Clone)]
struct R7(u32);
#[derive(Copy, Clone)]
struct R8(u32);

// The anomaly virtual (stand in for AdaptWu's AnomalyFired).
struct Anomaly;

#[derive(Copy, Clone)]
struct Mark(u32);

const CAP: usize = 8;

// Nine-resource Read set (the projection-width stress).
type Read9 = Cons<
    Resource<R0>,
    Cons<
        Resource<R1>,
        Cons<
            Resource<R2>,
            Cons<
                Resource<R3>,
                Cons<
                    Resource<R4>,
                    Cons<
                        Resource<R5>,
                        Cons<Resource<R6>, Cons<Resource<R7>, Cons<Resource<R8>, Empty>>>,
                    >,
                >,
            >,
        >,
    >,
>;

// Write: an Accum (observable output) plus the anomaly Virtual.
type WriteAV = Cons<Accum<Mark>, Cons<Virtual<Anomaly>, Empty>>;

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

// The keystone WU: nine-resource Read, Accum+Virtual Write, OnMeta<ScheduleEnd>,
// Ctx computed by `CtxFor` (NOT hand-spelled).
struct AdaptShapedWu;
impl BuilderInput for AdaptShapedWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl HasSchedule for AdaptShapedWu {
    type Sched = OnMeta<ScheduleEnd>;
}
impl WorkUnit<OnMeta<ScheduleEnd>> for AdaptShapedWu {
    type Read = Read9;
    type Write = WriteAV;
    type Hint = Hints;
    type Ctx<'frame> = CtxFor<'frame, Read9, WriteAV, OnMeta<ScheduleEnd>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // Read all nine resources through the projected Ctx (proves every one of
        // the nine index-witness chains resolves, the granularity-flagged risk),
        // append their sum, fire the anomaly virtual.
        let r0: &R0 = ctx.resources().resource();
        let r1: &R1 = ctx.resources().resource();
        let r2: &R2 = ctx.resources().resource();
        let r3: &R3 = ctx.resources().resource();
        let r4: &R4 = ctx.resources().resource();
        let r5: &R5 = ctx.resources().resource();
        let r6: &R6 = ctx.resources().resource();
        let r7: &R7 = ctx.resources().resource();
        let r8: &R8 = ctx.resources().resource();
        let sum = r0.0 + r1.0 + r2.0 + r3.0 + r4.0 + r5.0 + r6.0 + r7.0 + r8.0;
        // SAFETY: Mark reserved (CAP) and this frame's single append stays in capacity.
        unsafe { ctx.accums().append::<Mark, _>(Mark(sum)) };
        ctx.virtuals().fire::<Anomaly, _>();
    }
}

// A trivial Always consumer so the pipeline carries a consumer band plus the
// meta band (mirrors gate2_meta_metrics). Appends a sentinel before the keystone
// WU's meta-band append.
struct ConsumerWu;
impl BuilderInput for ConsumerWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ConsumerWu {
    type Read = Empty;
    type Write = Cons<Accum<Mark>, Empty>;
    type Hint = Hints;
    type Ctx<'frame> = CtxFor<'frame, Empty, Cons<Accum<Mark>, Empty>, Always>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // SAFETY: Mark reserved (CAP); one append per frame stays in capacity.
        unsafe { ctx.accums().append::<Mark, _>(Mark(9)) };
    }
}

#[test]
fn nine_resource_ctxfor_dispatches_through_onmeta() {
    let provider = BumpProvider::<16384>::new();
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(R0(42)))
        .with(Resource::new(R1(1)))
        .with(Resource::new(R2(2)))
        .with(Resource::new(R3(3)))
        .with(Resource::new(R4(4)))
        .with(Resource::new(R5(5)))
        .with(Resource::new(R6(6)))
        .with(Resource::new(R7(7)))
        .with(Resource::new(R8(8)))
        .with(Virtual::<Anomaly>::new())
        .with(Accum::<Mark>::new())
        .with(ConsumerWu)
        .with(AdaptShapedWu)
        .build(store(provider), USize(CAP))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let r = scheduler.run();
    assert!(matches!(r, Outcome::Ok(())));

    // The keystone WU read R0(42) through the projected nine-resource Ctx and
    // appended it after the consumer's sentinel(9). Reading the buffer back
    // proves the CtxFor-computed nine-resource Context dispatched.
    let len = scheduler.__bindings().__len_cell().get().0;
    let base = scheduler.__bindings().__ptr().as_ptr();
    assert_eq!(len, 2, "consumer and keystone WU both ran this frame");
    // SAFETY: two records appended this frame into a reset buffer.
    let buf = unsafe { [core::ptr::read(base.add(0)).0, core::ptr::read(base.add(1)).0] };
    // R0..R8 = 42,1,2,3,4,5,6,7,8; sum = 78.
    assert_eq!(
        buf,
        [9, 78],
        "consumer(9) -> keystone reads all nine resources through the CtxFor Ctx (sum 78)",
    );
}

// A3b: test-local resource values are bare scalars/markers with no Seq/Map
// collection members, so their L1 morsel footprint is zero.
impl hilavitkutin_api::footprint::ResourceFootprint for R0 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for R1 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for R2 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for R3 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for R4 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for R5 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for R6 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for R7 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for R8 {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
