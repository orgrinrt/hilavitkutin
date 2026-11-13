//! (d) a kernel taking the new capability: `Vel` kicked by a seeded draw, -1, 0 or 1.

use e2_unit::Vel;
use e2_vocab::{Cons, Draw, Empty, Reads, Vals};

pub type JitterReads = Cons<Vel, Empty>;
pub type JitterWrites = Cons<Vel, Empty>;

pub fn jitter(r: &impl Reads<JitterReads>, d: &impl Draw, w: &mut Vals<JitterWrites>) {
    let kick = (d.draw() % 3) as i32 - 1;
    w.set(Vel, r.get(Vel).wrapping_add(kick));
}
