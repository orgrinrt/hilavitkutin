//! A reach read of a plain column beside a writer of it, across cores.
//!
//! A read with a reach sees records other than the one the unit stands on, so
//! a writer of the column ahead of the reader has to have landed its whole
//! range on every core before any core starts the reader. The phase loop of
//! `run_parallel` keeps that by the barrier between two phases. A carrier with
//! an accumulator takes the unit-outer path instead, where each core walks the
//! whole carrier over its own slice of the records, and the tests here hold
//! the boundary on that path too.
//!
//! The column holds 20000 records, so four workers take 5000 each. A unit
//! sleeps at one record beside a slice edge, which makes the core on the other
//! side of the edge arrive first every time, whatever the machine does.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod common;
mod stable_provider;

use std::thread::sleep;
use std::time::Duration;

use arvo::{Cap, USize};
use common::FixedWorkers;
use hilavitkutin::dispatch::engine_ctx::CtxFor;
use hilavitkutin::scheduler::{BuildError, Scheduler};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::WorkUnit;
use hilavitkutin_api::{Reach, rec};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::{Maybe, Outcome};
use stable_provider::StableProvider;

const ONE: Cap = arvo_tensor::cap(1);
const RECORDS: usize = 20_000;
/// The last record of the first of four slices.
const EDGE: u32 = 4999;
const PAUSE: Duration = Duration::from_millis(200);

#[derive(Copy, Clone, PartialEq, Debug)]
struct Height(u32);

#[derive(Copy, Clone, PartialEq, Debug)]
struct Seen(u32);

#[derive(Copy, Clone, PartialEq, Debug)]
struct Noted(u32);

type Col = Column<Height>;
type Bare = Cons<Col, Empty>;
type Around = Cons<Reach<Col, ONE>, Empty>;
type Out = Cons<Column<Seen>, Empty>;

fn store() -> ArenaColumnStorage<StableProvider> {
    ArenaColumnStorage::new(StableProvider::new(1 << 19))
}

