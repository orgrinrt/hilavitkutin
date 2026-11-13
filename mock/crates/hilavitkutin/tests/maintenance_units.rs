//! Maintenance units: the setup and the close of a pass.
//!
//! A unit scheduled `OnMeta<V>` is dispatched once in a pass by one core over
//! the whole record range, on `run` and on both worker paths of
//! `run_parallel`. A close unit reads what the pass appended and applies it by
//! position, and what it changed seeds the readers of the next pass. A unit of
//! a leading band seeds its readers in the pass it runs in.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod common;
mod stable_provider;

use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::USize;
use common::FixedWorkers;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons,
    AccPtrNil,
    AccSnap,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    MetaRef,
    SnapCons,
    SnapNil,
    VirtNil,
};
use hilavitkutin::scheduler::{BuildError, Scheduler};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumReaderApi,
    ColumnWriterApi,
    EachApi,
    HasAccumReader,
    HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::meta::SchedulerMetrics;
use hilavitkutin_api::run_cfg::{PassStart, ScheduleEnd};
use hilavitkutin_api::store::{Accum, Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, HasSchedule, OnMeta, WorkUnit};
use hilavitkutin_api::{Differs, Written, rec};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::{Maybe, Outcome};
use stable_provider::StableProvider;

const RECORDS: usize = 600;
const SENTINEL: u32 = 0xDEAD;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Slot(u32);

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Height(u32);

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Seen(u32);

/// A queued change: the record to write and the value to write there.
#[derive(Copy, Clone)]
struct Change {
    at: u32,
    to: u32,
}

type Hints = (Immediate, Atomic, Normal);

fn store() -> ArenaColumnStorage<StableProvider> {
    store_of(1 << 16)
}

fn store_of(bytes: usize) -> ArenaColumnStorage<StableProvider> {
    ArenaColumnStorage::new(StableProvider::new(bytes))
}

fn counter() -> &'static AtomicUsize {
    Box::leak(Box::new(AtomicUsize::new(0)))
}

/// Fill `RECORDS` records at `base`.
///
/// # Safety
///
/// `base` is a column of `RECORDS` records and no pass is running.
unsafe fn fill<T: Copy>(base: *mut T, value: impl FnMut(usize) -> T) {
    // SAFETY: the caller's contract, at `RECORDS`.
    unsafe { fill_n(base, RECORDS, value) }
}

/// Fill `n` records at `base`.
///
/// # Safety
///
/// `base` is a column of `n` records and no pass is running.
unsafe fn fill_n<T: Copy>(base: *mut T, n: usize, mut value: impl FnMut(usize) -> T) {
    for i in 0 .. n {
        // SAFETY: inside the reservation, per the contract above.
        unsafe { base.add(i).write(value(i)) };
    }
}

/// Read `RECORDS` records at `base` through `raw`.
///
/// # Safety
///
/// `base` is a column of `RECORDS` initialised records and no pass is running.
unsafe fn all<T: Copy>(base: *const T, raw: impl Fn(T) -> u32) -> Vec<u32> {
    // SAFETY: the caller's contract, at `RECORDS`.
    unsafe { all_n(base, RECORDS, raw) }
}

/// Read `n` records at `base` through `raw`.
///
/// # Safety
///
/// `base` is a column of `n` initialised records and no pass is running.
unsafe fn all_n<T: Copy>(base: *const T, n: usize, raw: impl Fn(T) -> u32) -> Vec<u32> {
    // SAFETY: inside the reservation, per the contract above.
    (0 .. n)
        .map(|i| raw(unsafe { base.add(i).read() }))
        .collect()
}

/// What the column holds once the queued changes of a pass are applied: every
/// fifth slot `s` sends the value `3s` to the record `RECORDS - 1 - s`.
fn applied() -> Vec<u32> {
    let mut v = vec![0; RECORDS];
    for s in (0 .. RECORDS).step_by(5) {
        v[RECORDS - 1 - s] = 3 * s as u32;
    }
    v
}

type Slots = Cons<Column<Slot>, Empty>;
type Queue = Cons<Accum<Change>, Empty>;
type Heights = Cons<Column<Height>, Empty>;
type Sights = Cons<Column<Seen>, Empty>;

// Emit: an ordinary unit that queues a change for another record.
#[derive(Copy, Clone)]
struct Emit;
impl BuilderInput for Emit {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Emit {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Slots,
        Queue,
        SnapNil,
        ColPtrCons<Slot, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Change, AccPtrNil>,
    >;
    type Hint = Hints;
    type Read = Slots;
    type Write = Queue;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|read| {
            let s = read.get(Column::<Slot>::new()).0;
            if s % 5 == 0 {
                rec![Maybe::Is(Change {
                    at: RECORDS as u32 - 1 - s,
                    to: 3 * s,
                })]
            } else {
                rec![Maybe::Isnt]
            }
        });
    }
}

