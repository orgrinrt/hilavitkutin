//! An over-capacity accumulator append is a value at every worker count.
//!
//! `SkewWu` appends one value per record and three more on record 0, so a frame
//! asks `N + 3` appends of an accumulator reserved for `N`. `run` and
//! `run_parallel` on executors reporting 1, 2, 4 and `MAX_CORES` workers must
//! all return the same `AccumRefused`: registration position 1 (the guard
//! accumulator is registered first), `wanted` `N + 3`, `had` `N`. The accepted
//! records are, per core in ascending order, the first appends that core's
//! slice asked for, and nothing is written past them: `GuardWu`'s accumulator
//! is laid out directly after the refusing one in the arena and holds exactly
//! its own records. Every check runs over two frames, so the per-frame reset of
//! both counts is held too.
//!
//! Built with `DefaultRunCfg<Assert>`, the same frame's run returns `()` and
//! panics at the close with that refusal, at one worker and at `MAX_CORES`, and
//! a frame that refuses nothing returns `()` without panicking.
//!
//! The truncating mutant drops a refused append without counting it; the
//! refusal never reaches the output and the value tests here fail. It builds
//! only with the check's variable set: `HILAVITKUTIN_MUTANT_CHECK=1 cargo test
//! -p hilavitkutin --features mutant-truncating-append --test
//! overflow_is_a_value`.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! source lints.

mod common;
mod stable_provider;

use arvo::USize;
use common::FixedWorkers;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons,
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin::thread::MAX_CORES;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi,
    ColumnReaderApi,
    EachApi,
    HasAccumWriter,
    HasColumnReader,
    HasEach,
};
use hilavitkutin_api::platform::{MemoryProviderApi, ThreadPoolApi};
use hilavitkutin_api::run_cfg::DefaultRunCfg;
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{AccumRefused, Assert};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use notko::lend::Exhausted;
use stable_provider::StableProvider;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

/// Records per frame, and the reserved capacity of both accumulators.
const N: usize = 512;
/// Appends `SkewWu` makes on record 0 beyond the one per record.
const EXTRA: usize = 3;

#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);

/// The input column, seeded with each record's global index: the `each` index
/// is morsel-local, so a unit keyed on it would ask a different total per split.
#[derive(Copy, Clone)]
struct Inv(u32);

type ColIn = Cons<Column<Inv>, Empty>;
type AvW = Cons<Accum<Av>, Empty>;
type BvW = Cons<Accum<Bv>, Empty>;

/// One append per record, `EXTRA` more on record 0: `[0, 1, 2, 3]` for record 0,
/// `[10 * i]` for record `i`.
struct SkewWu;
impl BuilderInput for SkewWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for SkewWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColIn,
        AvW,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Av, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ColIn;
    type Write = AvW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let rec = unsafe { ctx.reader().read::<Inv, _>(i) }.0 as usize;
            for v in asked_by(rec).iter().flatten() {
                ctx.accums().append::<Av, _>(Av(*v));
            }
        });
    }
}

/// The appends record `i` asks for, as `SkewWu` makes them.
fn asked_by(i: usize) -> [Option<u32>; EXTRA + 1] {
    if i == 0 {
        [Some(0), Some(1), Some(2), Some(3)]
    } else {
        [Some(i as u32 * 10), None, None, None]
    }
}

/// One append per record, `1000 + i`: fits its reservation exactly.
struct GuardWu;
impl BuilderInput for GuardWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for GuardWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColIn,
        BvW,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Bv, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ColIn;
    type Write = BvW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| unsafe {
            let rec = ctx.reader().read::<Inv, _>(i).0;
            ctx.accums().append::<Bv, _>(Bv(1000 + rec));
        });
    }
}

/// `Outcome` carries no `Debug`; spell a run's output for a failure message.
fn describe(o: &Outcome<(), AccumRefused>) -> String {
    match o {
        Outcome::Ok(()) => "Ok(())".to_string(),
        Outcome::Err(r) => format!("Err({r:?})"),
    }
}

/// The refusal every worker count must return.
fn expected_refusal() -> Outcome<(), AccumRefused> {
    Outcome::Err(AccumRefused::new(USize(1), Exhausted {
        wanted: N + EXTRA,
        had:    N,
    }))
}

