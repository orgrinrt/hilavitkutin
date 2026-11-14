//! `Scheduler::run_parallel` over a fan-in: two column-disjoint producers in
//! phase 0 (one writes `Column<Av>`, one `Column<Bv>`, so they fall in distinct
//! trunks), and one combiner in phase 1 that reads both and records `Av + Bv`.
//! The two RAW edges (`Av` and `Bv`, both producer to combiner) put the combiner
//! after the waist, in its own phase.
//!
//! `run_parallel` dispatches the waist-bounded phases as per-core trunk
//! programs: phase 0 splits its two trunks across the cores (round-robin by
//! within-phase trunk rank), then the waist barrier, then phase 1's single
//! trunk. The recorded sum is correct (`Av(i*10) + Bv(i*100) = i*110`) only if
//! both phase-0 trunks were dispatched and the phase-1 unit ran after both. A
//! dropped trunk or a broken phase order corrupts the sum.
//!
//! The three-trunk wrap, the two-waist barrier reuse and the participation
//! guard each have their own fixture, in `gate2_run_parallel_three_trunks.rs`,
//! `gate2_run_parallel_two_waists.rs` and `gate2_run_parallel_participation.rs`.
//! The arena and the producers they share live under `tests/run_parallel/`.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::sync::atomic::{AtomicUsize, Ordering};

#[path = "run_parallel/arena.rs"]
mod arena;
mod common;
#[path = "run_parallel/producers.rs"]
mod producers;

use arena::{BumpProvider, HintT, Inv, store};
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
use hilavitkutin_api::platform::ThreadPoolApi;
use hilavitkutin_api::store::{Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use notko::Outcome;
use producers::{Av, Bv, ProducerA, ProducerB};

const N: usize = 4;

#[derive(Copy, Clone, Default)]
struct Zv(u32);

type ColZ = Cons<Column<Zv>, Empty>;
type ReadAB = Cons<Column<Av>, Cons<Column<Bv>, Empty>>;

// Combiner: reads Av + Bv, records their sum, so it runs in phase 1 after the waist.
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
            // SAFETY: both producers (ordered before this unit by the plan's RAW
            // edges on Av and Bv, and run in an earlier phase) wrote every
            // record the morsel covers; Zv reserved and exclusive here. Writing a
            // column (windowed) rather than a static-by-index is what makes the
            // combiner correct under head and tail convergence, which splits this
            // single-trunk phase across cores with non-zero morsel starts.
            let a: Av = unsafe { ctx.reader().read::<Av, _>(i) };
            let b: Bv = unsafe { ctx.reader().read::<Bv, _>(i) };
            unsafe { ctx.writer().write::<Zv, _>(i, Zv(a.0 + b.0)) };
        });
    }
}

/// A real `ThreadPoolApi` that counts `spawn` calls, delegating to the test
/// executor so workers actually run. Proves spawn-once across frames.
struct CountingPool {
    inner:  TestExecutor,
    spawns: AtomicUsize,
}

impl ThreadPoolApi for CountingPool {
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.spawns.fetch_add(1, Ordering::Relaxed);
        self.inner.spawn(f);
    }

    fn worker_count(&self) -> USize {
        self.inner.worker_count()
    }
}

#[test]
fn run_parallel_threaded_fan_in_is_correct() {
    let provider = BumpProvider::<16384>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::start())
        .with(Column::<Av>::start())
        .with(Column::<Bv>::start())
        .with(Column::<Zv>::start_from(Zv(u32::MAX)))
        .with(ProducerA)
        .with(ProducerB)
        .with(Combiner)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Zv (head column) starts at u32::MAX so an unwritten record is caught.
    // Host-populate In[i] = i (absolute). Columns from head: Zv(0), Bv(1), Av(2), In(3).
    // SAFETY: In reserved for N records of u32; the scheduler is alive.
    let in_base = scheduler
        .__bindings()
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

    // Av(i*10) + Bv(i*100) = i*110, written to Zv by the phase-1 combiner after
    // both phase-0 producer trunks finished (the waist barrier).
    let zv_base = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const Zv;
    for i in 0 .. N {
        // SAFETY: Zv holds N reserved records; the scheduler is alive.
        let z = unsafe { (*zv_base.add(i)).0 };
        assert_eq!(
            z,
            (i as u32) * 110,
            "rec {i}: threaded run_parallel dispatched both phase-0 trunks and ran \
             the phase-1 combiner after both"
        );
    }
}

#[test]
fn run_parallel_spawns_the_pool_once_across_frames() {
    let provider = BumpProvider::<16384>::new();
    let scheduler = Scheduler::builder()
        .with(Column::<Inv>::start())
        .with(Column::<Av>::start())
        .with(Column::<Bv>::start())
        .with(Column::<Zv>::start_from(Zv(u32::MAX)))
        .with(ProducerA)
        .with(ProducerB)
        .with(Combiner)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Host-populate In[i] = i. Columns from head: Zv(0), Bv(1), Av(2), In(3).
    // SAFETY: In reserved for N records of u32; the scheduler is alive.
    let in_base = scheduler
        .__bindings()
        .__tail()
        .__tail()
        .__tail()
        .__ptr()
        .as_ptr() as *mut u32;
    for i in 0 .. N {
        unsafe { *in_base.add(i) = i as u32 };
    }

    let pool = CountingPool {
        inner:  TestExecutor::new(),
        spawns: AtomicUsize::new(0),
    };
    let mut scheduler = core::pin::pin!(scheduler);
    // Two frames: spawn happens once (first call), workers park and are reused.
    let first = scheduler.as_mut().run_parallel(&pool);
    assert!(
        matches!(first, Outcome::Ok(())),
        "the first frame completes"
    );
    let second = scheduler.as_mut().run_parallel(&pool);
    assert!(
        matches!(second, Outcome::Ok(())),
        "the second frame completes on the parked workers"
    );

    assert_eq!(
        pool.spawns.load(Ordering::Relaxed),
        pool.worker_count().0,
        "the persistent pool spawns worker_count workers once, not per frame"
    );
    let zv_base = scheduler.as_ref().__bindings().__ptr().as_ptr() as *const Zv;
    for i in 0 .. N {
        // SAFETY: Zv holds N reserved records; the scheduler is alive.
        let z = unsafe { (*zv_base.add(i)).0 };
        assert_eq!(z, (i as u32) * 110, "rec {i} after two frames");
    }
}
