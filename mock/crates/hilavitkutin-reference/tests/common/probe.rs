//! The probe store: the three stored columns of the matrix, honest or carrying one of the mutants
//! the laws must fail.
//!
//! Its cells are atomics so that a mutant can hand one buffer out as both the read and the write
//! side of a step and still be `Sync` and `Send` as the contracts ask. The honest mode is the
//! probe's own control, and the laws hold over it: it serves reads by the column and position it
//! is asked for, writes only the cells its staged part holds, and on a swap commits exactly the
//! columns a step took.

use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::bind::Col;
use hilavitkutin_contracts::domain::{Cell, Domain, FormatId, Value, Word};
use hilavitkutin_contracts::num::{ColumnNo, Count, Index};
use hilavitkutin_contracts::store::{Opening, Read, Stage, Store, Taking};
use hilavitkutin_frozen_authors::{Plane, Spot};

use super::{CELLS, COLUMNS, Cells, EAST, L, SHAPE};

/// Which store the probe is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// The control.
    Honest,
    /// `store_skips_last_record`: drops every write to the last record.
    SkipsLastRecord,
    /// `store_updates_in_place`: its staged cells are its committed ones.
    UpdatesInPlace,
    /// `store_hands_staged_as_read`: serves reads from the staged cells.
    HandsStagedAsRead,
    /// `store_ignores_the_column`: serves every read and takes every write at column 0, whatever
    /// column the handle names.
    IgnoresColumn,
    /// `swap_drops_unwritten_columns`: a swap makes the whole staged side committed, the columns
    /// no step took included.
    SwapDropsUnwritten,
}

pub trait Pick: 'static {
    const MODE: Mode;
}

macro_rules! modes {
    ($($name:ident),*) => {$(
        pub struct $name;
        impl Pick for $name {
            const MODE: Mode = Mode::$name;
        }
    )*};
}

modes!(
    Honest,
    SkipsLastRecord,
    UpdatesInPlace,
    HandsStagedAsRead,
    IgnoresColumn,
    SwapDropsUnwritten
);

type Table = [[AtomicU64; CELLS]; COLUMNS];

fn table() -> Table {
    core::array::from_fn(|_| core::array::from_fn(|_| AtomicU64::new(0)))
}

fn load(cell: &AtomicU64) -> Word {
    Word::of(cell.load(Ordering::Relaxed).to_le_bytes())
}

fn store(cell: &AtomicU64, word: Word) {
    cell.store(u64::from_le_bytes(word.bytes()), Ordering::Relaxed);
}

pub struct Probe<M> {
    committed: Table,
    staged:    Table,
    written:   [AtomicBool; COLUMNS],
    mode:      PhantomData<fn() -> M>,
}

pub struct ProbeRead<'s, M> {
    cells: &'s Table,
    mode:  PhantomData<fn() -> M>,
}

pub struct ProbeWrite<'s, M> {
    cells:   &'s Table,
    holds:   [bool; COLUMNS],
    low:     usize,
    high:    usize,
    written: Option<&'s [AtomicBool; COLUMNS]>,
    mode:    PhantomData<fn() -> M>,
}

fn position(spot: Spot) -> Index {
    Index::from_host(spot.north.host() * EAST + spot.east.host())
}

fn spot(at: Index) -> Spot {
    Spot {
        north: Index::from_host(at.host() / EAST),
        east:  Index::from_host(at.host() % EAST),
    }
}

fn served<M: Pick>(column: ColumnNo) -> usize {
    if M::MODE == Mode::IgnoresColumn { 0 } else { column.host() }
}

impl<M: Pick> Read<Plane> for ProbeRead<'_, M> {
    fn extent(&self) -> Count {
        Count::from_host(CELLS)
    }

    fn get<V: Value>(&self, col: Col<Plane, V>, at: Index) -> V {
        let word = load(&self.cells[served::<M>(col.column())][at.host()]);
        V::from_cell(V::Cell::from_word(word))
    }

    fn near(&self, at: Index, step: <Plane as Domain>::Step) -> Maybe<Index> {
        match Plane::reach(SHAPE, spot(at), step) {
            Maybe::Is(there) => Maybe::Is(position(there)),
            Maybe::Isnt => Maybe::Isnt,
        }
    }
}

