use api::{BuilderInput, Column, Cons, Empty, RecordDispatch, RecordOp, Vals, rec};
use author::{HERE, HeatAt, Pos};

struct Mixed;

impl BuilderInput for Mixed {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Mixed {
    // Heat is a value per place of the plane, a position a value per record.
    type Read = Cons<HeatAt, Cons<Column<Pos>, Empty>>;
    type Write = Cons<HeatAt, Empty>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        rec![r.get(HERE)]
    }
}

fn main() {}
