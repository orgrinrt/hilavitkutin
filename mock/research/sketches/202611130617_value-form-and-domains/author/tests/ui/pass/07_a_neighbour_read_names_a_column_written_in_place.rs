use api::{Buffered, BuilderInput, Cons, Empty, Near, RecordDispatch, RecordOp, Vals, rec};
use author::{Cold, East1, Heat, Plane};

// Frame-buffered: its neighbours are the committed frame for the whole pass.
type Field = Buffered<Heat, Plane>;

struct Smear;

impl BuilderInput for Smear {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Smear {
    type Read = Cons<Near<Field, East1, Cold>, Empty>;
    type Write = Cons<Field, Empty>;

    fn apply(&self, _: &Vals<Self::Read>) -> Vals<Self::Write> {
        rec![Heat(0)]
    }
}

fn main() {}
