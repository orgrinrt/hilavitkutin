// A kernel keeps its handle past the call that handed it.
use e2_unit::Pos;
use e2_vocab::{Cons, Elem, Empty, Has, Place, Reads};

type L = Cons<Pos, Empty>;

/// The runner's own handle, one per row.
pub struct Handle(i32);

impl Reads<L> for Handle {
    fn get<M: Elem, P: Place>(&self, _member: M) -> M::Value
    where
        L: Has<M, P>,
    {
        panic!("never read")
    }
}

pub fn kernel<'k, R: Reads<L>>(r: &'k R, keep: &mut Option<&'k R>) {
    *keep = Some(r);
}

fn main() {
    let mut keep: Option<&Handle> = None;
    for i in 0 .. 3 {
        let h = Handle(i);
        kernel(&h, &mut None);
    }
    let _ = keep;
}
