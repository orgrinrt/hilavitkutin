// An author implements `Kernel` for the engine's own `Block`, around a kernel
// type of its own, so the orphan rule admits the impl, stating a mask of its
// own. The twin goes the one way an author has: a `BlockKernel`, wrapped in
// `Block`, whose mask the engine folds.
#![allow(unused)]
use author::World;
use engine::{Block, Kernel, RawLanes, Registry};

struct Liar;
struct MyPs;

impl Kernel<World, MyPs> for Block<Liar> {
    const LEN: u32 = 0;
    const MASK: u128 = 0;

    unsafe fn run_in<'s>(
        view: <World as Registry>::Shared<'s>,
        rows: u32,
        raw: &RawLanes<'s, World>,
    ) {
    }
}

fn main() {}
