//! Attempt f1: a unit that is a plain function. Its signature names the two
//! lists and nothing else, and there is no impl at all:
//!
//! ```ignore
//! fn drift(read: &ReadVals<'_, R>) -> WriteVals<W> { .. }
//! ```
//!
//! For the builder to take it bare, something has to be implemented for every
//! such function, with the lists read off its signature. `Registers` stands in
//! for `BuilderInput` here, since a fixture cannot implement the api's trait
//! for a foreign type; the wall is the same in the api's own crate.

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::rec;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::values::{ReadSet, ReadVals, WriteSet, WriteVals};
use unit::{Pos, Vel};

type R = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
type W = Cons<Column<Pos>, Empty>;

fn drift(read: &ReadVals<'_, R>) -> WriteVals<W> {
    let pos = read.get(Column::<Pos>::new());
    let vel = read.get(Column::<Vel>::new());
    rec![Pos(pos.0 + vel.0)]
}

trait Registers {}

impl<F, Rd, Wr> Registers for F
where
    Rd: ReadSet,
    Wr: WriteSet,
    F: for<'a> Fn(&ReadVals<'a, Rd>) -> WriteVals<Wr>,
{
}

fn takes(_unit: impl Registers) {}

fn main() {
    takes(drift);
}
