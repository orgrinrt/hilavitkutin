//! E8 adapt, core-idle axis, imbalanced pipeline: a width-2 fan-out / fan-in
//! (P1,P2 -> Mid -> Q1,Q2 -> Sink) on several cores parks at least one follower
//! at the interior waist, and the park lands in `SchedulerMetrics::idle_ns`.
//!
//! A width-1 chain or a shallow fan-in collapses to one phase (the grouping
//! orders the trunks within a single phase), but a level with two parallel
//! trunks feeding a dependent unit forces a real interior waist, which is the
//! park point the core-idle axis measures. This is the same shape the engine's
//! barrier-reuse test uses; it yields nphases > 1. A strictly increasing
//! counter clock makes any park record a positive delta, so the assertion is
//! deterministic, not timing-dependent; it is guarded for single-core hosts
//! where no follower parks.
//!
//! The single-core and accumulator cases are in `r5_core_idle.rs` and
//! `r5_core_idle_accum.rs`; the clock, the store and the input column they
//! share live under `tests/r5_core_idle/`.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

#[path = "r5_core_idle/clock.rs"]
mod clock;
mod common;
#[path = "r5_core_idle/input.rs"]
mod input;
mod stable_provider;

use arvo::USize;
use clock::{CounterClock, HintT, store};
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
use hilavitkutin_api::platform::ThreadPoolApi;
use hilavitkutin_api::store::{Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use input::{Inv, OneIn};
use notko::Outcome;
use stable_provider::StableProvider;

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
type ColMr = Cons<Column<Mv>, Empty>;
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
            // SAFETY: In host-populated; P1v reserved + exclusive; windowed.
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
        ColMr,
        SnapNil,
        ColPtrCons<P1v, ColPtrCons<P2v, ColPtrNil>>,
        ColPtrCons<Mv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = ReadP;
    type Write = ColMr;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: both producers ran in the prior phase (RAW on P1v/P2v); Mv reserved.
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
        ColMr,
        ColQ1,
        SnapNil,
        ColPtrCons<Mv, ColPtrNil>,
        ColPtrCons<Q1v, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = ColMr;
    type Write = ColQ1;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: Mid ran in the prior phase (RAW on Mv); Q1v reserved + exclusive.
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
        ColMr,
        ColQ2,
        SnapNil,
        ColPtrCons<Mv, ColPtrNil>,
        ColPtrCons<Q2v, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = ColMr;
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
            // SAFETY: Q1 and Q2 ran in the prior phase (RAW on Q1v/Q2v); Sv reserved.
            let a: Q1v = unsafe { ctx.reader().read::<Q1v, _>(i) };
            let b: Q2v = unsafe { ctx.reader().read::<Q2v, _>(i) };
            unsafe { ctx.writer().write::<Sv, _>(i, Sv(a.0 + b.0)) };
        });
    }
}

/// Macro: build the multi-phase fan-out/fan-in pipeline and populate In. Yields
/// nphases > 1, so the worker phase loop crosses an interior waist barrier.
macro_rules! two_waist {
    ($provider:expr) => {{
        let scheduler = Scheduler::builder()
            .clock(CounterClock::new())
            .with(Column::<Inv>::start())
            .with(Column::<P1v>::start())
            .with(Column::<P2v>::start())
            .with(Column::<Mv>::start())
            .with(Column::<Q1v>::start())
            .with(Column::<Q2v>::start())
            .with(Column::<Sv>::start())
            .with(P1)
            .with(P2)
            .with(Mid)
            .with(Q1)
            .with(Q2)
            .with(Sink)
            .build(store($provider), USize(N))
            .unwrap_or_else(|_| panic!("build should succeed"));
        // Columns from head: Sv(0), Q2v(1), Q1v(2), Mv(3), P2v(4), P1v(5), In(6).
        // SAFETY: In reserved for N records of Inv; the scheduler is alive.
        let in_base = scheduler
            .__bindings()
            .__tail()
            .__tail()
            .__tail()
            .__tail()
            .__tail()
            .__tail()
            .__ptr()
            .as_ptr();
        for i in 0 .. N {
            unsafe { *in_base.add(i) = Inv(i as u32) };
        }
        scheduler
    }};
}

// --- Measurement: an imbalanced multi-core column pipeline parks a follower. ---
#[test]
fn imbalanced_parallel_records_idle() {
    let provider = StableProvider::new(16384, 0);
    let scheduler = two_waist!(provider);

    let pool = TestExecutor::new();
    let ncores = pool.worker_count().0.max(1);
    let mut scheduler = core::pin::pin!(scheduler);
    let r = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(r, Outcome::Ok(())));

    let idle = scheduler.__idle_ns().to_raw();
    if ncores > 1 {
        // The phase-0 / phase-1 waist parks at least one follower; under the
        // strictly increasing counter clock that park records a positive delta.
        assert!(
            idle > 0,
            "a multi-core waist crossing parks a follower, which records nonzero idle (ncores={ncores})"
        );
    } else {
        // Single worker: the barrier takes the last-arriver path (expected == 1)
        // and never parks, so the single-core invariant applies.
        assert_eq!(
            idle, 0,
            "a single worker never parks at a waist, so it records no idle"
        );
    }

    // The frame ran across the interior waist whatever the worker count: Sv is
    // the head column, Q1v + Q2v at every record. In = i gives P1v = 2 i and
    // P2v = 3 i, Mv = 5 i, Q1v = 5 i and Q2v = 10 i, so Sv = 15 i.
    let sv = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const Sv;
    for i in 0 .. N {
        // SAFETY: Sv holds N reserved records; the scheduler is alive.
        let s = unsafe { (*sv.add(i)).0 };
        assert_eq!(s, (i as u32) * 15, "rec {i}: the sink saw both Q columns");
    }
}
