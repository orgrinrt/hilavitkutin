// A kernel asks for its row position.
use e1_unit::{Pos, Still, Vel};
use e1_vocab::{Cons, Ctx, Decl, Elem, Empty, Has, Off, Place, Runner};

pub struct K;
impl Decl for K {
    type Event = ();
    type Reach = Still;
    type Reads = Cons<Pos, Cons<Vel, Empty>>;
}

/// The runner's own context, which knows its row.
pub struct Cx {
    row: usize,
}

impl Cx {
    pub fn row(&self) -> usize {
        self.row
    }
}

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

pub fn kernel<X: Runner>(cx: &X::Cx<'_, K>) -> usize {
    cx.row()
}

fn main() {
    let _ = Cx {
        row: 0,
    };
}
