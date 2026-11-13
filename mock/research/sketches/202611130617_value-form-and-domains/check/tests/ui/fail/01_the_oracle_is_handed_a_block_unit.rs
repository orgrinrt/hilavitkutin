use author::Diffuse;
use check::{Tally, seeded};

fn main() {
    let mut world = seeded(2, 2, 1);
    // `Tally` names the engine's context, and the oracle has none to hand it.
    oracle::pass(&(Diffuse, (Tally, ())), &mut world);
}
