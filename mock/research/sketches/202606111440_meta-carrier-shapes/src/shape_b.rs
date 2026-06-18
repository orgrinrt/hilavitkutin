//! Shape B probe: dedicated meta carrier (a second cons-list, registered
//! separately, walked once per frame before/after consumer dispatch).
//!
//! Two sub-probes, both compile-probes against the real engine machinery:
//!
//! 1. REGISTRATION ROUTING (the builder typestate cost). A real Shape B builder
//!    must route each `.with(unit)` into one of TWO retained WuCons lists by
//!    lifecycle rank, at the type level. Probed here as a `MiniBuilder` whose
//!    `with` picks the destination list through a const-bool-keyed router
//!    (`ByRank<{ is_meta::<W::Sched>() }>`, a generic-const-expression in
//!    const-argument position) and appends through the engine's order-preserving
//!    `WuAppend`. The routed types are pinned by ascription, so mis-routing is a
//!    compile error.
//!
//! 2. TWO-CARRIER DISPATCH. The same `RunTrunkDispatch` + grouping machinery
//!    instantiates once per carrier: the meta carrier walks its rank bands once
//!    per frame (leading bands before the consumer loop, trailing after, plan
//!    band gated on plan-dirty), the consumer carrier walks ALL its phases per
//!    morsel with no band arithmetic at all (it holds no meta unit by
//!    construction). The driver signature carries the doubled bound block
//!    (2 x RunTrunkDispatch, 2 x BundleMasks, 4 witness lists), which is the
//!    same doubling `Scheduler::run`'s signature would take.
//!
//! Asserted: routing produces exactly the expected per-carrier types, meta
//! executes once per frame, consumers per morsel, lifecycle order holds, plan
//! band skips on a clean frame, consumer output is correct. `#[inline(never)]`
//! on the driver for the objdump pass.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(dead_code)]

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

use arvo::{Bool, Identity, USize};
use arvo_bitmask::{BitAccess, BitLogic};
use arvo_tensor::{Capacity, ConstCapacity};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, MetaRef, PtrNil, VirtNil,
};
use hilavitkutin::dispatch::morsel::MorselRange;
use hilavitkutin::dispatch::trunk_dispatch::RunTrunkDispatch;
use hilavitkutin::dispatch::{WuCons, WuNil};
use hilavitkutin::meta::MetaBlock;
use hilavitkutin::plan::grouping::{
    consumer_phase_end, phase_count, plan_phase_count, pre_consumer_phase_count, BundleMasks,
};
use hilavitkutin::plan::{DefaultPlanDims, PlanDims};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::meta::RANK_CONSUMER;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::{PassStart, PlanStage, ScheduleEnd};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, HasSchedule, Lifecycle, OnMeta, WorkUnit};
use hilavitkutin_api::work_unit_values::WuAppend;
use hilavitkutin_providers::ArenaColumnStorage;

// ---------------------------------------------------------------------------
// Execution log (same convention as shape_a).

const EV_PLAN: u32 = 1;
const EV_PASS: u32 = 2;
const EV_C1: u32 = 3;
const EV_C2: u32 = 4;
const EV_END: u32 = 5;

static LOG: [AtomicU32; 4096] = [const { AtomicU32::new(0) }; 4096];
static LOG_LEN: AtomicUsize = AtomicUsize::new(0);

fn log_event(e: u32) {
    let i = LOG_LEN.fetch_add(1, Ordering::Relaxed);
    LOG[i].store(e, Ordering::Relaxed);
}

fn take_log() -> Vec<u32> {
    let n = LOG_LEN.swap(0, Ordering::Relaxed);
    (0..n).map(|i| LOG[i].load(Ordering::Relaxed)).collect()
}

fn count(log: &[u32], e: u32) -> usize {
    log.iter().filter(|&&x| x == e).count()
}

// ---------------------------------------------------------------------------
// Fixture (same units as shape_a).

#[derive(Copy, Clone)]
struct In(u32);
#[derive(Copy, Clone)]
struct Mid(u32);
#[derive(Copy, Clone)]
struct Out(u32);
type One<T> = Cons<Column<T>, Empty>;

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

type MetaCtx<'frame> =
    EngineCtx<'frame, Empty, Empty, PtrNil, ColPtrNil, ColPtrNil, AccPtrNil, VirtNil, MetaRef<'frame>>;

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
    type Write = Empty;
    type Hint = Hints;
    type Ctx<'frame> = MetaCtx<'frame>;
    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {
        log_event(EV_PLAN);
    }
}

