//! The matrix the laws run over: a plane of five by four with three stored columns, its values in
//! 16 bits. Column 0 is read and written by no unit. Column 1 is author 1's heat, which `Diffuse`
//! writes. Column 2 is the flux `Mirror` writes from the heat at a place and east of it, so one
//! unit reads a column another writes in the same step.

#![allow(dead_code)]

pub mod probe;

use hilavitkutin_contracts::bind::{Binding, Bound, Coefficient, Role, UnitId, bind};
use hilavitkutin_contracts::digest::Digest;
use hilavitkutin_contracts::domain::{Cell, Column, Near, Own, Value, Word};
use hilavitkutin_contracts::kernel::{KernelId, ValueKernel};
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{
    Additive,
    ColumnNo,
    Count,
    Identity,
    Index,
    Int,
    Lane,
    Ordinal,
    USize,
};
use hilavitkutin_contracts::store::{Morsel, Store, Visit};
use hilavitkutin_contracts::{Maybe, Outcome};
use hilavitkutin_frozen_authors::{Diffuse, East, Extent, Heat, Plane, model};
use hilavitkutin_frozen_implementors::{Columns, Records};
use hilavitkutin_reference::{Run, run, step};

pub const EAST: usize = 5;
pub const NORTH: usize = 4;
pub const CELLS: usize = EAST * NORTH;
pub const COLUMNS: usize = 3;
/// Every stored cell of the store, column after column.
pub const WIDTH: usize = CELLS * COLUMNS;
pub const BITS: u32 = 16;
pub const FRAMES: usize = 4;
pub const RATE: i64 = 3;

pub const STILL: ColumnNo = ColumnNo::from_host(0);
pub const HEAT: ColumnNo = ColumnNo::from_host(1);
pub const FLUX: ColumnNo = ColumnNo::from_host(2);
pub const STORED: [ColumnNo; COLUMNS] = [STILL, HEAT, FLUX];

pub type L = Int<BITS>;
pub type List = Cons<Diffuse<L>, Cons<Mirror, Empty>>;

pub const SHAPE: Extent = Extent {
    east:  Count::from_host(EAST),
    north: Count::from_host(NORTH),
};

const fn morsel(size: usize) -> Morsel {
    match Morsel::of(Count::from_host(size)) {
        Maybe::Is(morsel) => morsel,
        Maybe::Isnt => panic!("a morsel holds a record"),
    }
}

/// Morsel sizes: one, a prime, the whole.
pub const MORSELS: [Morsel; 3] = [Morsel::ONE, morsel(7), morsel(CELLS)];

/// The flux at a place: what `Mirror` writes.
pub struct Flux;

impl Column for Flux {
    type Domain = Plane;
    type Value = L;

    const OUTSIDE: L = <L as Identity<Additive>>::IDENTITY;
}

/// A kernel of the tests' own: the flux at a place is its heat less the heat east of it.
pub struct Mirror;

impl ValueKernel for Mirror {
    type Domain = Plane;
    type Reads = Cons<Own<Heat<L>>, Cons<Near<Heat<L>, East>, Empty>>;
    type Roles = Cons<Heat<L>, Cons<Flux, Empty>>;
    type Writes = Cons<Own<Flux>, Empty>;

    const ID: KernelId = KernelId::new(USize::from_host(2));

    fn at((here, (east, ())): (L, (L, ()))) -> (L, ()) {
        (L::narrow(L::wide_sub(here.widen(), east.widen())), ())
    }
}

/// What the law tests need of a store besides the contracts: loading, reading and poisoning cells
/// by column and position.
pub trait Cells: Store<Plane> {
    fn make() -> Self;
    fn load(&mut self, column: ColumnNo, cells: &[Word; CELLS]);
    fn column(&self, column: ColumnNo) -> [Word; CELLS];
    fn poison(&mut self, word: Word);

    /// Every stored cell, column after column.
    fn cells(&self) -> [Word; WIDTH] {
        let mut out = [Word::ZERO; WIDTH];
        for (c, column) in STORED.iter().enumerate() {
            out[c * CELLS .. (c + 1) * CELLS].copy_from_slice(&self.column(*column));
        }
        out
    }
}

