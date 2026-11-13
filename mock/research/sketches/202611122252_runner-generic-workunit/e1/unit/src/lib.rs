//! The author's unit under E1: three kernels, each generic over the runner and handed
//! the runner's context for its declaration. This file does not change under any runner.

#![no_std]

use e1_vocab::{Cons, Ctx, Decl, Elem, Empty, Off, Reach, Runner, Vals};

pub struct Pos;
pub struct Vel;
pub struct Lag;
pub struct Heat;

impl Elem for Pos {
    type Value = i32;
}
impl Elem for Vel {
    type Value = i32;
}
impl Elem for Lag {
    type Value = i32;
}
impl Elem for Heat {
    type Value = i32;
}

/// Every member the unit's world holds.
pub type All = Cons<Pos, Cons<Vel, Cons<Lag, Cons<Heat, Empty>>>>;

/// A kernel that reaches nowhere.
pub struct Still;
impl Reach for Still {
    const EAST: i32 = 0;
    const NORTH: i32 = 0;
}

/// (a) the control: the values of its reads in, the values of its writes out.
pub type IntegrateReads = Cons<Pos, Cons<Vel, Empty>>;
pub type IntegrateWrites = Cons<Pos, Empty>;

pub fn integrate(r: &Vals<IntegrateReads>, w: &mut Vals<IntegrateWrites>) {
    w.set(Pos, r.get(Pos).wrapping_add(r.get(Vel)));
}

/// (b) a window: the neighbour on the two-axis key at an offset computed from `Lag`,
/// within reach.
pub type SmoothReads = Cons<Lag, Cons<Pos, Empty>>;
pub type SmoothWrites = Cons<Heat, Empty>;

pub struct SmoothReach;
impl Reach for SmoothReach {
    const EAST: i32 = 3;
    const NORTH: i32 = 2;
}

pub struct Smooth;
impl Decl for Smooth {
    type Event = ();
    type Reach = SmoothReach;
    type Reads = SmoothReads;
}

pub fn smooth<X: Runner>(cx: &X::Cx<'_, Smooth>, w: &mut Vals<SmoothWrites>) {
    let lag = cx.get(Lag);
    let span = 2 * SmoothReach::EAST + 1;
    let east = lag.rem_euclid(span) - SmoothReach::EAST;
    let north = (lag / span).rem_euclid(2 * SmoothReach::NORTH + 1) - SmoothReach::NORTH;
    let here = cx.get(Pos);
    let there = cx.at(Pos, Off {
        east,
        north,
    });
    w.set(Heat, here.wrapping_sub(there).wrapping_abs());
}

/// (c) an emitter: an event when this step carries `Pos` across the threshold.
pub type CrossReads = Cons<Pos, Cons<Vel, Empty>>;

pub const THRESHOLD: i32 = 1000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Crossed {
    pub from: i32,
    pub to:   i32,
}

pub struct Cross;
impl Decl for Cross {
    type Event = Crossed;
    type Reach = Still;
    type Reads = CrossReads;
}

pub fn cross<X: Runner>(cx: &mut X::Cx<'_, Cross>) {
    let from = cx.get(Pos);
    let to = from.wrapping_add(cx.get(Vel));
    if from < THRESHOLD && to >= THRESHOLD {
        cx.emit(Crossed {
            from,
            to,
        });
    }
}
