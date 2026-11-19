//! Three units in the block form, written against the api and the block
//! form's contract alone.
//!
//! Each declares its read list, its write list and its hint, and writes its
//! body against what it is handed. No line here names a context type, and no
//! line names the engine.

#![no_std]

use arvo::USize;
use derived::{Block, Records};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::member::Written;
use hilavitkutin_api::store::Column;

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

/// Writes each record's index inside its window.
#[derive(Copy, Clone)]
pub struct Stamp;

impl Block for Stamp {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Empty;
    type Write = Cons<Column<Seen>, Empty>;

    fn run(&self, records: &impl Records<Self::Read, Self::Write>) {
        records.each(|i| {
            // SAFETY: `i` comes from the window's own loop.
            unsafe { records.write(i, Seen(i.0 as i64)) };
        });
    }
}

/// The position plus the velocity, in place.
#[derive(Copy, Clone)]
pub struct Drift;

impl Block for Drift {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;

    fn run(&self, records: &impl Records<Self::Read, Self::Write>) {
        let mut i = USize(0);
        while i.0 < records.len().0 {
            // SAFETY: `i` is below the window's length.
            unsafe {
                let pos = records.read::<Pos, _>(i);
                let vel = records.read::<Vel, _>(i);
                records.write(i, Pos(pos.0 + vel.0));
            }
            i = USize(i.0 + 1);
        }
    }
}

/// Twice the position, as `Drift` left it in this frame.
#[derive(Copy, Clone)]
pub struct Warm;

impl Block for Warm {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Empty>;
    type Write = Cons<Written<Column<Glow>>, Empty>;

    fn run(&self, records: &impl Records<Self::Read, Self::Write>) {
        records.each(|i| {
            // SAFETY: `i` comes from the window's own loop.
            unsafe {
                let pos = records.read::<Pos, _>(i);
                records.write(i, Glow(pos.0 + pos.0));
            }
        });
    }
}
