//! The twin of `ui/fail/read_undeclared.rs`, reading `Vel`, which its read
//! list holds.

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::values::{ReadVals, WriteVals};
use hilavitkutin_api::work_unit::WorkUnit;
use hilavitkutin_api::{RecordOp, rec};
use unit::{Pos, Vel};

struct Twin;

impl BuilderInput for Twin {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Twin {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;
}

impl RecordOp for Twin {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let pos = read.get(Column::<Pos>::new());
        let vel = read.get(Column::<Vel>::new());
        rec![Pos(pos.0 + vel.0)]
    }
}

fn main() {}
