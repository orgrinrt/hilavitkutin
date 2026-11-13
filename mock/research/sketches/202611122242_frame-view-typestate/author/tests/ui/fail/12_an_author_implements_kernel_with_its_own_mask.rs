// An author implements the runner's `Kernel` itself, stating a mask of its
// own. The twin goes the one way an author has: a `BlockKernel`, wrapped in
// `Block`, whose mask the engine folds from the write list.
#![allow(unused)]
use author::{Pos, World};
use engine::{Frame, Kernel, RawLanes, Registry};
use notko_hlist::{Cons, Empty, Here};

struct Liar;

impl Kernel<World, Cons<Here, Empty>> for Liar {
    const LEN: u32 = 1;
    const MASK: u128 = 0;

    unsafe fn run_in<'s>(
        view: <World as Registry>::Shared<'s>,
        rows: u32,
        raw: &RawLanes<'s, World>,
    ) {
    }
}

fn main() {
    Frame::<World, 2>::new().step(Liar);
}
