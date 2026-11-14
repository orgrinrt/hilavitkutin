//! Nine units in the value form, in the sequence they are registered and run.
//!
//! Each is a unit struct with the three impls the api asks of a value-form
//! unit and nothing else: `BuilderInput`, `WorkUnit` with `execute` as the one
//! line the api documents, and `RecordOp` with the function from the values
//! read to the values written.

use arvo::Bool;
use hilavitkutin::dispatch::engine_ctx::CtxFor;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Accum, Column, Virtual};
use hilavitkutin_api::values::{ReadVals, WriteVals};
use hilavitkutin_api::work_unit::{HasSchedule, On, WorkUnit};
use hilavitkutin_api::{Differs, Reach, RecordOp, Written, rec};
use notko::Maybe;

use crate::values::{
    Alarms,
    Ash,
    Crossed,
    Edge,
    Field,
    Glow,
    Heat,
    LIMIT,
    Level,
    Num,
    ONE,
    Overheat,
    Peak,
    Pool,
    Pos,
    THRESHOLD,
    Vel,
    WIDE,
    ZERO,
    higher,
};

/// The position plus the velocity, in place.
#[derive(Copy, Clone)]
pub struct Drift;

impl BuilderInput for Drift {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Drift {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|r| self.apply(r));
    }
}

impl RecordOp for Drift {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let pos = read.get(Column::<Pos>::new());
        let vel = read.get(Column::<Vel>::new());
        rec![Pos(pos.0 + vel.0)]
    }
}

/// The heat of a place plus that of its four neighbours, on two axes.
#[derive(Copy, Clone)]
pub struct Spread;

impl BuilderInput for Spread {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Spread {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Reach<Field, WIDE>, Cons<Column<Edge>, Empty>>;
    type Write = Cons<Field, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|r| self.apply(r));
    }
}

/// The heat of a neighbour, or none where the records end.
const fn heat_of(side: Maybe<Heat>) -> Num {
    match side {
        Maybe::Is(heat) => heat.0,
        Maybe::Isnt => ZERO,
    }
}

impl RecordOp for Spread {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let around = read.get(Reach::<Field, WIDE>::new());
        let edge = read.get(Column::<Edge>::new());
        let east = if edge.east.0 { ZERO } else { heat_of(around.after::<ONE>()) };
        let west = if edge.west.0 { ZERO } else { heat_of(around.before::<ONE>()) };
        let north = heat_of(around.after::<WIDE>());
        let south = heat_of(around.before::<WIDE>());
        rec![Heat(around.here().0 + east + west + north + south)]
    }
}

/// An append where the position crossed the threshold in this frame.
#[derive(Copy, Clone)]
pub struct Cross;

impl BuilderInput for Cross {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Cross {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Accum<Crossed>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|r| self.apply(r));
    }
}

impl RecordOp for Cross {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let to = read.get(Column::<Pos>::new()).0;
        let from = to - read.get(Column::<Vel>::new()).0;
        if from.to_raw() < THRESHOLD.to_raw() && to.to_raw() >= THRESHOLD.to_raw() {
            rec![Maybe::Is(Crossed(to))]
        } else {
            rec![Maybe::Isnt]
        }
    }
}

/// Fires where the heat the frame began with is at or over the limit.
#[derive(Copy, Clone)]
pub struct Alarm;

impl BuilderInput for Alarm {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Alarm {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Field, Empty>;
    type Write = Cons<Virtual<Overheat>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|r| self.apply(r));
    }
}

impl RecordOp for Alarm {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let heat = read.get(Field::new()).0;
        rec![Bool(heat.to_raw() >= LIMIT.to_raw())]
    }
}

/// Runs in the frames `Overheat` fired in: the count plus one.
#[derive(Copy, Clone)]
pub struct Tally;

impl BuilderInput for Tally {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl HasSchedule for Tally {
    type Sched = On<Overheat>;
}

impl WorkUnit<On<Overheat>> for Tally {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write, On<Overheat>>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Alarms>, Empty>;
    type Write = Cons<Written<Column<Alarms>>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|r| self.apply(r));
    }
}

impl RecordOp<On<Overheat>> for Tally {
    fn apply(
        &self,
        read: &ReadVals<'_, <Self as WorkUnit<On<Overheat>>>::Read>,
    ) -> WriteVals<<Self as WorkUnit<On<Overheat>>>::Write> {
        let so_far = read.get(Column::<Alarms>::new()).0;
        rec![Alarms(so_far + Num::from_raw(1))]
    }
}

/// The highest of a record's level and its two neighbours'.
#[derive(Copy, Clone)]
pub struct Smooth;

impl BuilderInput for Smooth {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Smooth {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Reach<Pool, ONE>, Empty>;
    type Write = Cons<Differs<Pool>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|r| self.apply(r));
    }
}

/// The level of a neighbour, or none where the records end.
const fn level_of(side: Maybe<Level>) -> Num {
    match side {
        Maybe::Is(level) => level.0,
        Maybe::Isnt => ZERO,
    }
}

impl RecordOp for Smooth {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let around = read.get(Reach::<Pool, ONE>::new());
        let beside = higher(
            level_of(around.before::<ONE>()),
            level_of(around.after::<ONE>()),
        );
        rec![Level(higher(around.here().0, beside))]
    }
}

/// The higher of the peak and the level the frame began with.
#[derive(Copy, Clone)]
pub struct Crest;

impl BuilderInput for Crest {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Crest {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Pool, Cons<Column<Peak>, Empty>>;
    type Write = Cons<Differs<Column<Peak>>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|r| self.apply(r));
    }
}

impl RecordOp for Crest {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let level = read.get(Pool::new()).0;
        let peak = read.get(Column::<Peak>::new()).0;
        rec![Peak(higher(peak, level))]
    }
}

/// Twice the position, as `Drift` left it in this frame.
#[derive(Copy, Clone)]
pub struct Warm;

impl BuilderInput for Warm {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Warm {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Empty>;
    type Write = Cons<Column<Glow>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|r| self.apply(r));
    }
}

impl RecordOp for Warm {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let pos = read.get(Column::<Pos>::new()).0;
        rec![Glow(pos + pos)]
    }
}

/// The glow plus the limit, as `Warm` left it in this frame.
#[derive(Copy, Clone)]
pub struct Fade;

impl BuilderInput for Fade {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Fade {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Glow>, Empty>;
    type Write = Cons<Column<Ash>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|r| self.apply(r));
    }
}

impl RecordOp for Fade {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let glow = read.get(Column::<Glow>::new()).0;
        rec![Ash(glow + LIMIT)]
    }
}
