// Value kernel. As 04, with two lanes in the read list, so the read's lane is
// not inferred from the list; the twin reads one the list names.
#![allow(unused)]
use author::{Heat, Pos, Vel, World};
use engine::{Frame, Row, Value, ValueKernel, Values, View};
use notko_hlist::{Cons, Empty};

struct Stray;

impl ValueKernel<World> for Stray {
    type Reads = Cons<Pos, Cons<Heat, Empty>>;
    type Writes = Cons<Pos, Empty>;

    fn row(view: &View<'_, World, Self::Reads>, row: Row) -> Values<Self::Writes> {
        (view.get(Heat, row), ())
    }
}

fn main() {
    Frame::<World, 2>::new().step(Value(Stray));
}
