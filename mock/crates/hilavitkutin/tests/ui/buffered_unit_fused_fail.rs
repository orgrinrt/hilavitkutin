// A unit reads and writes a buffered column and declares its value function
// (`RecordOp`). The fused entry folds a carrier of units over plain columns
// into a chain that threads values in registers, and a buffered column is two
// reservations the boundary trades, which the chain does not carry. The unit
// is refused at compile time; through `run` and `run_parallel` it runs, as the
// buffered modules of `tests/host_import.rs` show.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Buffered, Snapshot};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{ReadVals, RecordOp, WriteVals, rec};
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};

#[derive(Copy, Clone, PartialEq)]
struct A(u32);

type Col = Buffered<A, Snapshot>;
type Cols = ColPtrCons<A, ColPtrNil, Snapshot>;
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

fn main() {
    let provider = StaticBumpMemory::new(std::vec![MaybeUninit::new(0); 4096].leak());
    let mut scheduler = Scheduler::builder()
        .with(Col::start_from(A(0)))
        .with(Raise)
        .build(ArenaColumnStorage::new(provider), USize(4))
        .unwrap_or_else(|_| panic!("build should succeed"));
    assert!(matches!(scheduler.run_fused(), notko::Outcome::Ok(())));
}
