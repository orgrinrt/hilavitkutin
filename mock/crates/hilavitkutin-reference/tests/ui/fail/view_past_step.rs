//! A committed view kept past the step: the swap needs the store the view still borrows.
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
    store.swap();
    let extent = committed.extent();
    assert_eq!(extent, Count::from_host(4));
}