struct PassWu;
impl BuilderInput for PassWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl HasSchedule for PassWu {
    type Sched = OnMeta<PassStart>;
}
impl WorkUnit<OnMeta<PassStart>> for PassWu {
    type Read = Empty;
    type Write = Empty;
    type Hint = Hints;
    type Ctx<'frame> = MetaCtx<'frame>;
    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {
        log_event(EV_PASS);
    }
}

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
    type Write = Empty;
    type Hint = Hints;
    type Ctx<'frame> = MetaCtx<'frame>;
    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {
        log_event(EV_END);
    }
}

struct C1;
impl BuilderInput for C1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for C1 {
    type Read = One<In>;
    type Write = One<Mid>;
    type Hint = Hints;
    type Ctx<'frame> =
        EngineCtx<'frame, One<In>, One<Mid>, PtrNil, ColPtrCons<In, ColPtrNil>, ColPtrCons<Mid, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        log_event(EV_C1);
        ctx.each().run(|i| {
            let v = unsafe { ctx.reader().read::<In, _>(i) };
            unsafe { ctx.writer().write::<Mid, _>(i, Mid(v.0 + 1)) };
        });
    }
}

struct C2;
impl BuilderInput for C2 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for C2 {
    type Read = One<Mid>;
    type Write = One<Out>;
    type Hint = Hints;
    type Ctx<'frame> =
        EngineCtx<'frame, One<Mid>, One<Out>, PtrNil, ColPtrCons<Mid, ColPtrNil>, ColPtrCons<Out, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        log_event(EV_C2);
        ctx.each().run(|i| {
            let m = unsafe { ctx.reader().read::<Mid, _>(i) };
            unsafe { ctx.writer().write::<Out, _>(i, Out(m.0 * 10)) };
        });
    }
}

// ---------------------------------------------------------------------------
// Sub-probe 1: type-level registration routing by lifecycle rank.

/// Whether a schedule is a meta lifecycle schedule (anything not consumer-rank).
const fn is_meta<S: Lifecycle>() -> bool {
    S::RANK.0 != RANK_CONSUMER.0
}

/// Const-bool-keyed router: two non-overlapping impls select the destination
/// list. No specialization; the const argument is the discriminant.
struct ByRank<const META: bool>;

trait Route<W, MetaL, ConsL> {
    type MetaOut;
    type ConsOut;
    fn route(w: W, m: MetaL, c: ConsL) -> (Self::MetaOut, Self::ConsOut);
}

impl<W, MetaL: WuAppend<W>, ConsL> Route<W, MetaL, ConsL> for ByRank<true> {
    type MetaOut = <MetaL as WuAppend<W>>::Out;
    type ConsOut = ConsL;
    fn route(w: W, m: MetaL, c: ConsL) -> (Self::MetaOut, Self::ConsOut) {
        (m.append(w), c)
    }
}

impl<W, MetaL, ConsL: WuAppend<W>> Route<W, MetaL, ConsL> for ByRank<false> {
    type MetaOut = MetaL;
    type ConsOut = <ConsL as WuAppend<W>>::Out;
    fn route(w: W, m: MetaL, c: ConsL) -> (Self::MetaOut, Self::ConsOut) {
        (m, c.append(w))
    }
}

/// Two retained carriers; `with` routes by rank at the type level.
struct MiniBuilder<MetaL, ConsL> {
    meta: MetaL,
    cons: ConsL,
}

impl MiniBuilder<WuNil, WuNil> {
    fn new() -> Self {
        Self { meta: WuNil, cons: WuNil }
    }
}

impl<MetaL, ConsL> MiniBuilder<MetaL, ConsL> {
    fn with<W>(
        self,
        w: W,
    ) -> MiniBuilder<
        <ByRank<{ is_meta::<<W as HasSchedule>::Sched>() }> as Route<W, MetaL, ConsL>>::MetaOut,
        <ByRank<{ is_meta::<<W as HasSchedule>::Sched>() }> as Route<W, MetaL, ConsL>>::ConsOut,
    >
    where
        W: HasSchedule,
        ByRank<{ is_meta::<<W as HasSchedule>::Sched>() }>: Route<W, MetaL, ConsL>,
    {
        let (meta, cons) =
            <ByRank<{ is_meta::<<W as HasSchedule>::Sched>() }> as Route<W, MetaL, ConsL>>::route(
                w, self.meta, self.cons,
            );
        MiniBuilder { meta, cons }
    }
}

// ---------------------------------------------------------------------------
// Memory provider (same as shape_a).

