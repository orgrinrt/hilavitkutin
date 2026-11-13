//! The store written while a table bound against it is live: the table holds the store.
use hilavitkutin_contracts::bind::{Binding, bind};
use hilavitkutin_contracts::domain::{Cell, Value};
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{ColumnNo, Count, Index, Int};
use hilavitkutin_contracts::{Maybe, Outcome};
use hilavitkutin_frozen_authors::{Diffuse, Extent, Plane};
use hilavitkutin_frozen_implementors::Columns;

type L = Int<16>;

fn main() {
    let shape = Extent {
        east:  Count::from_host(2),
        north: Count::from_host(2),
    };
    let Maybe::Is(mut store) = Columns::<Plane, 1, 4>::new(shape, [L::FORMAT]) else {
        return;
    };
    let bindings: [Binding<'_>; 0] = [];
    let Outcome::Ok(mut bound) =
        bind::<Plane, Cons<Diffuse<L>, Empty>, _, _>(&bindings, &mut store)
    else {
        return;
    };
    store.set(
        ColumnNo::from_host(0),
        Index::FIRST,
        L::from_raw(1).cell().word(),
    );
    bound.swap();
}
