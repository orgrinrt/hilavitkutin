//! Author units in the value form, written as a consumer writes them.
//!
//! Each names the engine's context type in `Ctx`, as every unit does today,
//! and writes `execute` as the one line the api documents. Nothing here knows
//! of the runner in the crate root.

use arvo::{Bool, Cap, Hot, Uint};
use hilavitkutin::dispatch::engine_ctx::CtxFor;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Accum, Buffered, Column, Exchange, Virtual};
use hilavitkutin_api::values::{ReadVals, WriteVals};
use hilavitkutin_api::work_unit::{HasSchedule, On, WorkUnit};
use hilavitkutin_api::{Reach, RecordOp, Written, rec};
use notko::Maybe;

/// The number every value here is made of.
pub type Num = Uint<32, Hot>;

/// How many records the plane is wide: a step north is this many records on.
pub const WIDE: Cap = arvo_tensor::cap(8);
/// A step east.
pub const ONE: Cap = arvo_tensor::cap(1);

/// Where a `Pos` counts as having crossed.
pub const THRESHOLD: Num = Num::from_raw(40);
/// The heat at which the plane is overheated.
pub const LIMIT: Num = Num::from_raw(500);

const ZERO: Num = Num::from_raw(0);

#[derive(Copy, Clone, PartialEq)]
pub struct Pos(pub Num);

#[derive(Copy, Clone, PartialEq)]
pub struct Vel(pub Num);

#[derive(Copy, Clone, PartialEq)]
pub struct Heat(pub Num);

/// Which sides of the plane a record lies on.
#[derive(Copy, Clone, PartialEq)]
pub struct Edge {
    pub west: Bool,
    pub east: Bool,
}

/// A record whose `Pos` crossed the threshold in the frame.
#[derive(Copy, Clone, PartialEq)]
pub struct Crossed(pub Num);

/// How many frames found the plane overheated.
#[derive(Copy, Clone, PartialEq)]
pub struct Alarms(pub Num);

/// Fired in a frame that began with a record at or over the limit.
pub struct Overheat;

type Field = Buffered<Heat, Exchange>;

/// The control: the values of its reads in, the values of its writes out.
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

/// A window on two axes: each record takes in its four neighbours.
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

fn part(side: Maybe<Heat>) -> Num {
    match side {
        Maybe::Is(heat) => heat.0,
        Maybe::Isnt => ZERO,
    }
}

impl RecordOp for Spread {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let around = read.get(Reach::<Field, WIDE>::new());
        let edge = read.get(Column::<Edge>::new());
        let east = if edge.east.0 { ZERO } else { part(around.after::<ONE>()) };
        let west = if edge.west.0 { ZERO } else { part(around.before::<ONE>()) };
        let north = part(around.after::<WIDE>());
        let south = part(around.before::<WIDE>());
        rec![Heat(around.here().0 + east + west + north + south)]
    }
}

/// An emitter into an accumulator: one value for each record that crossed.
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

/// An emitter of a virtual: fires where a record is at or over the limit.
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

/// Runs in the frames `Overheat` fired in, and counts them at every record.
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
