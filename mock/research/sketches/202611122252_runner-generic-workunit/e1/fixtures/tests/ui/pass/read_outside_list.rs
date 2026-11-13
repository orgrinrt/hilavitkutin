// A kernel reads a member its declaration does not name.
use e1_unit::{Heat, Pos, Still, Vel};
use e1_vocab::{Cons, Ctx, Decl, Empty, Runner};

pub struct K;
impl Decl for K {
    type Event = ();
    type Reach = Still;
    type Reads = Cons<Pos, Cons<Vel, Empty>>;
}

pub fn kernel<X: Runner>(cx: &X::Cx<'_, K>) -> i32 {
    cx.get(Vel)
}

fn main() {}
