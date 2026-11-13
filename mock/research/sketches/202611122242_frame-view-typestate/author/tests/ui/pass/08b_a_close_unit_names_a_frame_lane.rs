// Close unit. Names a frame lane among its stores, to read the frame through
// them; the twin names a landed store.
#![allow(unused)]
use author::{Landings, Pos, World};
use engine::{CloseUnit, Frame, Landed};
use notko_hlist::{Cons, Empty};

struct Reacher;

impl CloseUnit<World> for Reacher {
    type Stores = Cons<Landings, Empty>;

    fn close(landed: Landed<'_, World, Self::Stores>) {}
}

fn main() {
    Frame::<World, 2>::new().close(Reacher);
}
