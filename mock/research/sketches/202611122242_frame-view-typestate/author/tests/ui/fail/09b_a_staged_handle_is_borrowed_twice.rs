// Block kernel. Holds two exclusive borrows of one staged handle at once; the
// twin borrows two handles once each.
#![allow(unused)]
use author::{Pos, Vel, World};
use engine::{Block, BlockKernel, Frame, Staged, View};
use notko_hlist::{Cons, Empty};

struct Both;

impl BlockKernel<World> for Both {
    type Reads = Cons<Pos, Empty>;
    type Writes = Cons<Pos, Cons<Vel, Empty>>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {
        let (mut pos, (mut vel, ())) = out;
        for r in view.rows() {
            let a = &mut pos;
            let b = &mut pos;
            a.set(r, 1);
            b.set(r, 2);
        }
    }
}

fn main() {
    Frame::<World, 2>::new().step(Block(Both));
}
