// Block kernel. Reads its staged handle; the twin reads the view instead.
#![allow(unused)]
use author::{Pos, World};
use engine::{Block, BlockKernel, Frame, Staged, View};
use notko_hlist::{Cons, Empty};

struct Peek;

impl BlockKernel<World> for Peek {
    type Reads = Cons<Pos, Empty>;
    type Writes = Cons<Pos, Empty>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {
        let (pos, ()) = out;
        for r in view.rows() {
            let _ = view.get(Pos, r);
        }
    }
}

fn main() {
    Frame::<World, 2>::new().step(Block(Peek));
}
