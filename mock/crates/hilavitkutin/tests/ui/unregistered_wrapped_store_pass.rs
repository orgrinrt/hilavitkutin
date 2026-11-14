// A unit writes `Differs<Column<Height>>` and reads `Reach<Column<Slot>, ONE>`,
// and the builder registers both columns bare. `build` takes it: a wrapper in
// an access set is held by the store under it.
// The twin `unregistered_wrapped_store_fail.rs` differs only by the two
// `.with(Column::<..>::start())` lines.

use core::mem::MaybeUninit;

use arvo::{Cap, USize};
use hilavitkutin::dispatch::engine_ctx::{AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::store::{Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{Differs, Reach, rec};
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};

const ONE: Cap = arvo_tensor::cap(1);

#[derive(Copy, Clone, PartialEq, Default)]
struct Slot(u32);

#[derive(Copy, Clone, PartialEq, Default)]
struct Height(u32);

type Around = Cons<Reach<Column<Slot>, ONE>, Empty>;
type Changed = Cons<Differs<Column<Height>>, Empty>;

struct Lift;

impl BuilderInput for Lift {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Lift {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Around,
        Changed,
        SnapNil,
        ColPtrCons<Slot, ColPtrNil>,
        ColPtrCons<Height, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = Around;
    type Write = Changed;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|read| {
            let w = read.get(Reach::<Column<Slot>, ONE>::new());
            rec![Height(w.here().0)]
        });
    }
}

fn store(provider: StaticBumpMemory) -> ArenaColumnStorage<StaticBumpMemory> {
    ArenaColumnStorage::new(provider)
}

fn main() {
    let provider = StaticBumpMemory::new(std::vec![MaybeUninit::new(0); 4096].leak());
    let _ = Scheduler::builder()
        .with(Column::<Slot>::start())
        .with(Column::<Height>::start())
        .with(Lift)
        .build(store(provider), USize(0));
}
