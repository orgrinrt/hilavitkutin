//! The author's unit under E2: three kernels, each a function of the handles it takes.
//! This file does not change under any runner.

#![no_std]

use e2_vocab::{Cons, Elem, Emit, Empty, Off, Reach, Reads, Vals, Window};

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

pub fn smooth(
    r: &impl Reads<SmoothReads>,
    n: &impl Window<SmoothReads, SmoothReach>,
    w: &mut Vals<SmoothWrites>,
) {
    let lag = r.get(Lag);
    let span = 2 * SmoothReach::EAST + 1;
    let east = lag.rem_euclid(span) - SmoothReach::EAST;
    let north = (lag / span).rem_euclid(2 * SmoothReach::NORTH + 1) - SmoothReach::NORTH;
    let here = r.get(Pos);
    let there = n.at(Pos, Off {
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

pub fn cross(r: &impl Reads<CrossReads>, e: &mut impl Emit<Crossed>) {
    let from = r.get(Pos);
    let to = from.wrapping_add(r.get(Vel));
    if from < THRESHOLD && to >= THRESHOLD {
        e.emit(Crossed {
            from,
            to,
        });
    }
}