// Watch: an ordinary reader of the column the maintenance units write.
#[derive(Copy, Clone)]
struct Watch;
impl BuilderInput for Watch {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Watch {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Heights,
        Sights,
        SnapNil,
        ColPtrCons<Height, ColPtrNil>,
        ColPtrCons<Seen, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = Hints;
    type Read = Heights;
    type Write = Sights;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each()
            .map(|read| rec![Seen(read.get(Column::<Height>::new()).0)]);
    }
}

// The close unit, once for each member it may list the column under. It reads
// the pass's appends and writes each at the record the change names, which is
// a write at a record named from data.
macro_rules! close_unit {
    ($name:ident, $member:ty) => {
        #[derive(Copy, Clone)]
        struct $name(&'static AtomicUsize);
        impl BuilderInput for $name {
            type Dispatch = UnitDispatch<Self>;
            type Init = Self;
        }
        impl HasSchedule for $name {
            type Sched = OnMeta<ScheduleEnd>;
        }
        impl WorkUnit<OnMeta<ScheduleEnd>> for $name {
            type Ctx<'frame> = EngineCtx<
                'frame,
                Queue,
                Cons<$member, Empty>,
                SnapCons<AccSnap<Change>, SnapNil>,
                ColPtrNil,
                ColPtrCons<Height, ColPtrNil>,
                AccPtrNil,
                VirtNil,
                MetaRef<'frame>,
            >;
            type Hint = Hints;
            type Read = Queue;
            type Write = Cons<$member, Empty>;

            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                self.0.fetch_add(1, Ordering::Relaxed);
                ctx.appended().each::<Change, _, _>(|c| {
                    // SAFETY: a maintenance unit's context holds the whole
                    // record range and no other unit runs beside it; `at` is
                    // below the record count by how `Emit` forms it.
                    unsafe { ctx.write::<Height, _>(USize(c.at as usize), Height(c.to)) };
                });
            }
        }
    };
}
close_unit!(ApplyDiffers, Differs<Column<Height>>);
close_unit!(ApplyWritten, Written<Column<Height>>);
close_unit!(ApplyBare, Column<Height>);

// Three passes on one core. `$rests` says whether the reader rests in the
// third pass: under `Differs` the second pass's close wrote what the column
// held and seeded nothing, under the other two members every write counts.
macro_rules! close_on_one_core {
    ($test:ident, $close:ident, $rests:expr) => {
        #[test]
        fn $test() {
            let execs = counter();
            let mut scheduler = Scheduler::builder()
                .with(Column::<Slot>::start())
                .with(Column::<Height>::start())
                .with(Column::<Seen>::start())
                .with(Accum::<Change>::new())
                .with($close(execs))
                .with(Emit)
                .with(Watch)
                .build(store(), USize(RECORDS))
                .unwrap_or_else(|_| panic!("a close unit registers before the units it follows"));
            let seen = scheduler.__bindings().__tail().__ptr().as_ptr();
            let heights = scheduler.__bindings().__tail().__tail().__ptr().as_ptr();
            let slots = scheduler
                .__bindings()
                .__tail()
                .__tail()
                .__tail()
                .__ptr()
                .as_ptr();
            // SAFETY: before the first pass.
            unsafe {
                fill(slots, |i| Slot(i as u32));
                fill(heights, |_| Height(0));
            }

            // Pass 1: the reader runs ahead of the close and sees the column
            // as the build left it. The close applies the queue.
            assert!(matches!(scheduler.run(), Outcome::Ok(())));
            assert_eq!(execs.load(Ordering::Relaxed), 1, "the close ran once");
            // SAFETY: between passes.
            unsafe {
                assert!(all(seen, |s: Seen| s.0).iter().all(|v| *v == 0));
                assert_eq!(all(heights, |h: Height| h.0), applied());
                fill(seen, |_| Seen(SENTINEL));
            }

            // Pass 2: the close changed the column, so its reader runs.
            assert!(matches!(scheduler.run(), Outcome::Ok(())));
            assert_eq!(execs.load(Ordering::Relaxed), 2);
            // SAFETY: between passes.
            unsafe {
                assert_eq!(
                    all(seen, |s: Seen| s.0),
                    applied(),
                    "the next pass reads the close"
                );
                fill(seen, |_| Seen(SENTINEL));
            }

            // Pass 3: pass 2's close wrote the values the column held.
            assert!(matches!(scheduler.run(), Outcome::Ok(())));
            // SAFETY: between passes.
            let got = unsafe { all(seen, |s: Seen| s.0) };
            if $rests {
                assert!(
                    got.iter().all(|v| *v == SENTINEL),
                    "nothing changed, the reader rests"
                );
            } else {
                assert_eq!(got, applied(), "every write counts under this member");
            }
        }
    };
}
close_on_one_core!(
    a_close_under_differs_seeds_only_a_change,
    ApplyDiffers,
    true
);
close_on_one_core!(a_close_under_written_seeds_every_pass, ApplyWritten, false);
close_on_one_core!(
    a_close_over_a_bare_column_seeds_every_pass,
    ApplyBare,
    false
);

