//! Store one: every stored column contiguous, committed and staged as two whole tables that trade
//! places on a swap.

use core::mem;

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::bind::Col;
use hilavitkutin_contracts::domain::{Domain, FormatId, Value, Word};
use hilavitkutin_contracts::num::{Bool, ColumnNo, Count, Index};
use hilavitkutin_contracts::store::{Opening, Read, Store};

use crate::keys::Keys;
use crate::lanes::{Ahead, Lanes, value};

/// `COLUMNS` stored columns of up to `CAP` records, each column one contiguous run of cells.
pub struct Columns<D: Domain, const COLUMNS: usize, const CAP: usize> {
    keys:      Keys<D, CAP>,
    formats:   [FormatId; COLUMNS],
    committed: [[Word; CAP]; COLUMNS],
    staged:    [[Word; CAP]; COLUMNS],
    written:   [Bool; COLUMNS],
}

impl<D: Domain, const COLUMNS: usize, const CAP: usize> Columns<D, COLUMNS, CAP> {
    /// A store of `shape` with the stored columns in `formats`, every cell zero; nothing where the
    /// shape holds more than `CAP` records.
    pub fn new(shape: D::Shape, formats: [FormatId; COLUMNS]) -> Maybe<Self> {
        match Keys::of(shape) {
            Maybe::Is(keys) => {
                Maybe::Is(Self {
                    keys,
                    formats,
                    committed: [[Word::ZERO; CAP]; COLUMNS],
                    staged: [[Word::ZERO; CAP]; COLUMNS],
                    written: [Bool::FALSE; COLUMNS],
                })
            },
            Maybe::Isnt => Maybe::Isnt,
        }
    }

    /// Sets the committed cell of `column` at `at`.
    pub fn set(&mut self, column: ColumnNo, at: Index, word: Word) {
        self.committed[column.host()][at.host()] = word;
    }

    /// The committed cell of `column` at `at`.
    pub fn cell(&self, column: ColumnNo, at: Index) -> Word {
        self.committed[column.host()][at.host()]
    }

    /// Fills every staged cell with `word`: what a step must never read.
    pub fn poison_staged(&mut self, word: Word) {
        self.staged = [[word; CAP]; COLUMNS];
    }
}

/// The committed view of a [`Columns`] store.
pub struct ColumnsRead<'s, D: Domain, const COLUMNS: usize, const CAP: usize> {
    keys:  &'s Keys<D, CAP>,
    cells: &'s [[Word; CAP]; COLUMNS],
}

/// The staged cells of a [`Columns`] store, or a part of them.
pub type ColumnsWrite<'s, D, const COLUMNS: usize> = Lanes<'s, D, COLUMNS, Ahead>;

impl<D: Domain, const COLUMNS: usize, const CAP: usize> Read<D>
    for ColumnsRead<'_, D, COLUMNS, CAP>
{
    fn extent(&self) -> Count {
        self.keys.extent()
    }

    fn get<V: Value>(&self, col: Col<D, V>, at: Index) -> V {
        value(self.cells[col.column().host()][at.host()])
    }

    fn near(&self, at: Index, step: D::Step) -> Maybe<Index> {
        self.keys.near(at, step)
    }
}

impl<D: Domain, const COLUMNS: usize, const CAP: usize> Store<D> for Columns<D, COLUMNS, CAP> {
    type Committed<'s> = ColumnsRead<'s, D, COLUMNS, CAP>;
    type Staged<'s> = ColumnsWrite<'s, D, COLUMNS>;

    fn shape(&self) -> D::Shape {
        self.keys.shape()
    }

    fn extent(&self) -> Count {
        self.keys.extent()
    }

    fn format(&self, column: ColumnNo) -> Maybe<FormatId> {
        match self.formats.get(column.host()) {
            Some(format) => Maybe::Is(*format),
            None => Maybe::Isnt,
        }
    }

    fn step(&mut self, _opening: Opening) -> (Self::Committed<'_>, Self::Staged<'_>) {
        let extent = self.keys.extent();
        (
            ColumnsRead {
                keys:  &self.keys,
                cells: &self.committed,
            },
            Lanes::whole(&mut self.staged, extent, &mut self.written),
        )
    }

    fn committed(&self) -> Self::Committed<'_> {
        ColumnsRead {
            keys:  &self.keys,
            cells: &self.committed,
        }
    }

    fn swap(&mut self) {
        let mut c = 0;
        while c < COLUMNS {
            if !self.written[c].get() {
                self.staged[c] = self.committed[c];
            }
            self.written[c] = Bool::FALSE;
            c += 1;
        }
        mem::swap(&mut self.committed, &mut self.staged);
    }
}
