//! The host path: `Scheduler::import` and `Scheduler::dump` on a built scheduler.
//!
//! A host copies records into a column between frames, runs a frame, and
//! pushes the records of another column into a sink. The store is found by its
//! marker type, as a unit's context finds it, so an import into `Column<A>`
//! lands where a unit reading `Column<A>` looks. An import marks its store
//! changed, so the units reading it run in the next frame and the rest stay
//! rested; a range past the store's record count is refused whole.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod common;
mod fixed_workers;
mod stable_provider;

use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::USize;
use fixed_workers::FixedWorkers;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    ColSelector,
    EngineCtx,
    SnapNil,
};
use hilavitkutin::scheduler::{HostRefused, Scheduler};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Buffered, Column, Exchange, Snapshot};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{ReadVals, RecordOp, WriteVals, rec};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use notko::sink::{BulkPush, Push};
use stable_provider::{StaticBumpMemory, stable_provider};

#[derive(Copy, Clone, PartialEq, Debug)]
struct A(u32);
#[derive(Copy, Clone, PartialEq, Debug)]
struct B(u32);
#[derive(Copy, Clone, PartialEq, Debug)]
struct Other(u32);
#[derive(Copy, Clone, PartialEq, Debug)]
struct Out(u32);

fn store() -> ArenaColumnStorage<StaticBumpMemory> {
    ArenaColumnStorage::new(stable_provider(1 << 19, 0))
}

fn counter() -> &'static AtomicUsize {
    Box::leak(Box::new(AtomicUsize::new(0)))
}

/// A sink that keeps what it is given, and how many bulk pushes brought it.
struct Collect<T> {
    got:   Vec<T>,
    bulks: usize,
}

impl<T> Collect<T> {
    fn new() -> Self {
        Self {
            got:   Vec::new(),
            bulks: 0,
        }
    }
}

impl<T> Push<T> for Collect<T> {
    fn push(&mut self, item: T) {
        self.got.push(item);
    }
}

impl<T: Copy> BulkPush<T> for Collect<T> {
    fn push_bulk(&mut self, items: &[T]) {
        self.bulks += 1;
        self.got.extend_from_slice(items);
    }
}

