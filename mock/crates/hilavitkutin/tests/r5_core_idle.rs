//! E8 adapt, core-idle axis: per-core barrier-park idle through the waist
//! barrier into `SchedulerMetrics::idle_ns`.
//!
//! The waist barrier times its own follower park (a worker that finishes a
//! phase early and waits at the waist for slower cores). `run_parallel` reduces
//! the per-core `idle_accumulator` to the worst-core idle at `ScheduleEnd` and
//! resets it. A consumer reads `idle_ns` through an `OnMeta<ScheduleEnd>` hook;
//! these tests read it through the white-box `__idle_ns` accessor because an
//! accumulator readback would force the unit-outer no-barrier path (zero idle).
//!
//! Edge-case catalogue (the workspace TDD discipline: every edge case is a
//! test), one file each:
//!
//! - single-core `run`, here: no waist barrier crossed, idle stays zero
//!   (invariant).
//! - accumulator unit-outer parallel, `r5_core_idle_accum.rs`: the carrier
//!   bypasses the barrier, idle stays zero (invariant).
//! - imbalanced multi-core column pipeline, `r5_core_idle_parallel.rs`: at
//!   least one follower parks, idle is positive (the measurement case). A
//!   strictly increasing counter clock makes any park record a positive delta,
//!   so the assertion is deterministic, not timing-dependent; guarded for
//!   single-core hosts where no follower parks.
//!
//! The clock, the store and the input column they share live under
//! `tests/r5_core_idle/`.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

#[path = "r5_core_idle/clock.rs"]
mod clock;
#[path = "r5_core_idle/input.rs"]
mod input;
mod stable_provider;

use arvo::USize;
use clock::{CounterClock, HintT, store};
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
use input::{Inv, OneIn};
use notko::Outcome;
use stable_provider::stable_provider;

const N: usize = 4;

#[derive(Copy, Clone, Default)]
struct Av(u32);
#[derive(Copy, Clone, Default)]
struct Bv(u32);
#[derive(Copy, Clone, Default)]
struct Zv(u32);

type ColA = Cons<Column<Av>, Empty>;
type ColB = Cons<Column<Bv>, Empty>;
type ColZ = Cons<Column<Zv>, Empty>;
type ReadAB = Cons<Column<Av>, Cons<Column<Bv>, Empty>>;

// ProducerA: In -> Av (phase 0, one trunk). ProducerB: In -> Bv (phase 0, a
// disjoint trunk). Combiner: Av + Bv -> Zv (phase 1, after the waist). The two
// phase-0 trunks plus the phase-1 combiner give exactly one interior waist, the
// park point the idle axis measures.
struct ProducerA;
impl BuilderInput for ProducerA {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ProducerA {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColA,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Av, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColA;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for N records; Av reserved + exclusive; windowed.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(inp.0 * 10)) };
        });
    }
}

struct ProducerB;
impl BuilderInput for ProducerB {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ProducerB {
    type Ctx<'frame> = EngineCtx<
        'frame,
        OneIn,
        ColB,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Bv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = OneIn;
    type Write = ColB;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: as ProducerA, for Bv.
            let inp = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(inp.0 * 100)) };
        });
    }
}

struct Combiner;
impl BuilderInput for Combiner {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Combiner {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadAB,
        ColZ,
        SnapNil,
        ColPtrCons<Av, ColPtrCons<Bv, ColPtrNil>>,
        ColPtrCons<Zv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = ReadAB;
    type Write = ColZ;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: both producers ran in phase 0 (RAW edges on Av/Bv); Zv
            // reserved + exclusive here.
            let a: Av = unsafe { ctx.reader().read::<Av, _>(i) };
            let b: Bv = unsafe { ctx.reader().read::<Bv, _>(i) };
            unsafe { ctx.writer().write::<Zv, _>(i, Zv(a.0 + b.0)) };
        });
    }
}

/// Macro: build the two-phase fan-in pipeline and host-populate `In[i] = i`.
/// A macro rather than a fn so the concrete `Scheduler` type stays inferred
/// (returning it from a fn would need to spell every opaque generic param).
macro_rules! fan_in {
    ($provider:expr) => {{
        let scheduler = Scheduler::builder()
            .clock(CounterClock::new())
            .with(Column::<Inv>::start())
            .with(Column::<Av>::start())
            .with(Column::<Bv>::start())
            .with(Column::<Zv>::start())
            .with(ProducerA)
            .with(ProducerB)
            .with(Combiner)
            .build(store($provider), USize(N))
            .unwrap_or_else(|_| panic!("build should succeed"));
        // Columns from head: Zv(0), Bv(1), Av(2), In(3). Populate In = i.
        // SAFETY: In reserved for N records of Inv; the scheduler is alive.
        let in_base = scheduler
            .__bindings()
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

// --- Invariant: single-core `run` crosses no waist barrier, so idle is zero. ---
#[test]
fn single_core_run_no_idle() {
    let provider = stable_provider(16384, 0);
    let mut scheduler = fan_in!(provider);
    let r = scheduler.run();
    assert!(matches!(r, Outcome::Ok(())));
    assert_eq!(
        scheduler.__idle_ns().to_raw(),
        0,
        "single-core run takes no waist-barrier park, so it records no idle"
    );
    // The frame itself ran: Zv is the head column, Combiner's Av + Bv at every
    // record (In = i, so Av = 10 i and Bv = 100 i).
    let zv = scheduler.__bindings().__ptr().as_ptr() as *const Zv;
    for i in 0 .. N {
        // SAFETY: Zv holds N reserved records; the scheduler is alive.
        let z = unsafe { (*zv.add(i)).0 };
        assert_eq!(
            z,
            (i as u32) * 110,
            "rec {i}: the fan-in combined both producers"
        );
    }
}
