// A declaration's read list names one member twice, and the kernel reads it once.
use e1_unit::{Pos, Still, Vel};
use e1_vocab::{Cons, Ctx, Decl, Empty, Runner};

pub struct K;
impl Decl for K {
    type Event = ();
    type Reach = Still;
    type Reads = Cons<Pos, Cons<Vel, Empty>>;
}

pub fn kernel<X: Runner>(cx: &X::Cx<'_, K>) -> i32 {
    cx.get(Pos)
}

fn main() {
    let _ = Vel;
}
