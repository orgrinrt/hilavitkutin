//! Twin: the committed view is read inside the step, and the swap comes after it is gone.
use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::domain::Value;
use hilavitkutin_contracts::num::{Count, Int};
use hilavitkutin_contracts::store::{Read, Store};
use hilavitkutin_frozen_authors::{Extent, Plane};
use hilavitkutin_frozen_implementors::Columns;

fn main() {
    let shape = Extent {
        east:  Count::from_host(2),
        north: Count::from_host(2),
    };
    let Maybe::Is(mut store) = Columns::<Plane, 1, 4>::new(shape, [Int::<16>::FORMAT]) else {
        return;
    };
    let (committed, _staged) = store.step();
    let extent = committed.extent();
    store.swap();
    assert_eq!(extent, Count::from_host(4));
}
