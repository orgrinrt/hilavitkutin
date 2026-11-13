//! Twin: the table is bound first, and the run takes what the bind returned.
use hilavitkutin_contracts::bind::{Binding, bind};
use hilavitkutin_contracts::digest::Digest;
use hilavitkutin_contracts::domain::Value;
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Count, Int};
use hilavitkutin_contracts::{Maybe, Outcome};
use hilavitkutin_frozen_authors::{Diffuse, Extent, Plane};
use hilavitkutin_frozen_implementors::{Columns, Forward};
use hilavitkutin_reference::{Run, run};

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
    let mut digests = [Digest::new(); 1];
    let mut out = Run::new(&mut digests);
    if let Outcome::Ok(bound) = bind::<Plane, Cons<Diffuse<L>, Empty>, _>(&bindings, &store) {
        let _ = run(
            &bound,
            &mut store,
            &Forward,
            Count::ONE,
            Count::ONE,
            &mut out,
        );
    }
}
