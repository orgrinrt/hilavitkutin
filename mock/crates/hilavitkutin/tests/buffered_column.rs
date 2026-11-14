//! A buffered column on the engine, in both modes.
//!
//! A unit reads `Buffered<Height, M>` with a reach of one and writes the same
//! store. Every read of a pass sees the side the last boundary froze, so the
//! result is the same on one core and across workers, and it is the result of
//! the reference step below applied once per pass. The boundary between two
//! passes brings the live side in: `Snapshot` copies it over the frozen side,
//! `Exchange` lets the two reservations trade places. Neither moves a base.
//!
//! The column holds 20000 records, more than three windows of a fiber that
//! writes one four-byte column, so a read with a reach crosses a window edge.
//! `the_record_range_is_cut_into_several_windows` holds that premise.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod common;
mod fixed_workers;
mod stable_provider;

use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Cap, USize};
use fixed_workers::FixedWorkers;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    ColSelector,
    EngineCtx,
    SnapNil,
};
use hilavitkutin::scheduler::{BuildError, Scheduler};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Buffered, Column, Exchange, Snapshot, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{Reach, rec};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::{Maybe, Outcome};
use stable_provider::{StaticBumpMemory, stable_provider};

const ONE: Cap = arvo_tensor::cap(1);
const RECORDS: usize = 20_000;
const SENTINEL: u32 = 0xDEAD;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Height(u32);

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Seen(u32);

fn store() -> ArenaColumnStorage<StaticBumpMemory> {
    ArenaColumnStorage::new(stable_provider(1 << 19, 0))
}

