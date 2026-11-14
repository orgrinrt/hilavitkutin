use api::{BuilderInput, Cons, Empty, RecordDispatch, RecordOp, Vals, rec};
use author::{HERE, HeatAt};

struct Plain;

impl BuilderInput for Plain {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Plain {
    // The value form has no place to name a context.
    type Ctx<'frame> = ();
    type Read = Cons<HeatAt, Empty>;
    type Write = Cons<HeatAt, Empty>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        rec![r.get(HERE)]
    }
}

fn main() {}
