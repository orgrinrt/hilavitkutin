//! Two units that name their lists and nothing else, written against the api
//! alone.
//!
//! Each declares its read list, its write list and its hint, and its body is
//! a function from the values of the read list at one record to the values of
//! the write list at that record. No line here names a context type, a trait
//! the body is handed, or the engine.
//!
//! The two spellings are the same units. Without the feature `one-impl` a
//! unit is three impls the api already ships the traits of: `BuilderInput`,
//! `WorkUnit` (the declaration) and `RecordOp` (the body). With it a unit is
//! one impl of the api copy's `Unit`, and the three are granted by blankets.

#![no_std]

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::member::Written;
use hilavitkutin_api::rec;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::values::{ReadVals, WriteVals};

/// A position.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Pos(pub i64);

/// A velocity.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Vel(pub i64);

/// Twice a position.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Glow(pub i64);

/// The index a record had inside the window it was run in.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Seen(pub i64);

/// The position plus the velocity, in place.
#[derive(Copy, Clone)]
pub struct Drift;

/// Twice the position, as `Drift` left it in this frame.
#[derive(Copy, Clone)]
pub struct Warm;

#[cfg(not(feature = "one-impl"))]
mod three_impls {
    use hilavitkutin_api::RecordOp;
    use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
    use hilavitkutin_api::work_unit::WorkUnit;

    use super::*;

    impl BuilderInput for Drift {
        type Dispatch = UnitDispatch<Self>;
        type Init = Self;
    }

    impl WorkUnit for Drift {
        type Hint = (Immediate, Atomic, Normal);
        type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
        type Write = Cons<Column<Pos>, Empty>;
    }

    impl RecordOp for Drift {
        fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
            let pos = read.get(Column::<Pos>::new());
            let vel = read.get(Column::<Vel>::new());
            rec![Pos(pos.0 + vel.0)]
        }
    }

    impl BuilderInput for Warm {
        type Dispatch = UnitDispatch<Self>;
        type Init = Self;
    }

    impl WorkUnit for Warm {
        type Hint = (Immediate, Atomic, Normal);
        type Read = Cons<Column<Pos>, Empty>;
        type Write = Cons<Written<Column<Glow>>, Empty>;
    }

    impl RecordOp for Warm {
        fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
            let pos = read.get(Column::<Pos>::new());
            rec![Glow(pos.0 + pos.0)]
        }
    }
}

#[cfg(feature = "one-impl")]
mod one_impl {
    use hilavitkutin_api::unit::Unit;

    use super::*;

    impl Unit for Drift {
        type Hint = (Immediate, Atomic, Normal);
        type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
        type Write = Cons<Column<Pos>, Empty>;

        fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
            let pos = read.get(Column::<Pos>::new());
            let vel = read.get(Column::<Vel>::new());
            rec![Pos(pos.0 + vel.0)]
        }
    }

    impl Unit for Warm {
        type Hint = (Immediate, Atomic, Normal);
        type Read = Cons<Column<Pos>, Empty>;
        type Write = Cons<Written<Column<Glow>>, Empty>;

        fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
            let pos = read.get(Column::<Pos>::new());
            rec![Glow(pos.0 + pos.0)]
        }
    }
}