// Count: writes one four-byte column and counts its dispatches, one a window.
#[derive(Copy, Clone)]
struct Count(&'static AtomicUsize);

impl BuilderInput for Count {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Count {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        Cons<Column<Seen>, Empty>,
        SnapNil,
        ColPtrNil,
        ColPtrCons<Seen, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Empty;
    type Write = Cons<Column<Seen>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        self.0.fetch_add(1, Ordering::Relaxed);
        ctx.each().map(|_read| rec![Seen(1)]);
    }
}

// The premise of every test below that leans on a window edge.
#[test]
fn the_record_range_is_cut_into_several_windows() {
    let windows: &'static AtomicUsize = Box::leak(Box::new(AtomicUsize::new(0)));
    let mut scheduler = Scheduler::builder()
        .with(Column::<Seen>::start())
        .with(Count(windows))
        .build(store(), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert!(
        windows.load(Ordering::Relaxed) >= 3,
        "{} windows over {RECORDS} records",
        windows.load(Ordering::Relaxed),
    );
}

fn part(side: Maybe<Height>) -> u32 {
    match side {
        Maybe::Is(h) => h.0,
        Maybe::Isnt => 0,
    }
}

/// The reference step: each record becomes itself plus its two neighbours.
fn step(v: &[u32]) -> Vec<u32> {
    (0 .. v.len())
        .map(|i| {
            let before = if i > 0 { v[i - 1] } else { 0 };
            let after = if i + 1 < v.len() { v[i + 1] } else { 0 };
            v[i].wrapping_add(before).wrapping_add(after)
        })
        .collect()
}

fn start() -> Vec<u32> {
    (0 .. RECORDS as u32).map(|i| i % 7 + 1).collect()
}

/// Fill `RECORDS` records at `base`.
///
/// # Safety
///
/// `base` is a column side of `RECORDS` records and no pass is running.
unsafe fn fill<T: Copy>(base: *mut T, mut value: impl FnMut(usize) -> T) {
    for i in 0 .. RECORDS {
        // SAFETY: inside the reservation, per the contract above.
        unsafe { base.add(i).write(value(i)) };
    }
}

/// Read `RECORDS` records at `base` through `raw`.
///
/// # Safety
///
/// `base` is a column side of `RECORDS` initialised records and no pass is
/// running.
unsafe fn all<T: Copy>(base: *const T, raw: impl Fn(T) -> u32) -> Vec<u32> {
    // SAFETY: inside the reservation, per the contract above.
    (0 .. RECORDS)
        .map(|i| raw(unsafe { base.add(i).read() }))
        .collect()
}

/// Hold `got` against `want` and name the first record that differs, where
/// `assert_eq` would print both columns whole.
#[track_caller]
fn same(got: &[u32], want: &[u32], what: &str) {
    assert_eq!(got.len(), want.len(), "{what}: the record counts");
    if let Some(i) = (0 .. got.len()).find(|i| got[*i] != want[*i]) {
        panic!(
            "{what}: record {i} holds {} where {} is due",
            got[i], want[i]
        );
    }
}

macro_rules! unit {
    ($name:ident, read $read:ty as $rcols:ty, write $write:ty as $wcols:ty, | $r:ident | $body:expr) => {
        #[derive(Copy, Clone)]
        struct $name;

        impl BuilderInput for $name {
            type Dispatch = UnitDispatch<Self>;
            type Init = Self;
        }

        impl WorkUnit<Always> for $name {
            type Ctx<'frame> = EngineCtx<'frame, $read, $write, SnapNil, $rcols, $wcols, AccPtrNil>;
            type Hint = (Immediate, Atomic, Normal);
            type Read = $read;
            type Write = $write;

            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                ctx.each().map(|$r| $body);
            }
        }
    };
}

macro_rules! suite {
    ($name:ident, $mode:ty) => {
        mod $name {
            use super::*;

            type Col = Buffered<Height, $mode>;
            type Cols = ColPtrCons<Height, ColPtrNil, $mode>;
            type Around = Cons<Reach<Col, ONE>, Empty>;
            type Bare = Cons<Col, Empty>;
            type Out = Cons<Column<Seen>, Empty>;
            type OutCols = ColPtrCons<Seen, ColPtrNil>;

            // Each record becomes itself plus its two neighbours.
            unit!(Spread, read Around as Cols, write Bare as Cols, |read| {
                let w = read.get(Reach::<Col, ONE>::new());
                rec![Height(
                    w.here()
                        .0
                        .wrapping_add(part(w.before::<ONE>()))
                        .wrapping_add(part(w.after::<ONE>()))
                )]
            });
            unit!(Raise, read Bare as Cols, write Bare as Cols, |read| {
                rec![Height(read.get(Col::new()).0 + 1)]
            });
            unit!(Double, read Bare as Cols, write Bare as Cols, |read| {
                rec![Height(read.get(Col::new()).0 * 2)]
            });
            unit!(Keep, read Bare as Cols, write Bare as Cols, |read| {
                rec![read.get(Col::new())]
            });
            // Climbs by one a pass and stops at two.
            unit!(Settle, read Bare as Cols, write Bare as Cols, |read| {
                rec![Height((read.get(Col::new()).0 + 1).min(2))]
            });
            // Raises the even values and returns the odd ones as it read them.
            unit!(Half, read Bare as Cols, write Bare as Cols, |read| {
                let v = read.get(Col::new()).0;
                rec![Height(if v % 2 == 0 { v + 1 } else { v })]
            });
            unit!(Fill, read Empty as ColPtrNil, write Bare as Cols, |_read| rec![Height(7)]);
            unit!(Watch, read Bare as Cols, write Out as OutCols, |read| {
                rec![Seen(read.get(Col::new()).0)]
            });

            // A plain column and a buffered one of one value type are two
            // stores, and a unit that lists both reaches each by its own kind.
            type Two = Cons<Column<Height>, Cons<Col, Empty>>;
            type TwoCols = ColPtrCons<Height, ColPtrCons<Height, ColPtrNil, $mode>>;
            unit!(Both, read Two as TwoCols, write Out as OutCols, |read| {
                rec![Seen(read.get(Column::<Height>::new()).0 + read.get(Col::new()).0 * 100)]
            });

            #[test]
            fn a_plain_and_a_buffered_column_of_one_type_are_two_stores() {
                let mut scheduler = Scheduler::builder()
                    .with(Column::<Height>::start_from(Height(3)))
                    .with(Col::start_from(Height(5)))
                    .with(Column::<Seen>::start())
                    .with(Both)
                    .build(store(), USize(RECORDS))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                assert!(matches!(scheduler.run(), Outcome::Ok(())));
                let seen = scheduler.__bindings().__ptr().as_ptr();
                // SAFETY: between passes.
                let got = unsafe { all(seen, |s: Seen| s.0) };
                assert!(got.iter().all(|v| *v == 503), "each read reached its own store");
            }

            // A host that finds a column by its type gets the bases of that
            // column's binding: the frozen and the live side of a buffered
            // column, and the one base of a plain column for both.
            #[test]
            fn a_column_found_by_type_answers_the_bases_of_its_binding() {
                let scheduler = Scheduler::builder()
                    .with(Column::<Height>::start())
                    .with(Col::start_from(Height(0)))
                    .with(Column::<Seen>::start())
                    .with(Both)
                    .build(store(), USize(RECORDS))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                let b = scheduler.__bindings();
                let buffered = <_ as ColSelector<Height, _, $mode>>::get(b);
                assert_eq!(buffered.__read().as_ptr(), b.__tail().__frozen().as_ptr());
                assert_eq!(buffered.__write().as_ptr(), b.__tail().__live().as_ptr());
                let plain = <_ as ColSelector<Height, _>>::get(b);
                assert_eq!(plain.__read().as_ptr(), b.__tail().__tail().__ptr().as_ptr());
                assert_eq!(plain.__write().as_ptr(), b.__tail().__tail().__ptr().as_ptr());
            }

            #[test]
            fn a_reach_read_and_a_write_of_one_store_on_one_core() {
                let mut scheduler = Scheduler::builder()
                    .with(Col::start_from(Height(0)))
                    .with(Spread)
                    .build(store(), USize(RECORDS))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                let mut want = start();
                let first = scheduler.__bindings().__frozen().as_ptr();
                let second = scheduler.__bindings().__live().as_ptr();
                assert_ne!(first, second, "two reservations stand behind the store");
                // SAFETY: before the first pass, on the side its reads see.
                unsafe { fill(first, |i| Height(want[i])) };
                for pass in 1 ..= 3 {
                    assert!(matches!(scheduler.run(), Outcome::Ok(())));
                    want = step(&want);
                    let frozen = scheduler.__bindings().__frozen().as_ptr();
                    let live = scheduler.__bindings().__live().as_ptr();
                    // SAFETY: between passes.
                    let got = unsafe { all(frozen, |h: Height| h.0) };
                    same(&got, &want, "the frozen side holds the pass's result");
                    assert!(
                        (frozen == first && live == second) || (frozen == second && live == first),
                        "pass {pass}: the two bases are the two the build reserved",
                    );
                }
            }

            #[test]
            fn a_reach_read_and_a_write_of_one_store_across_workers() {
                let scheduler = Scheduler::builder()
                    .with(Col::start_from(Height(0)))
                    .with(Spread)
                    .build(store(), USize(RECORDS))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                let pool = FixedWorkers::<4>;
                let mut scheduler = core::pin::pin!(scheduler);
                let mut want = start();
                let first = scheduler.as_ref().__bindings().__frozen().as_ptr();
                let second = scheduler.as_ref().__bindings().__live().as_ptr();
                // SAFETY: before the first pass, on the side its reads see.
                unsafe { fill(first, |i| Height(want[i])) };
                for pass in 1 ..= 3 {
                    assert!(matches!(scheduler.as_mut().run_parallel(&pool), Outcome::Ok(())));
                    want = step(&want);
                    let frozen = scheduler.as_ref().__bindings().__frozen().as_ptr();
                    let live = scheduler.as_ref().__bindings().__live().as_ptr();
                    // SAFETY: between passes, every worker parked.
                    let got = unsafe { all(frozen, |h: Height| h.0) };
                    same(&got, &want, "one result whatever the worker count");
                    assert!(
                        (frozen == first && live == second) || (frozen == second && live == first),
                        "pass {pass}: the two bases are the two the build reserved",
                    );
                }
            }

            // A write lands on the live side at every record, the ones whose
            // value equals the frozen side's too. The live side starts as
            // something else, so a store made only where the value differs
            // would leave the sentinel at every odd record.
            #[test]
            fn a_value_equal_to_the_frozen_one_still_lands_on_the_live_side() {
                let mut scheduler = Scheduler::builder()
                    .with(Col::start_from(Height(0)))
                    .with(Half)
                    .build(store(), USize(RECORDS))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                let frozen = scheduler.__bindings().__frozen().as_ptr();
                let live = scheduler.__bindings().__live().as_ptr();
                // SAFETY: before the first pass.
                unsafe {
                    fill(frozen, |i| Height(i as u32));
                    fill(live, |_| Height(SENTINEL));
                }
                assert!(matches!(scheduler.run(), Outcome::Ok(())));
                let want: Vec<u32> = (0 .. RECORDS as u32)
                    .map(|i| if i % 2 == 0 { i + 1 } else { i })
                    .collect();
                let frozen = scheduler.__bindings().__frozen().as_ptr();
                // SAFETY: between passes.
                let got = unsafe { all(frozen, |h: Height| h.0) };
                same(&got, &want, "every record of the pass's write is on the frozen side");
            }

            // A reader of a buffered column sees what the last pass left, so it
            // does not wait on the writer and the two register in either
            // sequence. The reader's output is the same in both.
            macro_rules! reader_and_writer {
                ($test:ident, $a:ident, $b:ident) => {
                    #[test]
                    fn $test() {
                        let mut scheduler = Scheduler::builder()
                            .with(Col::start_from(Height(10)))
                            .with(Column::<Seen>::start())
                            .with($a)
                            .with($b)
                            .build(store(), USize(RECORDS))
                            .unwrap_or_else(|_| panic!("no edge stands between the two"));
                        assert!(matches!(scheduler.run(), Outcome::Ok(())));
                        let seen = scheduler.__bindings().__ptr().as_ptr();
                        // SAFETY: between passes.
                        let got = unsafe { all(seen, |s: Seen| s.0) };
                        assert!(got.iter().all(|v| *v == 10), "pass 1 reads what the build left");
                        assert!(matches!(scheduler.run(), Outcome::Ok(())));
                        // SAFETY: between passes.
                        let got = unsafe { all(seen, |s: Seen| s.0) };
                        assert!(got.iter().all(|v| *v == 11), "pass 2 reads what pass 1 wrote");
                    }
                };
            }
            reader_and_writer!(the_writer_registered_first, Raise, Watch);
            reader_and_writer!(the_reader_registered_first, Watch, Raise);

            // Two writers both read the frozen side, so the second does not
            // build on the first. The one later in the sequence stores last, and
            // its values are the ones the boundary brings in.
            macro_rules! two_writers {
                ($test:ident, $a:ident, $b:ident, $want:expr) => {
                    #[test]
                    fn $test() {
                        let mut scheduler = Scheduler::builder()
                            .with(Col::start_from(Height(10)))
                            .with($a)
                            .with($b)
                            .build(store(), USize(RECORDS))
                            .unwrap_or_else(|_| panic!("build should succeed"));
                        assert!(matches!(scheduler.run(), Outcome::Ok(())));
                        let frozen = scheduler.__bindings().__frozen().as_ptr();
                        // SAFETY: between passes.
                        let got = unsafe { all(frozen, |h: Height| h.0) };
                        assert!(got.iter().all(|v| *v == $want), "the later writer's values");
                    }
                };
            }
            two_writers!(the_later_of_two_writers_is_brought_in, Raise, Double, 20);
            two_writers!(the_later_of_two_writers_the_other_way, Double, Raise, 11);

            #[test]
            fn a_store_nobody_wrote_is_left_alone() {
                let mut scheduler = Scheduler::builder()
                    .with(Col::start_from(Height(10)))
                    .with(Column::<Seen>::start())
                    .with(Watch)
                    .build(store(), USize(RECORDS))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                let frozen = scheduler.__bindings().__tail().__frozen().as_ptr();
                let live = scheduler.__bindings().__tail().__live().as_ptr();
                // SAFETY: before the first pass. The live side is given values
                // a boundary would bring in if it ran.
                unsafe { fill(live, |_| Height(SENTINEL)) };
                assert!(matches!(scheduler.run(), Outcome::Ok(())));
                assert_eq!(scheduler.__bindings().__tail().__frozen().as_ptr(), frozen);
                // SAFETY: between passes.
                let got = unsafe { all(frozen, |h: Height| h.0) };
                assert!(got.iter().all(|v| *v == 10), "the frozen side was not touched");
            }

            #[test]
            fn a_writer_that_returns_what_it_read_changes_nothing() {
                let mut scheduler = Scheduler::builder()
                    .with(Col::start_from(Height(10)))
                    .with(Column::<Seen>::start())
                    .with(Keep)
                    .with(Watch)
                    .build(store(), USize(RECORDS))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                let frozen = scheduler.__bindings().__tail().__frozen().as_ptr();
                assert!(matches!(scheduler.run(), Outcome::Ok(())));
                assert_eq!(
                    scheduler.__bindings().__tail().__frozen().as_ptr(),
                    frozen,
                    "no boundary ran for a store whose values did not change",
                );
                // The second pass has nothing to do: the reader's output is
                // not written again.
                let seen = scheduler.__bindings().__ptr().as_ptr();
                // SAFETY: between passes.
                unsafe { fill(seen, |_| Seen(SENTINEL)) };
                assert!(matches!(scheduler.run(), Outcome::Ok(())));
                // SAFETY: between passes.
                let got = unsafe { all(seen, |s: Seen| s.0) };
                assert!(got.iter().all(|v| *v == SENTINEL), "the reader rested");
            }

            #[test]
            fn readers_run_the_pass_after_a_change_and_rest_once_it_settles() {
                let mut scheduler = Scheduler::builder()
                    .with(Col::start_from(Height(0)))
                    .with(Column::<Seen>::start())
                    .with(Settle)
                    .with(Watch)
                    .build(store(), USize(RECORDS))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                let seen = scheduler.__bindings().__ptr().as_ptr();
                // Passes 1 to 3: the store changed in the pass before (or the
                // frame is the first), so the reader runs and sees 0, 1, 2.
                for want in 0 .. 3 {
                    // SAFETY: between passes.
                    unsafe { fill(seen, |_| Seen(SENTINEL)) };
                    assert!(matches!(scheduler.run(), Outcome::Ok(())));
                    // SAFETY: between passes.
                    let got = unsafe { all(seen, |s: Seen| s.0) };
                    assert!(got.iter().all(|v| *v == want), "the reader saw {want}");
                }
                // Pass 3 wrote the value the store already held, so pass 4
                // finds nothing changed and nobody runs.
                // SAFETY: between passes.
                unsafe { fill(seen, |_| Seen(SENTINEL)) };
                assert!(matches!(scheduler.run(), Outcome::Ok(())));
                // SAFETY: between passes.
                let got = unsafe { all(seen, |s: Seen| s.0) };
                assert!(got.iter().all(|v| *v == SENTINEL), "the reader rested");
                let frozen = scheduler.__bindings().__tail().__frozen().as_ptr();
                // SAFETY: between passes.
                let held = unsafe { all(frozen, |h: Height| h.0) };
                assert!(held.iter().all(|v| *v == 2), "the store holds the settled value");
            }

            #[test]
            fn a_store_filled_in_the_first_pass_is_read_from_the_second() {
                let mut scheduler = Scheduler::builder()
                    .with(Col::start_from(Height(0)))
                    .with(Column::<Seen>::start())
                    .with(Fill)
                    .with(Watch)
                    .build(store(), USize(RECORDS))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                let seen = scheduler.__bindings().__ptr().as_ptr();
                assert!(matches!(scheduler.run(), Outcome::Ok(())));
                // SAFETY: between passes.
                let got = unsafe { all(seen, |s: Seen| s.0) };
                assert!(got.iter().all(|v| *v == 0), "pass 1 reads the store's start");
                assert!(matches!(scheduler.run(), Outcome::Ok(())));
                // SAFETY: between passes.
                let got = unsafe { all(seen, |s: Seen| s.0) };
                assert!(got.iter().all(|v| *v == 7), "pass 2 reads what pass 1 filled");
            }
        }
    };
}

suite!(snapshot, Snapshot);
suite!(exchange, Exchange);

// The control for the two registration sequences above: over a plain column
// the reader waits on the writer, and registering it first is refused.
mod plain {
    use super::*;

    type Col = Column<Height>;
    type Cols = ColPtrCons<Height, ColPtrNil>;
    type Bare = Cons<Col, Empty>;
    type Out = Cons<Column<Seen>, Empty>;
    type OutCols = ColPtrCons<Seen, ColPtrNil>;

    unit!(Raise, read Bare as Cols, write Bare as Cols, |read| {
        rec![Height(read.get(Col::new()).0 + 1)]
    });
    unit!(Watch, read Bare as Cols, write Out as OutCols, |read| {
        rec![Seen(read.get(Col::new()).0)]
    });

    #[test]
    fn a_plain_column_s_reader_waits_on_its_writer() {
        let mut scheduler = Scheduler::builder()
            .with(Col::start_from(Height(10)))
            .with(Column::<Seen>::start())
            .with(Raise)
            .with(Watch)
            .build(store(), USize(RECORDS))
            .unwrap_or_else(|_| panic!("build should succeed"));
        assert!(matches!(scheduler.run(), Outcome::Ok(())));
        let seen = scheduler.__bindings().__ptr().as_ptr();
        // SAFETY: between passes.
        let got = unsafe { all(seen, |s: Seen| s.0) };
        assert!(
            got.iter().all(|v| *v == 11),
            "the reader sees the same pass's write"
        );
    }

    // A read with a reach of a plain column sees its neighbours as the
    // writer of the same pass left them, past the edge of its own window as
    // well. The plan opens a phase at such a reader, so the writer has covered
    // the whole range before the reader starts.
    type Around = Cons<Reach<Col, ONE>, Empty>;
    unit!(Spread, read Around as Cols, write Out as OutCols, |read| {
        let w = read.get(Reach::<Col, ONE>::new());
        rec![Seen(
            w.here()
                .0
                .wrapping_add(part(w.before::<ONE>()))
                .wrapping_add(part(w.after::<ONE>()))
        )]
    });

    fn raised() -> Vec<u32> {
        start().iter().map(|v| v + 1).collect()
    }

    #[test]
    fn a_reach_reader_of_a_plain_column_follows_its_whole_writer_on_one_core() {
        let mut scheduler = Scheduler::builder()
            .with(Col::start())
            .with(Column::<Seen>::start())
            .with(Raise)
            .with(Spread)
            .build(store(), USize(RECORDS))
            .unwrap_or_else(|_| panic!("build should succeed"));
        let col = scheduler.__bindings().__tail().__ptr().as_ptr();
        let seen = scheduler.__bindings().__ptr().as_ptr();
        let from = start();
        // SAFETY: before the first pass.
        unsafe { fill(col, |i| Height(from[i])) };
        assert!(matches!(scheduler.run(), Outcome::Ok(())));
        // SAFETY: between passes.
        let got = unsafe { all(seen, |s: Seen| s.0) };
        same(
            &got,
            &step(&raised()),
            "every neighbour read was the writer's value",
        );
    }

    #[test]
    fn a_reach_reader_of_a_plain_column_follows_its_whole_writer_across_workers() {
        let scheduler = Scheduler::builder()
            .with(Col::start())
            .with(Column::<Seen>::start())
            .with(Raise)
            .with(Spread)
            .build(store(), USize(RECORDS))
            .unwrap_or_else(|_| panic!("build should succeed"));
        let pool = FixedWorkers::<4>;
        let mut scheduler = core::pin::pin!(scheduler);
        let view = scheduler.as_ref();
        let col = view.__bindings().__tail().__ptr().as_ptr();
        let seen = view.__bindings().__ptr().as_ptr();
        let from = start();
        // SAFETY: before the first pass.
        unsafe { fill(col, |i| Height(from[i])) };
        assert!(matches!(
            scheduler.as_mut().run_parallel(&pool),
            Outcome::Ok(())
        ));
        // SAFETY: between passes, every worker parked.
        let got = unsafe { all(seen, |s: Seen| s.0) };
        same(
            &got,
            &step(&raised()),
            "every neighbour read was the writer's value",
        );
    }

    #[test]
    fn a_plain_column_s_reader_registered_first_is_refused() {
        let built = Scheduler::builder()
            .with(Col::start())
            .with(Column::<Seen>::start())
            .with(Watch)
            .with(Raise)
            .build(store(), USize(RECORDS));
        assert!(matches!(
            built,
            Outcome::Err(BuildError::NonTopologicalRegistration { .. })
        ));
    }
}
