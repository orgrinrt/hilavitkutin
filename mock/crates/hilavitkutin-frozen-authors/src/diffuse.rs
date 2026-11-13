//! Author 1: `Diffuse`, a value kernel over `Plane` generic over its lane.
//!
//! At each place the new heat is the old heat, plus the east and north neighbours' heat, minus
//! the west and south neighbours', plus the rate: six terms summed exactly in the lane's
//! accumulator and adapted back once. A neighbour outside the plane reads as the lane's zero.

use core::marker::PhantomData;

use hilavitkutin_contracts::domain::{Column, Near, Own, Param, Resource, Value};
use hilavitkutin_contracts::kernel::{KernelId, ValueKernel};
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Additive, Identity, Lane, USize};

use crate::plane::{East, North, Plane, South, West};

/// The heat at a place.
pub struct Heat<L>(PhantomData<fn() -> L>);

impl<L: Lane + Value + Identity<Additive>> Column for Heat<L> {
    type Domain = Plane;
    type Value = L;

    const OUTSIDE: L = L::IDENTITY;
}

/// The rate added at every place, one value for the step.
pub struct Rate<L>(PhantomData<fn() -> L>);

impl<L: Lane + Value> Resource for Rate<L> {
    type Value = L;
}

/// The kernel.
pub struct Diffuse<L>(PhantomData<fn() -> L>);

/// The terms `Diffuse` sums: it may not sum more than its lane holds exactly.
const TERMS: USize = USize::from_host(6);

impl<L: Lane + Value + Identity<Additive>> ValueKernel for Diffuse<L> {
    type Domain = Plane;
    type Reads = Cons<
        Own<Heat<L>>,
        Cons<
            Near<Heat<L>, East>,
            Cons<
                Near<Heat<L>, West>,
                Cons<Near<Heat<L>, North>, Cons<Near<Heat<L>, South>, Cons<Param<Rate<L>>, Empty>>>,
            >,
        >,
    >;
    type Roles = Cons<Heat<L>, Empty>;
    type Writes = Cons<Own<Heat<L>>, Empty>;

    const ID: KernelId = {
        assert!(
            TERMS.le(L::MAX_TERMS).get(),
            "the lane cannot hold Diffuse's sum exactly"
        );
        KernelId::new(USize::from_host(1))
    };

    fn at(
        (here, (east, (west, (north, (south, (rate, ())))))): (L, (L, (L, (L, (L, (L, ())))))),
    ) -> (L, ()) {
        let sum = L::wide_add(here.widen(), east.widen());
        let sum = L::wide_sub(sum, west.widen());
        let sum = L::wide_add(sum, north.widen());
        let sum = L::wide_sub(sum, south.widen());
        let sum = L::wide_add(sum, rate.widen());
        (L::narrow(sum), ())
    }
}
