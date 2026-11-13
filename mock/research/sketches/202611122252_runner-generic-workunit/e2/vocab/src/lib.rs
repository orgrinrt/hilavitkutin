//! E2: one small trait per handed thing, and no runner trait. A kernel is a function
//! generic over the handles it takes.

#![no_std]

pub use rg_owed::{Cons, Elem, Empty, Has, Place, Project, Vals};

/// An offset on the world's two-axis key.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Off {
    pub east:  i32,
    pub north: i32,
}

/// How far a window may reach either side of its key, per axis.
pub trait Reach {
    const EAST: i32;
    const NORTH: i32;
}

/// The values of list `L` at the kernel's row.
pub trait Reads<L: Project> {
    fn get<M: Elem, P: Place>(&self, member: M) -> M::Value
    where
        L: Has<M, P>;
}

/// The values of list `L` at a neighbour of the kernel's key, within reach `R`. An
/// offset past the reach saturates to it, per axis.
pub trait Window<L: Project, R: Reach> {
    fn at<M: Elem, P: Place>(&self, member: M, off: Off) -> M::Value
    where
        L: Has<M, P>;
}

/// Where events of type `E` go.
pub trait Emit<E> {
    fn emit(&mut self, e: E);
}

/// A seeded draw for the kernel's key at this tick, the same whatever the layout.
pub trait Draw {
    fn draw(&self) -> u32;
}
