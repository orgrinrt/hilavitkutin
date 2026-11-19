//! Attempt d2: a unit that is a declaration alone, with no body of either
//! kind, registered and run. `WorkUnit` no longer asks for a body, so the
//! refusal has to come from the run.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Column, StartsFromDefault};
use hilavitkutin_api::work_unit::WorkUnit;
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};
use unit::Pos;

struct Bodiless;

impl BuilderInput for Bodiless {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Bodiless {
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Pos>, Empty>;
    type Write = Cons<Column<Pos>, Empty>;
}

fn main() {
    let memory = StaticBumpMemory::new(vec![MaybeUninit::new(0u8); 1 << 16].leak());
    let mut sched = Scheduler::builder()
        .with(Column::<Pos>::start())
        .with(Bodiless)
        .build(ArenaColumnStorage::new(memory), USize(8))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let _ = sched.run();
}
