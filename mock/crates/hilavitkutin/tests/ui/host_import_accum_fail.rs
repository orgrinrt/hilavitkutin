// A host imports records into the registered `Column<A>`, runs a frame, and
// dumps `Column<B>`, which a unit writes from it. The `_fail` fixtures named
// `host_import_*` differ from this one in the store type of one call: an
// `Accum`, a `Virtual`, or a column the builder never registered, whose
// records the unregistered fixture passes in place of the `A` records.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::rec;
#[allow(unused_imports)]
use hilavitkutin_api::store::{Accum, Column, Virtual};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};
use notko::Outcome;
use notko::sink::{BulkPush, Push};

#[derive(Copy, Clone, PartialEq)]
struct A(u32);
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
struct B(u32);
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
struct Other(u32);

struct Sink(u32);

impl<T> Push<T> for Sink {
    fn push(&mut self, _: T) {
        self.0 += 1;
    }
}

impl<T> BulkPush<T> for Sink {}

#[derive(Copy, Clone)]
struct Twice;

impl BuilderInput for Twice {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Twice {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Cons<Column<A>, Empty>,
        Cons<Column<B>, Empty>,
        SnapNil,
        ColPtrCons<A, ColPtrNil>,
        ColPtrCons<B, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<A>, Empty>;
    type Write = Cons<Column<B>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each()
            .map(|read| rec![B(read.get(Column::<A>::new()).0 * 2)]);
    }
}

fn store(provider: StaticBumpMemory) -> ArenaColumnStorage<StaticBumpMemory> {
    ArenaColumnStorage::new(provider)
}

fn main() {
    let memory = StaticBumpMemory::new(std::vec![MaybeUninit::new(0); 4096].leak());
    let mut scheduler = Scheduler::builder()
        .with(Column::<A>::start_from(A(0)))
        .with(Column::<B>::start_from(B(0)))
        .with(Twice)
        .build(store(memory), USize(4))
        .unwrap_or_else(|_| panic!("build should succeed"));
    assert!(matches!(
        scheduler.import::<Accum<A>, _, _>(USize(0), &[A(1), A(2)]),
        Outcome::Ok(())
    ));
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    let mut sink = Sink(0);
    assert!(matches!(
        scheduler.dump::<Column<B>, _, _>(USize(0), USize(4), &mut sink),
        Outcome::Ok(())
    ));
    assert_eq!(sink.0, 4);
}
