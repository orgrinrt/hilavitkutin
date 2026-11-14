//! The plane, the values its stores hold, and the numbers the units compare
//! with.

use arvo::{Bool, Cap, Hot, USize, Uint};
use arvo_tensor::cap_size;
use hilavitkutin_api::store::{Buffered, Exchange, Snapshot};

/// The number every value here is made of.
pub type Num = Uint<32, Hot>;

/// How many places the plane is wide: a step north is this many records on.
pub const WIDE: Cap = arvo_tensor::cap(8);
/// A step east.
pub const ONE: Cap = arvo_tensor::cap(1);
/// How many lines of `WIDE` places the plane holds.
pub const TALL: USize = USize(6);
/// How many records the plane is.
pub const RECORDS: USize = USize(cap_size(WIDE) * TALL.0);

/// Where a `Pos` counts as having crossed.
pub const THRESHOLD: Num = Num::from_raw(40);
/// The heat at which the plane is overheated.
pub const LIMIT: Num = Num::from_raw(500);

pub(crate) const ZERO: Num = Num::from_raw(0);

/// The higher of two numbers.
pub(crate) const fn higher(one: Num, other: Num) -> Num {
    if one.to_raw() >= other.to_raw() { one } else { other }
}

/// A position, moved in place every frame.
#[derive(Copy, Clone, PartialEq)]
pub struct Pos(pub Num);

/// A velocity, never written.
#[derive(Copy, Clone, PartialEq)]
pub struct Vel(pub Num);

/// The heat of a place.
#[derive(Copy, Clone, PartialEq)]
pub struct Heat(pub Num);

/// Which sides of the plane a place lies on.
#[derive(Copy, Clone, PartialEq)]
pub struct Edge {
    /// The place is the first of its line.
    pub west: Bool,
    /// The place is the last of its line.
    pub east: Bool,
}

/// A position that crossed the threshold in the frame.
#[derive(Copy, Clone, PartialEq)]
pub struct Crossed(pub Num);

/// How many frames found the plane overheated.
#[derive(Copy, Clone, PartialEq)]
pub struct Alarms(pub Num);

/// A level that spreads along the line until it is even.
#[derive(Copy, Clone, PartialEq)]
pub struct Level(pub Num);

/// The highest level a record has held.
#[derive(Copy, Clone, PartialEq)]
pub struct Peak(pub Num);

/// Twice the position.
#[derive(Copy, Clone, PartialEq)]
pub struct Glow(pub Num);

/// The glow plus the limit.
#[derive(Copy, Clone, PartialEq)]
pub struct Ash(pub Num);

/// Fired in a frame that began with a place at or over the limit.
pub struct Overheat;

/// The heat of each place: read as the frame began, traded at the boundary.
pub type Field = Buffered<Heat, Exchange>;

/// The level of each record: read as the frame began, copied at the boundary.
pub type Pool = Buffered<Level, Snapshot>;
