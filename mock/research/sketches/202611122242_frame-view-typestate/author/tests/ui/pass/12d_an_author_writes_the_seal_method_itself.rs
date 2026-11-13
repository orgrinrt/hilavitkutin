// An author implements `Kernel` for `Block` of a kernel of its own, as 12c,
// and writes the seal method too, naming the engine's token by its path. The
// twin goes the one way an author has: a `BlockKernel`, wrapped in `Block`,
// whose mask the engine folds.
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
