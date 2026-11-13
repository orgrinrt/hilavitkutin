//! A kernel reading a neighbour it did not declare: its reads are only what its list hands it.
use hilavitkutin_contracts::domain::Own;
use hilavitkutin_contracts::kernel::{KernelId, ValueKernel};
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Int, Lane, USize};
use hilavitkutin_frozen_authors::{Heat, Plane};

type L = Int<16>;
struct Shift;

impl ValueKernel for Shift {
    type Domain = Plane;
    type Reads = Cons<Own<Heat<L>>, Empty>;
    type Roles = Cons<Heat<L>, Empty>;
    type Writes = Cons<Own<Heat<L>>, Empty>;

    const ID: KernelId = KernelId::new(USize::from_host(7));

    fn at((here, (east, ())): (L, (L, ()))) -> (L, ()) {
        (L::narrow(L::wide_add(here.widen(), east.widen())), ())
    }
}

fn main() {}
