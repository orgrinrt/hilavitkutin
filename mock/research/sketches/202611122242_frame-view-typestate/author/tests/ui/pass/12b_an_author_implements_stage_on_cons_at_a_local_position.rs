// An author implements `Stage` for the engine's own `Cons`, at a position
// list of its own, so the orphan rule admits the impl, stating a mask of its
// own. The twin goes the one way an author has: a `BlockKernel` writing the
// same list, whose mask the engine folds.
#![allow(unused)]
use author::{Pos, World};
use engine::{Block, BlockKernel, Frame, Staged, View};
use notko_hlist::{Cons, Empty};

struct Writer;

impl BlockKernel<World> for Writer {
    type Reads = Empty;
    type Writes = Cons<Pos, Empty>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {}
}

fn main() {
    Frame::<World, 2>::new().step(Block(Writer));
}
