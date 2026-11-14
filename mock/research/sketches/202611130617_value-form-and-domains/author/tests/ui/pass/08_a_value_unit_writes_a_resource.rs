use api::{BuilderInput, Column, Cons, Empty, RecordDispatch, RecordOp, Vals, rec};
use author::{POS, Pos, RATE, Rate};

struct Sum;

impl BuilderInput for Sum {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Sum {
    // The one value is read at every record.
    type Read = Cons<Column<Pos>, Cons<Rate, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        rec![Pos(r.get(POS).0 + r.get(RATE))]
    }
}

fn main() {}
