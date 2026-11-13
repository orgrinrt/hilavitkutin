//! The store: committed and staged columns of one domain's records, and the order a runner
//! visits them in.
//!
//! A record is found by its position, an [`Index`] in key order, `0` to the extent. Positions are
//! the store's and the runner's: no kernel is handed one. How a store lays its cells out behind a
//! position is its own, and two stores of one domain may lay them out differently.
//!
//! A step reads the committed view and writes the staged one; [`Store::swap`] makes the staged
//! columns committed. A column no unit wrote in a step keeps its committed values across the
//! swap.

use notko::Maybe;

use crate::bind::Col;
use crate::domain::{Domain, FormatId, Value};
use crate::num::{Count, Index, USize};

/// The committed view of a step: what every read of the step reads.
pub trait Read<D: Domain> {
    /// How many records the store holds.
    fn extent(&self) -> Count;
    /// The value of `col` at the record at `at`.
    fn get<V: Value>(&self, col: Col<D, V>, at: Index) -> V;
    /// The position of the record `step` away from the record at `at`, or nothing outside the
    /// shape.
    fn near(&self, at: Index, step: D::Step) -> Maybe<Index>;
}

/// The staged columns of a step: what every write of the step writes.
pub trait Write<D: Domain> {
    /// Sets `col` at the record at `at` to `value`.
    fn put<V: Value>(&mut self, col: Col<D, V>, at: Index, value: V);
}

/// A store of one domain's records: one shape, its stored columns each in a declared format,
/// committed and staged.
pub trait Store<D: Domain> {
    /// The committed view, borrowed for a step.
    type Committed<'s>: Read<D>
    where
        Self: 's;
    /// The staged columns, borrowed for a step.
    type Staged<'s>: Write<D>
    where
        Self: 's;

    /// The shape the store holds.
    fn shape(&self) -> D::Shape;
    /// How many records the shape holds.
    fn extent(&self) -> Count;
    /// The declared format of the stored column `column`, or nothing past the last.
    fn format(&self, column: USize) -> Maybe<FormatId>;
    /// The committed view and the staged columns of one step, borrowed together.
    fn step(&mut self) -> (Self::Committed<'_>, Self::Staged<'_>);
    /// The committed view outside a step.
    fn committed(&self) -> Self::Committed<'_>;
    /// Makes the staged columns committed.
    fn swap(&mut self);
}

/// What a visiting order hands its records to: each morsel announced, then its records.
pub trait Visitor {
    /// A morsel of `len` records begins.
    fn morsel(&mut self, len: Count);
    /// The record at `at`.
    fn record(&mut self, at: Index);
}

/// A visiting order: which records, in which morsels, in which order. A visit owes every position
/// below the extent exactly once, in morsels of `size` records but the last, which may be short.
pub trait Visit {
    /// Hands the records of a store of `extent` records to `on`.
    fn visit<V: Visitor>(&self, extent: Count, size: Count, on: &mut V);
}