// Twice: reads `Column<A>`, writes `B = 2A`, and counts its dispatches.
#[derive(Copy, Clone)]
struct Twice(&'static AtomicUsize);

impl BuilderInput for Twice {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Twice {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Cons<Column<A>, Empty>,
        Cons<Column<B>, Empty>,
        SnapNil,
        ColPtrCons<A, ColPtrNil>,
        ColPtrCons<B, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<A>, Empty>;
    type Write = Cons<Column<B>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        self.0.fetch_add(1, Ordering::Relaxed);
        ctx.each().map(|read| self.apply(read));
    }
}

impl RecordOp for Twice {
    fn apply(
        &self,
        read: &ReadVals<'_, Cons<Column<A>, Empty>>,
    ) -> WriteVals<Cons<Column<B>, Empty>> {
        rec![B(read.get(Column::<A>::new()).0 * 2)]
    }
}

// Lone: reads `Column<Other>`, which no import touches, and counts.
#[derive(Copy, Clone)]
struct Lone(&'static AtomicUsize);

impl BuilderInput for Lone {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Lone {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Cons<Column<Other>, Empty>,
        Cons<Column<Out>, Empty>,
        SnapNil,
        ColPtrCons<Other, ColPtrNil>,
        ColPtrCons<Out, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Other>, Empty>;
    type Write = Cons<Column<Out>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        self.0.fetch_add(1, Ordering::Relaxed);
        ctx.each()
            .map(|read| rec![Out(read.get(Column::<Other>::new()).0 + 1)]);
    }
}

macro_rules! two_units {
    ($records:expr, $twice:expr, $lone:expr) => {
        Scheduler::builder()
            .with(Column::<A>::start_from(A(0)))
            .with(Column::<B>::start_from(B(0)))
            .with(Column::<Other>::start_from(Other(0)))
            .with(Column::<Out>::start_from(Out(0)))
            .with(Twice($twice))
            .with(Lone($lone))
            .build(store(), USize($records))
            .unwrap_or_else(|_| panic!("build should succeed"))
    };
}

fn b_values(sink: &Collect<B>) -> Vec<u32> {
    sink.got.iter().map(|b| b.0).collect()
}

fn a_records(values: &[u32]) -> Vec<A> {
    values.iter().map(|v| A(*v)).collect()
}

#[test]
fn an_import_reaches_the_unit_reading_the_store_and_a_dump_reads_its_output() {
    let mut scheduler = two_units!(4, counter(), counter());
    assert!(matches!(
        scheduler.import::<Column<A>, _, _>(USize(0), &a_records(&[10, 20, 30, 40])),
        Outcome::Ok(())
    ));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    let mut sink = Collect::new();
    assert!(matches!(
        scheduler.dump::<Column<B>, _, _>(USize(0), USize(4), &mut sink),
        Outcome::Ok(())
    ));
    assert_eq!(b_values(&sink), [20, 40, 60, 80]);
    assert_eq!(sink.bulks, 1, "the dump is one bulk push");
}

#[test]
fn an_import_and_a_dump_at_an_offset_touch_only_their_range() {
    let mut scheduler = two_units!(6, counter(), counter());
    assert!(matches!(
        scheduler.import::<Column<A>, _, _>(USize(2), &a_records(&[5, 6])),
        Outcome::Ok(())
    ));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    let mut whole = Collect::new();
    assert!(matches!(
        scheduler.dump::<Column<B>, _, _>(USize(0), USize(6), &mut whole),
        Outcome::Ok(())
    ));
    assert_eq!(b_values(&whole), [0, 0, 10, 12, 0, 0]);
    let mut part = Collect::new();
    assert!(matches!(
        scheduler.dump::<Column<B>, _, _>(USize(3), USize(2), &mut part),
        Outcome::Ok(())
    ));
    assert_eq!(b_values(&part), [12, 0]);
}

// After a cold frame and a clean one, an import makes the unit reading the
// store run, and the unit reading another store stays rested. A twin writes
// the same records into the column memory without the import: its reader
// stays rested and its output keeps the old values.
#[test]
fn an_import_runs_the_readers_of_its_store_and_rests_the_rest() {
    let (twice, lone) = (counter(), counter());
    let mut scheduler = two_units!(4, twice, lone);
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    let (t0, l0) = (twice.load(Ordering::Relaxed), lone.load(Ordering::Relaxed));
    assert!(t0 > 0 && l0 > 0, "the cold frame ran both");
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        twice.load(Ordering::Relaxed),
        t0,
        "a clean frame rests the reader of A"
    );

    assert!(matches!(
        scheduler.import::<Column<A>, _, _>(USize(0), &a_records(&[1, 2, 3, 4])),
        Outcome::Ok(())
    ));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert!(
        twice.load(Ordering::Relaxed) > t0,
        "the import ran the reader of A"
    );
    assert_eq!(
        lone.load(Ordering::Relaxed),
        l0,
        "the reader of Other stayed rested"
    );
    let mut sink = Collect::new();
    assert!(matches!(
        scheduler.dump::<Column<B>, _, _>(USize(0), USize(4), &mut sink),
        Outcome::Ok(())
    ));
    assert_eq!(b_values(&sink), [2, 4, 6, 8]);

    let (twin_twice, twin_lone) = (counter(), counter());
    let mut twin = two_units!(4, twin_twice, twin_lone);
    assert!(matches!(twin.run(), Outcome::Ok(())));
    let before = twin_twice.load(Ordering::Relaxed);
    let base = <_ as ColSelector<A, _>>::get(twin.__bindings())
        .__read()
        .as_ptr();
    for (i, v) in [1, 2, 3, 4].into_iter().enumerate() {
        // SAFETY: between frames, inside the four records the build reserved.
        unsafe { base.add(i).write(A(v)) };
    }
    assert!(matches!(twin.run(), Outcome::Ok(())));
    assert_eq!(
        twin_twice.load(Ordering::Relaxed),
        before,
        "an unmarked write rests the reader"
    );
    let mut stale = Collect::new();
    assert!(matches!(
        twin.dump::<Column<B>, _, _>(USize(0), USize(4), &mut stale),
        Outcome::Ok(())
    ));
    assert_eq!(
        b_values(&stale),
        [0, 0, 0, 0],
        "the twin diverges from the import"
    );
}

#[test]
fn a_range_past_the_count_is_refused_and_moves_nothing() {
    let twice = counter();
    let mut scheduler = two_units!(4, twice, counter());
    assert!(matches!(
        scheduler.import::<Column<A>, _, _>(USize(0), &a_records(&[1, 2, 3, 4])),
        Outcome::Ok(())
    ));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    let ran = twice.load(Ordering::Relaxed);

    let past = HostRefused::PastCount {
        count: USize(4),
    };
    for (at, values) in [
        (3, &[8, 9][..]),
        (5, &[8][..]),
        (0, &[8, 8, 8, 8, 8][..]),
        (usize::MAX, &[8][..]),
    ] {
        match scheduler.import::<Column<A>, _, _>(USize(at), &a_records(values)) {
            Outcome::Err(e) => assert_eq!(e, past, "import at {at} of {}", values.len()),
            Outcome::Ok(()) => panic!("import at {at} of {} records was taken", values.len()),
        }
    }
    let mut sink = Collect::new();
    for (at, len) in [(3, 2), (5, 0), (0, 5), (usize::MAX, 2)] {
        match scheduler.dump::<Column<A>, _, _>(USize(at), USize(len), &mut sink) {
            Outcome::Err(e) => assert_eq!(e, past, "dump at {at} of {len}"),
            Outcome::Ok(()) => panic!("dump at {at} of {len} records was taken"),
        }
    }
    assert!(sink.got.is_empty(), "a refused dump pushes nothing");

    let mut held = Collect::new();
    assert!(matches!(
        scheduler.dump::<Column<A>, _, _>(USize(0), USize(4), &mut held),
        Outcome::Ok(())
    ));
    assert_eq!(
        held.got,
        a_records(&[1, 2, 3, 4]),
        "a refused import is not clipped into the range"
    );
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        twice.load(Ordering::Relaxed),
        ran,
        "a refused import marks nothing"
    );
}

