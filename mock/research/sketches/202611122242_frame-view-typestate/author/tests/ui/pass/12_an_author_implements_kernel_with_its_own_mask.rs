// An author implements the runner's `Kernel` itself, stating a mask of its
// own. The twin goes the one way an author has: a `BlockKernel`, wrapped in
// `Block`, whose mask the engine folds from the write list.
#![allow(unused)]
use author::{Pos, World};
use engine::{Block, BlockKernel, Frame, Staged, View};
use notko_hlist::{Cons, Empty};

struct Liar;

impl BlockKernel<World> for Liar {
    type Reads = Empty;
    type Writes = Cons<Pos, Empty>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {}
}

fn main() {
    Frame::<World, 2>::new().step(Block(Liar));
}
