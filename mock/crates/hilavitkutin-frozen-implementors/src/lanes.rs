//! The staged side both stores hand out: each stored column one run of cells, in one of two slot
//! orders, cut by column and by position into disjoint parts.
//!
//! A part holds, for each stored column, either the cells of its run of positions or nothing, and
//! the whole staged side also holds the store's flags of which columns a step took. Cutting moves
//! the borrows apart, so no two parts ever hold one cell.

use core::marker::PhantomData;
use core::mem;

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::bind::Col;
use hilavitkutin_contracts::domain::{Cell, Domain, Value, Word};
use hilavitkutin_contracts::num::{Bool, ColumnNo, Count, Index};
use hilavitkutin_contracts::store::{Stage, Taking};

/// Where the cell of a position sits in a part's run of cells.
pub trait Order: 'static {
    /// The cell of `at` in a part holding `low` up to `high`.
    fn cell(low: Index, high: Index, at: Index) -> usize;
    /// Cuts a part's run at `at`: the cells of the positions below `at`, then the rest.
    fn cut(cells: &mut [Word], low: Index, high: Index, at: Index) -> (&mut [Word], &mut [Word]);
}

/// Position `at` in slot `at`.
pub struct Ahead;

/// Position `at` in slot `extent - 1 - at`: the last position first.
pub struct Behind;

const fn gap(from: Index, to: Index) -> usize {
    match to.since(from) {
        Maybe::Is(n) => n.host(),
        Maybe::Isnt => 0,
    }
}

impl Order for Ahead {
    fn cell(low: Index, _high: Index, at: Index) -> usize {
        gap(low, at)
    }

    fn cut(cells: &mut [Word], low: Index, _high: Index, at: Index) -> (&mut [Word], &mut [Word]) {
        cells.split_at_mut(gap(low, at))
    }
}

impl Order for Behind {
    fn cell(_low: Index, high: Index, at: Index) -> usize {
        gap(at.next(), high)
    }

    fn cut(cells: &mut [Word], _low: Index, high: Index, at: Index) -> (&mut [Word], &mut [Word]) {
        let (from, below) = cells.split_at_mut(gap(at, high));
        (below, from)
    }
}

/// Some of the staged cells: for each of `COLUMNS` stored columns, the cells of the positions from
/// `low` up to `high`, or nothing.
pub struct Lanes<'s, D, const COLUMNS: usize, O> {
    low:     Index,
    high:    Index,
    cells:   [Maybe<&'s mut [Word]>; COLUMNS],
    written: Maybe<&'s mut [Bool; COLUMNS]>,
    marker:  PhantomData<fn() -> (D, O)>,
}

impl<'s, D, const COLUMNS: usize, O: Order> Lanes<'s, D, COLUMNS, O> {
    /// The whole staged side: every column's first `extent` cells, and the written flags.
    pub fn whole<const CAP: usize>(
        cells: &'s mut [[Word; CAP]; COLUMNS],
        extent: Count,
        written: &'s mut [Bool; COLUMNS],
    ) -> Self {
        let mut runs = cells.iter_mut();
        Self {
            low:     Index::FIRST,
            high:    extent.end(),
            cells:   core::array::from_fn(|_| {
                match runs.next() {
                    Some(run) => Maybe::Is(&mut run[.. extent.host()]),
                    None => Maybe::Isnt,
                }
            }),
            written: Maybe::Is(written),
            marker:  PhantomData,
        }
    }

    fn empty(low: Index, high: Index) -> Self {
        Self {
            low,
            high,
            cells: core::array::from_fn(|_| Maybe::Isnt),
            written: Maybe::Isnt,
            marker: PhantomData,
        }
    }
}

impl<D: Domain, const COLUMNS: usize, O: Order> Stage<D> for Lanes<'_, D, COLUMNS, O> {
    fn take<T: Taking>(mut self, columns: &T) -> (Self, Self) {
        let mut taken = Self::empty(self.low, self.high);
        let mut c = 0;
        while c < COLUMNS {
            if columns.holds(ColumnNo::from_host(c)).get() {
                taken.cells[c] = mem::replace(&mut self.cells[c], Maybe::Isnt);
                if let Maybe::Is(written) = &mut self.written {
                    written[c] = Bool::TRUE;
                }
            }
            c += 1;
        }
        (taken, self)
    }

    fn split(mut self, at: Index) -> (Self, Self) {
        let at = at.max(self.low).min(self.high);
        let mut below = Self::empty(self.low, at);
        let mut from = Self::empty(at, self.high);
        from.written = mem::replace(&mut self.written, Maybe::Isnt);
        let mut c = 0;
        while c < COLUMNS {
            if let Maybe::Is(cells) = mem::replace(&mut self.cells[c], Maybe::Isnt) {
                let (low, high) = O::cut(cells, self.low, self.high, at);
                below.cells[c] = Maybe::Is(low);
                from.cells[c] = Maybe::Is(high);
            }
            c += 1;
        }
        (below, from)
    }

    fn put<V: Value>(&mut self, col: Col<D, V>, at: Index, value: V) {
        let cell = O::cell(self.low, self.high, at);
        if let Maybe::Is(cells) = &mut self.cells[col.column().host()] {
            cells[cell] = value.cell().word();
        }
    }
}

/// The value of a word under `V`.
pub fn value<V: Value>(word: Word) -> V {
    V::from_cell(V::Cell::from_word(word))
}
