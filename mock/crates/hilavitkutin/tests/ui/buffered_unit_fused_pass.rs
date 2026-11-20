// The twin of `buffered_unit_fused_fail.rs` over a plain column: the same
// unit, with its value function, over `Column<A>`, which the fused entry runs.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{ReadVals, RecordOp, WriteVals, rec};
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};

#[derive(Copy, Clone, PartialEq)]
struct A(u32);

type Col = Column<A>;
type Cols = ColPtrCons<A, ColPtrNil>;
type Bare = Cons<Col, Empty>;

#[derive(Copy, Clone)]
struct Raise;

impl BuilderInput for Raise {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Raise {
    type Ctx<'frame> = EngineCtx<'frame, Bare, Bare, SnapNil, Cols, Cols, AccPtrNil>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Bare;
    type Write = Bare;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|read| self.apply(read));
    }
}

impl RecordOp for Raise {
    fn apply(&self, read: &ReadVals<'_, Bare>) -> WriteVals<Bare> {
        rec![A(read.get(Col::new()).0 + 1)]
    }
}

fn store(provider: StaticBumpMemory) -> ArenaColumnStorage<StaticBumpMemory> {
    ArenaColumnStorage::new(provider)
}

fn main() {
    let provider = StaticBumpMemory::new(std::vec![MaybeUninit::new(0); 4096].leak());
    let mut scheduler = Scheduler::builder()
        .with(Col::start_from(A(0)))
        .with(Raise)
        .build(store(provider), USize(4))
        .unwrap_or_else(|_| panic!("build should succeed"));
    assert!(matches!(scheduler.run_fused(), notko::Outcome::Ok(())));
}
