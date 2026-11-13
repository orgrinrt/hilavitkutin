//! Twin: what the kernel needs of its lane is said on the impl, where every caller sees it.
use core::marker::PhantomData;

use hilavitkutin_contracts::domain::{Own, Value};
use hilavitkutin_contracts::kernel::{KernelId, ValueKernel};
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Additive, Identity, Lane, USize};
use hilavitkutin_frozen_authors::{Heat, Plane};

struct Keep<L>(PhantomData<L>);

impl<L: Lane + Value + Identity<Additive> + Ord> ValueKernel for Keep<L> {
    type Domain = Plane;
    type Reads = Cons<Own<Heat<L>>, Empty>;
    type Writes = Cons<Own<Heat<L>>, Empty>;

    const ID: KernelId = KernelId::new(USize::from_host(7));

    fn at((here, ()): (L, ())) -> (L, ()) {
        (here.max(L::IDENTITY), ())
    }
}

fn main() {}
