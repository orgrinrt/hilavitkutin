use api::{BuilderInput, Cons, Empty, RecordDispatch, RecordOp, Vals, rec};
use author::{HERE, HeatAt};
// The author's crate does not depend on the engine.
use engine::EngineCtx;

struct Plain;

impl BuilderInput for Plain {
    type Dispatch = RecordDispatch<Self>;
}

impl RecordOp for Plain {
    type Read = Cons<HeatAt, Empty>;
    type Write = Cons<HeatAt, Empty>;

    fn apply(&self, r: &Vals<Self::Read>) -> Vals<Self::Write> {
        rec![r.get(HERE)]
    }
}

fn main() {}
