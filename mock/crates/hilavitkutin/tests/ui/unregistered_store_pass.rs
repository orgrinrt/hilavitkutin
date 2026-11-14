// A unit reads `Resource<InA>` and writes what it read into `Column<Out>`, and
// the builder registers both, so `build` takes the unit and one frame runs over
// four records. Each record of `Out` holds the resource's value afterwards, so
// the unit ran and read the registered value. The twin
// `unregistered_store_fail.rs` differs only by the `.with(Resource::new(InA(7)))`
// line, which it leaves out, and is refused for it.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    SnapCons,
    SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::store::{Column, Resource};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{ResourceRead, rec};
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};

const RECORDS: usize = 4;

#[derive(Copy, Clone)]
struct InA(u32);

impl hilavitkutin_api::footprint::ResourceFootprint for InA {
    const L1_BYTES: USize = USize(0);
}

impl ResourceRead for InA {
    type Val<'a> = Self;

    fn read<'a>(&'a self) -> Self {
        *self
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Out(u32);

type ReadA = Cons<Resource<InA>, Empty>;
type WriteOut = Cons<Column<Out>, Empty>;

struct ReadsInA;

impl BuilderInput for ReadsInA {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for ReadsInA {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadA,
        WriteOut,
        SnapCons<InA, SnapNil>,
        ColPtrNil,
        ColPtrCons<Out, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadA;
    type Write = WriteOut;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|read| {
            let a: InA = read.get(Resource::<InA>::default());
            rec![Out(a.0)]
        });
    }
}

fn store(provider: StaticBumpMemory) -> ArenaColumnStorage<StaticBumpMemory> {
    ArenaColumnStorage::new(provider)
}

fn main() {
    let provider = StaticBumpMemory::new(std::vec![MaybeUninit::new(0); 4096].leak());
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(InA(7)))
        .with(Column::<Out>::start_from(Out(0)))
        .with(ReadsInA)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build refused a unit whose read set is registered"));
    assert!(matches!(scheduler.run(), notko::Outcome::Ok(())));
    // The column registered last is the bindings head.
    let out = scheduler.__bindings().__ptr().as_ptr();
    for i in 0 .. RECORDS {
        // SAFETY: `Out` holds `RECORDS` reserved records and no pass is running.
        let got: Out = unsafe { out.add(i).read() };
        assert_eq!(got.0, 7, "record {i} holds the resource the unit read");
    }
}
