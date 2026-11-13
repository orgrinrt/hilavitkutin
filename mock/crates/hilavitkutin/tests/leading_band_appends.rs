//! The leading meta bands' appends survive `run_parallel`, and count.
//!
//! An `OnMeta<PassStart>` unit appends `K` records on the main thread before the
//! workers start; an `Always` consumer appends one marker per dispatch over its
//! record slice, once per participating core. Each core's region starts after
//! the records the leading band accepted, so the merge keeps them in front of
//! the consumer markers: `[7; K]` and then one `9` per participating core. With
//! room for them all the run returns `Ok`. When the leading band fills the
//! reservation, every core's region has no room, each marker is refused, and
//! the run returns the refusal with every append counted: `wanted` is `K` plus
//! the markers, `had` the reservation.
//!
//! Runs on executors reporting 1, 2, 4 and `MAX_CORES` workers, over two frames
//! each, so the per-frame reset is held too. Before this round the merge
//! compacted from position zero over the leading band's records, so they left
//! both counts and the run returned `Ok` with them gone.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod stable_provider;

mod common;

use arvo::USize;
use common::FixedWorkers;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons,
    AccPtrNil,
    ColPtrNil,
    EngineCtx,
    MetaRef,
    SnapNil,
    VirtNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin::thread::MAX_CORES;
use hilavitkutin_api::AccumRefused;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, HasAccumWriter};
use hilavitkutin_api::platform::{MemoryProviderApi, ThreadPoolApi};
use hilavitkutin_api::run_cfg::PassStart;
use hilavitkutin_api::store::Accum;
use hilavitkutin_api::work_unit::{Always, HasSchedule, OnMeta, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use notko::lend::Exhausted;
use stable_provider::StableProvider;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

/// Records per frame, and the accumulator's reserved capacity. Large enough
/// that the last core's region keeps room for its marker after the leading
/// band's records at every worker count, `MAX_CORES` included.
const RECORDS: usize = 1024;
/// Records the leading band appends when they fit.
const LEADING: usize = 2;

#[derive(Copy, Clone)]
struct Mark(u32);

type AccW = Cons<Accum<Mark>, Empty>;

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

type ConsumerCtx<'frame> = EngineCtx<
    'frame,
    Empty,
    AccW,
    SnapNil,
    ColPtrNil,
    ColPtrNil,
    AccPtrCons<'frame, Mark, AccPtrNil>,
>;

type MetaCtx<'frame> = EngineCtx<
    'frame,
    Empty,
    AccW,
    SnapNil,
    ColPtrNil,
    ColPtrNil,
    AccPtrCons<'frame, Mark, AccPtrNil>,
    VirtNil,
    MetaRef<'frame>,
>;

/// Leading band: appends `K` sevens on the main thread before the publish.
struct LeadWu<const K: usize>;
impl<const K: usize> BuilderInput for LeadWu<K> {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl<const K: usize> HasSchedule for LeadWu<K> {
    type Sched = OnMeta<PassStart>;
}
impl<const K: usize> WorkUnit<OnMeta<PassStart>> for LeadWu<K> {
    type Ctx<'frame> = MetaCtx<'frame>;
    type Hint = Hints;
    type Read = Empty;
    type Write = AccW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        for _ in 0 .. K {
            // SAFETY: the leading band runs once per frame on the main thread,
            // the one appender at that point; a refused append writes nothing.
            unsafe { ctx.accums().append::<Mark, _>(Mark(7)) };
        }
    }
}

/// Consumer band: one nine per dispatch over its record slice.
struct ConsumerWu;
impl BuilderInput for ConsumerWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for ConsumerWu {
    type Ctx<'frame> = ConsumerCtx<'frame>;
    type Hint = Hints;
    type Read = Empty;
    type Write = AccW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // SAFETY: each core appends into its own region; past it the append is
        // refused and writes nothing.
        unsafe { ctx.accums().append::<Mark, _>(Mark(9)) };
    }
}

/// Participating cores under the ceil record split (mirrors the worker math).
fn participating(workers: usize) -> usize {
    let per = RECORDS.div_ceil(workers);
    (0 .. workers).filter(|c| c * per < RECORDS).count()
}

/// Run two frames with a leading band of `K` records on `W` workers and check
/// the merged buffer and the output against `expect`.
fn check<const K: usize, const W: usize>(expect: Outcome<(), AccumRefused>) {
    let scheduler = Scheduler::builder()
        .with(Accum::<Mark>::new())
        .with(LeadWu::<K>)
        .with(ConsumerWu)
        .build(store(StableProvider::new(32768)), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let pool = FixedWorkers::<W>;
    assert_eq!(pool.worker_count().0, W);
    let markers = participating(W);
    let kept = (K + markers).min(RECORDS);
    let leading_kept = K.min(RECORDS);
    let mut scheduler = core::pin::pin!(scheduler);
    for frame in 0 .. 2 {
        let out = scheduler.as_mut().run_parallel(&pool);
        assert!(
            out == expect,
            "workers {W}, frame {frame}: the output is the expected one",
        );
        let bindings = scheduler.as_ref().get_ref().__bindings();
        assert_eq!(
            bindings.__len_cell().get().0,
            kept,
            "workers {W}, frame {frame}: the leading records and the markers that fit are live",
        );
        let base = bindings.__ptr().as_ptr();
        for k in 0 .. kept {
            // SAFETY: `kept` records are live in the merged prefix.
            let m = unsafe { core::ptr::read(base.add(k)).0 };
            let want = if k < leading_kept { 7 } else { 9 };
            assert_eq!(m, want, "workers {W}, frame {frame}, slot {k}");
        }
    }
}

fn fits<const W: usize>() {
    check::<LEADING, W>(Outcome::Ok(()));
}

fn fills<const W: usize>() {
    let wanted = RECORDS + participating(W);
    check::<RECORDS, W>(Outcome::Err(AccumRefused::new(USize(0), Exhausted {
        wanted,
        had: RECORDS,
    })));
}

#[test]
fn one_worker_keeps_the_leading_band_records() {
    fits::<1>();
}

#[test]
fn two_workers_keep_the_leading_band_records() {
    fits::<2>();
}

#[test]
fn four_workers_keep_the_leading_band_records() {
    fits::<4>();
}

#[test]
fn max_cores_workers_keep_the_leading_band_records() {
    fits::<MAX_CORES>();
}

#[test]
fn one_worker_refuses_past_a_leading_band_that_filled_the_reservation() {
    fills::<1>();
}

#[test]
fn two_workers_refuse_past_a_leading_band_that_filled_the_reservation() {
    fills::<2>();
}

#[test]
fn four_workers_refuse_past_a_leading_band_that_filled_the_reservation() {
    fills::<4>();
}

#[test]
fn max_cores_workers_refuse_past_a_leading_band_that_filled_the_reservation() {
    fills::<MAX_CORES>();
}
