//! A unit handed another store's view: a Unit has no public field, so its handles stay with the view they were checked against.
use hilavitkutin_contracts::bind::{Binding, Unit, UnitFn, bind};
use hilavitkutin_contracts::domain::Value;
use hilavitkutin_contracts::kernel::ValueKernel;
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Count, Int};
use hilavitkutin_contracts::store::{Read, Store};
use hilavitkutin_contracts::{Maybe, Outcome};
use hilavitkutin_frozen_authors::{Diffuse, Extent, Plane};
use hilavitkutin_frozen_implementors::Columns;

type L = Int<16>;
type List = Cons<Diffuse<L>, Empty>;

struct Swap<'o, R> {
    other: &'o R,
}

impl<R: Read<Plane>> UnitFn<Plane, R> for Swap<'_, R> {
    fn unit<'v, K: ValueKernel<Domain = Plane>>(&mut self, unit: Unit<'v, K, R>)
    where
        R: 'v,
    {
        let _moved = Unit::<K, R> {
            committed: self.other,
            ..unit
        };
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
    let Maybe::Is(other) = Columns::<Plane, 1, 4>::new(shape, [L::FORMAT]) else {
        return;
    };
    let other = other.committed();
    let bindings: [Binding<'_>; 0] = [];
    if let Outcome::Ok(bound) = bind::<Plane, List, _, _>(&bindings, &mut store) {
        bound.each(&mut Swap {
            other: &other,
        });
    }
}
