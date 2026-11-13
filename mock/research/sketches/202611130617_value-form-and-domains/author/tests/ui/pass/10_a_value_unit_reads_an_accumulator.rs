use api::{BuilderInput, Column, Cons, Empty, RecordDispatch, RecordOp, Vals, rec};
use author::{POS, Pos};

struct Peek;

impl BuilderInput for Peek {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Peek {
    type Read = Cons<Column<Pos>, Empty>;
    type Write = Cons<Column<Pos>, Empty>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        rec![r.get(POS)]
    }
}

fn main() {}
