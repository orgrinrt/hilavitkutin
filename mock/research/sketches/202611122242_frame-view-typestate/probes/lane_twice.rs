//! Fixture 05's failing file as a binary: a write list naming `Pos` twice.
//! `cargo check --bin lane_twice` passes and `cargo build --bin lane_twice`
//! fails, because the runner's `const` block is evaluated only when the step
//! is monomorphised. Outputs: `out/lane_twice.check.txt`, `out/lane_twice.build.txt`.
#![allow(unused)]
use author::{Pos, World};
use engine::{Block, BlockKernel, Frame, Staged, View};
use notko_hlist::{Cons, Empty};

struct Twice;

impl BlockKernel<World> for Twice {
    type Reads = Cons<Pos, Empty>;
    type Writes = Cons<Pos, Cons<Pos, Empty>>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {}
}

fn main() {
    Frame::<World, 2>::new().step(Block(Twice));
}
