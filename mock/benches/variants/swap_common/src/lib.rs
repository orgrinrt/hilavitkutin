//! Shared workloads for the swap benches (spec S7, round 202607200500).
//!
//! Two preparations over the real engine:
//!
//! `prepare_band` builds the asymmetry carrier: a `Replaceable` value
//! resource feeding a producer-to-consumer column chain, an unrelated
//! chain off a second input, a `PlanAffecting` config resource, and an
//! `OnMeta<PlanStage>` plan unit whose body carries a small fixed
//! synthetic cost (a stand-in for the future plan recompute; the adapt
//! subsystem that fills the band is sequenced later, so the bench
//! measures the DISPATCH asymmetry between a value swap's dirty cone
//! and a plan swap's cone plus band, not the eventual recompute
//! itself). The three arms differ only in what the untimed setup does
//! before the timed frame: nothing (`SwapMode::None`, the all-clean
//! skip baseline), `replace_value` (`SwapMode::Value`), or
//! `replace_resource` (`SwapMode::Plan`).
//!
//! `prepare_cost_*` builds a minimal one-resource scheduler per value
//! size (64 B, 1 KiB, 64 KiB) and returns a closure whose timed body is
//! exactly one `replace_value` install, so the arm's median is the
//! witnessed blob write cost at that size.

use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::rc::Rc;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, MetaRef, SnapCons, SnapNil, VirtNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
    HasResourceProvider, ResourceProviderApi,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::PlanStage;
use hilavitkutin_api::store::{Column, Resource};
use hilavitkutin_api::work_unit::{Always, HasSchedule, OnMeta, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
pub use rcm_common::{fnv1a_u32_slice, HeapBump};

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

// ----- asymmetry carrier types -----

/// Replaceable value input: the producer's seed.
#[derive(Copy, Clone)]
pub struct SwapVal(pub u64);
impl hilavitkutin_api::store::Replaceable for SwapVal {}
impl hilavitkutin_api::footprint::ResourceFootprint for SwapVal {
    const L1_BYTES: USize = USize(0);
}

/// Plan-affecting config input.
#[derive(Copy, Clone)]
pub struct SwapCfg(pub u64);
impl hilavitkutin_api::run_cfg::PlanAffecting for SwapCfg {}
impl hilavitkutin_api::footprint::ResourceFootprint for SwapCfg {
    const L1_BYTES: USize = USize(0);
}

/// Unrelated chain input, never swapped: its cone must skip on every
/// non-first frame in all three arms.
#[derive(Copy, Clone)]
pub struct OtherIn(pub u64);
impl hilavitkutin_api::footprint::ResourceFootprint for OtherIn {
    const L1_BYTES: USize = USize(0);
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct C1(pub u32);
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct C2(pub u32);
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct C3(pub u32);
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Cp(pub u32);

type ReadV = Cons<Resource<SwapVal>, Empty>;
type WriteC1 = Cons<Column<C1>, Empty>;
type ReadC1 = Cons<Column<C1>, Empty>;
type WriteC2 = Cons<Column<C2>, Empty>;
type ReadO = Cons<Resource<OtherIn>, Empty>;
type WriteC3 = Cons<Column<C3>, Empty>;
type WriteCp = Cons<Column<Cp>, Empty>;

// Producer: reads the swappable value, writes C1. The value swap's cone.
struct ProdWu;
impl BuilderInput for ProdWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ProdWu {
    type Read = ReadV;
    type Write = WriteC1;
    type Hint = Hints;
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadV,
        WriteC1,
        SnapCons<SwapVal, SnapNil>,
        ColPtrNil,
        ColPtrCons<C1, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let seed: &SwapVal = ctx.resources().resource();
        let base = seed.0 as u32;
        ctx.each().run(|i| {
            // SAFETY: build reserved C1 for the record count; exclusive writer.
            unsafe {
                ctx.writer()
                    .write::<C1, _>(i, C1(base.wrapping_add(i.0 as u32)))
            };
        });
    }
}

// Consumer: reads C1, writes C2 (the second hop of the cone).
struct ConsWu;
impl BuilderInput for ConsWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ConsWu {
    type Read = ReadC1;
    type Write = WriteC2;
    type Hint = Hints;
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadC1,
        WriteC2,
        SnapNil,
        ColPtrCons<C1, ColPtrNil>,
        ColPtrCons<C2, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: the producer wrote every record the morsel covers.
            let v: C1 = unsafe { ctx.reader().read::<C1, _>(i) };
            // SAFETY: C2 reserved + exclusive.
            unsafe { ctx.writer().write::<C2, _>(i, C2(v.0.rotate_left(7))) };
        });
    }
}

