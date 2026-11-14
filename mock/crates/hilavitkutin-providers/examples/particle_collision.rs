//! `particle_collision`: steps particles along a line and reflects them off a
//! bound.
//!
//! Every particle starts at the origin with a speed of one to four, by record.
//! `Step` adds each velocity to its position, then negates the velocity of a
//! particle that ended the step past the bound. A particle past the bound
//! keeps its position, so it travels back the next frame. `Census` runs after
//! it and appends how many particles are moving backwards. Positions and
//! velocities are fixed-point, and the records span more than one window of
//! the position column.
//!
//! The scheduler is built over the `hilavitkutin` crate's `mmap` memory
//! provider. The example runs twelve frames, reads both columns and the count
//! back, and checks every record against the position and velocity worked out
//! by hand for its speed.
//!
//! Run it with `cargo run -p hilavitkutin-providers --example particle_collision`.

use arvo::{Hot, Signed, USize};
use hilavitkutin::OsMemoryProvider;
use hilavitkutin::dispatch::engine_ctx::CtxFor;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi,
    ColumnReaderApi,
    EachApi,
    HasAccumWriter,
    HasColumnReader,
    HasEach,
    HasReduce,
    ReduceApi,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::rec;
use hilavitkutin_api::store::{Accum, Column, StartsFromDefault};
use hilavitkutin_api::work_unit::WorkUnit;
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

/// A coordinate or a speed: signed, 15 integer bits and 16 fractional bits.
type Length = Signed<15, 16, Hot>;

/// How many particles the example holds. More than one window of a column.
const RECORDS: usize = 10_000;
/// How many frames the example runs.
const FRAMES: usize = 12;
/// The fractional bits of a `Length`, as a shift of the raw container.
const FRAC: u32 = 16;
/// The bound, in whole units. A particle past it is reflected.
const BOUND: i32 = 20;

/// A position along the line.
#[derive(Copy, Clone, PartialEq, Default)]
struct Pos(Length);

/// A velocity along the line.
#[derive(Copy, Clone, PartialEq, Default)]
struct Vel(Length);

/// How many particles are moving backwards at the end of a frame.
#[derive(Copy, Clone, PartialEq)]
struct Reflected(USize);

impl Default for Reflected {
    fn default() -> Self {
        Self(USize(0))
    }
}

/// Moves each particle, and reflects it where it ends past the bound.
#[derive(Copy, Clone)]
struct Step;

impl BuilderInput for Step {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Step {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|read| {
            let pos = read.get(Column::<Pos>::new()).0;
            let vel = read.get(Column::<Vel>::new()).0;
            let moved = pos + vel;
            let turned =
                if moved.to_raw() > BOUND << FRAC { Length::from_raw(0) - vel } else { vel };
            rec![Pos(moved), Vel(turned)]
        });
    }
}

/// Appends how many particles have a negative velocity.
#[derive(Copy, Clone)]
struct Census;

impl BuilderInput for Census {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Census {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Vel>, Empty>;
    type Write = Cons<Accum<Reflected>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let backwards = ReduceApi::run(ctx.reduce(), 0_usize, |seen, i| {
            // SAFETY: `i` is a record of the morsel, `Step` wrote it earlier
            // in the frame, and the plan gives no unit a write to it while
            // this one runs.
            let vel = unsafe { ctx.reader().read::<Vel, _>(i) };
            if vel.0.to_raw() < 0 { seen + 1 } else { seen }
        });
        ctx.accums()
            .append::<Reflected, _>(Reflected(USize(backwards)));
    }
}

/// The column storage over the `mmap` provider. The return type leaves the
/// column cap at its default, which anchors it at the `build` call.
fn store() -> ArenaColumnStorage<OsMemoryProvider> {
    ArenaColumnStorage::new(OsMemoryProvider::new())
}

/// The speed of particle `i`, in whole units per frame.
fn speed_of(i: usize) -> i32 {
    (i % 4) as i32 + 1
}

/// The position and the velocity, in whole units, that a particle of the given
/// speed holds after twelve frames from the origin with a bound of twenty.
///
/// Speed one reaches twelve and never passes the bound. Speed two passes it at
/// frame eleven (22), turns, and is back at twenty. Speed three passes it at
/// frame seven (21), turns, and covers five frames down to six. Speed four
/// passes it at frame six (24), turns, and covers six frames down to zero.
fn after_twelve_frames(speed: i32) -> (i32, i32) {
    match speed {
        1 => (12, 1),
        2 => (20, -2),
        3 => (6, -3),
        _ => (0, -4),
    }
}

fn main() {
    let mut scheduler = Scheduler::builder()
        .with(Column::<Pos>::start())
        .with(Column::<Vel>::start())
        .with(Accum::<Reflected>::new())
        .with(Step)
        .with(Census)
        .build(store(), USize(RECORDS))
        .unwrap_or_else(|refused| panic!("the particles build: {refused:?}"));

    // The last store registered leads the bindings.
    let reflections = scheduler.__bindings();
    let vels = reflections.__tail();
    let poss = vels.__tail();
    let pos_base = poss.__ptr().as_ptr();
    let vel_base = vels.__ptr().as_ptr();
    for i in 0 .. RECORDS {
        // SAFETY: each column holds `RECORDS` records and no frame is running.
        unsafe {
            pos_base.add(i).write(Pos(Length::from_raw(0)));
            vel_base
                .add(i)
                .write(Vel(Length::from_raw(speed_of(i) << FRAC)));
        }
    }

    for _ in 0 .. FRAMES {
        // Both columns are state the unit rewrites and the host seeded, so
        // each frame marks them changed and every unit runs.
        scheduler.mark_dirty::<Column<Pos>, _>();
        scheduler.mark_dirty::<Column<Vel>, _>();
        assert!(matches!(scheduler.run(), Outcome::Ok(())));
    }

    let mut backwards = 0_usize;
    for i in 0 .. RECORDS {
        // SAFETY: the frames are over and each column holds `RECORDS` records.
        let (pos, vel) = unsafe { (pos_base.add(i).read().0, vel_base.add(i).read().0) };
        let (want_pos, want_vel) = after_twelve_frames(speed_of(i));
        assert_eq!(pos.to_raw(), want_pos << FRAC, "particle {i} position");
        assert_eq!(vel.to_raw(), want_vel << FRAC, "particle {i} velocity");
        if want_vel < 0 {
            backwards += 1;
        }
    }

    // Speeds two, three and four are reflected: three particles in every four.
    assert_eq!(
        backwards,
        RECORDS / 4 * 3,
        "three speeds in four are reflected"
    );
    let reflections = scheduler.__bindings();
    assert_eq!(reflections.__len_cell().get().0, 1, "one census per frame");
    // SAFETY: the frame appended one `Reflected` at the base.
    let census = unsafe { reflections.__ptr().as_ptr().read() };
    assert_eq!(
        census.0.0, backwards,
        "the census counts the reflected particles"
    );

    println!("particle_collision: {RECORDS} particles, {FRAMES} frames, {backwards} reflected");
}
