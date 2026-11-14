use api::{Accum, BuilderInput, Column, Cons, Empty, RecordDispatch, RecordOp, Vals, rec};
use author::{Crossed, POS, Pos};

struct Half;

impl BuilderInput for Half {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Half {
    type Read = Cons<Column<Pos>, Empty>;
    type Write = Cons<Column<Pos>, Cons<Accum<Crossed>, Empty>>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        // No append at this record, said as a value.
        rec![r.get(POS), None]
    }
}

fn main() {}
