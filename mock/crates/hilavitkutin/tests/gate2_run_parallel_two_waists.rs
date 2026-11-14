//! `Scheduler::run_parallel` across two interior waists in one frame.
//!
//! The worker-side sense-reversing waist barrier is reused within a single
//! frame: two interior waists back to back, no frame sync between them, which
//! is exactly the reset race the sense flip fixes. Shape: P1 and P2 (phase 0),
//! a Mid fan-in (waist 1), a Q1 and Q2 fan-out, a Sink fan-in (waist 2). Per
//! record i: `P1 = 2i`, `P2 = 3i`, `Mid = 5i`, `Q1 = 5i`, `Q2 = 10i`, `Sink = 15i`.

#[path = "run_parallel/arena.rs"]
mod arena;
mod common;

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

const N: usize = 4;

#[derive(Copy, Clone, Default)]
struct P1v(u32);
#[derive(Copy, Clone, Default)]
struct P2v(u32);
#[derive(Copy, Clone, Default)]
struct Mv(u32);
#[derive(Copy, Clone, Default)]
struct Q1v(u32);
#[derive(Copy, Clone, Default)]
struct Q2v(u32);
#[derive(Copy, Clone, Default)]
struct Sv(u32);

type ColP1 = Cons<Column<P1v>, Empty>;
type ColP2 = Cons<Column<P2v>, Empty>;
type ColM = Cons<Column<Mv>, Empty>;
type ColQ1 = Cons<Column<Q1v>, Empty>;
type ColQ2 = Cons<Column<Q2v>, Empty>;
type ColS = Cons<Column<Sv>, Empty>;
type ReadP = Cons<Column<P1v>, Cons<Column<P2v>, Empty>>;
type ReadQ = Cons<Column<Q1v>, Cons<Column<Q2v>, Empty>>;

struct P1;
impl BuilderInput for P1 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for P1 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColP1,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<P1v, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColP1;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated; P1v reserved and exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<P1v, _>(i, P1v(inp.0 * 2)) };
        });
    }
}

struct P2;
impl BuilderInput for P2 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for P2 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColP2,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<P2v, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColP2;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: as P1, for P2v.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<P2v, _>(i, P2v(inp.0 * 3)) };
        });
    }
}

struct Mid;
impl BuilderInput for Mid {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Mid {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadP,
        ColM,
        SnapNil,
        ColPtrCons<P1v, ColPtrCons<P2v, ColPtrNil>>,
        ColPtrCons<Mv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = ReadP;
    type Write = ColM;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: both producers ran in the prior phase (RAW edges on P1v, P2v);
            // Mv reserved and exclusive here.
            let a: P1v = unsafe { ctx.reader().read::<P1v, _>(i) };
            let b: P2v = unsafe { ctx.reader().read::<P2v, _>(i) };
            unsafe { ctx.writer().write::<Mv, _>(i, Mv(a.0 + b.0)) };
        });
    }
}

struct Q1;
impl BuilderInput for Q1 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Q1 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColM,
        ColQ1,
        SnapNil,
        ColPtrCons<Mv, ColPtrNil>,
        ColPtrCons<Q1v, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = ColM;
    type Write = ColQ1;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Mid ran in the prior phase (RAW on Mv); Q1v reserved and exclusive.
            let m: Mv = unsafe { ctx.reader().read::<Mv, _>(i) };
            unsafe { ctx.writer().write::<Q1v, _>(i, Q1v(m.0)) };
        });
    }
}

struct Q2;
impl BuilderInput for Q2 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Q2 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColM,
        ColQ2,
        SnapNil,
        ColPtrCons<Mv, ColPtrNil>,
        ColPtrCons<Q2v, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = ColM;
    type Write = ColQ2;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: as Q1, doubling Mv into the disjoint Q2v column.
            let m: Mv = unsafe { ctx.reader().read::<Mv, _>(i) };
            unsafe { ctx.writer().write::<Q2v, _>(i, Q2v(m.0 * 2)) };
        });
    }
}

struct Sink;
impl BuilderInput for Sink {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Sink {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadQ,
        ColS,
        SnapNil,
        ColPtrCons<Q1v, ColPtrCons<Q2v, ColPtrNil>>,
        ColPtrCons<Sv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = ReadQ;
    type Write = ColS;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Q1 and Q2 ran in the prior phase (RAW on Q1v, Q2v); Sv
            // reserved and exclusive. A column write (windowed) is correct under
            // the head and tail convergence that splits this single-trunk phase.
            let a: Q1v = unsafe { ctx.reader().read::<Q1v, _>(i) };
            let b: Q2v = unsafe { ctx.reader().read::<Q2v, _>(i) };
            unsafe { ctx.writer().write::<Sv, _>(i, Sv(a.0 + b.0)) };
        });
    }
}

#[test]
fn run_parallel_threaded_two_waists_reuses_barrier() {
    let provider = BumpProvider::<32768>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::start())
        .with(Column::<P1v>::start())
        .with(Column::<P2v>::start())
        .with(Column::<Mv>::start())
        .with(Column::<Q1v>::start())
        .with(Column::<Q2v>::start())
        .with(Column::<Sv>::start_from(Sv(u32::MAX)))
        .with(P1)
        .with(P2)
        .with(Mid)
        .with(Q1)
        .with(Q2)
        .with(Sink)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Sv (head column) starts at u32::MAX. Host-populate In[i] = i. Columns from head:
    // Sv(0), Q2v(1), Q1v(2), Mv(3), P2v(4), P1v(5), In(6).
    // SAFETY: In reserved for N records of u32; the scheduler is alive.
    let in_base = scheduler
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *mut u32;
    for i in 0 .. N {
        unsafe {
            *in_base.add(i) = i as u32;
        }
    }

    let pool = TestExecutor::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let result = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(result, Outcome::Ok(())));

    // Sv = Q1(5i) + Q2(10i) = 15i, correct only if both interior waist barriers
    // ordered their phases and the barrier was reused correctly for the second
    // waist (the sense flip; a stale-count reset would deadlock or corrupt here).
    let sv_base = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const Sv;
    for i in 0 .. N {
        // SAFETY: Sv holds N reserved records; the scheduler is alive.
        let s = unsafe { (*sv_base.add(i)).0 };
        assert_eq!(
            s,
            (i as u32) * 15,
            "rec {i}: two-waist frame ran P1,P2 -> Mid -> Q1,Q2 -> Sink in phase order \
             with the worker-side barrier reused across both waists"
        );
    }
}
