//! `Scheduler::run_parallel` with three trunks on two cores.
//!
//! Phase 0 holds three column-disjoint producers, so three trunks. On two cores
//! the round-robin trunk ownership wraps: core 0 owns within-phase ranks 0 and 2,
//! core 1 owns rank 1, so core 0 dispatches two trunks in one phase. This
//! exercises the `rank % ncores` wrap that a one-trunk-per-core fixture cannot.
//! A combiner in phase 1 reads all three. Per record i: `Av = 10i`, `Bv = 100i`,
//! `Cv = 1000i`, `Wv = 1110i`.

#[path = "run_parallel/arena.rs"]
mod arena;
mod common;
#[path = "run_parallel/producer_c.rs"]
mod producer_c;
#[path = "run_parallel/producers.rs"]
mod producers;

use arena::{HintT, Inv, stable_provider, store};
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

const N: usize = 4;

#[derive(Copy, Clone, Default)]
struct Wv(u32);

type ColW = Cons<Column<Wv>, Empty>;
type ReadABC = Cons<Column<Av>, Cons<Column<Bv>, Cons<Column<Cv>, Empty>>>;

// Combiner3: reads Av + Bv + Cv, records their sum, so it runs in phase 1 after the waist.
struct Combiner3;
impl BuilderInput for Combiner3 {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Combiner3 {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadABC,
        ColW,
        SnapNil,
        ColPtrCons<Av, ColPtrCons<Bv, ColPtrCons<Cv, ColPtrNil>>>,
        ColPtrCons<Wv, ColPtrNil>,
    >;
    type Hint = HintT;
    type Read = ReadABC;
    type Write = ColW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: all three producers ran in phase 0 (RAW edges on Av, Bv, Cv);
            // Wv reserved and exclusive here.
            let a: Av = unsafe { ctx.reader().read::<Av, _>(i) };
            let b: Bv = unsafe { ctx.reader().read::<Bv, _>(i) };
            let c: Cv = unsafe { ctx.reader().read::<Cv, _>(i) };
            unsafe { ctx.writer().write::<Wv, _>(i, Wv(a.0 + b.0 + c.0)) };
        });
    }
}

#[test]
fn run_parallel_three_trunks_two_cores_matches_single_core() {
    let provider = stable_provider(32768, 0);
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::start())
        .with(Column::<Av>::start())
        .with(Column::<Bv>::start())
        .with(Column::<Cv>::start())
        .with(Column::<Wv>::start_from(Wv(u32::MAX)))
        .with(ProducerA)
        .with(ProducerB)
        .with(ProducerC)
        .with(Combiner3)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Wv (head column) starts at u32::MAX. Host-populate In[i] = i. Columns from head:
    // Wv(0), Cv(1), Bv(2), Av(3), In(4).
    // SAFETY: In reserved for N records of u32; the scheduler is alive.
    let in_base = scheduler
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr();
    for i in 0 .. N {
        unsafe {
            *in_base.add(i) = Inv(i as u32);
        }
    }

    // Two cores, so phase 0's three trunks distribute round-robin: core 0 gets
    // ranks 0 and 2, core 1 gets rank 1. A dropped trunk (a rank-wrap bug where
    // core 0 fails to run its second trunk) corrupts the sum.
    let pool = TestExecutor::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let result = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(result, Outcome::Ok(())));

    // Wv = Av(10i) + Bv(100i) + Cv(1000i) = 1110i, correct only if all three
    // phase-0 trunks were dispatched (including both trunks core 0 owns) and the
    // phase-1 combiner ran after the waist.
    let wv_base = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const Wv;
    for i in 0 .. N {
        // SAFETY: Wv holds N reserved records; the scheduler is alive.
        let w = unsafe { (*wv_base.add(i)).0 };
        assert_eq!(
            w,
            (i as u32) * 1110,
            "rec {i}: all three phase-0 trunks dispatched (core 0 ran two) and the \
             phase-1 combiner ran after the waist"
        );
    }
}
