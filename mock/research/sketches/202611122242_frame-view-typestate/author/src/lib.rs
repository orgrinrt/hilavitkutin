//! Sketch S3, the author side: lanes, a registry, one kernel of each form and
//! a close unit, written against `engine` the way a game crate would.
//!
//! No feature line: everything the sketch checks is the engine's, and this
//! crate only declares and instantiates.

#![no_std]

use engine::{
    BlockKernel,
    CloseUnit,
    FrameLane,
    Landed,
    LandedStore,
    Lane,
    Row,
    Staged,
    ValueKernel,
    Values,
    View,
};
use notko_hlist::{Cons, Empty};

/// Where each row is, one fixed-point step per unit.
pub struct Pos;
/// How far each row moves per step.
pub struct Vel;
/// What each row holds of heat, which halves every step.
pub struct Heat;
/// How many steps have landed, written by the close only.
pub struct Landings;

impl Lane for Pos {
    type Item = i64;

    const INIT: i64 = 0;
}
impl Lane for Vel {
    type Item = i64;

    const INIT: i64 = 3;
}
impl Lane for Heat {
    type Item = i64;

    const INIT: i64 = 64;
}
impl Lane for Landings {
    type Item = i64;

    const INIT: i64 = 0;
}

impl FrameLane for Pos {}
impl FrameLane for Vel {}
impl FrameLane for Heat {}
impl LandedStore for Landings {}

/// Every lane the frame holds, in registry order.
pub type World = Cons<Pos, Cons<Vel, Cons<Heat, Cons<Landings, Empty>>>>;

/// A block kernel: moves every row by its velocity.
pub struct Drift;

impl BlockKernel<World> for Drift {
    type Reads = Cons<Pos, Cons<Vel, Empty>>;
    type Writes = Cons<Pos, Empty>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {
        let (mut pos, ()) = out;
        for r in view.rows() {
            pos.set(r, view.get(Pos, r) + view.get(Vel, r));
        }
    }
}

/// A value kernel: halves every row's heat.
pub struct Cool;

impl ValueKernel<World> for Cool {
    type Reads = Cons<Heat, Empty>;
    type Writes = Cons<Heat, Empty>;

    fn row(view: &View<'_, World, Self::Reads>, row: Row) -> Values<Self::Writes> {
        (view.get(Heat, row) / 2, ())
    }
}

/// A close unit: counts the landings.
pub struct Count;

impl CloseUnit<World> for Count {
    type Stores = Cons<Landings, Empty>;

    fn close(mut landed: Landed<'_, World, Self::Stores>) {
        for r in landed.rows() {
            let n = landed.get(Landings, r);
            landed.set(Landings, r, n + 1);
        }
    }
}