struct HeapBump {
    buf: UnsafeCell<Box<[MaybeUninit<u8>]>>,
    cap: usize,
    used: Cell<usize>,
}
impl HeapBump {
    fn new(bytes: usize) -> Self {
        let v: Box<[MaybeUninit<u8>]> = (0..bytes).map(|_| MaybeUninit::uninit()).collect();
        Self { buf: UnsafeCell::new(v), cap: bytes, used: Cell::new(0) }
    }
}
unsafe impl Send for HeapBump {}
unsafe impl Sync for HeapBump {}
impl MemoryProviderApi for HeapBump {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = unsafe { (*self.buf.get()).as_mut_ptr() as *mut u8 };
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > self.cap {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// ---------------------------------------------------------------------------
// Sub-probe 2: two-carrier dispatch. The doubled bound block below (two
// RunTrunkDispatch instantiations, two BundleMasks, four witness lists) is the
// measured builder/scheduler signature cost of Shape B.

const N: usize = 1024;
const MORSEL: usize = 256;

/// Two-carrier dispatch rig: both carriers + bindings pinned concrete on the
/// struct so the driver method infers ONLY the four witness lists (the same
/// inference-shape fix shape_a needed: free fns generic over carrier, bindings,
/// AND witness lists stall the old solver on the higher-ranked
/// AccumProject/VirtualProject GAT normalization).
struct Rig2<'a, MC, CC, B, Stores, CU, CS, Adj> {
    meta: &'a MC,
    cons: &'a CC,
    bindings: &'a B,
    mb: &'a MetaBlock,
    dirty: Adj,
    _dims: core::marker::PhantomData<(Stores, CU, CS)>,
}

impl<'a, MC, CC, B, Stores, CU, CS, Adj> Rig2<'a, MC, CC, B, Stores, CU, CS, Adj>
where
    CU: Capacity + ConstCapacity,
    CS: Capacity,
    Adj: BitAccess + Identity + Copy,
{
    #[inline(never)]
    fn frame<WitM, GWM, WitC, GWC>(&self, total: usize, msize: usize, epoch: USize, plan_dirty: bool)
    where
        MC: RunTrunkDispatch<MC, B, WitM, GWM, Stores, CU, CS, Adj, 0>,
        MC: BundleMasks<Stores, GWM, CS>,
        CC: RunTrunkDispatch<CC, B, WitC, GWC, Stores, CU, CS, Adj, 0>,
        CC: BundleMasks<Stores, GWC, CS>,
    {
        // Meta carrier, leading bands: its own rank-band grouping (no consumer
        // units in this carrier, so its waist analysis is over meta units alone).
        let m_nphases = phase_count::<MC, Stores, GWM, CU, CS, Adj>().0;
        let m_plan = plan_phase_count::<MC, Stores, GWM, CU, CS, Adj>().0;
        let m_pre = pre_consumer_phase_count::<MC, Stores, GWM, CU, CS, Adj>().0;
        let m_cend = consumer_phase_end::<MC, Stores, GWM, CU, CS, Adj>().0;
        let start = if plan_dirty { 0 } else { m_plan };
        let mut p = start;
        while p < m_pre {
            self.meta.dispatch(
                self.meta,
                USize(p),
                self.mb,
                self.bindings,
                MorselRange::new(USize::ZERO, USize::ZERO),
                self.dirty,
                epoch,
            );
            p += 1;
        }
        // Consumer carrier: ALL its phases per morsel; no band arithmetic exists
        // on this walk because the carrier cannot hold a meta unit.
        let c_nphases = phase_count::<CC, Stores, GWC, CU, CS, Adj>().0;
        let mut s = 0;
        while s < total {
            let len = msize.min(total - s);
            let mut p = 0;
            while p < c_nphases {
                self.cons.dispatch(
                    self.cons,
                    USize(p),
                    self.mb,
                    self.bindings,
                    MorselRange::new(USize(s), USize(len)),
                    self.dirty,
                    epoch,
                );
                p += 1;
            }
            s += len;
        }
        // Meta carrier, trailing bands: once per frame.
        let mut p = m_cend;
        while p < m_nphases {
            self.meta.dispatch(
                self.meta,
                USize(p),
                self.mb,
                self.bindings,
                MorselRange::new(USize::ZERO, USize::ZERO),
                self.dirty,
                epoch,
            );
            p += 1;
        }
    }
}

// ---------------------------------------------------------------------------

type CUx = <DefaultPlanDims as PlanDims>::Units;
type CSx = <DefaultPlanDims as PlanDims>::Stores;
type AdjX = <DefaultPlanDims as PlanDims>::AdjRow;
type StoresL = Cons<Column<In>, Cons<Column<Mid>, Cons<Column<Out>, Empty>>>;

fn main() {
    let morsels = N / MORSEL;
    // The engine builder supplies the bindings (the data plane is shared in
    // Shape B too; only the unit carrier splits).
    let provider = HeapBump::new(8 * 1024 * 1024);
    let scheduler = Scheduler::builder()
        .with(Column::<In>::new())
        .with(Column::<Mid>::new())
        .with(Column::<Out>::new())
        .with(C1)
        .with(C2)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build"));

    let out_base = scheduler.__bindings().__ptr().as_ptr() as *mut Out;
    let mid_base = scheduler.__bindings().__tail().__ptr().as_ptr() as *mut Mid;
    let in_base = scheduler.__bindings().__tail().__tail().__ptr().as_ptr() as *mut In;
    for i in 0..N {
        unsafe { *in_base.add(i) = In(i as u32) };
    }
    let reset_out = |mid: *mut Mid, out: *mut Out| {
        for i in 0..N {
            unsafe {
                *mid.add(i) = Mid(0xDEAD_BEEF);
                *out.add(i) = Out(0xDEAD_BEEF);
            }
        }
    };

    // Routing probe: one mixed registration sequence, two routed carriers.
    let b = MiniBuilder::new().with(PlanWu).with(PassWu).with(C1).with(C2).with(EndWu);
    // Compile-time routing assertion: the ascriptions fail to compile if any
    // unit routed to the wrong list or order was not preserved.
    let meta_carrier: WuCons<PlanWu, WuCons<PassWu, WuCons<EndWu, WuNil>>> = b.meta;
    let cons_carrier: WuCons<C1, WuCons<C2, WuNil>> = b.cons;
    println!("routing: meta=[PlanWu, PassWu, EndWu] cons=[C1, C2] (type-ascription checked)");

    let mb = MetaBlock::default();
    let all = AdjX::default().bitnot();
    let rig: Rig2<'_, _, _, _, StoresL, CUx, CSx, _> = Rig2 {
        meta: &meta_carrier,
        cons: &cons_carrier,
        bindings: scheduler.__bindings(),
        mb: &mb,
        dirty: all,
        _dims: core::marker::PhantomData,
    };

