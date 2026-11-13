// A handle's impl adds a bound the trait did not state.
use e2_vocab::{Elem, Has, Place, Project, Reads};

pub struct Handle;

impl<L: Project> Reads<L> for Handle {
    fn get<M: Elem, P: Place>(&self, _member: M) -> M::Value
    where
        L: Has<M, P>,
        M: Copy,
    {
        panic!("never read")
    }
}

fn main() {}
