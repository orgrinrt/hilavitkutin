//! Shape A probe: shared consumer carrier, meta bands hoisted out of the morsel loop.
//!
//! The shipped state (truth-of-impl, `scheduler/mod.rs::run` + `dispatch_trunks`):
//! ONE WuCons carrier holds meta and consumer units; the rank-outer grouping
//! renumbers `(rank, waist)` pairs into contiguous lifecycle bands; the
//! record-bearing morsel-outer path loops morsels OUTER and phases INNER, so the
//! meta bands (leading plan / pass-start, trailing schedule-end) dispatch once
//! per MORSEL, not once per frame. That is the wart this probe measures, first
//! through the real `Scheduler::run` and then through a sketch-local replica of
//! the same loop shape over the real `RunTrunkDispatch` machinery.
//!
//! The fix direction probed: hoist the band walk outside the morsel loop using
//! the band const fns the grouping already ships (`plan_phase_count`,
//! `pre_consumer_phase_count`, `consumer_phase_end`, `phase_count`): leading
//! meta bands once per frame (empty morsel, all-ones mask, the shipped
//! `run_parallel` designated-thread shape), consumer bands per morsel, trailing
//! meta bands once per frame. The engine is NOT edited; the restructure lives in
//! this sketch's `hoisted_frame` driver, generic over the same bound block as
//! the shipped `dispatch_trunks`.
//!
//! Asserted: meta execution counts (per-frame vs per-morsel), lifecycle ordering
//! (plan -> pass-start -> consumers -> schedule-end), plan-band skip on a clean
//! frame, and consumer output correctness over the morsel loop. `#[inline(never)]`
//! on both drivers so objdump has symbols for the codegen observations.

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
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::{PassStart, PlanStage, ScheduleEnd};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, HasSchedule, OnMeta, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

// ---------------------------------------------------------------------------
// Execution log: every execute() appends its event id, so a frame's dispatch
// order and per-unit counts are assertable after the fact.

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
// Fixture: two RAW-chained consumer units over three columns, three meta units
// (one per lifecycle point the bands distinguish: plan / pass-start leading,
// schedule-end trailing).

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

// Meta Ctx: no store access; the 9th param is MetaRef (OnMeta units are meta
// work units, MetaPtrFor pins it).
type MetaCtx<'frame> =
    EngineCtx<'frame, Empty, Empty, PtrNil, ColPtrNil, ColPtrNil, AccPtrNil, VirtNil, MetaRef<'frame>>;

struct PlanWu; // OnMeta<PlanStage>, rank 0: leading plan band, dirty frames only
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

struct PassWu; // OnMeta<PassStart>, rank 2: leading band, every frame
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

struct EndWu; // OnMeta<ScheduleEnd>, rank 4: trailing epilogue band
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

struct C1; // consumer: In -> Mid
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

struct C2; // consumer: Mid -> Out (RAW edge on Mid)
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
// Memory provider (heap-bump; sketch-only host fixture).

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
// Drivers. Both are generic over the same bound block as the shipped
// `Scheduler::dispatch_trunks` and drive the real const-gated RunTrunkDispatch
// walk; only the loop nesting differs.

const N: usize = 1024;
const MORSEL: usize = 256; // matches DefaultRunCfg::MORSEL_SIZE, so 4 morsels per frame

/// Dispatch rig: carrier + bindings pinned concrete on a struct so the driver
/// methods infer ONLY the two witness lists, exactly the inference shape the
/// shipped `scheduler.run::<_, _>()` call sites prove. (A first attempt with
/// free fns generic over carrier, bindings, Adj, AND the witness lists stalled
/// the old trait solver on the higher-ranked `AccumProject` / `VirtualProject`
/// GAT normalization in `RunFiber`'s Ctx-equality bound: E0271 with the
/// projections left unnormalized. Pinning everything but `Witnesses, GW` on the
/// rig resolves it; see FINDINGS.)
struct Rig<'a, C, B, Stores, CU, CS, Adj> {
    carrier: &'a C,
    bindings: &'a B,
    mb: &'a MetaBlock,
    dirty: Adj,
    _dims: core::marker::PhantomData<(Stores, CU, CS)>,
}

