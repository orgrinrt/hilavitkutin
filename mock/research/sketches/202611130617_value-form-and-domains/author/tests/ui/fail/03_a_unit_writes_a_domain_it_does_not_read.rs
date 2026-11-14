use api::{BuilderInput, Column, Cons, Empty, RecordDispatch, RecordOp, Vals, rec};
use author::{HERE, HeatAt, Pos};

struct Across;

impl BuilderInput for Across {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Across {
    type Read = Cons<HeatAt, Empty>;
    // Reads the plane, writes a column of the default domain.
    type Write = Cons<Column<Pos>, Empty>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        rec![Pos(r.get(HERE).0)]
    }
}

fn main() {}
