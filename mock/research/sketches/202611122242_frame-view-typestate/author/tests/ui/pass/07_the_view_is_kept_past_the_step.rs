// Block kernel. Keeps its view in a static, past the step; the twin stores
// nothing there.
#![allow(unused)]
use std::sync::Mutex;

use author::{Pos, World};
use engine::{Block, BlockKernel, Frame, Staged, View};
use notko_hlist::{Cons, Empty};

static KEPT: Mutex<Option<View<'static, World, Cons<Pos, Empty>>>> = Mutex::new(None);

struct Keeper;

impl BlockKernel<World> for Keeper {
    type Reads = Cons<Pos, Empty>;
    type Writes = Cons<Pos, Empty>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {
        *KEPT.lock().unwrap() = None;
    }
}

fn main() {
    Frame::<World, 2>::new().step(Block(Keeper));
}
