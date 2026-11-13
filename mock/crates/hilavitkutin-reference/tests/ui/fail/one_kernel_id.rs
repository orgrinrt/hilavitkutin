//! Two kernels of one list declaring one id: binding against the list fails the build.
use hilavitkutin_contracts::bind::{Binding, bind};
use hilavitkutin_contracts::domain::{Own, Value};
use hilavitkutin_contracts::kernel::{KernelId, ValueKernel};
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Count, Int, USize};
use hilavitkutin_contracts::{Maybe, Outcome};
use hilavitkutin_frozen_authors::{Diffuse, Extent, Heat, Plane};
use hilavitkutin_frozen_implementors::Columns;

type L = Int<16>;
struct Keep;

impl ValueKernel for Keep {
    type Domain = Plane;
    type Reads = Cons<Own<Heat<L>>, Empty>;
    type Roles = Cons<Heat<L>, Empty>;
    type Writes = Cons<Own<Heat<L>>, Empty>;

    const ID: KernelId = KernelId::new(USize::from_host(1));

    fn at(reads: (L, ())) -> (L, ()) {
        reads
    }
}

fn main() {
    let shape = Extent {
        east:  Count::from_host(2),
        north: Count::from_host(2),
    };
    let Maybe::Is(mut store) = Columns::<Plane, 1, 4>::new(shape, [L::FORMAT]) else {
        return;
    };
    let bindings: [Binding<'_>; 0] = [];
    let bound = bind::<Plane, Cons<Diffuse<L>, Cons<Keep, Empty>>, _, _>(&bindings, &mut store);
    assert!(matches!(bound, Outcome::Ok(_)));
}