/// The merged `Av` buffer at `workers` cores: each core's slice of `per` records
/// keeps the first `region` appends it asked for, in ascending core order.
fn expected_av(workers: usize) -> [u32; N] {
    let per = N.div_ceil(workers);
    let mut out = [0u32; N];
    let mut w = 0;
    let mut lo = 0;
    while lo < N {
        let hi = (lo + per).min(N);
        let mut kept = 0;
        'slice: for i in lo .. hi {
            for v in asked_by(i).iter().flatten() {
                if kept == hi - lo {
                    break 'slice;
                }
                out[w] = *v;
                w += 1;
                kept += 1;
            }
        }
        lo = hi;
    }
    assert_eq!(w, N, "every slice fills its region");
    out
}

/// Build the stores `Inv`, `Bv`, `Av` in that order, then the given units, and
/// seed `Inv` with each record's index. The store list is prepended, so the
/// bindings head is `Av`, its tail `Bv`, then `Inv`; the arena reserves in that
/// order, which puts `Bv` directly after `Av`.
macro_rules! build {
    (@finish [$($finish:tt)+] $($unit:expr),+) => {{
        let scheduler = Scheduler::builder()
            .with(Column::<Inv>::new())
            .with(Accum::<Bv>::new())
            .with(Accum::<Av>::new())
            $(.with($unit))+
            .$($finish)+(store(StableProvider::new(16384)), USize(N))
            .unwrap_or_else(|_| panic!("build should succeed"));
        let in_base = scheduler.__bindings().__tail().__tail().__ptr().as_ptr() as *mut u32;
        for i in 0 .. N {
            // SAFETY: `Inv` reserves N u32-shaped records; written once here.
            unsafe { *in_base.add(i) = i as u32 };
        }
        scheduler
    }};
    (assert: $($unit:expr),+) => {
        build!(@finish [build_with::<DefaultRunCfg<Assert>, _, _>] $($unit),+)
    };
    ($($unit:expr),+) => {
        build!(@finish [build] $($unit),+)
    };
}

/// Check one closed frame: the output, both live lengths, both buffers, and that
/// the guard sits directly after the refusing accumulator.
macro_rules! check_frame {
    ($result:expr, $bindings:expr, $workers:expr, $label:expr) => {{
        let result = $result;
        let av = $bindings;
        let bv = av.__tail();
        assert!(
            result == expected_refusal(),
            "{}: the run returns the refusal, got {}",
            $label,
            describe(&result)
        );
        assert_eq!(
            av.__len_cell().get(),
            USize(N),
            "{}: Av keeps its reservation",
            $label
        );
        assert_eq!(
            bv.__len_cell().get(),
            USize(N),
            "{}: the guard keeps its own records",
            $label
        );
        let av_base = av.__ptr().as_ptr() as *const u32;
        let bv_base = bv.__ptr().as_ptr() as *const u32;
        assert_eq!(
            bv_base,
            av_base.wrapping_add(N),
            "{}: the guard is the next region in the arena, so a write past Av lands in it",
            $label
        );
        // SAFETY: both accumulators reserved N u32-shaped records and the frame
        // left N live in each; the scheduler is alive and every worker parked.
        let got_av: [u32; N] = core::array::from_fn(|k| unsafe { *av_base.add(k) });
        let got_bv: [u32; N] = core::array::from_fn(|k| unsafe { *bv_base.add(k) });
        assert_eq!(
            got_av,
            expected_av($workers),
            "{}: Av holds each core's first appends",
            $label
        );
        let want_bv: [u32; N] = core::array::from_fn(|k| 1000 + k as u32);
        assert_eq!(
            got_bv, want_bv,
            "{}: nothing refused was written into the guard",
            $label
        );
    }};
}

fn check_parallel<P: ThreadPoolApi>(pool: &P, workers: usize) {
    let scheduler = build!(SkewWu, GuardWu);
    let mut scheduler = core::pin::pin!(scheduler);
    for frame in 0 .. 2 {
        let result = scheduler.as_mut().run_parallel(pool);
        check_frame!(
            result,
            scheduler.as_ref().get_ref().__bindings(),
            workers,
            format_args!("{workers} workers, frame {frame}")
        );
    }
}

#[test]
fn run_returns_the_refusal() {
    let mut scheduler = build!(SkewWu, GuardWu);
    for frame in 0 .. 2 {
        let result = scheduler.run();
        check_frame!(
            result,
            scheduler.__bindings(),
            1,
            format_args!("run, frame {frame}")
        );
    }
}

#[test]
fn one_worker_returns_the_same_refusal() {
    check_parallel(&FixedWorkers::<1>, 1);
}

