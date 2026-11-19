//! Attempt f2: the plain function of attempt f1, with the lists carried as a
//! marker parameter of the registering trait and inferred at the call.
//!
//! `Registers<(Rd, Wr)>` stands in for a registration trait with a marker
//! parameter. The api's `BuilderInput` has none, so this shows that the lists
//! of a function can be inferred where the trait leaves room for them, and
//! nothing about `BuilderInput` as it is.

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

trait Registers<Lists> {}

impl<F, Rd, Wr> Registers<(Rd, Wr)> for F
where
    Rd: ReadSet,
    Wr: WriteSet,
    F: for<'a> Fn(&ReadVals<'a, Rd>) -> WriteVals<Wr>,
{
}

fn takes<Lists>(_unit: impl Registers<Lists>) {}

fn main() {
    takes(drift);
}
