// A unit writes `Differs<Column<Height>>` and reads `Reach<Column<Slot>, ONE>`,
// and the builder registers both columns bare. `build` takes it: a wrapper in
// an access set is held by the store under it. `Slot` starts from a known value
// and one frame runs over four records, so each record of `Height` holds that
// value afterwards: the unit ran and read through the wrapper.
// The twin `unregistered_wrapped_store_fail.rs` differs only by the two
// `.with(Column::<..>::start_from(..))` lines, which it leaves out.

use core::mem::MaybeUninit;

use arvo::{Cap, USize};
use hilavitkutin::dispatch::engine_ctx::{AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{Differs, Reach, rec};
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};

const ONE: Cap = arvo_tensor::cap(1);
const RECORDS: usize = 4;

#[derive(Copy, Clone, PartialEq)]
struct Slot(u32);

#[derive(Copy, Clone, PartialEq)]
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
    let mut scheduler = Scheduler::builder()
        .with(Column::<Slot>::start_from(Slot(5)))
        .with(Column::<Height>::start_from(Height(0)))
        .with(Lift)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build refused a unit whose wrapped stores are registered"));
    assert!(matches!(scheduler.run(), notko::Outcome::Ok(())));
    // The column registered last is the bindings head.
    let height = scheduler.__bindings().__ptr().as_ptr();
    for i in 0 .. RECORDS {
        // SAFETY: `Height` holds `RECORDS` reserved records and no pass is running.
        let got: Height = unsafe { height.add(i).read() };
        assert_eq!(
            got.0, 5,
            "record {i} holds the value read through the wrapper"
        );
    }
}
