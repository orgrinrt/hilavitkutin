//! What a column, plain or buffered, holds before any unit has written it.
//!
//! A `Column<T>` and a `Buffered<T, M>` are registered with the value every
//! record starts from, and the build writes that value to every record: of the
//! one reservation of a plain column, and of both sides of a buffered one. The
//! first pass reads it, and a record no unit wrote in a pass still holds it
//! after the boundary.
//!
//! Every scheduler here is built over memory that is `0xA5` at every byte, so
//! a reservation the build left alone reads as 2779096485 and not as a zero
//! that could pass for a start. No test here writes a column by hand: the
//! hidden bases are read after the passes and nothing else.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod stable_provider;

use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::rec;
use hilavitkutin_api::store::{Buffered, Column, Exchange, Snapshot, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use stable_provider::StableProvider;

const RECORDS: usize = 20_000;

/// What the output column starts from: no start under test, and not what
/// memory the build left alone reads as.
const UNSEEN: u32 = 0x0BAD_5EED;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Height(u32);

/// Starts from its constructor, which is not its default.
#[derive(Copy, Clone, PartialEq, Debug)]
struct Built(u32);

impl notko::HasTrivialCtor for Built {
    fn new() -> Self {
        Built(7)
    }
}

impl Default for Built {
    fn default() -> Self {
        Built(9)
    }
}

/// Starts from its default, which is zero: the start that memory which
/// happens to be zero would pass for.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct Zeroed(u32);

#[derive(Copy, Clone, PartialEq, Debug)]
struct Seen(u32);

fn store() -> ArenaColumnStorage<StableProvider> {
    ArenaColumnStorage::new(StableProvider::filled(1 << 19, 0xA5))
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

/// Hold every record of `got` to `want` and name the first that differs.
#[track_caller]
fn every(got: &[u32], want: u32, what: &str) {
    assert_eq!(got.len(), RECORDS, "{what}: the record count");
    if let Some(i) = got.iter().position(|v| *v != want) {
        panic!("{what}: record {i} holds {} where {want} is due", got[i]);
    }
}

mod plain {
    use super::*;

    // Writes what it read of a plain column of `$t` to the output. Nothing
    // writes the column it reads, so what it reads is what the build left.
    macro_rules! watch {
        ($unit:ident, $t:ident) => {
            #[derive(Copy, Clone)]
            struct $unit;

            impl BuilderInput for $unit {
                type Dispatch = UnitDispatch<Self>;
                type Init = Self;
            }

            impl WorkUnit<Always> for $unit {
                type Ctx<'frame> = EngineCtx<
                    'frame,
                    Cons<Column<$t>, Empty>,
                    Cons<Column<Seen>, Empty>,
                    SnapNil,
                    ColPtrCons<$t, ColPtrNil>,
                    ColPtrCons<Seen, ColPtrNil>,
                    AccPtrNil,
                >;
                type Hint = (Immediate, Atomic, Normal);
                type Read = Cons<Column<$t>, Empty>;
                type Write = Cons<Column<Seen>, Empty>;

                fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                    ctx.each()
                        .map(|read| rec![Seen(read.get(Column::<$t>::new()).0)]);
                }
            }
        };
    }
    watch!(WatchHeight, Height);
    watch!(WatchBuilt, Built);
    watch!(WatchZeroed, Zeroed);

    // What the first pass read of a column nothing wrote, and the column
    // after it.
    macro_rules! the_first_pass_reads {
        ($test:ident, $unit:ident, $t:ident, $started:expr, $want:expr) => {
            #[test]
            fn $test() {
                let mut scheduler = Scheduler::builder()
                    .with($started)
                    .with(Column::<Seen>::start_from(Seen(UNSEEN)))
                    .with($unit)
                    .build(store(), USize(RECORDS))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                assert!(matches!(scheduler.run(), Outcome::Ok(())));
                let seen = scheduler.__bindings().__ptr().as_ptr();
                // SAFETY: between passes.
                let got = unsafe { all(seen, |s: Seen| s.0) };
                every(&got, $want, "what the first pass read");
                let column = scheduler.__bindings().__tail().__ptr().as_ptr();
                // SAFETY: between passes.
                let got = unsafe { all(column, |v: $t| v.0) };
                every(&got, $want, "the column");
            }
        };
    }
    the_first_pass_reads!(
        an_explicit_start,
        WatchHeight,
        Height,
        Column::<Height>::start_from(Height(5)),
        5
    );
    the_first_pass_reads!(
        the_constructor_of_a_type_that_has_one,
        WatchBuilt,
        Built,
        Column::<Built>::start(),
        7
    );
    the_first_pass_reads!(
        the_default_of_a_type_that_has_no_constructor,
        WatchZeroed,
        Zeroed,
        Column::<Zeroed>::start(),
        0
    );

    // A column a scheduler is built with and no unit names holds its start
    // all the same: the write is the build's and waits on no reader.
    #[test]
    fn a_column_no_unit_names_holds_its_start() {
        let scheduler = Scheduler::builder()
            .with(Column::<Height>::start_from(Height(5)))
            .build(store(), USize(RECORDS))
            .unwrap_or_else(|_| panic!("build should succeed"));
        let column = scheduler.__bindings().__ptr().as_ptr();
        // SAFETY: no pass has run.
        let got = unsafe { all(column, |h: Height| h.0) };
        every(&got, 5, "the column");
    }
}

macro_rules! suite {
    ($name:ident, $mode:ty) => {
        mod $name {
            use super::*;

            // Writes what it read of a buffered column of `$t` to the output.
            macro_rules! watch {
                ($unit:ident, $t:ident) => {
                    #[derive(Copy, Clone)]
                    struct $unit;

                    impl BuilderInput for $unit {
                        type Dispatch = UnitDispatch<Self>;
                        type Init = Self;
                    }

                    impl WorkUnit<Always> for $unit {
                        type Ctx<'frame> = EngineCtx<
                            'frame,
                            Cons<Buffered<$t, $mode>, Empty>,
                            Cons<Column<Seen>, Empty>,
                            SnapNil,
                            ColPtrCons<$t, ColPtrNil, $mode>,
                            ColPtrCons<Seen, ColPtrNil>,
                            AccPtrNil,
                        >;
                        type Hint = (Immediate, Atomic, Normal);
                        type Read = Cons<Buffered<$t, $mode>, Empty>;
                        type Write = Cons<Column<Seen>, Empty>;

                        fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                            ctx.each()
                                .map(|read| rec![Seen(read.get(Buffered::<$t, $mode>::new()).0)]);
                        }
                    }
                };
            }
            watch!(WatchHeight, Height);
            watch!(WatchBuilt, Built);
            watch!(WatchZeroed, Zeroed);

            // What the first pass read, and both sides after it: the store
            // is one nothing wrote, so the start is on both.
            macro_rules! the_first_pass_reads {
                ($test:ident, $unit:ident, $t:ident, $started:expr, $want:expr) => {
                    #[test]
                    fn $test() {
                        let mut scheduler = Scheduler::builder()
                            .with($started)
                            .with(Column::<Seen>::start_from(Seen(UNSEEN)))
                            .with($unit)
                            .build(store(), USize(RECORDS))
                            .unwrap_or_else(|_| panic!("build should succeed"));
                        assert!(matches!(scheduler.run(), Outcome::Ok(())));
                        let seen = scheduler.__bindings().__ptr().as_ptr();
                        // SAFETY: between passes.
                        let got = unsafe { all(seen, |s: Seen| s.0) };
                        every(&got, $want, "what the first pass read");
                        let frozen = scheduler.__bindings().__tail().__frozen().as_ptr();
                        let live = scheduler.__bindings().__tail().__live().as_ptr();
                        assert_ne!(frozen, live, "two reservations stand behind the store");
                        // SAFETY: between passes.
                        let got = unsafe { all(frozen, |v: $t| v.0) };
                        every(&got, $want, "the frozen side");
                        // SAFETY: between passes.
                        let got = unsafe { all(live, |v: $t| v.0) };
                        every(&got, $want, "the live side");
                    }
                };
            }
            the_first_pass_reads!(
                an_explicit_start,
                WatchHeight,
                Height,
                Buffered::<Height, $mode>::start_from(Height(5)),
                5
            );
            the_first_pass_reads!(
                the_constructor_of_a_type_that_has_one,
                WatchBuilt,
                Built,
                Buffered::<Built, $mode>::start(),
                7
            );
            the_first_pass_reads!(
                the_default_of_a_type_that_has_no_constructor,
                WatchZeroed,
                Zeroed,
                Buffered::<Zeroed, $mode>::start(),
                0
            );

            // Writes the store in the first window it is handed and in no
            // other, so most records of the pass are ones no unit wrote.
            #[derive(Copy, Clone)]
            struct FirstWindow(&'static AtomicUsize);

            impl BuilderInput for FirstWindow {
                type Dispatch = UnitDispatch<Self>;
                type Init = Self;
            }

            impl WorkUnit<Always> for FirstWindow {
                type Ctx<'frame> = EngineCtx<
                    'frame,
                    Empty,
                    Cons<Buffered<Height, $mode>, Empty>,
                    SnapNil,
                    ColPtrNil,
                    ColPtrCons<Height, ColPtrNil, $mode>,
                    AccPtrNil,
                >;
                type Hint = (Immediate, Atomic, Normal);
                type Read = Empty;
                type Write = Cons<Buffered<Height, $mode>, Empty>;

                fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                    if self.0.fetch_add(1, Ordering::Relaxed) == 0 {
                        ctx.each().map(|_read| rec![Height(9)]);
                    }
                }
            }

            // The boundary brings the whole live side in. A record the pass
            // did not write is on that side as the build left it.
            #[test]
            fn a_record_no_unit_wrote_holds_the_start_after_the_boundary() {
                let windows: &'static AtomicUsize = Box::leak(Box::new(AtomicUsize::new(0)));
                let mut scheduler = Scheduler::builder()
                    .with(Buffered::<Height, $mode>::start_from(Height(5)))
                    .with(FirstWindow(windows))
                    .build(store(), USize(RECORDS))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                assert!(matches!(scheduler.run(), Outcome::Ok(())));
                assert!(
                    windows.load(Ordering::Relaxed) >= 3,
                    "{} windows over {RECORDS} records",
                    windows.load(Ordering::Relaxed),
                );
                let frozen = scheduler.__bindings().__frozen().as_ptr();
                // SAFETY: between passes.
                let got = unsafe { all(frozen, |h: Height| h.0) };
                let written = got.iter().filter(|v| **v == 9).count();
                let started = got.iter().filter(|v| **v == 5).count();
                assert!(written > 0, "the first window was written");
                assert!(started > 0, "the other windows were not");
                assert_eq!(
                    written + started,
                    RECORDS,
                    "every record holds the write or the start, and record {:?} holds neither",
                    got.iter().position(|v| *v != 9 && *v != 5),
                );
            }
        }
    };
}

suite!(snapshot, Snapshot);
suite!(exchange, Exchange);
