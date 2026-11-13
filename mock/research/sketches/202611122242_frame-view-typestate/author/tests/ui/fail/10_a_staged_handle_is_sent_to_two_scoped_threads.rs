// Block kernel. Sends one staged handle to two scoped threads; the twin sends
// each thread its own handle.
#![allow(unused)]
use author::{Pos, Vel, World};
use engine::{Block, BlockKernel, Frame, Staged, View};
use notko_hlist::{Cons, Empty};

struct Split;

impl BlockKernel<World> for Split {
    type Reads = Cons<Pos, Empty>;
    type Writes = Cons<Pos, Cons<Vel, Empty>>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {
        let (mut pos, (mut vel, ())) = out;
        let r = view.rows().next().unwrap();
        std::thread::scope(|s| {
            s.spawn(move || pos.set(r, 1));
            s.spawn(move || pos.set(r, 2));
        });
    }
}

fn main() {
    Frame::<World, 2>::new().step(Block(Split));
}
