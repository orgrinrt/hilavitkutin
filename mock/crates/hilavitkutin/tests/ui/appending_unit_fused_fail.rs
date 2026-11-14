// A unit declares its value function (`RecordOp`) and its write list holds an
// accumulator beside the column, so every record appends one value. The fused
// entry folds a carrier of units that read one column and write one column
// into a chain that threads values in registers, and a chain has nowhere to
// append. The carrier is refused at compile time. The twin
// `appending_unit_fused_pass.rs` differs only in the entry it calls: `run`
// executes the unit's own body, which appends.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons,
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::store::{Accum, Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{ReadVals, RecordOp, WriteVals, rec};
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};
use notko::Maybe;

#[derive(Copy, Clone, Default)]
struct Xa(u32);

#[derive(Copy, Clone, PartialEq, Default)]
struct Ya(u32);

#[derive(Copy, Clone)]
struct Log(u32);

type ReadX = Cons<Column<Xa>, Empty>;
type WriteBoth = Cons<Column<Ya>, Cons<Accum<Log>, Empty>>;

#[derive(Copy, Clone)]
struct MapAndLogWu;

impl BuilderInput for MapAndLogWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for MapAndLogWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadX,
        WriteBoth,
        SnapNil,
        ColPtrCons<Xa, ColPtrNil>,
        ColPtrCons<Ya, ColPtrNil>,
        AccPtrCons<'frame, Log, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadX;
    type Write = WriteBoth;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|read| self.apply(read));
    }
}

impl RecordOp for MapAndLogWu {
    fn apply(&self, read: &ReadVals<'_, ReadX>) -> WriteVals<WriteBoth> {
        let x = read.get(Column::<Xa>::new());
        rec![Ya(x.0 + 1), Maybe::Is(Log(x.0 + 7))]
    }
}

fn store(provider: StaticBumpMemory) -> ArenaColumnStorage<StaticBumpMemory> {
    ArenaColumnStorage::new(provider)
}

fn main() {
    let provider = StaticBumpMemory::new(std::vec![MaybeUninit::new(0); 8192].leak());
    // `Log` is registered last, so its accumulator is the head of the bindings.
    let mut scheduler = Scheduler::builder()
        .with(Column::<Xa>::start())
        .with(Column::<Ya>::start())
        .with(Accum::<Log>::new())
        .with(MapAndLogWu)
        .build(store(provider), USize(4))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let outcome = scheduler.run_fused();
    assert!(matches!(outcome, notko::Outcome::Ok(())));
    assert_eq!(scheduler.__bindings().__len_cell().get(), USize(4));
    let logged = scheduler.__bindings().__ptr().as_ptr();
    // SAFETY: four values were appended, so the first of them is initialised.
    assert_eq!(unsafe { core::ptr::read(logged) }.0, 7);
}