// Unrelated: reads OtherIn, writes C3. Skips on all non-first frames.
struct OtherWu;
impl BuilderInput for OtherWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for OtherWu {
    type Read = ReadO;
    type Write = WriteC3;
    type Hint = Hints;
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadO,
        WriteC3,
        SnapCons<OtherIn, SnapNil>,
        ColPtrNil,
        ColPtrCons<C3, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let seed: &OtherIn = ctx.resources().resource();
        let base = seed.0 as u32;
        ctx.each().run(|i| {
            // SAFETY: C3 reserved + exclusive.
            unsafe { ctx.writer().write::<C3, _>(i, C3(base ^ i.0 as u32)) };
        });
    }
}

// Plan unit: OnMeta<PlanStage>, dispatched only inside the plan band. A
// fixed synthetic cost stands in for the future plan recompute.
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
    type Write = WriteCp;
    type Hint = Hints;
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        WriteCp,
        SnapNil,
        ColPtrNil,
        ColPtrCons<Cp, ColPtrNil>,
        AccPtrNil,
        VirtNil,
        MetaRef<'frame>,
    >;
    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {
        // Synthetic plan work: a fixed hash chain approximating a small
        // plan recompute so the band carries nonzero representative cost.
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for i in 0..4096u64 {
            h = (h ^ i).wrapping_mul(0x0000_0100_0000_01b3);
        }
        core::hint::black_box(h);
    }
}

/// Which untimed swap the arm performs before each timed frame.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SwapMode {
    /// No swap: the all-clean skip frame is the baseline.
    None,
    /// `replace_value` on the `Replaceable` input: the dirty-cone arm.
    Value,
    /// `replace_resource` on the `PlanAffecting` input: the plan-band arm.
    Plan,
}

/// Prepared asymmetry carrier behind type-erased closures.
pub struct PreparedBand {
    /// Untimed per-call setup: perform the arm's swap (or nothing).
    pub swap: Box<dyn FnMut()>,
    /// The timed unit: one frame.
    pub run_frame: Box<dyn FnMut()>,
    /// FNV over the cone outputs for harness validation.
    pub finish: Box<dyn FnMut() -> u64>,
}