// The same carrier across workers. It holds an accumulator, so the workers
// take a record slice each and the close runs on the calling thread after the
// regions are merged. The changes name distinct records, so applying them
// commutes and the column is the same at every worker count.
macro_rules! close_across_workers {
    ($test:ident, $workers:literal) => {
        #[test]
        fn $test() {
            let execs = counter();
            let scheduler = Scheduler::builder()
                .with(Column::<Slot>::start())
                .with(Column::<Height>::start())
                .with(Column::<Seen>::start())
                .with(Accum::<Change>::new())
                .with(Emit)
                .with(Watch)
                .with(ApplyDiffers(execs))
                .build(store(), USize(RECORDS))
                .unwrap_or_else(|_| panic!("build should succeed"));
            let pool = FixedWorkers::<$workers>;
            let mut scheduler = core::pin::pin!(scheduler);
            let view = scheduler.as_ref();
            let b = view.__bindings();
            let seen = b.__tail().__ptr().as_ptr();
            let heights = b.__tail().__tail().__ptr().as_ptr();
            let slots = b.__tail().__tail().__tail().__ptr().as_ptr();
            // SAFETY: before the first pass.
            unsafe {
                fill(slots, |i| Slot(i as u32));
                fill(heights, |_| Height(0));
            }
            assert!(matches!(
                scheduler.as_mut().run_parallel(&pool),
                Outcome::Ok(())
            ));
            assert_eq!(
                execs.load(Ordering::Relaxed),
                1,
                "once in the pass, not once a core"
            );
            // SAFETY: between passes, every worker parked.
            unsafe { assert_eq!(all(heights, |h: Height| h.0), applied()) };
            assert!(matches!(
                scheduler.as_mut().run_parallel(&pool),
                Outcome::Ok(())
            ));
            assert_eq!(execs.load(Ordering::Relaxed), 2);
            // SAFETY: between passes, every worker parked.
            unsafe { assert_eq!(all(seen, |s: Seen| s.0), applied()) };
        }
    };
}
close_across_workers!(a_close_applies_the_queue_with_one_worker, 1);
close_across_workers!(a_close_applies_the_queue_with_two_workers, 2);
close_across_workers!(a_close_applies_the_queue_with_four_workers, 4);

// Collect: a close unit that writes the values it meets in the sequence it
// meets them. The sequence follows the per-core regions after a parallel run,
// so it is not promised across worker counts. The set of values is.
#[derive(Copy, Clone)]
struct Collect;
impl BuilderInput for Collect {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl HasSchedule for Collect {
    type Sched = OnMeta<ScheduleEnd>;
}
impl WorkUnit<OnMeta<ScheduleEnd>> for Collect {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Queue,
        Cons<Written<Column<Seen>>, Empty>,
        SnapCons<AccSnap<Change>, SnapNil>,
        ColPtrNil,
        ColPtrCons<Seen, ColPtrNil>,
        AccPtrNil,
        VirtNil,
        MetaRef<'frame>,
    >;
    type Hint = Hints;
    type Read = Queue;
    type Write = Cons<Written<Column<Seen>>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let mut k = 0;
        ctx.appended().each::<Change, _, _>(|c| {
            // SAFETY: the pass appends one change for every fifth record, so
            // `k` stays below the record count.
            unsafe { ctx.write::<Seen, _>(USize(k), Seen(c.to)) };
            k += 1;
        });
    }
}

