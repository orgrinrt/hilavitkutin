//! The matrix the laws run over: author 1 on a plane of five by four, its heat in 16 bits.

#![allow(dead_code)]

pub mod probe;

use hilavitkutin_contracts::Outcome;
use hilavitkutin_contracts::bind::{Binding, Bound, Coefficient, Role, UnitId, bind};
use hilavitkutin_contracts::digest::Digest;
use hilavitkutin_contracts::domain::{Cell, Value, Word};
use hilavitkutin_contracts::kernel::ValueKernel;
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Count, Index, Int, USize};
use hilavitkutin_contracts::store::{Store, Visit};
use hilavitkutin_frozen_authors::{Diffuse, Extent, Plane, model};
use hilavitkutin_frozen_implementors::{Columns, Records};
use hilavitkutin_reference::{Run, run, step, swap};

pub const EAST: usize = 5;
pub const NORTH: usize = 4;
pub const CELLS: usize = EAST * NORTH;
pub const BITS: u32 = 16;
pub const FRAMES: usize = 4;
pub const RATE: i64 = 3;

pub type L = Int<BITS>;
pub type List = Cons<Diffuse<L>, Empty>;

pub const SHAPE: Extent = Extent {
    east:  Count::from_host(EAST),
    north: Count::from_host(NORTH),
};
/// Morsel sizes: one, a prime, the whole.
pub const MORSELS: [Count; 3] = [Count::ONE, Count::from_host(7), Count::from_host(CELLS)];

/// What the law tests need of a store besides the contracts: loading, reading and poisoning
/// cells of column 0 by position.
pub trait Cells: Store<Plane> {
    fn make() -> Self;
    fn load(&mut self, cells: &[Word; CELLS]);
    fn cells(&self) -> [Word; CELLS];
    fn poison(&mut self, word: Word);
}

macro_rules! frozen_cells {
    ($($store:ident),*) => {$(
        impl Cells for $store<Plane, 1, CELLS> {
            fn make() -> Self {
                match $store::new(SHAPE, [L::FORMAT]) {
                    hilavitkutin_contracts::Maybe::Is(store) => store,
                    hilavitkutin_contracts::Maybe::Isnt => panic!("the plane fits"),
                }
            }

            fn load(&mut self, cells: &[Word; CELLS]) {
                for (at, word) in cells.iter().enumerate() {
                    self.set(USize::from_host(0), Index::from_host(at), *word);
                }
            }

            fn cells(&self) -> [Word; CELLS] {
                core::array::from_fn(|at| self.cell(USize::from_host(0), Index::from_host(at)))
            }

            fn poison(&mut self, word: Word) {
                self.poison_staged(word);
            }
        }
    )*};
}

frozen_cells!(Columns, Records);

pub fn word(v: i64) -> Word {
    L::from_raw(v).cell().word()
}

/// A start state that saturates in places: a fixed scramble over the lane's range.
pub fn start() -> [i64; CELLS] {
    core::array::from_fn(|i| ((i as i64 * 7919 + 104_729) % 65_521) - 32_760)
}

pub fn words(heat: &[i64; CELLS]) -> [Word; CELLS] {
    core::array::from_fn(|i| word(heat[i]))
}

/// The model's cells after each of `FRAMES` frames, frame after frame.
pub fn model_frames() -> [Word; CELLS * FRAMES] {
    let mut grid = [[0i64; EAST]; NORTH];
    let s = start();
    for i in 0 .. CELLS {
        grid[i / EAST][i % EAST] = s[i];
    }
    let mut out = [Word::ZERO; CELLS * FRAMES];
    for f in 0 .. FRAMES {
        grid = model::diffuse::<EAST, NORTH>(BITS, &grid, RATE);
        for i in 0 .. CELLS {
            out[f * CELLS + i] = word(grid[i / EAST][i % EAST]);
        }
    }
    out
}

/// The one binding: author 1 reading and writing column 0, its rate a coefficient.
pub struct Table {
    roles:  [Role; 6],
    params: [Coefficient; 1],
}

impl Table {
    pub fn new() -> Self {
        let role = Role {
            column: USize::from_host(0),
            format: L::FORMAT,
        };
        Self {
            roles:  [role; 6],
            params: [Coefficient {
                format: L::FORMAT,
                word:   word(RATE),
            }],
        }
    }

    pub fn binding(&self) -> Binding<'_> {
        Binding {
            unit:   UnitId::new(USize::from_host(0)),
            kernel: Diffuse::<L>::ID,
            roles:  &self.roles,
            params: &self.params,
        }
    }
}

pub fn bound<'t, S: Store<Plane>>(
    bindings: &'t [Binding<'t>],
    store: &S,
) -> Bound<'t, Plane, List> {
    match bind::<Plane, List, S>(bindings, store) {
        Outcome::Ok(bound) => bound,
        Outcome::Err(refusal) => panic!("the table binds: {refusal:?}"),
    }
}

/// B1's record: a run's cells after each frame, frame after frame.
pub fn cells_per_frame<S: Cells, V: Visit>(visit: &V, morsel: Count) -> [Word; CELLS * FRAMES] {
    let table = Table::new();
    let bindings = [table.binding()];
    let mut store = S::make();
    store.load(&words(&start()));
    let bound = bound(&bindings, &store);
    let mut out = [Word::ZERO; CELLS * FRAMES];
    for f in 0 .. FRAMES {
        step(&bound, &mut store, visit, morsel);
        swap(&mut store);
        out[f * CELLS .. (f + 1) * CELLS].copy_from_slice(&store.cells());
    }
    out
}

/// B2's record: the digest stream of a run of `FRAMES` frames.
pub fn digest_stream<S: Cells, V: Visit>(visit: &V, morsel: Count) -> [Word; FRAMES] {
    let table = Table::new();
    let bindings = [table.binding()];
    let mut store = S::make();
    store.load(&words(&start()));
    let bound = bound(&bindings, &store);
    let mut digests = [Digest::new(); FRAMES];
    let mut out = Run::new(&mut digests);
    match run(
        &bound,
        &mut store,
        visit,
        morsel,
        Count::from_host(FRAMES),
        &mut out,
    ) {
        Outcome::Ok(()) => {},
        Outcome::Err(refusal) => panic!("the run has room: {refusal:?}"),
    }
    assert_eq!(out.frames(), Count::from_host(FRAMES));
    core::array::from_fn(|f| digests[f].value())
}

/// B3's record: the committed cells at a step's start, at its end, and after its swap with the
/// staged storage poisoned with each of two words.
pub struct Isolation {
    pub start:    [Word; CELLS],
    pub end:      [Word; CELLS],
    pub poisoned: [[Word; CELLS]; 2],
}

pub fn isolation<S: Cells, V: Visit>(visit: &V, morsel: Count) -> Isolation {
    let table = Table::new();
    let bindings = [table.binding()];
    let poisons = [Word::of([0xAA; 8]), Word::of([0x55; 8])];
    let mut record = Isolation {
        start:    [Word::ZERO; CELLS],
        end:      [Word::ZERO; CELLS],
        poisoned: [[Word::ZERO; CELLS]; 2],
    };
    for (k, poison) in poisons.iter().enumerate() {
        let mut store = S::make();
        store.load(&words(&start()));
        store.poison(*poison);
        let bound = bound(&bindings, &store);
        record.start = store.cells();
        step(&bound, &mut store, visit, morsel);
        if k == 0 {
            record.end = store.cells();
        }
        swap(&mut store);
        record.poisoned[k] = store.cells();
    }
    record
}
