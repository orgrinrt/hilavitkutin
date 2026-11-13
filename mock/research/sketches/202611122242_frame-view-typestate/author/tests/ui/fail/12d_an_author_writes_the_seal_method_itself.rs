// An author implements `Kernel` for `Block` of a kernel of its own, as 12c,
// and writes the seal method too, naming the engine's token by its path. The
// twin goes the one way an author has: a `BlockKernel`, wrapped in `Block`,
// whose mask the engine folds.
#![allow(unused)]
use author::World;
use engine::{Block, Kernel, RawLanes, Registry};

struct Liar;
struct MyPs;

impl Kernel<World, MyPs> for Block<Liar> {
    const LEN: u32 = 0;
    const MASK: u128 = 0;

    fn seal(_: engine::sealed::Token) {}

    unsafe fn run_in<'s>(
        view: <World as Registry>::Shared<'s>,
        rows: u32,
        raw: &RawLanes<'s, World>,
    ) {
    }
}

fn main() {}
