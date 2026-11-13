// Close unit. Asks its landed stores for a view; the twin asks for their rows.
#![allow(unused)]
use author::{Landings, World};
use engine::{CloseUnit, Frame, Landed};
use notko_hlist::{Cons, Empty};

struct Peeker;

impl CloseUnit<World> for Peeker {
    type Stores = Cons<Landings, Empty>;

    fn close(landed: Landed<'_, World, Self::Stores>) {
        let _ = landed.view();
    }
}

fn main() {
    Frame::<World, 2>::new().close(Peeker);
}
