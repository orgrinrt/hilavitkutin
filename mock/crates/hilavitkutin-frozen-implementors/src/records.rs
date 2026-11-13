//! Store two: every record's cells side by side, records laid out last key first, two buffers
//! with a flag saying which one is committed.

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::bind::Col;
use hilavitkutin_contracts::domain::{Cell, Domain, FormatId, Value, Word};
use hilavitkutin_contracts::num::{Bool, Count, Index, USize};
use hilavitkutin_contracts::store::{Read, Store, Write};

use crate::keys::Keys;

/// `COLUMNS` stored columns of up to `CAP` records, each record one run of its cells, the record
/// at position `at` in slot `extent - 1 - at`.
pub struct Records<D: Domain, const COLUMNS: usize, const CAP: usize> {
    keys:    Keys<D, CAP>,
    formats: [FormatId; COLUMNS],
    buffers: [[[Word; COLUMNS]; CAP]; 2],
    front:   Bool,
    written: [Bool; COLUMNS],
}

const fn slot(extent: Count, at: Index) -> usize {
    extent.host() - 1 - at.host()
}

const fn side(front: Bool) -> usize {
    if front.get() { 1 } else { 0 }
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
                    buffers: [[[Word::ZERO; COLUMNS]; CAP]; 2],
                    front: Bool::FALSE,
                    written: [Bool::FALSE; COLUMNS],
                })
            },
            Maybe::Isnt => Maybe::Isnt,
        }
    }

    /// Sets the committed cell of `column` at `at`.
    pub fn set(&mut self, column: USize, at: Index, word: Word) {
        let s = slot(self.keys.extent(), at);
        self.buffers[side(self.front)][s][column.host()] = word;
    }

    /// The committed cell of `column` at `at`.
    pub fn cell(&self, column: USize, at: Index) -> Word {
        self.buffers[side(self.front)][slot(self.keys.extent(), at)][column.host()]
    }

    /// Fills every staged cell with `word`: what a step must never read.
    pub fn poison_staged(&mut self, word: Word) {
        self.buffers[side(self.front.not())] = [[word; COLUMNS]; CAP];
    }
}

pub struct RecordsRead<'s, D: Domain, const COLUMNS: usize, const CAP: usize> {
    keys:  &'s Keys<D, CAP>,
    cells: &'s [[Word; COLUMNS]; CAP],
}

pub struct RecordsWrite<'s, const COLUMNS: usize, const CAP: usize> {
    extent:  Count,
    cells:   &'s mut [[Word; COLUMNS]; CAP],
    written: &'s mut [Bool; COLUMNS],
}

impl<D: Domain, const COLUMNS: usize, const CAP: usize> Read<D>
    for RecordsRead<'_, D, COLUMNS, CAP>
{
    fn extent(&self) -> Count {
        self.keys.extent()
    }

    fn get<V: Value>(&self, col: Col<D, V>, at: Index) -> V {
        let word = self.cells[slot(self.keys.extent(), at)][col.column().host()];
        V::from_cell(V::Cell::from_word(word))
    }

    fn near(&self, at: Index, step: D::Step) -> Maybe<Index> {
        self.keys.near(at, step)
    }
}

impl<D: Domain, const COLUMNS: usize, const CAP: usize> Write<D>
    for RecordsWrite<'_, COLUMNS, CAP>
{
    fn put<V: Value>(&mut self, col: Col<D, V>, at: Index, value: V) {
        self.cells[slot(self.extent, at)][col.column().host()] = value.cell().word();
        self.written[col.column().host()] = Bool::TRUE;
    }
}

impl<D: Domain, const COLUMNS: usize, const CAP: usize> Store<D> for Records<D, COLUMNS, CAP> {
    type Committed<'s> = RecordsRead<'s, D, COLUMNS, CAP>;
    type Staged<'s> = RecordsWrite<'s, COLUMNS, CAP>;

    fn shape(&self) -> D::Shape {
        self.keys.shape()
    }

    fn extent(&self) -> Count {
        self.keys.extent()
    }

    fn format(&self, column: USize) -> Maybe<FormatId> {
        match self.formats.get(column.host()) {
            Some(format) => Maybe::Is(*format),
            None => Maybe::Isnt,
        }
    }

    fn step(&mut self) -> (Self::Committed<'_>, Self::Staged<'_>) {
        let extent = self.keys.extent();
        let [zero, one] = &mut self.buffers;
        let (committed, staged) = if self.front.get() { (one, zero) } else { (zero, one) };
        (
            RecordsRead {
                keys:  &self.keys,
                cells: committed,
            },
            RecordsWrite {
                extent,
                cells: staged,
                written: &mut self.written,
            },
        )
    }

    fn committed(&self) -> Self::Committed<'_> {
        RecordsRead {
            keys:  &self.keys,
            cells: &self.buffers[side(self.front)],
        }
    }

    fn swap(&mut self) {
        let committed = side(self.front);
        let staged = side(self.front.not());
        let mut c = 0;
        while c < COLUMNS {
            if !self.written[c].get() {
                let mut s = 0;
                while s < CAP {
                    self.buffers[staged][s][c] = self.buffers[committed][s][c];
                    s += 1;
                }
            }
            self.written[c] = Bool::FALSE;
            c += 1;
        }
        self.front = self.front.not();
    }
}
