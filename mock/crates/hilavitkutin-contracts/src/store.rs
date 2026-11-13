//! The store: committed and staged columns of one domain's records, and the order a runner
//! visits them in.
//!
//! A record is found by its position, an [`Index`] in key order, `0` to the extent. Positions are
//! the store's and the runner's: no kernel is handed one. How a store lays its cells out behind a
//! position is its own, and two stores of one domain may lay them out differently.
//!
//! A step reads the committed view and writes the staged cells. The committed view is shared, and
//! `Sync`, so any number of workers read it at once. The staged cells are handed out only through a
//! checked table ([`crate::bind::Bound`]): the store opens a step when the table asks it to, with
//! an [`Opening`] nothing else can make, and the table cuts the staged cells into one part per unit
//! holding exactly the stored columns the unit writes ([`Stage::take`]). A part is `Send`, and a
//! runner cuts it further by position ([`Stage::split`]), so the parts a threaded runner hands its
//! workers are disjoint by construction: by column between units, by position within one.
//!
//! [`Store::swap`] makes the staged cells committed. A column no unit took in a step keeps its
//! committed values across the swap.

use notko::Maybe;

use crate::bind::Col;
use crate::domain::{Domain, FormatId, Value};
use crate::num::{Bool, ColumnNo, Count, Index};

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

mod sealed {
    pub(crate) trait Sealed {}
}

/// The stored columns a part of the staged cells is cut to: the columns one unit writes.
///
/// Sealed: only this crate makes a set, from a checked table, so a store's [`Stage::take`] is
/// called by the bind's own step and by nothing else.
#[allow(private_bounds)]
pub trait Taking: sealed::Sealed {
    /// Whether the set holds the stored column `column`.
    fn holds(&self, column: ColumnNo) -> Bool;
}

/// A set of stored columns, read off one unit's write handles.
pub(crate) struct Writes<'h, H>(pub(crate) &'h H);

impl<H> sealed::Sealed for Writes<'_, H> {}

impl<H: crate::bind::roles::Columns> Taking for Writes<'_, H> {
    fn holds(&self, column: ColumnNo) -> Bool {
        self.0.holds(column)
    }
}

/// Some of the staged cells of a step: every cell of some stored columns at some run of
/// positions. A store's whole staged side is one; [`take`](Self::take) and
/// [`split`](Self::split) cut it into disjoint parts.
pub trait Stage<D: Domain>: Send + Sized {
    /// Cuts off the cells of the columns `columns` holds, at every position this part holds:
    /// the part holding those, then the rest. The store notes the columns as written, so the swap
    /// commits their staged cells.
    fn take<T: Taking>(self, columns: &T) -> (Self, Self);
    /// Cuts at `at`, a position from the first this part holds up to one past its last: the part
    /// holding the positions below `at`, then the part holding `at` and those after it.
    fn split(self, at: Index) -> (Self, Self);
    /// Sets `col` at `at` to `value`. `at` is a position this part holds and `col` a column it
    /// holds; the bind's step checks both before it calls.
    fn put<V: Value>(&mut self, col: Col<D, V>, at: Index, value: V);
}

/// Leave to open a step, which only the checked table's step holds.
pub struct Opening(());

impl Opening {
    pub(crate) const fn new() -> Self {
        Self(())
    }
}

/// A store of one domain's records: one shape, its stored columns each in a declared format,
/// committed and staged.
pub trait Store<D: Domain> {
    /// The committed view, borrowed for a step, shared between every worker of it.
    type Committed<'s>: Read<D> + Sync
    where
        Self: 's;
    /// The staged cells, borrowed for a step, cut into parts that each go to one worker.
    type Staged<'s>: Stage<D>
    where
        Self: 's;

    /// The shape the store holds.
    fn shape(&self) -> D::Shape;
    /// How many records the shape holds.
    fn extent(&self) -> Count;
    /// The declared format of the stored column `column`, or nothing past the last.
    fn format(&self, column: ColumnNo) -> Maybe<FormatId>;
    /// The committed view and the whole staged side of one step, borrowed together.
    fn step(&mut self, opening: Opening) -> (Self::Committed<'_>, Self::Staged<'_>);
    /// The committed view outside a step.
    fn committed(&self) -> Self::Committed<'_>;
    /// Makes the staged cells of every column taken since the last swap committed.
    fn swap(&mut self);
}

/// The size of a morsel: a number of records that is never zero.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Morsel(Count);

impl Morsel {
    /// One record.
    pub const ONE: Self = Self(Count::ONE);

    /// Morsels of `size` records, or nothing for zero.
    pub const fn of(size: Count) -> Maybe<Self> {
        if size.gt(Count::ZERO).get() { Maybe::Is(Self(size)) } else { Maybe::Isnt }
    }

    /// How many records a morsel holds.
    pub const fn size(self) -> Count {
        self.0
    }
}

/// What a visiting order hands its records to: each morsel announced, then its records.
pub trait Visitor {
    /// A morsel of `len` records begins.
    fn morsel(&mut self, len: Count);
    /// The record at `at`.
    fn record(&mut self, at: Index);
}

/// A visiting order: which records, in which morsels, in which order. A visit owes every position
/// below the extent exactly once, and no other, in morsels of `morsel` records but the last, which
/// may be short; an extent below the morsel size is one short morsel, and an extent of zero none.
pub trait Visit {
    /// Hands the records of a store of `extent` records to `on`.
    fn visit<V: Visitor>(&self, extent: Count, morsel: Morsel, on: &mut V);
}