    // Plan-dirty frame: every band; meta once per frame, consumers per morsel.
    reset_out(mid_base, out_base);
    rig.frame::<_, _, _, _>(N, MORSEL, USize(201), true);
    let log = take_log();
    println!(
        "two-carrier (plan-dirty): plan={} pass={} end={} c1={} c2={} order={:?}",
        count(&log, EV_PLAN),
        count(&log, EV_PASS),
        count(&log, EV_END),
        count(&log, EV_C1),
        count(&log, EV_C2),
        &log
    );
    assert_eq!(count(&log, EV_PLAN), 1, "two-carrier: plan band once per frame");
    assert_eq!(count(&log, EV_PASS), 1, "two-carrier: pass-start band once per frame");
    assert_eq!(count(&log, EV_END), 1, "two-carrier: schedule-end band once per frame");
    assert_eq!(count(&log, EV_C1), morsels, "two-carrier: consumers per morsel");
    assert_eq!(count(&log, EV_C2), morsels, "two-carrier: consumers per morsel");
    assert_eq!(log[0], EV_PLAN);
    assert_eq!(log[1], EV_PASS);
    assert_eq!(*log.last().unwrap(), EV_END);
    for i in 0..N {
        let o = unsafe { core::ptr::read(out_base.add(i)) };
        assert_eq!(o.0, (i as u32 + 1) * 10, "two-carrier: Out[{i}]");
    }

    // Clean frame: plan band skipped on the meta carrier; the rest unchanged.
    rig.frame::<_, _, _, _>(N, MORSEL, USize(202), false);
    let log = take_log();
    println!(
        "two-carrier (clean): plan={} pass={} end={} c1={} c2={}",
        count(&log, EV_PLAN),
        count(&log, EV_PASS),
        count(&log, EV_END),
        count(&log, EV_C1),
        count(&log, EV_C2)
    );
    assert_eq!(count(&log, EV_PLAN), 0, "two-carrier clean: plan band skipped");
    assert_eq!(count(&log, EV_PASS), 1);
    assert_eq!(count(&log, EV_END), 1);

    println!(
        "WORKS: shape B dedicated meta carrier. Rank routing resolves at the type level \
         (const-bool-keyed Route impls + WuAppend, no specialization); the same \
         RunTrunkDispatch/grouping machinery instantiates once per carrier; meta runs once \
         per frame around a consumer morsel loop that carries no band arithmetic."
    );
}
