//! Attempt d1: a unit that offers both bodies, a positional `execute` and a
//! value function, registered and run. The engine picks the body by which the
//! unit offers, so a unit offering both leaves it nothing to pick by.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::CtxFor;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Column, StartsFromDefault};
use hilavitkutin_api::values::{ReadVals, WriteVals};
use hilavitkutin_api::work_unit::{Positional, WorkUnit};
use hilavitkutin_api::{RecordOp, rec};
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};
use unit::Pos;

struct Both;

impl BuilderInput for Both {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Both {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Empty>;
    type Write = Cons<Column<Pos>, Empty>;
}

impl Positional for Both {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|r| self.apply(r));
    }
}

impl RecordOp for Both {
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
        let pos = read.get(Column::<Pos>::new());
        rec![Pos(pos.0 + pos.0)]
    }
}

fn main() {
    let memory = StaticBumpMemory::new(vec![MaybeUninit::new(0u8); 1 << 16].leak());
    let mut sched = Scheduler::builder()
        .with(Column::<Pos>::start())
        .with(Both)
        .build(ArenaColumnStorage::new(memory), USize(8))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let _ = sched.run();
}