impl<'a, C, B, Stores, CU, CS, Adj> Rig<'a, C, B, Stores, CU, CS, Adj>
where
    CU: Capacity + ConstCapacity,
    CS: Capacity,
    Adj: BitAccess + Identity + Copy,
{
    /// Replica of the SHIPPED single-core record-bearing loop shape (`run`'s
    /// morsel-outer arm + `dispatch_trunks`): morsels outer, ALL phases inner.
    /// The meta bands ride inside the morsel loop, so meta units fire once per
    /// morsel.
    #[inline(never)]
    fn current_shape_frame<Witnesses, GW>(&self, total: usize, msize: usize, epoch: USize, plan_dirty: bool)
    where
        C: RunTrunkDispatch<C, B, Witnesses, GW, Stores, CU, CS, Adj, 0>,
        C: BundleMasks<Stores, GW, CS>,
    {
        let nphases = phase_count::<C, Stores, GW, CU, CS, Adj>().0;
        let start = if plan_dirty { 0 } else { plan_phase_count::<C, Stores, GW, CU, CS, Adj>().0 };
        let mut s = 0;
        while s < total {
            let len = msize.min(total - s);
            let mut p = start;
            while p < nphases {
                self.carrier.dispatch(
                    self.carrier,
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
    }

    /// The Shape A fix direction: band walk OUTSIDE the morsel loop. Leading
    /// meta bands once per frame (empty morsel, the shipped run_parallel
    /// designated-thread shape), consumer bands per morsel, trailing meta bands
    /// once per frame. Same const fns, same dispatch walk; only the nesting
    /// moves.
    #[inline(never)]
    fn hoisted_frame<Witnesses, GW>(&self, total: usize, msize: usize, epoch: USize, plan_dirty: bool)
    where
        C: RunTrunkDispatch<C, B, Witnesses, GW, Stores, CU, CS, Adj, 0>,
        C: BundleMasks<Stores, GW, CS>,
    {
        let nphases = phase_count::<C, Stores, GW, CU, CS, Adj>().0;
        let plan = plan_phase_count::<C, Stores, GW, CU, CS, Adj>().0;
        let pre = pre_consumer_phase_count::<C, Stores, GW, CU, CS, Adj>().0;
        let cend = consumer_phase_end::<C, Stores, GW, CU, CS, Adj>().0;
        // leading meta bands: once per frame, plan band skipped on a clean frame
        let start = if plan_dirty { 0 } else { plan };
        let mut p = start;
        while p < pre {
            self.carrier.dispatch(
                self.carrier,
                USize(p),
                self.mb,
                self.bindings,
                MorselRange::new(USize::ZERO, USize::ZERO),
                self.dirty,
                epoch,
            );
            p += 1;
        }
        // consumer bands: per morsel
        let mut s = 0;
        while s < total {
            let len = msize.min(total - s);
            let mut p = pre;
            while p < cend {
                self.carrier.dispatch(
                    self.carrier,
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
        // trailing meta bands: once per frame
        let mut p = cend;
        while p < nphases {
            self.carrier.dispatch(
                self.carrier,
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
// Sketch-local store numbering for the grouping masks (type-keyed bindings
// projection is independent of this ordering).
type StoresL = Cons<Column<In>, Cons<Column<Mid>, Cons<Column<Out>, Empty>>>;

fn main() {
    let morsels = N / MORSEL;
    let provider = HeapBump::new(8 * 1024 * 1024);
    let mut scheduler = Scheduler::builder()
        .with(Column::<In>::new())
        .with(Column::<Mid>::new())
        .with(Column::<Out>::new())
        .with(PlanWu)
        .with(PassWu)
        .with(C1)
        .with(C2)
        .with(EndWu)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build"));

    // Column bases (bindings cons head = last-registered store).
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

    // -------------------------------------------------------------------
    // Baseline: the SHIPPED Scheduler::run, frame 1 (plan-dirty, all units
    // dirty, no accumulator -> morsel-outer path). The wart: meta units fire
    // once per MORSEL.
    reset_out(mid_base, out_base);
    let _ = scheduler.run::<_, _>();
    let log = take_log();
    println!(
        "shipped run frame1: plan={} pass={} end={} c1={} c2={} (morsels={})",
        count(&log, EV_PLAN),
        count(&log, EV_PASS),
        count(&log, EV_END),
        count(&log, EV_C1),
        count(&log, EV_C2),
        morsels
    );
    assert_eq!(count(&log, EV_PLAN), morsels, "shipped wart: plan band fired per morsel");
    assert_eq!(count(&log, EV_PASS), morsels, "shipped wart: pass-start band fired per morsel");
    assert_eq!(count(&log, EV_END), morsels, "shipped wart: schedule-end band fired per morsel");
    assert_eq!(count(&log, EV_C1), morsels);
    assert_eq!(count(&log, EV_C2), morsels);

    // Shipped frame 2 (clean): the incremental dirty mask is empty and the
    // morsel-outer walk gates EVERY member on its dirty bit, so the meta units
    // do not run at all (second face of the same wart: lifecycle hooks should
    // run every frame).
    let _ = scheduler.run::<_, _>();
    let log = take_log();
    println!(
        "shipped run frame2 (clean): plan={} pass={} end={} c1={} c2={}",
        count(&log, EV_PLAN),
        count(&log, EV_PASS),
        count(&log, EV_END),
        count(&log, EV_C1),
        count(&log, EV_C2)
    );
    assert_eq!(count(&log, EV_PASS), 0, "shipped wart: clean frame skips meta units entirely");
    assert_eq!(count(&log, EV_END), 0, "shipped wart: clean frame skips meta units entirely");

    // -------------------------------------------------------------------
    // Sketch drivers over the same machinery. Hand-built carrier (same units,
    // same order); bindings shared with the scheduler; sketch-local MetaBlock.
    let carrier = WuCons {
        head: PlanWu,
        tail: WuCons {
            head: PassWu,
            tail: WuCons { head: C1, tail: WuCons { head: C2, tail: WuCons { head: EndWu, tail: WuNil } } },
        },
    };
    let mb = MetaBlock::default();
    let all = AdjX::default().bitnot();
    let rig: Rig<'_, _, _, StoresL, CUx, CSx, _> = Rig {
        carrier: &carrier,
        bindings: scheduler.__bindings(),
        mb: &mb,
        dirty: all,
        _dims: core::marker::PhantomData,
    };

    // Replica of the shipped loop shape: same wart, same counts.
    reset_out(mid_base, out_base);
    rig.current_shape_frame::<_, _>(N, MORSEL, USize(101), true);
    let log = take_log();
    println!(
        "replica (bands inside morsel loop): plan={} pass={} end={} c1={} c2={}",
        count(&log, EV_PLAN),
        count(&log, EV_PASS),
        count(&log, EV_END),
        count(&log, EV_C1),
        count(&log, EV_C2)
    );
    assert_eq!(count(&log, EV_PLAN), morsels);
    assert_eq!(count(&log, EV_PASS), morsels);
    assert_eq!(count(&log, EV_END), morsels);

    // Hoisted bands, plan-dirty frame: meta once per frame, consumers per
    // morsel, lifecycle order preserved.
    reset_out(mid_base, out_base);
    rig.hoisted_frame::<_, _>(N, MORSEL, USize(102), true);
    let log = take_log();
    println!(
        "hoisted (plan-dirty): plan={} pass={} end={} c1={} c2={} order={:?}",
        count(&log, EV_PLAN),
        count(&log, EV_PASS),
        count(&log, EV_END),
        count(&log, EV_C1),
        count(&log, EV_C2),
        &log
    );
    assert_eq!(count(&log, EV_PLAN), 1, "hoisted: plan band once per frame");
    assert_eq!(count(&log, EV_PASS), 1, "hoisted: pass-start band once per frame");
    assert_eq!(count(&log, EV_END), 1, "hoisted: schedule-end band once per frame");
    assert_eq!(count(&log, EV_C1), morsels, "hoisted: consumers still per morsel");
    assert_eq!(count(&log, EV_C2), morsels, "hoisted: consumers still per morsel");
    // Ordering: plan first, pass-start second, schedule-end last, consumers between.
    assert_eq!(log[0], EV_PLAN);
    assert_eq!(log[1], EV_PASS);
    assert_eq!(*log.last().unwrap(), EV_END);
    // Consumer output correct across the morsel loop: Out[i] = (In[i]+1)*10.
    for i in 0..N {
        let o = unsafe { core::ptr::read(out_base.add(i)) };
        assert_eq!(o.0, (i as u32 + 1) * 10, "hoisted: Out[{i}]");
    }

    // Hoisted bands, clean frame: plan band skipped, pass-start + schedule-end
    // still once per frame (the all-ones meta mask cures the clean-frame skip).
    rig.hoisted_frame::<_, _>(N, MORSEL, USize(103), false);
    let log = take_log();
    println!(
        "hoisted (clean): plan={} pass={} end={} c1={} c2={}",
        count(&log, EV_PLAN),
        count(&log, EV_PASS),
        count(&log, EV_END),
        count(&log, EV_C1),
        count(&log, EV_C2)
    );
    assert_eq!(count(&log, EV_PLAN), 0, "hoisted clean: plan band skipped");
    assert_eq!(count(&log, EV_PASS), 1, "hoisted clean: pass-start still once per frame");
    assert_eq!(count(&log, EV_END), 1, "hoisted clean: schedule-end still once per frame");

    println!(
        "WORKS: shape A hoisted-band dispatch. Same carrier, same grouping const fns, same \
         RunTrunkDispatch walk; moving the band walk outside the morsel loop turns per-morsel \
         meta into once-per-frame meta on the record-bearing path, preserves lifecycle order \
         and consumer per-morsel dispatch, and keeps the plan-band clean-frame skip."
    );
}
