// A kernel reads a member its list does not name.
use e2_unit::{Heat, Pos, Vel};
use e2_vocab::{Cons, Empty, Reads};

type L = Cons<Pos, Cons<Vel, Empty>>;

pub fn kernel(r: &impl Reads<L>) -> i32 {
    r.get(Vel)
}

fn main() {}
