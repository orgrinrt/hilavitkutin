//! `Scheduler::run_parallel` with more trunks and cores than records.
//!
//! The worker entry computes a ceil record slice per core and returns early
//! when that slice is empty (`run_this` in the scheduler). Trunk ownership is by
//! rank, not by record slice, so on a machine with more cores than records a
//! core can own a trunk and still return before dispatching it. Five
//! column-disjoint producers against one record reaches a core in that range;
//! every other parallel fixture tops out at three trunks and cannot.

#[path = "run_parallel/arena.rs"]
mod arena;
mod common;
#[path = "run_parallel/producer_c.rs"]
mod producer_c;
#[path = "run_parallel/producers.rs"]
mod producers;

use arena::{BumpProvider, HintT, Inv, OneIn, store};
use arvo::USize;
use common::TestExecutor;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
};
use hilavitkutin_api::store::{Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use notko::Outcome;
use producer_c::{Cv, ProducerC};
use producers::{Av, Bv, ProducerA, ProducerB};

#[derive(Copy, Clone, Default)]
struct Dv(u32);
#[derive(Copy, Clone, Default)]
struct Ev(u32);

type ColD = Cons<Column<Dv>, Empty>;
type ColE = Cons<Column<Ev>, Empty>;

/// ProducerD: reads In, writes Dv = In*7. A fourth trunk in phase 0.
struct ProducerD;
impl BuilderInput for ProducerD {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ProducerD {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColD,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Dv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColD;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated; Dv reserved and exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Dv, _>(i, Dv(inp.0 * 7)) };
        });
    }
}

/// ProducerE: reads In, writes Ev = In*9. A fifth trunk in phase 0, and the one
/// whose rank lands on a core with an empty record slice.
struct ProducerE;
impl BuilderInput for ProducerE {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ProducerE {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColE,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Ev, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColE;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated; Ev reserved and exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Ev, _>(i, Ev(inp.0 * 9)) };
        });
    }
}

/// Every producer runs when there are more cores than records.
///
/// `run_parallel`'s worker entry gates participation on a per-core ceil record
/// slice, so with one record and eight cores only core 0 enters; the rest
/// return before dispatching. Trunk ownership is by rank, not by record slice,
/// which raises the question of whether a unit whose rank maps to a core that
/// opted out runs at all.
///
/// It does, and this pins that. The case was written expecting a failure: the
/// two ownership models disagree about which cores are idle, so a rank owner
/// with an empty slice looked like it would strand its work. Running it showed
/// otherwise. A single-trunk phase takes the whole-phase-mask branch, where each
/// participating core dispatches every unit in the phase rather than only the
/// trunks it owns, so no unit depends on a core that opted out.
///
/// Kept as a regression test rather than deleted. The property it pins is not
/// obvious from either ownership model alone, it holds today for a reason that
/// changes when the whole-phase-mask branch is removed, and it is exactly what
/// would break silently if participation and dispatch ownership were changed
/// independently.
#[test]
fn every_producer_runs_when_cores_outnumber_records() {
    let provider = BumpProvider::<32768>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::start())
        .with(Column::<Av>::start())
        .with(Column::<Bv>::start())
        .with(Column::<Cv>::start())
        .with(Column::<Dv>::start())
        .with(Column::<Ev>::start_from(Ev(u32::MAX)))
        .with(ProducerA)
        .with(ProducerB)
        .with(ProducerC)
        .with(ProducerD)
        .with(ProducerE)
        .build(store(provider), USize(1))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Columns from head: Ev(0), Dv(1), Cv(2), Bv(3), Av(4), In(5).
    // SAFETY: In reserved for the one record, as u32; the scheduler is alive.
    let in_base = scheduler
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *mut u32;
    unsafe {
        *in_base = 1;
    }

    let pool = TestExecutor::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let result = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(result, Outcome::Ok(())));

    // Every producer's column, walked from the head binding: Ev, Dv, Cv, Bv, Av.
    let sched_ref = scheduler.as_ref();
    let b = sched_ref.__bindings();
    // SAFETY: each column holds the reserved record; the scheduler is alive.
    let ev = unsafe { (*(b.__ptr().as_ptr() as *const Ev)).0 };
    let dv = unsafe { (*(b.__tail().__ptr().as_ptr() as *const Dv)).0 };
    let cv = unsafe { (*(b.__tail().__tail().__ptr().as_ptr() as *const Cv)).0 };
    let bv = unsafe { (*(b.__tail().__tail().__tail().__ptr().as_ptr() as *const Bv)).0 };
    let av = unsafe { (*(b.__tail().__tail().__tail().__tail().__ptr().as_ptr() as *const Av)).0 };
    // In[0] = 1, so each producer's multiplier is its expected value.
    assert_eq!(av, 10, "ProducerA did not run");
    assert_eq!(bv, 100, "ProducerB did not run");
    assert_eq!(cv, 1000, "ProducerC did not run");
    assert_eq!(dv, 7, "ProducerD did not run");
    assert_eq!(
        ev, 9,
        "ProducerE did not run: the start value left in place means no core dispatched it, \
         which is what happens if participation and dispatch ownership disagree"
    );
}
