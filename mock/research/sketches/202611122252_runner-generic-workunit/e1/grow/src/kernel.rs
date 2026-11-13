//! (d) a kernel taking the new capability: `Vel` kicked by a seeded draw, -1, 0 or 1.

use e1_unit::{Still, Vel};
use e1_vocab::{Cons, Ctx, Decl, Empty, Runner, Vals};

pub type JitterReads = Cons<Vel, Empty>;
pub type JitterWrites = Cons<Vel, Empty>;

pub struct Jitter;
impl Decl for Jitter {
    type Event = ();
    type Reach = Still;
    type Reads = JitterReads;
}

pub fn jitter<X: Runner>(cx: &X::Cx<'_, Jitter>, w: &mut Vals<JitterWrites>) {
    let kick = (cx.draw() % 3) as i32 - 1;
    w.set(Vel, cx.get(Vel).wrapping_add(kick));
}
