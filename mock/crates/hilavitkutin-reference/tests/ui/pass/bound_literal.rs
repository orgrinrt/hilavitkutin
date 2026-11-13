//! Twin: the table is made by the bind, which checks it.
use core::marker::PhantomData;

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::bind::{Binding, Bound, bind};
use hilavitkutin_contracts::domain::Value;
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Count, Int};
use hilavitkutin_frozen_authors::{Diffuse, Extent, Plane};
use hilavitkutin_frozen_implementors::Columns;

type L = Int<16>;
type List = Cons<Diffuse<L>, Empty>;

fn main() {
    let shape = Extent {
        east:  Count::from_host(2),
        north: Count::from_host(2),
    };
    let Maybe::Is(mut store) = Columns::<Plane, 1, 4>::new(shape, [L::FORMAT]) else {
        return;
    };
    let bindings: [Binding<'_>; 0] = [];
    let _: Maybe<Bound<'_, Plane, List, _, _>> = match bind(&bindings, &mut store) {
        hilavitkutin_contracts::Outcome::Ok(bound) => Maybe::Is(bound),
        hilavitkutin_contracts::Outcome::Err(_) => Maybe::Isnt,
    };
    let _ = PhantomData::<()>;
}
