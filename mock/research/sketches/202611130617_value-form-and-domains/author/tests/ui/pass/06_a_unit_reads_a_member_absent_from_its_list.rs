use api::{BuilderInput, Column, Cons, Empty, RecordDispatch, RecordOp, Vals, rec};
use author::{POS, Pos, VEL, Vel};

struct Step;

impl BuilderInput for Step {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Step {
    type Read = Cons<Column<Pos>, Cons<Column<Vel>, Empty>>;
    type Write = Cons<Column<Pos>, Empty>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        rec![Pos(r.get(POS).0 + r.get(VEL).0)]
    }
}

fn main() {}
