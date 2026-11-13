//! The probe store: one column of a five by four plane, honest or carrying one of the three
//! mutants the laws must fail.
//!
//! Its cells are `Cell`s so that the mutants can hand one buffer out as both the read and the
//! write side of a step; the honest mode is the probe's own control, and the laws hold over it.

use core::cell::Cell as Slot;
use core::marker::PhantomData;

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::bind::Col;
use hilavitkutin_contracts::domain::{Cell, Domain, FormatId, Value, Word};
use hilavitkutin_contracts::num::{Count, Index, USize};
use hilavitkutin_contracts::store::{Read, Store, Write};
use hilavitkutin_frozen_authors::{Plane, Spot};

use super::{CELLS, Cells, EAST, L, SHAPE};

/// Which store the probe is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// The control: reads committed, writes staged.
    Honest,
    /// `store_skips_last_record`: drops the write to the last record.
    SkipsLastRecord,
    /// `store_updates_in_place`: its staged column is its committed one.
    UpdatesInPlace,
    /// `store_hands_staged_as_read`: serves reads from the staged column.
    HandsStagedAsRead,
}

pub trait Pick {
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

modes!(Honest, SkipsLastRecord, UpdatesInPlace, HandsStagedAsRead);

pub struct Probe<M> {
    committed: [Slot<Word>; CELLS],
    staged:    [Slot<Word>; CELLS],
    written:   Slot<bool>,
    mode:      PhantomData<M>,
}

pub struct ProbeRead<'s> {
    cells: &'s [Slot<Word>; CELLS],
}

pub struct ProbeWrite<'s> {
    cells:     &'s [Slot<Word>; CELLS],
    written:   &'s Slot<bool>,
    skip_last: bool,
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

impl Read<Plane> for ProbeRead<'_> {
    fn extent(&self) -> Count {
        Count::from_host(CELLS)
    }

    fn get<V: Value>(&self, _col: Col<Plane, V>, at: Index) -> V {
        V::from_cell(V::Cell::from_word(self.cells[at.host()].get()))
    }

    fn near(&self, at: Index, step: <Plane as Domain>::Step) -> Maybe<Index> {
        match Plane::reach(SHAPE, spot(at), step) {
            Maybe::Is(there) => Maybe::Is(position(there)),
            Maybe::Isnt => Maybe::Isnt,
        }
    }
}

impl Write<Plane> for ProbeWrite<'_> {
    fn put<V: Value>(&mut self, _col: Col<Plane, V>, at: Index, value: V) {
        if self.skip_last && at.host() == CELLS - 1 {
            return;
        }
        self.cells[at.host()].set(value.cell().word());
        self.written.set(true);
    }
}

impl<M: Pick> Store<Plane> for Probe<M> {
    type Committed<'s>
        = ProbeRead<'s>
    where
        M: 's;
    type Staged<'s>
        = ProbeWrite<'s>
    where
        M: 's;

    fn shape(&self) -> <Plane as Domain>::Shape {
        SHAPE
    }

    fn extent(&self) -> Count {
        Count::from_host(CELLS)
    }

    fn format(&self, column: USize) -> Maybe<FormatId> {
        if column.host() == 0 { Maybe::Is(L::FORMAT) } else { Maybe::Isnt }
    }

    fn step(&mut self) -> (ProbeRead<'_>, ProbeWrite<'_>) {
        let (read, write) = match M::MODE {
            Mode::Honest | Mode::SkipsLastRecord => (&self.committed, &self.staged),
            Mode::UpdatesInPlace => (&self.committed, &self.committed),
            Mode::HandsStagedAsRead => (&self.staged, &self.staged),
        };
        let skip_last = M::MODE == Mode::SkipsLastRecord;
        (
            ProbeRead {
                cells: read,
            },
            ProbeWrite {
                cells: write,
                written: &self.written,
                skip_last,
            },
        )
    }

    fn committed(&self) -> ProbeRead<'_> {
        ProbeRead {
            cells: &self.committed,
        }
    }

    fn swap(&mut self) {
        if self.written.get() && M::MODE != Mode::UpdatesInPlace {
            for at in 0 .. CELLS {
                self.committed[at].set(self.staged[at].get());
            }
        }
        self.written.set(false);
    }
}

impl<M: Pick> Cells for Probe<M> {
    fn make() -> Self {
        Self {
            committed: core::array::from_fn(|_| Slot::new(Word::ZERO)),
            staged:    core::array::from_fn(|_| Slot::new(Word::ZERO)),
            written:   Slot::new(false),
            mode:      PhantomData,
        }
    }

    fn load(&mut self, cells: &[Word; CELLS]) {
        for at in 0 .. CELLS {
            self.committed[at].set(cells[at]);
        }
    }

    fn cells(&self) -> [Word; CELLS] {
        core::array::from_fn(|at| self.committed[at].get())
    }

    fn poison(&mut self, word: Word) {
        for at in 0 .. CELLS {
            self.staged[at].set(word);
        }
    }
}
