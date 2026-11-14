use api::{BuilderInput, Column, Cons, Empty, RecordDispatch, RecordOp, Vals, rec};
use author::{POS, Pos, Rate};

struct Sum;

impl BuilderInput for Sum {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Sum {
    type Read = Cons<Column<Pos>, Empty>;
    // One value, written once a record: the last record would win.
    type Write = Cons<Rate, Empty>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        rec![r.get(POS).0]
    }
}

fn main() {}
