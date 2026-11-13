//! Store two: every record's committed cells side by side, records laid out last key first; the
//! staged cells a column at a time in the same slot order, folded into the records on a swap.
//!
//! The staged side is kept by column because a step cuts it by column, one part to each unit, and
//! a record's cells side by side cannot be cut that way without sharing a record between two
//! parts.

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::bind::Col;
use hilavitkutin_contracts::domain::{Domain, FormatId, Value, Word};
use hilavitkutin_contracts::num::{Bool, ColumnNo, Count, Index};
use hilavitkutin_contracts::store::{Opening, Read, Store};

use crate::keys::Keys;
use crate::lanes::{Behind, Lanes, value};

/// `COLUMNS` stored columns of up to `CAP` records, each committed record one run of its cells,
/// the record at position `at` in slot `extent - 1 - at`.
pub struct Records<D: Domain, const COLUMNS: usize, const CAP: usize> {
    keys:      Keys<D, CAP>,
    formats:   [FormatId; COLUMNS],
    committed: [[Word; COLUMNS]; CAP],
    staged:    [[Word; CAP]; COLUMNS],
    written:   [Bool; COLUMNS],
}

fn slot(extent: Count, at: Index) -> usize {
    match extent.end().since(at.next()) {
        Maybe::Is(n) => n.host(),
        Maybe::Isnt => 0,
    }
}

impl<D: Domain, const COLUMNS: usize, const CAP: usize> Records<D, COLUMNS, CAP> {
    /// A store of `shape` with the stored columns in `formats`, every cell zero; nothing where the
    /// shape holds more than `CAP` records.
    pub fn new(shape: D::Shape, formats: [FormatId; COLUMNS]) -> Maybe<Self> {
        match Keys::of(shape) {
            Maybe::Is(keys) => {
                Maybe::Is(Self {
                    keys,
                    formats,
                    committed: [[Word::ZERO; COLUMNS]; CAP],
                    staged: [[Word::ZERO; CAP]; COLUMNS],
                    written: [Bool::FALSE; COLUMNS],
                })
            },
            Maybe::Isnt => Maybe::Isnt,
        }
    }

    /// Sets the committed cell of `column` at `at`.
    pub fn set(&mut self, column: ColumnNo, at: Index, word: Word) {
        let s = slot(self.keys.extent(), at);
        self.committed[s][column.host()] = word;
    }

    /// The committed cell of `column` at `at`.
    pub fn cell(&self, column: ColumnNo, at: Index) -> Word {
        self.committed[slot(self.keys.extent(), at)][column.host()]
    }

    /// Fills every staged cell with `word`: what a step must never read.
    pub fn poison_staged(&mut self, word: Word) {
        self.staged = [[word; CAP]; COLUMNS];
    }
}

/// The committed view of a [`Records`] store.
pub struct RecordsRead<'s, D: Domain, const COLUMNS: usize, const CAP: usize> {
    keys:  &'s Keys<D, CAP>,
    cells: &'s [[Word; COLUMNS]; CAP],
}

/// The staged cells of a [`Records`] store, or a part of them.
pub type RecordsWrite<'s, D, const COLUMNS: usize> = Lanes<'s, D, COLUMNS, Behind>;

impl<D: Domain, const COLUMNS: usize, const CAP: usize> Read<D>
    for RecordsRead<'_, D, COLUMNS, CAP>
{
    fn extent(&self) -> Count {
        self.keys.extent()
    }

    fn get<V: Value>(&self, col: Col<D, V>, at: Index) -> V {
        value(self.cells[slot(self.keys.extent(), at)][col.column().host()])
    }

    fn near(&self, at: Index, step: D::Step) -> Maybe<Index> {
        self.keys.near(at, step)
    }
}

impl<D: Domain, const COLUMNS: usize, const CAP: usize> Store<D> for Records<D, COLUMNS, CAP> {
    type Committed<'s> = RecordsRead<'s, D, COLUMNS, CAP>;
    type Staged<'s> = RecordsWrite<'s, D, COLUMNS>;

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
            RecordsRead {
                keys:  &self.keys,
                cells: &self.committed,
            },
            Lanes::whole(&mut self.staged, extent, &mut self.written),
        )
    }

    fn committed(&self) -> Self::Committed<'_> {
        RecordsRead {
            keys:  &self.keys,
            cells: &self.committed,
        }
    }

    fn swap(&mut self) {
        let extent = self.keys.extent().host();
        let mut c = 0;
        while c < COLUMNS {
            if self.written[c].get() {
                let mut s = 0;
                while s < extent {
                    self.committed[s][c] = self.staged[c][s];
                    s += 1;
                }
            }
            self.written[c] = Bool::FALSE;
            c += 1;
        }
    }
}
