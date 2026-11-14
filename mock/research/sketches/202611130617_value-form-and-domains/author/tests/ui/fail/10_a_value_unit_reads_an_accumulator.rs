use api::{Accum, BuilderInput, Column, Cons, Empty, RecordDispatch, RecordOp, Vals, rec};
use author::{Crossed, POS, Pos};

struct Peek;

impl BuilderInput for Peek {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Peek {
    // An accumulator has no value at a record.
    type Read = Cons<Column<Pos>, Cons<Accum<Crossed>, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        rec![r.get(POS)]
    }
}

fn main() {}
