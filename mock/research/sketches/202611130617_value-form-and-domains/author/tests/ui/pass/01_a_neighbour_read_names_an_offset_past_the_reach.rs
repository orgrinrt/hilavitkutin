use api::{BuilderInput, Cons, Empty, Near, RecordDispatch, RecordOp, Shift, Vals, rec};
use author::{Cold, East, HERE, HeatAt};

// One east. The plane's reach lists it.
type Far = Cons<Shift<East, 1>, Empty>;

struct Leap;

impl BuilderInput for Leap {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Leap {
    type Read = Cons<HeatAt, Cons<Near<HeatAt, Far, Cold>, Empty>>;
    type Write = Cons<HeatAt, Empty>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        rec![r.get(HERE)]
    }
}

fn main() {}
