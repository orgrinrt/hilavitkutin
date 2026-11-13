//! Twin: the member's column is in the role list.
use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::bind::{Binding, bind};
use hilavitkutin_contracts::domain::{Column, Own, Value};
use hilavitkutin_contracts::kernel::{KernelId, ValueKernel};
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Additive, Count, Identity, Int, USize};
use hilavitkutin_frozen_authors::{Extent, Heat, Plane};
use hilavitkutin_frozen_implementors::Columns;

type L = Int<16>;

struct Flux;

impl Column for Flux {
    type Domain = Plane;
    type Value = L;

    const OUTSIDE: L = <L as Identity<Additive>>::IDENTITY;
}

struct Keep;

impl ValueKernel for Keep {
    type Domain = Plane;
    type Reads = Cons<Own<Heat<L>>, Empty>;
    type Roles = Cons<Flux, Cons<Heat<L>, Empty>>;
    type Writes = Cons<Own<Heat<L>>, Empty>;

    const ID: KernelId = KernelId::new(USize::from_host(7));

    fn at(reads: (L, ())) -> (L, ()) {
        reads
    }
}

fn main() {
    let shape = Extent {
        east:  Count::from_host(2),
        north: Count::from_host(2),
    };
    let Maybe::Is(mut store) = Columns::<Plane, 2, 4>::new(shape, [L::FORMAT; 2]) else {
        return;
    };
    let bindings: [Binding<'_>; 0] = [];
    let _ = bind::<Plane, Cons<Keep, Empty>, _, _>(&bindings, &mut store);
    let _ = Flux::OUTSIDE;
}
