// Value kernel. Its read list names a landed store; the twin names a frame
// lane.
#![allow(unused)]
use author::{Landings, Pos, World};
use engine::{Frame, Row, Value, ValueKernel, Values, View};
use notko_hlist::{Cons, Empty};

struct Tally;

impl ValueKernel<World> for Tally {
    type Reads = Cons<Pos, Empty>;
    type Writes = Cons<Pos, Empty>;

    fn row(view: &View<'_, World, Self::Reads>, row: Row) -> Values<Self::Writes> {
        (0, ())
    }
}

fn main() {
    Frame::<World, 2>::new().step(Value(Tally));
}
