// Block kernel and value kernel in one step. Both write lists name `Pos`; the
// twin's value kernel writes `Heat`. Position-mask form, fired in `main`.
#![allow(unused)]
use author::{Heat, Pos, Vel, World};
use engine::{Block, BlockKernel, Frame, Row, Staged, Value, ValueKernel, Values, View};
use notko_hlist::{Cons, Empty};

struct Mover;

impl BlockKernel<World> for Mover {
    type Reads = Cons<Pos, Empty>;
    type Writes = Cons<Pos, Cons<Vel, Empty>>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {}
}

struct Other;

impl ValueKernel<World> for Other {
    type Reads = Cons<Heat, Empty>;
    type Writes = Cons<Heat, Empty>;

    fn row(view: &View<'_, World, Self::Reads>, row: Row) -> Values<Self::Writes> {
        (0, ())
    }
}

fn main() {
    Frame::<World, 2>::new().step2(Block(Mover), Value(Other));
}
