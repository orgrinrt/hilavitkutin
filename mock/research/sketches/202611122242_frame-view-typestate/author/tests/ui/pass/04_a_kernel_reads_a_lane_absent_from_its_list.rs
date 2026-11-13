// Value kernel. Reads a lane its read list does not name; the twin reads one it
// does.
#![allow(unused)]
use author::{Pos, Vel, World};
use engine::{Frame, Row, Value, ValueKernel, Values, View};
use notko_hlist::{Cons, Empty};

struct Stray;

impl ValueKernel<World> for Stray {
    type Reads = Cons<Pos, Empty>;
    type Writes = Cons<Pos, Empty>;

    fn row(view: &View<'_, World, Self::Reads>, row: Row) -> Values<Self::Writes> {
        (view.get(Pos, row), ())
    }
}

fn main() {
    Frame::<World, 2>::new().step(Value(Stray));
}
