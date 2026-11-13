//! A run asked of bindings that never passed the bind: it takes a checked table only.
use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::bind::Binding;
use hilavitkutin_contracts::digest::Digest;
use hilavitkutin_contracts::domain::Value;
use hilavitkutin_contracts::num::{Count, Int};
use hilavitkutin_frozen_authors::{Extent, Plane};
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
    let _ = run(
        &bindings,
        &mut store,
        &Forward,
        Count::ONE,
        Count::ONE,
        &mut out,
    );
}
