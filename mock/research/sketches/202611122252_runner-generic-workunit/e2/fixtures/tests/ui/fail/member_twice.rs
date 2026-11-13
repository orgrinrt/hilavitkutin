// A read list names one member twice, and the kernel reads it once.
use e2_unit::{Pos, Vel};
use e2_vocab::{Cons, Empty, Reads};

type L = Cons<Pos, Cons<Pos, Empty>>;

pub fn kernel(r: &impl Reads<L>) -> i32 {
    r.get(Pos)
}

fn main() {
    let _ = Vel;
}
