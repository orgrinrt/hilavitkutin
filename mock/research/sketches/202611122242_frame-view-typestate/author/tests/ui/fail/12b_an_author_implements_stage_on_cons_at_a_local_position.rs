// An author implements `Stage` for the engine's own `Cons`, at a position
// list of its own, so the orphan rule admits the impl, stating a mask of its
// own. The twin goes the one way an author has: a `BlockKernel` writing the
// same list, whose mask the engine folds.
#![allow(unused)]
use author::{Pos, World};
use engine::{RawLanes, Stage};
use notko_hlist::{Cons, Empty};

struct MyPs;

impl Stage<World, MyPs> for Cons<Pos, Empty> {
    const LEN: u32 = 0;
    const MASK: u128 = 0;

    unsafe fn parts<'s>(raw: &RawLanes<'s, World>) -> Self::Parts<'s> {
        loop {}
    }
}

fn main() {}