#[test]
fn two_workers_return_the_same_refusal() {
    check_parallel(&FixedWorkers::<2>, 2);
}

#[test]
fn four_workers_return_the_same_refusal() {
    check_parallel(&FixedWorkers::<4>, 4);
}

#[test]
fn max_cores_workers_return_the_same_refusal() {
    check_parallel(&FixedWorkers::<MAX_CORES>, MAX_CORES);
}

/// Two appends on record 0, none on record `N - 1`: `N` appends in all, which
/// fits the reservation, but more than one on core 0's slice.
struct FitsWu;
impl BuilderInput for FitsWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for FitsWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColIn,
        AvW,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Av, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ColIn;
    type Write = AvW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let rec = unsafe { ctx.reader().read::<Inv, _>(i) }.0;
            let n = if rec == 0 {
                2
            } else if rec as usize == N - 1 {
                0
            } else {
                1
            };
            for _ in 0 .. n {
                ctx.accums().append::<Av, _>(Av(rec));
            }
        });
    }
}

/// The intended behaviour the per-core region cap does not reach yet: a frame
/// that fits the reservation is accepted at every worker count, with the
/// buffer `run` produces.
#[test]
#[ignore = "catalogue: run_parallel caps a core's accumulator region at its record slice, so a multi-append skew that fits the reservation is refused at more than one worker; tracked #run_parallel_refuses_a_multi_append_skew_that_fits_the_reser"]
fn a_skew_that_fits_the_reservation_is_accepted_at_every_worker_count() {
    let want: [u32; N] = core::array::from_fn(|k| if k == 0 { 0 } else { k as u32 - 1 });

    let mut single = build!(FitsWu);
    assert!(
        single.run() == Outcome::Ok(()),
        "run accepts a frame that fits"
    );

    fn parallel<P: ThreadPoolApi>(pool: &P, want: &[u32; N], workers: usize) {
        let scheduler = build!(FitsWu);
        let mut scheduler = core::pin::pin!(scheduler);
        let result = scheduler.as_mut().run_parallel(pool);
        assert!(
            result == Outcome::Ok(()),
            "{workers} workers accept a frame that fits, got {}",
            describe(&result)
        );
        let av = scheduler.as_ref().get_ref().__bindings();
        assert_eq!(
            av.__len_cell().get(),
            USize(N),
            "{workers} workers keep every append"
        );
        let base = av.__ptr().as_ptr() as *const u32;
        // SAFETY: N records reserved and, on success, N live.
        let got: [u32; N] = core::array::from_fn(|k| unsafe { *base.add(k) });
        assert_eq!(&got, want, "{workers} workers: the same buffer as run");
    }
    parallel(&FixedWorkers::<1>, &want, 1);
    parallel(&FixedWorkers::<2>, &want, 2);
    parallel(&FixedWorkers::<4>, &want, 4);
    parallel(&FixedWorkers::<MAX_CORES>, &want, MAX_CORES);
}

/// The assert configuration's run output is `()`, and its run panics, at the
/// close and on the calling thread, when an accumulator refused an append.
fn assert_parallel<P: ThreadPoolApi>(pool: &P) {
    let scheduler = build!(assert: SkewWu, GuardWu);
    let mut scheduler = core::pin::pin!(scheduler);
    let () = scheduler.as_mut().run_parallel(pool);
}

#[test]
#[should_panic(
    expected = "under the assert configuration: AccumRefused { accum: USize(1), exhausted: Exhausted { wanted: 515, had: 512 } }"
)]
fn the_assert_configuration_panics_at_one_worker() {
    assert_parallel(&FixedWorkers::<1>);
}

#[test]
#[should_panic(
    expected = "under the assert configuration: AccumRefused { accum: USize(1), exhausted: Exhausted { wanted: 515, had: 512 } }"
)]
fn the_assert_configuration_panics_at_max_cores_workers() {
    assert_parallel(&FixedWorkers::<MAX_CORES>);
}

#[test]
fn the_assert_configuration_returns_unit_when_nothing_refused() {
    let mut single = build!(assert: GuardWu);
    let () = single.run();
    fn parallel<P: ThreadPoolApi>(pool: &P) {
        let scheduler = build!(assert: GuardWu);
        let mut scheduler = core::pin::pin!(scheduler);
        let () = scheduler.as_mut().run_parallel(pool);
    }
    parallel(&FixedWorkers::<1>);
    parallel(&FixedWorkers::<MAX_CORES>);
}
