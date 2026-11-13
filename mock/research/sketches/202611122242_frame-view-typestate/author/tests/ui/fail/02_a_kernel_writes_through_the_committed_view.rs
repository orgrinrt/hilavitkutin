// Block kernel. Writes through the committed view; the twin writes through its
// staged handle.
#![allow(unused)]
use author::{Pos, World};
use engine::{Block, BlockKernel, Frame, Staged, View};
use notko_hlist::{Cons, Empty};

struct Scribble;

impl BlockKernel<World> for Scribble {
    type Reads = Cons<Pos, Empty>;
    type Writes = Cons<Pos, Empty>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {
        let (mut pos, ()) = out;
        for r in view.rows() {
            view.set(r, 1);
        }
    }
}

fn main() {
    Frame::<World, 2>::new().step(Block(Scribble));
}
