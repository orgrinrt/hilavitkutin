//! A kernel asking where it is: `at` is handed values and nothing else.
use hilavitkutin_contracts::domain::Own;
use hilavitkutin_contracts::kernel::{KernelId, ValueKernel};
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Index, Int, USize};
use hilavitkutin_frozen_authors::{Heat, Plane};

type L = Int<16>;
struct Where;

impl ValueKernel for Where {
    type Domain = Plane;
    type Reads = Cons<Own<Heat<L>>, Empty>;
    type Roles = Cons<Heat<L>, Empty>;
    type Writes = Cons<Own<Heat<L>>, Empty>;

    const ID: KernelId = KernelId::new(USize::from_host(7));

    fn at((here, ()): (L, ()), at: Index) -> (L, ()) {
        let _ = at;
        (here, ())
    }
}

fn main() {}
