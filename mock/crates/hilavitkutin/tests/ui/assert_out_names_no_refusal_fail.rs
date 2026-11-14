// A consumer of the assert configuration, `DefaultRunCfg<Assert>`, cannot name
// `AccumRefused` in its run's output: under that choice `run` returns `()` and
// panics at the close when an accumulator refused, so there is no refusal value
// to bind. The twin `assert_out_names_no_refusal_pass.rs` differs only by the
// type the run's output is bound to, and the `AccumRefused` import that type needs.
use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{ColPtrNil, EngineCtx, SnapCons, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::run_cfg::DefaultRunCfg;
use hilavitkutin_api::store::Resource;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{AccumRefused, Assert};
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};

#[derive(Copy, Clone)]
struct InA(u32);

impl hilavitkutin_api::footprint::ResourceFootprint for InA {
    const L1_BYTES: USize = USize(0);
}

type ReadA = Cons<Resource<InA>, Empty>;

struct ReadsInA;

impl BuilderInput for ReadsInA {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for ReadsInA {
    type Ctx<'frame> =
        EngineCtx<'frame, ReadA, Empty, SnapCons<InA, SnapNil>, ColPtrNil, ColPtrNil>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadA;
    type Write = Empty;

    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {}
}

fn store(provider: StaticBumpMemory) -> ArenaColumnStorage<StaticBumpMemory> {
    ArenaColumnStorage::new(provider)
}

fn main() {
    let provider = StaticBumpMemory::new(std::vec![MaybeUninit::new(0); 4096].leak());
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(InA(1)))
        .with(ReadsInA)
        .build_with::<DefaultRunCfg<Assert>, _, _>(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let _: notko::Outcome<(), AccumRefused> = scheduler.run();
}