pub fn prepare_band(mode: SwapMode, seed: u64, records: usize) -> PreparedBand {
    let provider = HeapBump::new(8 * records * 4 + 8 * 64 + (1 << 16));
    let sched = Scheduler::builder()
        .with(Column::<C1>::new())
        .with(Column::<C2>::new())
        .with(Column::<C3>::new())
        .with(Column::<Cp>::new())
        .with(Resource::new(SwapVal(seed)))
        .with(Resource::new(SwapCfg(seed ^ 0xA5A5)))
        .with(Resource::new(OtherIn(seed.rotate_left(17))))
        .with(ProdWu)
        .with(ConsWu)
        .with(OtherWu)
        .with(PlanWu)
        .build(store(provider), USize(records))
        .unwrap_or_else(|_| panic!("engine build should succeed"));
    let sched = Rc::new(RefCell::new(sched));

    // Warm frame: the cold first frame runs everything and leaves the
    // steady state every arm times from.
    {
        let mut s = sched.borrow_mut();
        let _ = s.run();
    }

    // The swapped value is a pure function of the seed (never a call
    // counter): the harness determinism check calls the arm twice per
    // seed and compares outputs, and dirty-marking is unconditional, so
    // re-installing the same value still pays the full cone or band.
    let swap_rc = Rc::clone(&sched);
    let swapped_val = seed ^ 0x5150_C0DE_5150_C0DE;
    let swap = Box::new(move || match mode {
        SwapMode::None => {}
        SwapMode::Value => swap_rc.borrow_mut().replace_value(SwapVal(swapped_val)),
        SwapMode::Plan => swap_rc.borrow_mut().replace_resource(SwapCfg(swapped_val)),
    }) as Box<dyn FnMut()>;

    let frame_rc = Rc::clone(&sched);
    let run_frame = Box::new(move || {
        let r = frame_rc.borrow_mut().run();
        core::hint::black_box(&r);
    }) as Box<dyn FnMut()>;

    let finish_rc = Rc::clone(&sched);
    let finish = Box::new(move || {
        let s = finish_rc.borrow();
        let b = s.__bindings();
        // Bindings head is the last-registered store; the columns
        // registered first sit deepest. Read C2 (cone output) only: at
        // depth counted from the resources inward. Resolve C2 through
        // the tail walk: registration order C1,C2,C3,Cp,Rv,Rcfg,Rother
        // puts C2 at depth 5 from the head.
        let base = b
            .__tail()
            .__tail()
            .__tail()
            .__tail()
            .__tail()
            .__ptr()
            .as_ptr() as *const u32;
        // SAFETY: C2 holds `records` reserved records written on the
        // last cone-dirty frame.
        let slice = unsafe { core::slice::from_raw_parts(base, records) };
        fnv1a_u32_slice(slice)
    }) as Box<dyn FnMut() -> u64>;

    PreparedBand {
        swap,
        run_frame,
        finish,
    }
}

// ----- swap install cost (value sizes) -----

macro_rules! def_cost {
    ($ty:ident, $words:expr, $prep:ident) => {
        /// Fixed-size swappable payload for the install-cost arm.
        #[derive(Copy, Clone)]
        pub struct $ty(pub [u32; $words]);
        impl hilavitkutin_api::store::Replaceable for $ty {}
        impl hilavitkutin_api::footprint::ResourceFootprint for $ty {
            const L1_BYTES: USize = USize(0);
        }

        /// Prepared install-cost closure: the timed body is one
        /// `replace_value` of the payload.
        pub fn $prep(seed: u64) -> (Box<dyn FnMut(u64)>, Box<dyn FnMut() -> u64>) {
            let provider = HeapBump::new(($words * 4) * 2 + (1 << 16));
            let sched = Scheduler::builder()
                .with(Resource::new($ty([seed as u32; $words])))
                .build(store(provider), USize(0))
                .unwrap_or_else(|_| panic!("engine build should succeed"));
            let sched = Rc::new(RefCell::new(sched));

            let swap_rc = Rc::clone(&sched);
            let swap = Box::new(move |x: u64| {
                swap_rc.borrow_mut().replace_value($ty([x as u32; $words]));
            }) as Box<dyn FnMut(u64)>;

            let finish_rc = Rc::clone(&sched);
            let finish = Box::new(move || {
                let s = finish_rc.borrow();
                // SAFETY: the one-record resource column is live.
                let v = unsafe { &*s.__bindings().__ptr().as_ptr() };
                let arr: &$ty = v;
                fnv1a_u32_slice(&arr.0[..1]) ^ fnv1a_u32_slice(&arr.0[$words - 1..])
            }) as Box<dyn FnMut() -> u64>;

            (swap, finish)
        }
    };
}

def_cost!(V64B, 16, prepare_cost_64b);
def_cost!(V1K, 256, prepare_cost_1k);
def_cost!(V64K, 16384, prepare_cost_64k);

// Silence the unused-import lint for items only some cfg paths use.
const _: fn() = || {
    let _ = core::mem::size_of::<UnsafeCell<MaybeUninit<u8>>>();
    let _b: Bool = Bool(false);
};