macro_rules! unit {
    ($(#[$doc:meta])* $name:ident, read $read:ty, write $write:ty, | $r:ident | $body:expr) => {
        $(#[$doc])*
        #[derive(Copy, Clone)]
        struct $name;

        impl BuilderInput for $name {
            type Dispatch = UnitDispatch<Self>;
            type Init = Self;
        }

        impl WorkUnit for $name {
            type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
            type Hint = (Immediate, Atomic, Normal);
            type Read = $read;
            type Write = $write;

            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                ctx.each().map(|$r| $body);
            }
        }
    };
}

fn part(side: Maybe<Height>) -> u32 {
    match side {
        Maybe::Is(h) => h.0,
        Maybe::Isnt => 0,
    }
}

fn around(read_here: Height, before: Maybe<Height>, after: Maybe<Height>) -> Seen {
    Seen(
        read_here
            .0
            .wrapping_add(part(before))
            .wrapping_add(part(after)),
    )
}

unit!(
    /// Raises every record by one, and is slow at the last record of the first
    /// slice, so the core of the second slice is done with its own long before.
    SlowRaise,
    read Bare,
    write Bare,
    |read| {
        let v = read.get(Col::new()).0;
        if v == EDGE {
            sleep(PAUSE);
        }
        rec![Height(v + 1)]
    }
);

unit!(
    /// Raises every record by one.
    Raise,
    read Bare,
    write Bare,
    |read| rec![Height(read.get(Col::new()).0 + 1)]
);

unit!(
    /// Each record with its two neighbours.
    Spread,
    read Around,
    write Out,
    |read| {
        let w = read.get(Reach::<Col, ONE>::new());
        rec![around(w.here(), w.before::<ONE>(), w.after::<ONE>())]
    }
);

unit!(
    /// Appends nothing. It is here to put an accumulator in the carrier, which
    /// is what sends `run_parallel` down the unit-outer path.
    Note,
    read Out,
    write Cons<Accum<Noted>, Empty>,
    |_read| rec![Maybe::<Noted>::Isnt]
);

/// What `Spread` has to write when it reads `heights`.
fn spread_of(heights: &[u32]) -> Vec<u32> {
    (0 .. heights.len())
        .map(|i| {
            let before = if i > 0 { heights[i - 1] } else { 0 };
            let after = if i + 1 < heights.len() { heights[i + 1] } else { 0 };
            heights[i].wrapping_add(before).wrapping_add(after)
        })
        .collect()
}

/// The records of `seen` that differ from `want`, the first eight of them.
///
/// # Safety
///
/// `seen` is a column of `RECORDS` initialised records and no pass is running.
unsafe fn wrong(seen: *const Seen, want: &[u32]) -> Vec<(usize, u32, u32)> {
    (0 .. RECORDS)
        // SAFETY: inside the reservation, per the contract above.
        .map(|i| (i, unsafe { seen.add(i).read() }.0, want[i]))
        .filter(|(_, got, want)| got != want)
        .take(8)
        .collect()
}

/// Fill the `Height` column with its record indices.
///
/// # Safety
///
/// `col` is a column of `RECORDS` records and no pass is running.
unsafe fn fill(col: *mut Height) {
    for i in 0 .. RECORDS {
        // SAFETY: inside the reservation, per the contract above.
        unsafe { col.add(i).write(Height(i as u32)) };
    }
}

// The writer stands ahead of the reader, and the carrier has an accumulator.
// The writer of the first slice is still at its last record when the core of
// the second slice reaches the reader, which reads that record across the edge.
#[test]
fn a_reach_read_waits_for_the_writer_ahead_of_it_on_every_core() {
    let scheduler = Scheduler::builder()
        .with(Accum::<Noted>::new())
        .with(Col::new())
        .with(Column::<Seen>::new())
        .with(SlowRaise)
        .with(Spread)
        .with(Note)
        .build(store(), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let pool = FixedWorkers::<4>;
    let mut scheduler = core::pin::pin!(scheduler);
    let bindings = scheduler.as_ref().get_ref().__bindings();
    let seen = bindings.__ptr().as_ptr();
    let col = bindings.__tail().__ptr().as_ptr();
    // SAFETY: the column's reservation, before the first pass.
    unsafe { fill(col) };

    assert!(matches!(
        scheduler.as_mut().run_parallel(&pool),
        Outcome::Ok(())
    ));

    let raised: Vec<u32> = (0 .. RECORDS as u32).map(|i| i + 1).collect();
    // SAFETY: the pass is over and every worker is parked.
    let bad = unsafe { wrong(seen, &spread_of(&raised)) };
    assert!(
        bad.is_empty(),
        "neighbours read before their writer landed: {bad:?}"
    );
}

// The same carrier without the accumulator takes the phase loop, which has
// kept the boundary all along. The control for the test above.
#[test]
fn the_phase_loop_keeps_the_boundary_ahead_of_a_reach_read() {
    let scheduler = Scheduler::builder()
        .with(Col::new())
        .with(Column::<Seen>::new())
        .with(SlowRaise)
        .with(Spread)
        .build(store(), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let pool = FixedWorkers::<4>;
    let mut scheduler = core::pin::pin!(scheduler);
    let bindings = scheduler.as_ref().get_ref().__bindings();
    let seen = bindings.__ptr().as_ptr();
    let col = bindings.__tail().__ptr().as_ptr();
    // SAFETY: the column's reservation, before the first pass.
    unsafe { fill(col) };

    assert!(matches!(
        scheduler.as_mut().run_parallel(&pool),
        Outcome::Ok(())
    ));

    let raised: Vec<u32> = (0 .. RECORDS as u32).map(|i| i + 1).collect();
    // SAFETY: the pass is over and every worker is parked.
    let bad = unsafe { wrong(seen, &spread_of(&raised)) };
    assert!(
        bad.is_empty(),
        "neighbours read before their writer landed: {bad:?}"
    );
}

// A writer listed behind a reader of its column never runs behind it: the
// reader waits on every writer of the column wherever the writer is listed, so
// such a registration is no topological sequence and `build` refuses it. A
// reach read therefore has no writer of its column behind it on any path, and
// the boundary ahead of the reader is the only one it needs.
#[test]
fn a_writer_listed_behind_a_reach_read_of_its_column_is_refused() {
    let plain = Scheduler::builder()
        .with(Col::new())
        .with(Column::<Seen>::new())
        .with(Spread)
        .with(Raise)
        .build(store(), USize(RECORDS));
    assert!(matches!(
        plain,
        Outcome::Err(BuildError::NonTopologicalRegistration {
            producer: USize(1),
            consumer: USize(0),
            ..
        })
    ));

    let with_an_accumulator = Scheduler::builder()
        .with(Accum::<Noted>::new())
        .with(Col::new())
        .with(Column::<Seen>::new())
        .with(Spread)
        .with(Raise)
        .with(Note)
        .build(store(), USize(RECORDS));
    assert!(matches!(
        with_an_accumulator,
        Outcome::Err(BuildError::NonTopologicalRegistration {
            producer: USize(1),
            consumer: USize(0),
            ..
        })
    ));
}