#[test]
fn an_empty_import_marks_nothing() {
    let (twice, lone) = (counter(), counter());
    let mut scheduler = two_units!(4, twice, lone);
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    let (t0, l0) = (twice.load(Ordering::Relaxed), lone.load(Ordering::Relaxed));
    assert!(matches!(
        scheduler.import::<Column<A>, _, _>(USize(4), &[]),
        Outcome::Ok(())
    ));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        twice.load(Ordering::Relaxed),
        t0,
        "the reader of A stayed rested"
    );
    assert_eq!(
        lone.load(Ordering::Relaxed),
        l0,
        "the reader of Other stayed rested"
    );
    let mut sink = Collect::new();
    assert!(matches!(
        scheduler.dump::<Column<B>, _, _>(USize(4), USize(0), &mut sink),
        Outcome::Ok(())
    ));
    assert!(
        sink.got.is_empty() && sink.bulks == 0,
        "an empty dump pushes nothing"
    );
}

// A buffered column, in both modes: each frame imports, runs a unit raising
// every record by one, and dumps. Exchange trades the two reservations at
// every boundary, so the even and the odd frame read different sides, and an
// import that wrote one side only would be lost on one of them.
macro_rules! buffered {
    ($name:ident, $mode:ty) => {
        mod $name {
            use super::*;

            type Col = Buffered<A, $mode>;
            type Cols = ColPtrCons<A, ColPtrNil, $mode>;
            type Bare = Cons<Col, Empty>;

            #[derive(Copy, Clone)]
            struct Raise;

            impl BuilderInput for Raise {
                type Dispatch = UnitDispatch<Self>;
                type Init = Self;
            }

            impl WorkUnit<Always> for Raise {
                type Ctx<'frame> = EngineCtx<'frame, Bare, Bare, SnapNil, Cols, Cols, AccPtrNil>;
                type Hint = (Immediate, Atomic, Normal);
                type Read = Bare;
                type Write = Bare;

                fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                    ctx.each().map(|read| rec![A(read.get(Col::new()).0 + 1)]);
                }
            }

            #[test]
            fn every_frame_reads_the_import_on_whichever_side_is_frozen() {
                let mut scheduler = Scheduler::builder()
                    .with(Col::start_from(A(0)))
                    .with(Raise)
                    .build(store(), USize(4))
                    .unwrap_or_else(|_| panic!("build should succeed"));
                for frame in 1 ..= 4u32 {
                    let values = [frame * 10, frame * 10 + 1, frame * 10 + 2, frame * 10 + 3];
                    assert!(matches!(
                        scheduler.import::<Col, _, _>(USize(0), &a_records(&values)),
                        Outcome::Ok(())
                    ));
                    assert!(matches!(scheduler.run(), Outcome::Ok(())));
                    let mut sink = Collect::new();
                    assert!(matches!(
                        scheduler.dump::<Col, _, _>(USize(0), USize(4), &mut sink),
                        Outcome::Ok(())
                    ));
                    let want: Vec<A> = values.iter().map(|v| A(v + 1)).collect();
                    assert_eq!(sink.got, want, "frame {frame}");
                }
            }
        }
    };
}