impl<M: Pick> ProbeWrite<'_, M> {
    fn part(&self, low: usize, high: usize) -> Self {
        Self {
            cells: self.cells,
            holds: [false; COLUMNS],
            low,
            high,
            written: None,
            mode: PhantomData,
        }
    }
}

impl<M: Pick> Stage<Plane> for ProbeWrite<'_, M> {
    fn take<T: Taking>(mut self, columns: &T) -> (Self, Self) {
        let mut taken = self.part(self.low, self.high);
        for c in 0 .. COLUMNS {
            if self.holds[c] && columns.holds(ColumnNo::from_host(c)).get() {
                self.holds[c] = false;
                taken.holds[c] = true;
                if let Some(written) = self.written {
                    written[c].store(true, Ordering::Relaxed);
                }
            }
        }
        (taken, self)
    }

    fn split(self, at: Index) -> (Self, Self) {
        let at = at.host().clamp(self.low, self.high);
        let mut below = self.part(self.low, at);
        let mut from = self.part(at, self.high);
        below.holds = self.holds;
        from.holds = self.holds;
        from.written = self.written;
        (below, from)
    }

    fn put<V: Value>(&mut self, col: Col<Plane, V>, at: Index, value: V) {
        let at = at.host();
        if !self.holds[col.column().host()] || at < self.low || at >= self.high {
            return;
        }
        if M::MODE == Mode::SkipsLastRecord && at == CELLS - 1 {
            return;
        }
        store(
            &self.cells[served::<M>(col.column())][at],
            value.cell().word(),
        );
    }
}

impl<M: Pick> Store<Plane> for Probe<M> {
    type Committed<'s> = ProbeRead<'s, M>;
    type Staged<'s> = ProbeWrite<'s, M>;

    fn shape(&self) -> <Plane as Domain>::Shape {
        SHAPE
    }

    fn extent(&self) -> Count {
        Count::from_host(CELLS)
    }

    fn format(&self, column: ColumnNo) -> Maybe<FormatId> {
        if column.host() < COLUMNS { Maybe::Is(L::FORMAT) } else { Maybe::Isnt }
    }

    fn step(&mut self, _opening: Opening) -> (ProbeRead<'_, M>, ProbeWrite<'_, M>) {
        let (read, write) = match M::MODE {
            Mode::UpdatesInPlace => (&self.committed, &self.committed),
            Mode::HandsStagedAsRead => (&self.staged, &self.staged),
            _ => (&self.committed, &self.staged),
        };
        (
            ProbeRead {
                cells: read,
                mode:  PhantomData,
            },
            ProbeWrite {
                cells:   write,
                holds:   [true; COLUMNS],
                low:     0,
                high:    CELLS,
                written: Some(&self.written),
                mode:    PhantomData,
            },
        )
    }

    fn committed(&self) -> ProbeRead<'_, M> {
        ProbeRead {
            cells: &self.committed,
            mode:  PhantomData,
        }
    }

    fn swap(&mut self) {
        for c in 0 .. COLUMNS {
            let taken = self.written[c].swap(false, Ordering::Relaxed);
            let commits = match M::MODE {
                Mode::UpdatesInPlace => false,
                Mode::SwapDropsUnwritten => true,
                _ => taken,
            };
            if commits {
                for at in 0 .. CELLS {
                    store(&self.committed[c][at], load(&self.staged[c][at]));
                }
            }
        }
    }
}

impl<M: Pick> Cells for Probe<M> {
    fn make() -> Self {
        Self {
            committed: table(),
            staged:    table(),
            written:   core::array::from_fn(|_| AtomicBool::new(false)),
            mode:      PhantomData,
        }
    }

    fn load(&mut self, column: ColumnNo, cells: &[Word; CELLS]) {
        for at in 0 .. CELLS {
            store(&self.committed[column.host()][at], cells[at]);
        }
    }

    fn column(&self, column: ColumnNo) -> [Word; CELLS] {
        core::array::from_fn(|at| load(&self.committed[column.host()][at]))
    }

    fn poison(&mut self, word: Word) {
        for column in &self.staged {
            for cell in column {
                store(cell, word);
            }
        }
    }
}
