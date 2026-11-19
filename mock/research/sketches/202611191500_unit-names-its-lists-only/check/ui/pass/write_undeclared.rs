//! The twin of `ui/fail/write_undeclared.rs`, returning a `Glow`, which its
//! write list holds.

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::member::Written;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::values::{ReadVals, WriteVals};
use hilavitkutin_api::work_unit::WorkUnit;
use hilavitkutin_api::{RecordOp, rec};
use unit::{Glow, Pos};

struct Twin;

impl BuilderInput for Twin {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Twin {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Empty>;
    type Write = Cons<Written<Column<Glow>>, Empty>;
}

impl RecordOp for Twin {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let pos = read.get(Column::<Pos>::new());
        rec![Glow(pos.0 + pos.0)]
    }
}

fn main() {}
