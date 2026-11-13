// A kernel asks for its row position.
use e2_unit::{Pos, Vel};
use e2_vocab::{Cons, Elem, Empty, Has, Place, Reads};

type L = Cons<Pos, Cons<Vel, Empty>>;

/// The runner's own handle, which knows its row.
pub struct Handle {
    row: usize,
}

impl Handle {
    pub fn row(&self) -> usize {
        self.row
    }
}

impl Reads<L> for Handle {
    fn get<M: Elem, P: Place>(&self, _member: M) -> M::Value
    where
        L: Has<M, P>,
    {
        panic!("never read")
    }
}

pub fn kernel(r: &Handle) -> usize {
    r.row()
}

fn main() {
    let _ = Handle {
        row: 0,
    };
}
