//! The loop both hosts run: author 1 over the frozen stores, through the
//! reference runner, its digest stream kept frame by frame.
//!
//! `no_std`, no alloc, and nothing in it that a host could vary but the
//! target it is built for. A case names a store, a visiting order and a
//! morsel size, and one case lends too little digest storage so that the
//! runner's refusal is compared across targets as well.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use hilavitkutin_contracts::bind::{Binding, Coefficient, Role, UnitId, bind};
use hilavitkutin_contracts::digest::Digest;
use hilavitkutin_contracts::domain::{Cell, Value, Word};
use hilavitkutin_contracts::kernel::ValueKernel;
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Count, Index, Int, USize};
use hilavitkutin_contracts::store::{Store, Visit};
use hilavitkutin_contracts::{Maybe, Outcome};
use hilavitkutin_frozen_authors::{Diffuse, Extent, Plane};
use hilavitkutin_frozen_implementors::{Columns, Forward, Permuted, Records};
use hilavitkutin_reference::{Refusal, Run, run};

/// The plane is sixteen cells east by twelve north.
const EAST: usize = 16;
const NORTH: usize = 12;
const CELLS: usize = EAST * NORTH;

/// The frames a case runs, and the most digests a case keeps.
pub const FRAMES: usize = 64;

/// How many cases there are, numbered from zero.
pub const CASES: USize = USize::from_host(4);

/// The lane: 16 bits, so the diffusion saturates in places within the run.
type L = Int<16>;
type List = Cons<Diffuse<L>, Empty>;

const SHAPE: Extent = Extent {
    east:  Count::from_host(EAST),
    north: Count::from_host(NORTH),
};
const RATE: i64 = 3;

/// What a case left: how many digests it wrote, and the frame its run
/// refused at, if it refused.
pub struct Report {
    pub written: Count,
    pub refused: Maybe<Count>,
}

/// Runs case `case` into `out`. A case past [`CASES`] writes nothing.
pub fn run_case(case: USize, out: &mut [Word; FRAMES]) -> Report {
    let mut digests = [Digest::new(); FRAMES];
    let report = match case.host() {
        0 => stream::<Columns<Plane, 1, CELLS>, _>(&Forward, Count::from_host(7), &mut digests),
        1 => {
            stream::<Records<Plane, 1, CELLS>, _>(
                &Permuted {
                    seed: USize::from_host(11),
                },
                Count::ONE,
                &mut digests,
            )
        },
        2 => {
            stream::<Columns<Plane, 1, CELLS>, _>(
                &Permuted {
                    seed: USize::from_host(1_000_003),
                },
                Count::from_host(CELLS),
                &mut digests,
            )
        },
        3 => {
            stream::<Records<Plane, 1, CELLS>, _>(
                &Forward,
                Count::from_host(5),
                &mut digests[.. FRAMES / 2],
            )
        },
        _ => {
            Report {
                written: Count::ZERO,
                refused: Maybe::Isnt,
            }
        },
    };
    for (at, digest) in digests.iter().enumerate().take(report.written.host()) {
        out[at] = digest.value();
    }
    #[cfg(feature = "control")]
    if report.written.host() > 0 {
        // The control: a value that depends on the target, which the comparison has to see.
        let mut bytes = out[0].bytes();
        bytes[0] ^= core::mem::size_of::<usize>() as u8;
        out[0] = Word::of(bytes);
    }
    report
}

/// A store the loop can build and fill.
trait Fill: Store<Plane> + Sized {
    fn make() -> Maybe<Self>;
    fn load(&mut self, at: Index, word: Word);
}

macro_rules! fill {
    ($($store:ident),*) => {$(
        impl Fill for $store<Plane, 1, CELLS> {
            fn make() -> Maybe<Self> {
                $store::new(SHAPE, [L::FORMAT])
            }

            fn load(&mut self, at: Index, word: Word) {
                self.set(USize::from_host(0), at, word);
            }
        }
    )*};
}

fill!(Columns, Records);

fn word(v: i64) -> Word {
    L::from_raw(v).cell().word()
}

/// One run of `FRAMES` frames, its digests into `digests`.
fn stream<S: Fill, V: Visit>(visit: &V, morsel: Count, digests: &mut [Digest]) -> Report {
    let none = Report {
        written: Count::ZERO,
        refused: Maybe::Isnt,
    };
    let Maybe::Is(mut store) = S::make() else {
        return none;
    };
    for at in 0 .. CELLS {
        // A fixed scramble over the lane's range, so some cells start saturated.
        let v = ((at as i64 * 7919 + 104_729) % 65_521) - 32_760;
        store.load(Index::from_host(at), word(v));
    }
    let role = Role {
        column: USize::from_host(0),
        format: L::FORMAT,
    };
    let roles = [role; 6];
    let params = [Coefficient {
        format: L::FORMAT,
        word:   word(RATE),
    }];
    let bindings = [Binding {
        unit:   UnitId::new(USize::from_host(0)),
        kernel: Diffuse::<L>::ID,
        roles:  &roles,
        params: &params,
    }];
    let Outcome::Ok(bound) = bind::<Plane, List, S>(&bindings, &store) else {
        return none;
    };
    let mut out = Run::new(digests);
    let refused = match run(
        &bound,
        &mut store,
        visit,
        morsel,
        Count::from_host(FRAMES),
        &mut out,
    ) {
        Outcome::Ok(()) => Maybe::Isnt,
        Outcome::Err(Refusal::DigestsFull {
            frame,
        }) => Maybe::Is(frame),
    };
    Report {
        written: out.frames(),
        refused,
    }
}
