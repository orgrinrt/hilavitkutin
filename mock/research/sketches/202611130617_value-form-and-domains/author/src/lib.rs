//! Two units in the value form, written against the api alone. This file does
//! not change under either runner.

#![no_std]

use api::{
    Accum,
    Axis,
    Buffered,
    BuilderInput,
    Column,
    Cons,
    Domain,
    Edge,
    Elem,
    Empty,
    Near,
    RecordDispatch,
    RecordOp,
    Resource,
    Shift,
    Vals,
    Virtual,
    rec,
};

// ---------------------------------------------------------------------------
// A domain of two axes, keyed north first.
// ---------------------------------------------------------------------------

pub struct North;
impl Elem for North {
    type Value = usize;
}
impl Axis for North {}

pub struct East;
impl Elem for East {
    type Value = usize;
}
impl Axis for East {}

pub type East1 = Cons<Shift<East, 1>, Empty>;
pub type West1 = Cons<Shift<East, -1>, Empty>;
pub type North1 = Cons<Shift<North, 1>, Empty>;
pub type South1 = Cons<Shift<North, -1>, Empty>;
/// One east and one north at once: an offset on two axes.
pub type NorthEast1 = Cons<Shift<North, 1>, Cons<Shift<East, 1>, Empty>>;

pub struct Plane;
impl Domain for Plane {
    type Axes = Cons<North, Cons<East, Empty>>;
    type Reach = Cons<East1, Cons<West1, Cons<North1, Cons<South1, Cons<NorthEast1, Empty>>>>>;
}

// ---------------------------------------------------------------------------
// Diffuse: a neighbour reader on two axes, over a frame-buffered store.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Heat(pub i64);

/// Past the plane the heat reads as none.
pub struct Cold;
impl Edge<Heat> for Cold {
    const OUTSIDE: Heat = Heat(0);
}

pub type HeatAt = Buffered<Heat, Plane>;
pub type Rate = Resource<i64>;

pub const HERE: HeatAt = Buffered::new();
pub const EAST: Near<HeatAt, East1, Cold> = Near::new();
pub const WEST: Near<HeatAt, West1, Cold> = Near::new();
pub const NORTH: Near<HeatAt, North1, Cold> = Near::new();
pub const SOUTH: Near<HeatAt, South1, Cold> = Near::new();
pub const NORTH_EAST: Near<HeatAt, NorthEast1, Cold> = Near::new();
pub const RATE: Rate = Resource::new();

/// The new heat at a place: the old, plus east and north, minus west and
/// south, plus twice north-east, plus the rate.
pub struct Diffuse;

impl BuilderInput for Diffuse {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Diffuse {
    type Read = Cons<
        HeatAt,
        Cons<
            Near<HeatAt, East1, Cold>,
            Cons<
                Near<HeatAt, West1, Cold>,
                Cons<
                    Near<HeatAt, North1, Cold>,
                    Cons<
                        Near<HeatAt, South1, Cold>,
                        Cons<Near<HeatAt, NorthEast1, Cold>, Cons<Rate, Empty>>,
                    >,
                >,
            >,
        >,
    >;
    type Write = Cons<HeatAt, Empty>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        let sum = r.get(HERE).0 + r.get(EAST).0 + r.get(NORTH).0 - r.get(WEST).0 - r.get(SOUTH).0
            + 2 * r.get(NORTH_EAST).0
            + r.get(RATE);
        rec![Heat(sum)]
    }
}

// ---------------------------------------------------------------------------
// Drift: plain columns of the default domain, written in place, with an
// append and a fire.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pos(pub i64);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vel(pub i64);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Crossed {
    pub from: i64,
    pub to:   i64,
}
pub struct Alarm;

pub const POS: Column<Pos> = Column::new();
pub const VEL: Column<Vel> = Column::new();
pub const CROSSED: Accum<Crossed> = Accum::new();
pub const ALARM: Virtual<Alarm> = Virtual::new();

pub const THRESHOLD: i64 = 100;

/// Moves a position by its velocity, and reports a crossing of the threshold.
pub struct Drift;

impl BuilderInput for Drift {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Drift {
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Cons<Accum<Crossed>, Cons<Virtual<Alarm>, Empty>>>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        let from = r.get(POS).0;
        let to = from + r.get(VEL).0;
        let crossed = from < THRESHOLD && to >= THRESHOLD;
        rec![
            Pos(to),
            if crossed {
                Some(Crossed {
                    from,
                    to,
                })
            } else {
                None
            },
            crossed
        ]
    }
}