macro_rules! frozen_cells {
    ($($store:ident),*) => {$(
        impl Cells for $store<Plane, COLUMNS, CELLS> {
            fn make() -> Self {
                match $store::new(SHAPE, [L::FORMAT; COLUMNS]) {
                    Maybe::Is(store) => store,
                    Maybe::Isnt => panic!("the plane fits"),
                }
            }

            fn load(&mut self, column: ColumnNo, cells: &[Word; CELLS]) {
                for (at, word) in cells.iter().enumerate() {
                    self.set(column, Index::from_host(at), *word);
                }
            }

            fn column(&self, column: ColumnNo) -> [Word; CELLS] {
                core::array::from_fn(|at| self.cell(column, Index::from_host(at)))
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

/// The input streams the matrix starts from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Input {
    /// Fixed scrambles over the lane's range, different per column, saturating in places.
    Scramble,
    /// Every cell zero.
    Zero,
    /// The lane's lowest and highest values in turn, so every sum saturates.
    Edges,
}

pub const INPUTS: [Input; 3] = [Input::Scramble, Input::Zero, Input::Edges];

/// The start state of a stored column.
pub fn start(input: Input, column: usize) -> [i64; CELLS] {
    let low = -(1i64 << (BITS - 1));
    let high = (1i64 << (BITS - 1)) - 1;
    match input {
        Input::Scramble => {
            let mul = [104_723i64, 7919, 3571][column];
            let add = [11i64, 104_729, 52_361][column];
            core::array::from_fn(|i| ((i as i64 * mul + add) % 65_521) - 32_760)
        },
        Input::Zero => [0; CELLS],
        Input::Edges => core::array::from_fn(|i| if (i + column) % 2 == 0 { low } else { high }),
    }
}

pub fn words(values: &[i64; CELLS]) -> [Word; CELLS] {
    core::array::from_fn(|i| word(values[i]))
}

/// A store loaded with the start state.
pub fn loaded<S: Cells>(input: Input) -> S {
    let mut store = S::make();
    for (c, column) in STORED.iter().enumerate() {
        store.load(*column, &words(&start(input, c)));
    }
    store
}

fn grid(cells: &[i64; CELLS]) -> [[i64; EAST]; NORTH] {
    let mut grid = [[0i64; EAST]; NORTH];
    for i in 0 .. CELLS {
        grid[i / EAST][i % EAST] = cells[i];
    }
    grid
}

/// The model of `Mirror`: the heat at a place less the heat east of it, zero off the plane,
/// clamped into the lane.
fn mirror(heat: &[[i64; EAST]; NORTH]) -> [[i64; EAST]; NORTH] {
    let low = -(1i64 << (BITS - 1));
    let high = (1i64 << (BITS - 1)) - 1;
    core::array::from_fn(|n| {
        core::array::from_fn(|e| {
            let east = if e + 1 < EAST { heat[n][e + 1] } else { 0 };
            (heat[n][e] - east).clamp(low, high)
        })
    })
}

/// The model's stored cells after each of `FRAMES` frames, frame after frame, each frame column
/// after column.
pub fn model_frames(input: Input) -> [Word; WIDTH * FRAMES] {
    let still = start(input, 0);
    let mut heat = grid(&start(input, 1));
    let mut out = [Word::ZERO; WIDTH * FRAMES];
    for f in 0 .. FRAMES {
        let flux = mirror(&heat);
        heat = model::diffuse::<EAST, NORTH>(BITS, &heat, RATE);
        let frame = &mut out[f * WIDTH .. (f + 1) * WIDTH];
        for i in 0 .. CELLS {
            frame[i] = word(still[i]);
            frame[CELLS + i] = word(heat[i / EAST][i % EAST]);
            frame[2 * CELLS + i] = word(flux[i / EAST][i % EAST]);
        }
    }
    out
}

/// The table: unit 0 is author 1 on column 1 with its rate a coefficient, unit 1 is `Mirror`
/// reading column 1 and writing column 2.
pub struct Table {
    heat:   [Role; 1],
    mirror: [Role; 2],
    params: [Coefficient; 1],
}

impl Table {
    pub fn new() -> Self {
        let role = |column| {
            Role {
                column,
                format: L::FORMAT,
            }
        };
        Self {
            heat:   [role(HEAT)],
            mirror: [role(HEAT), role(FLUX)],
            params: [Coefficient {
                format: L::FORMAT,
                word:   word(RATE),
            }],
        }
    }

    pub fn bindings(&self) -> [Binding<'_>; 2] {
        [
            Binding {
                unit:   UnitId::new(USize::from_host(0)),
                kernel: Diffuse::<L>::ID,
                roles:  &self.heat,
                params: &self.params,
            },
            Binding {
                unit:   UnitId::new(USize::from_host(1)),
                kernel: Mirror::ID,
                roles:  &self.mirror,
                params: &[],
            },
        ]
    }
}

pub fn bound<'t, S: Store<Plane>, P>(
    bindings: &'t [Binding<'t>],
    store: &'t mut S,
) -> Bound<'t, Plane, List, S, P>
where
    List: hilavitkutin_contracts::kernel::Kernels<Plane, P>,
{
    match bind::<Plane, List, S, P>(bindings, store) {
        Outcome::Ok(bound) => bound,
        Outcome::Err(refusal) => panic!("the table binds: {refusal:?}"),
    }
}

fn stepped<S: Store<Plane>, L2: hilavitkutin_contracts::kernel::Kernels<Plane, P>, P, V: Visit>(
    bound: &mut Bound<'_, Plane, L2, S, P>,
    visit: &V,
    morsel: Morsel,
) {
    match step(bound, visit, morsel) {
        Outcome::Ok(()) => {},
        Outcome::Err(outside) => panic!("the visit stays inside: {outside:?}"),
    }
}

/// B1's record: a run's stored cells after each frame, frame after frame.
pub fn cells_per_frame<S: Cells, V: Visit>(
    visit: &V,
    morsel: Morsel,
    input: Input,
) -> [Word; WIDTH * FRAMES] {
    let table = Table::new();
    let bindings = table.bindings();
    let mut store = loaded::<S>(input);
    let mut bound = bound(&bindings, &mut store);
    let mut out = [Word::ZERO; WIDTH * FRAMES];
    for f in 0 .. FRAMES {
        stepped(&mut bound, visit, morsel);
        bound.swap();
        out[f * WIDTH .. (f + 1) * WIDTH].copy_from_slice(&bound.store().cells());
    }
    out
}

/// B2's record: the digest stream of a run of `FRAMES` frames.
pub fn digest_stream<S: Cells, V: Visit>(visit: &V, morsel: Morsel) -> [Word; FRAMES] {
    let table = Table::new();
    let bindings = table.bindings();
    let mut store = loaded::<S>(Input::Scramble);
    let mut bound = bound(&bindings, &mut store);
    let mut digests = [Digest::new(); FRAMES];
    let mut out = Run::new(&mut digests);
    match run(
        &mut bound,
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
    pub start:    [Word; WIDTH],
    pub end:      [Word; WIDTH],
    pub poisoned: [[Word; WIDTH]; 2],
}

pub const POISONS: [Word; 2] = [Word::of([0xAA; 8]), Word::of([0x55; 8])];

pub fn isolation<S: Cells, V: Visit>(visit: &V, morsel: Morsel) -> Isolation {
    let table = Table::new();
    let bindings = table.bindings();
    let mut record = Isolation {
        start:    [Word::ZERO; WIDTH],
        end:      [Word::ZERO; WIDTH],
        poisoned: [[Word::ZERO; WIDTH]; 2],
    };
    for (k, poison) in POISONS.iter().enumerate() {
        let mut store = loaded::<S>(Input::Scramble);
        store.poison(*poison);
        let mut bound = bound(&bindings, &mut store);
        record.start = bound.store().cells();
        stepped(&mut bound, visit, morsel);
        if k == 0 {
            record.end = bound.store().cells();
        }
        bound.swap();
        record.poisoned[k] = bound.store().cells();
    }
    record
}

/// The first word of a stored column within a record of [`Cells::cells`].
pub const fn first_of(column: ColumnNo) -> Ordinal {
    Ordinal::from_host(column.host() * CELLS)
}
