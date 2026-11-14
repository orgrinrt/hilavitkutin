//! E8 adapt, core-idle axis, accumulator carrier: a carrier with an
//! accumulator runs unit-outer, each core walking its own slice of the records
//! with no waist barrier between them, so `SchedulerMetrics::idle_ns` stays
//! zero.
//!
//! The same frame is also held to what it appends. A core appends once, over
//! the slice it walks, and a core handed no records walks nothing, so the live
//! count after the merge is the number of cores with a non-empty slice. The
//! tests pin the worker count with `FixedWorkers`, so that number is the same
//! on every host.
//!
//! The single-core and imbalanced-pipeline cases are in `r5_core_idle.rs` and
//! `r5_core_idle_parallel.rs`; the clock and the store they share live under
//! `tests/r5_core_idle/`.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

#[path = "r5_core_idle/clock.rs"]
mod clock;
mod common;
mod fixed_workers;
mod stable_provider;

use arvo::USize;
use clock::{CounterClock, HintT, store};
use fixed_workers::FixedWorkers;
use hilavitkutin::dispatch::engine_ctx::{AccPtrCons, AccPtrNil, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, HasAccumWriter};
use hilavitkutin_api::platform::ThreadPoolApi;
use hilavitkutin_api::store::Accum;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use notko::Outcome;
use stable_provider::StableProvider;

// Accumulator capacity headroom over the per-frame appends (an append past
// the reserved capacity, which equals the record count at build, is refused).
const RECORDS: usize = 64;

#[derive(Copy, Clone)]
struct Mark(u64);
type AccW = Cons<Accum<Mark>, Empty>;

struct AccumWu;
impl BuilderInput for AccumWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for AccumWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        AccW,
        SnapNil,
        ColPtrNil,
        ColPtrNil,
        AccPtrCons<'frame, Mark, AccPtrNil>,
    >;
    type Hint = HintT;
    type Read = Empty;
    type Write = AccW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        // Mark reserved (RECORDS) with headroom over this frame's appends.
        ctx.accums().append::<Mark, _>(Mark(7));
    }
}

/// Run one frame of the accumulator carrier on `pool` and hold it to zero idle
/// and to one append per core that walks a slice.
///
/// `walking` is the number of cores that walk a slice, worked out by hand at
/// the call site rather than by the engine's slicing, so a wrong slice formula
/// shows as a wrong count. Each walking core appends the one value the unit
/// makes, and every append fits, so the merged live count is that number.
fn accum_frame<P: ThreadPoolApi>(pool: &P, walking: usize) {
    let provider = StableProvider::new(16384, 0);
    let scheduler = Scheduler::builder()
        .clock(CounterClock::new())
        .with(Accum::<Mark>::new())
        .with(AccumWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let mut scheduler = core::pin::pin!(scheduler);
    let r = scheduler.as_mut().run_parallel(pool);
    assert!(matches!(r, Outcome::Ok(())));
    assert_eq!(
        scheduler.__idle_ns().to_raw(),
        0,
        "an accumulator carrier runs unit-outer with no waist barrier, so it records no idle"
    );

    let workers = pool.worker_count().0;
    let view = scheduler.as_ref();
    let marks = view.__bindings();
    let live = marks.__len_cell().get().0;
    assert_eq!(
        live, walking,
        "{workers} workers over {RECORDS} records: {walking} cores walk a slice, one append each"
    );
    for k in 0 .. live {
        // SAFETY: the first `live` records were initialised by the appends.
        let mark = unsafe { (*marks.__ptr().as_ptr().add(k)).0 };
        assert_eq!(
            mark, 7,
            "rec {k}: the appended value is the one the unit made"
        );
    }
}

// --- Invariant: an accumulator carrier takes the unit-outer no-barrier path. ---
#[test]
fn accum_unit_outer_no_idle() {
    // Four workers over 64 records: four slices of 16, and every core walks one.
    accum_frame(&FixedWorkers::<4>, 4);
}

#[test]
fn accum_unit_outer_no_idle_with_surplus_cores() {
    // Forty workers over 64 records: slices of two, so cores 0 to 31 walk one
    // and the eight after them are handed an empty slice and append nothing.
    accum_frame(&FixedWorkers::<40>, 32);
}