buffered!(snapshot, Snapshot);
buffered!(exchange, Exchange);

// One history of imports, frames and dumps, under each of the three entries.
fn history(mut step: impl FnMut(&[u32]) -> Vec<u32>) -> Vec<Vec<u32>> {
    [[1, 2, 3, 4], [9, 9, 9, 9], [0, 7, 0, 7]]
        .iter()
        .map(|v| step(v))
        .collect()
}

fn want_history() -> Vec<Vec<u32>> {
    history(|v| v.iter().map(|x| x * 2).collect())
}

#[test]
fn run_and_run_parallel_and_run_fused_give_one_history() {
    let mut serial = two_units!(4, counter(), counter());
    let got = history(|v| {
        assert!(matches!(
            serial.import::<Column<A>, _, _>(USize(0), &a_records(v)),
            Outcome::Ok(())
        ));
        assert!(matches!(serial.run(), Outcome::Ok(())));
        let mut sink = Collect::new();
        assert!(matches!(
            serial.dump::<Column<B>, _, _>(USize(0), USize(4), &mut sink),
            Outcome::Ok(())
        ));
        b_values(&sink)
    });
    assert_eq!(got, want_history(), "run");

    let mut fused = Scheduler::builder()
        .with(Column::<A>::start_from(A(0)))
        .with(Column::<B>::start_from(B(0)))
        .with(Twice(counter()))
        .build(store(), USize(4))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let got = history(|v| {
        assert!(matches!(
            fused.import::<Column<A>, _, _>(USize(0), &a_records(v)),
            Outcome::Ok(())
        ));
        assert!(matches!(fused.run_fused(), Outcome::Ok(())));
        let mut sink = Collect::new();
        assert!(matches!(
            fused.dump::<Column<B>, _, _>(USize(0), USize(4), &mut sink),
            Outcome::Ok(())
        ));
        b_values(&sink)
    });
    assert_eq!(got, want_history(), "run_fused");

    let pool = FixedWorkers::<4>;
    let parallel = two_units!(4, counter(), counter());
    let mut parallel = core::pin::pin!(parallel);
    let got = history(|v| {
        // SAFETY: between frames every worker is parked, and the import moves
        // nothing out of the pinned scheduler.
        let parked = unsafe { parallel.as_mut().get_unchecked_mut() };
        assert!(matches!(
            parked.import::<Column<A>, _, _>(USize(0), &a_records(v)),
            Outcome::Ok(())
        ));
        assert!(matches!(
            parallel.as_mut().run_parallel(&pool),
            Outcome::Ok(())
        ));
        let mut sink = Collect::new();
        assert!(matches!(
            parallel
                .as_ref()
                .dump::<Column<B>, _, _>(USize(0), USize(4), &mut sink),
            Outcome::Ok(())
        ));
        b_values(&sink)
    });
    assert_eq!(got, want_history(), "run_parallel");
}

// The import of one record into a column cut into several windows reruns the
// one window that covers it. Red: the import marks the whole store, so every
// window of the reader reruns.
#[test]
#[ignore = "catalogue: an import of one record marks its whole store changed, so every window of its reader reruns; needs a mark finer than the store; tracked #a_host_import_marks_its_whole_store_changed"]
fn an_import_of_one_record_reruns_only_the_morsel_that_covers_it() {
    const RECORDS: usize = 20_000;
    let twice = counter();
    let mut scheduler = two_units!(RECORDS, twice, counter());
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    let windows = twice.load(Ordering::Relaxed);
    assert!(
        windows >= 2,
        "the premise: {windows} windows over {RECORDS} records"
    );
    assert!(matches!(
        scheduler.import::<Column<A>, _, _>(USize(0), &[A(1)]),
        Outcome::Ok(())
    ));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        twice.load(Ordering::Relaxed) - windows,
        1,
        "only the window holding record 0 reruns"
    );
}
