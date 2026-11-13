//! `Plane`: a domain of two axes, east and north, keyed north first.

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::domain::{Domain, Offset};
use hilavitkutin_contracts::num::{Count, Index};

/// Records on a grid of two axes.
pub struct Plane;

/// A record's key: its place on each axis. Key order is north first, then east.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Spot {
    pub north: Index,
    pub east:  Index,
}

/// The run-time extent of a plane: how many places along each axis.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Extent {
    pub east:  Count,
    pub north: Count,
}

/// A move along one axis.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shift {
    Back(Count),
    Still,
    Fore(Count),
}

/// A move along both axes, plain data.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Toward {
    pub east:  Shift,
    pub north: Shift,
}

const fn shift(at: Index, by: Shift, extent: Count) -> Maybe<Index> {
    let moved = match by {
        Shift::Back(n) => {
            match at.before(n) {
                Maybe::Is(there) => there,
                Maybe::Isnt => return Maybe::Isnt,
            }
        },
        Shift::Still => at,
        Shift::Fore(n) => at.after(n),
    };
    if moved.within(extent).get() { Maybe::Is(moved) } else { Maybe::Isnt }
}

impl Domain for Plane {
    type Key = Spot;
    type Shape = Extent;
    type Step = Toward;

    fn first(shape: Extent) -> Maybe<Spot> {
        if Index::FIRST
            .within(shape.east)
            .and(Index::FIRST.within(shape.north))
            .get()
        {
            Maybe::Is(Spot {
                north: Index::FIRST,
                east:  Index::FIRST,
            })
        } else {
            Maybe::Isnt
        }
    }

    fn next(shape: Extent, key: Spot) -> Maybe<Spot> {
        let east = key.east.next();
        if east.within(shape.east).get() {
            return Maybe::Is(Spot {
                north: key.north,
                east,
            });
        }
        let north = key.north.next();
        if north.within(shape.north).get() {
            Maybe::Is(Spot {
                north,
                east: Index::FIRST,
            })
        } else {
            Maybe::Isnt
        }
    }

    fn reach(shape: Extent, key: Spot, step: Toward) -> Maybe<Spot> {
        match (
            shift(key.east, step.east, shape.east),
            shift(key.north, step.north, shape.north),
        ) {
            (Maybe::Is(east), Maybe::Is(north)) => {
                Maybe::Is(Spot {
                    north,
                    east,
                })
            },
            _ => Maybe::Isnt,
        }
    }
}

macro_rules! offsets {
    ($($name:ident $doc:literal { east: $east:expr, north: $north:expr },)*) => {$(
        #[doc = $doc]
        pub struct $name;

        impl Offset for $name {
            type Domain = Plane;
            const STEP: Toward = Toward { east: $east, north: $north };
        }
    )*};
}

offsets! {
    East "One place east." { east: Shift::Fore(Count::ONE), north: Shift::Still },
    West "One place west." { east: Shift::Back(Count::ONE), north: Shift::Still },
    North "One place north." { east: Shift::Still, north: Shift::Fore(Count::ONE) },
    South "One place south." { east: Shift::Still, north: Shift::Back(Count::ONE) },
}
