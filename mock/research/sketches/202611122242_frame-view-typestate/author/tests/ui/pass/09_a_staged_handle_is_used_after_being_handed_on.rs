// Block kernel. Hands its staged handle on, then uses the old binding; the
// twin uses the new one.
#![allow(unused)]
use author::{Pos, World};
use engine::{Block, BlockKernel, Frame, Staged, View};
use notko_hlist::{Cons, Empty};

struct Relay;

impl BlockKernel<World> for Relay {
    type Reads = Cons<Pos, Empty>;
    type Writes = Cons<Pos, Empty>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {
        let (mut pos, ()) = out;
        let mut handed = pos;
        for r in view.rows() {
            handed.set(r, 1);
        }
    }
}

fn main() {
    Frame::<World, 2>::new().step(Block(Relay));
}
