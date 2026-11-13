// A context's impl adds a bound the trait did not state.
use e1_vocab::{Ctx, Decl, Elem, Has, Off, Place};

pub struct Cx;

impl<D: Decl> Ctx<D> for Cx {
    fn get<M: Elem, P: Place>(&self, _member: M) -> M::Value
    where
        D::Reads: Has<M, P>,
        M: Copy,
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

fn main() {}
