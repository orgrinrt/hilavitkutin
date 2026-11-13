//! E1: one runner trait whose associated context supplies every accessor. A kernel is a
//! function generic over the runner, handed the runner's context for its declaration.

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

/// What a kernel declares: what it reads, how far it reaches, what it emits.
pub trait Decl {
    type Reads: Project;
    type Reach: Reach;
    type Event;
}

/// Every accessor a kernel can be handed, for the declaration `D`.
pub trait Ctx<D: Decl> {
    /// A value of the kernel's reads at its key.
    fn get<M: Elem, P: Place>(&self, member: M) -> M::Value
    where
        D::Reads: Has<M, P>;

    /// A value of the kernel's reads at a neighbour of its key, within its reach. An
    /// offset past the reach saturates to it, per axis.
    fn at<M: Elem, P: Place>(&self, member: M, off: Off) -> M::Value
    where
        D::Reads: Has<M, P>;

    /// An event of the kernel's declared type.
    fn emit(&mut self, e: D::Event);

    /// A seeded draw for the kernel's key at this tick, the same whatever the layout.
    fn draw(&self) -> u32;
}

/// The runner: one trait, its context the whole surface. The where-clause is the first
/// runner's: a context that borrows a sink of the declared event (E0309 without it).
pub trait Runner {
    type Cx<'a, D: Decl>: Ctx<D>
    where
        D::Event: 'a;
}
