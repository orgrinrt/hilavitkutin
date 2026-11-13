// A kernel keeps its context past the call that handed it.
use e1_unit::{Pos, Still};
use e1_vocab::{Cons, Ctx, Decl, Elem, Empty, Has, Off, Place, Runner};

pub struct K;
impl Decl for K {
    type Event = ();
    type Reach = Still;
    type Reads = Cons<Pos, Empty>;
}

/// The runner's own context, one per row.
pub struct Cx(i32);

impl<D: Decl> Ctx<D> for Cx {
    fn get<M: Elem, P: Place>(&self, _member: M) -> M::Value
    where
        D::Reads: Has<M, P>,
    {
        panic!("never read")
    }

    fn at<M: Elem, P: Place>(&self, _member: M, _off: Off) -> M::Value
    where
        D::Reads: Has<M, P>,
    {
        panic!("never read")
    }

    fn emit(&mut self, _e: D::Event) {}

    fn draw(&self) -> u32 {
        0
    }
}

pub struct Run;
impl Runner for Run {
    type Cx<'a, D: Decl>
        = Cx
    where
        D::Event: 'a;
}

pub fn kernel<'k, X: Runner>(cx: &'k X::Cx<'k, K>, keep: &mut Option<&'k X::Cx<'k, K>>) {
    *keep = Some(cx);
}

fn main() {
    let mut keep: Option<&Cx> = None;
    for i in 0 .. 3 {
        let h = Cx(i);
        kernel::<Run>(&h, &mut None);
    }
    let _ = keep;
}