fn collected<const W: usize>() -> Vec<u32> {
    let scheduler = Scheduler::builder()
        .with(Column::<Slot>::start())
        .with(Column::<Seen>::start())
        .with(Accum::<Change>::new())
        .with(Emit)
        .with(Collect)
        .build(store(), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let pool = FixedWorkers::<W>;
    let mut scheduler = core::pin::pin!(scheduler);
    let view = scheduler.as_ref();
    let b = view.__bindings();
    let seen = b.__tail().__ptr().as_ptr();
    let slots = b.__tail().__tail().__ptr().as_ptr();
    // SAFETY: before the first pass.
    unsafe {
        fill(slots, |i| Slot(i as u32));
        fill(seen, |_| Seen(SENTINEL));
    }
    assert!(matches!(
        scheduler.as_mut().run_parallel(&pool),
        Outcome::Ok(())
    ));
    // SAFETY: between passes, every worker parked.
    let got = unsafe { all(seen, |s: Seen| s.0) };
    let count = RECORDS / 5;
    assert!(
        got[count ..].iter().all(|v| *v == SENTINEL),
        "the reader met one value for each append and no more",
    );
    got[.. count].to_vec()
}

#[test]
fn the_reader_meets_every_append_at_any_worker_count() {
    let want: Vec<u32> = (0 .. RECORDS as u32).step_by(5).map(|s| 3 * s).collect();
    // One worker appends in record sequence, which is the sequence of `run`.
    assert_eq!(collected::<1>(), want);
    // More workers: the sequence follows the regions and is not promised.
    for mut got in [collected::<2>(), collected::<3>(), collected::<4>()] {
        got.sort_unstable();
        assert_eq!(got, want, "the same set of values");
    }
}

// Setup: a unit of the leading band that writes a column from the pass count.
// It writes 1 in the first two passes and 9 from the third on.
#[derive(Copy, Clone)]
struct Setup(&'static AtomicUsize);
impl BuilderInput for Setup {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl HasSchedule for Setup {
    type Sched = OnMeta<PassStart>;
}
impl WorkUnit<OnMeta<PassStart>> for Setup {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        Heights,
        SnapNil,
        ColPtrNil,
        ColPtrCons<Height, ColPtrNil>,
        AccPtrNil,
        VirtNil,
        MetaRef<'frame>,
    >;
    type Hint = Hints;
    type Read = Empty;
    type Write = Heights;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        self.0.fetch_add(1, Ordering::Relaxed);
        let pass = ctx.meta::<SchedulerMetrics>().pass_count.get().0;
        let value = if pass >= 3 { 9 } else { 1 };
        ctx.each().map(|_read| rec![Height(value)]);
    }
}

// Tally: a close unit with a loop over records and no accumulator.
#[derive(Copy, Clone)]
struct Tally(&'static AtomicUsize);
impl BuilderInput for Tally {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl HasSchedule for Tally {
    type Sched = OnMeta<ScheduleEnd>;
}
impl WorkUnit<OnMeta<ScheduleEnd>> for Tally {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Sights,
        Cons<Column<Slot>, Empty>,
        SnapNil,
        ColPtrCons<Seen, ColPtrNil>,
        ColPtrCons<Slot, ColPtrNil>,
        AccPtrNil,
        VirtNil,
        MetaRef<'frame>,
    >;
    type Hint = Hints;
    type Read = Sights;
    type Write = Cons<Column<Slot>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        self.0.fetch_add(1, Ordering::Relaxed);
        ctx.each()
            .map(|read| rec![Slot(read.get(Column::<Seen>::new()).0 + 100)]);
    }
}

#[test]
fn a_leading_band_s_write_reaches_its_readers_in_the_same_pass() {
    let execs = counter();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Height>::start())
        .with(Column::<Seen>::start())
        .with(Setup(execs))
        .with(Watch)
        .build(store(), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let seen = scheduler.__bindings().__ptr().as_ptr();
    let heights = scheduler.__bindings().__tail().__ptr().as_ptr();
    // SAFETY: before the first pass.
    unsafe { fill(heights, |_| Height(0)) };
    // Passes 1 and 2: the column holds 1 after the setup.
    for _ in 0 .. 2 {
        assert!(matches!(scheduler.run(), Outcome::Ok(())));
        // SAFETY: between passes.
        unsafe { assert!(all(seen, |s: Seen| s.0).iter().all(|v| *v == 1)) };
    }
    // Pass 3: nothing changed in pass 2, so only the setup's own write of
    // this pass can bring the reader in. It does, in the same pass.
    // SAFETY: between passes.
    unsafe { fill(seen, |_| Seen(SENTINEL)) };
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        execs.load(Ordering::Relaxed),
        3,
        "the setup runs once a pass"
    );
    // SAFETY: between passes.
    unsafe { assert!(all(seen, |s: Seen| s.0).iter().all(|v| *v == 9)) };
}

#[test]
fn a_reader_registered_ahead_of_a_leading_band_s_writer_follows_it_all_the_same() {
    // The bands run in the sequence of their ranks and not along the
    // registration, so the setup still runs before the reader registered
    // ahead of it, and the reader sees the setup's write of the same pass.
    let mut scheduler = Scheduler::builder()
        .with(Column::<Height>::start())
        .with(Column::<Seen>::start())
        .with(Watch)
        .with(Setup(counter()))
        .build(store(), USize(RECORDS))
        .unwrap_or_else(|_| panic!("two ranks take the sequence of their ranks"));
    let seen = scheduler.__bindings().__ptr().as_ptr();
    let heights = scheduler.__bindings().__tail().__ptr().as_ptr();
    // SAFETY: before the first pass.
    unsafe { fill(heights, |_| Height(0)) };
    for want in [1, 1, 9, 9] {
        // SAFETY: between passes.
        unsafe { fill(seen, |_| Seen(SENTINEL)) };
        assert!(matches!(scheduler.run(), Outcome::Ok(())));
        // SAFETY: between passes.
        unsafe { assert!(all(seen, |s: Seen| s.0).iter().all(|v| *v == want)) };
    }
}

// The control: inside one rank the walk follows the registration, so a reader
// registered ahead of its writer is refused.
#[derive(Copy, Clone)]
struct Lift;
impl BuilderInput for Lift {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}
impl WorkUnit<Always> for Lift {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Slots,
        Heights,
        SnapNil,
        ColPtrCons<Slot, ColPtrNil>,
        ColPtrCons<Height, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = Hints;
    type Read = Slots;
    type Write = Heights;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each()
            .map(|read| rec![Height(read.get(Column::<Slot>::new()).0)]);
    }
}

#[test]
fn a_reader_registered_ahead_of_a_writer_of_its_own_rank_is_refused() {
    let built = Scheduler::builder()
        .with(Column::<Slot>::start())
        .with(Column::<Height>::start())
        .with(Column::<Seen>::start())
        .with(Watch)
        .with(Lift)
        .build(store(), USize(RECORDS));
    assert!(matches!(
        built,
        Outcome::Err(BuildError::NonTopologicalRegistration { .. })
    ));
}

// A carrier with no accumulator under `run_parallel`: the workers walk the
// phases and split the trunks of each between them. The two meta bands are
// still run once each, by one core, over the whole range. The range is far
// wider than a fiber's window, so a band that rode the morsel windows would
// run once a window and the counts below would say so.
const WIDE: usize = 40_000;

macro_rules! bands_on_the_phase_loop {
    ($test:ident, $workers:literal) => {
        #[test]
        fn $test() {
            let setups = counter();
            let closes = counter();
            let scheduler = Scheduler::builder()
                .with(Column::<Height>::start())
                .with(Column::<Seen>::start())
                .with(Column::<Slot>::start())
                .with(Setup(setups))
                .with(Watch)
                .with(Tally(closes))
                .build(store_of(1 << 20), USize(WIDE))
                .unwrap_or_else(|_| panic!("build should succeed"));
            let pool = FixedWorkers::<$workers>;
            let mut scheduler = core::pin::pin!(scheduler);
            let view = scheduler.as_ref();
            let b = view.__bindings();
            let slots = b.__ptr().as_ptr();
            let seen = b.__tail().__ptr().as_ptr();
            let heights = b.__tail().__tail().__ptr().as_ptr();
            // SAFETY: before the first pass.
            unsafe {
                fill_n(heights, WIDE, |_| Height(0));
                fill_n(slots, WIDE, |_| Slot(SENTINEL));
            }
            for pass in 1 ..= 2 {
                assert!(matches!(
                    scheduler.as_mut().run_parallel(&pool),
                    Outcome::Ok(())
                ));
                assert_eq!(
                    setups.load(Ordering::Relaxed),
                    pass,
                    "the setup ran once in the pass"
                );
                assert_eq!(
                    closes.load(Ordering::Relaxed),
                    pass,
                    "the close ran once in the pass"
                );
                // SAFETY: between passes, every worker parked.
                unsafe {
                    assert!(
                        all_n(heights, WIDE, |h: Height| h.0)
                            .iter()
                            .all(|v| *v == 1)
                    );
                    assert!(all_n(seen, WIDE, |s: Seen| s.0).iter().all(|v| *v == 1));
                    assert!(
                        all_n(slots, WIDE, |s: Slot| s.0).iter().all(|v| *v == 101),
                        "the close's one dispatch covered every record",
                    );
                }
            }
        }
    };
}
bands_on_the_phase_loop!(the_bands_run_once_with_one_worker, 1);
bands_on_the_phase_loop!(the_bands_run_once_with_four_workers, 4);
