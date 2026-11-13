// Block kernel. Its write list names one lane twice; the twin names two lanes.
// Position-mask form: the check is the runner's inline `const` block, which
// fires where the kernel is instantiated, here in `main`.
#![allow(unused)]
use author::{Pos, Vel, World};
use engine::{Block, BlockKernel, Frame, Staged, View};
use notko_hlist::{Cons, Empty};

struct Twice;

impl BlockKernel<World> for Twice {
    type Reads = Cons<Pos, Empty>;
    type Writes = Cons<Pos, Cons<Vel, Empty>>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {}
}

fn main() {
    Frame::<World, 2>::new().step(Block(Twice));
}
