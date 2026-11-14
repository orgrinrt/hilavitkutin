//! SchedulerBuilder stress shapes: the recursive proofs at realistic depth.
//!
//! Fifty units in one builder validate the recursive `HList` accumulator, one
//! unit with sixteen stores in its read set validates the recursive `HoldsAll`
//! proof, and a `Depth` assertion fixes the length of a fifty-cell list at
//! compile time. The units name a stub context and their bodies never run.
//!
//! The store-only tests are in `scheduler_builder.rs` and the units over a
//! resource and a kit in `scheduler_builder_units.rs`. The arena and the stub
//! context live under `tests/scheduler_builder/`.

#[path = "scheduler_builder/arena.rs"]
mod arena;
#[path = "scheduler_builder/shim.rs"]
mod shim;

use arena::{TestProvider, store};
use arvo::USize;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::footprint::ResourceFootprint;
use hilavitkutin_api::{
    Always,
    Atomic,
    BuilderInput,
    Cons,
    Depth,
    Empty,
    Immediate,
    Normal,
    Resource,
    UnitDispatch,
    WorkUnit,
    read,
    write,
};
use hilavitkutin_providers::ArenaColumnStorage;
use shim::TestCtx;

// The fifty-unit registration needs more plan and meta storage than the
// default 4096 bytes: with the small arena `build` returns `AllocationFailed`.
fn store_for_fifty() -> ArenaColumnStorage<TestProvider<65536>> {
    ArenaColumnStorage::new(TestProvider::<65536>::new())
}

// ---------------------------------------------------------------------
// Wus uncap stress: 50 WUs in one builder. Validates the recursive
// HList accumulator handles realistic depth.
// ---------------------------------------------------------------------

#[derive(Copy, Clone)]
struct NoStores;

impl BuilderInput for NoStores {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for NoStores {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![];
    type Write = write![];

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

#[test]
fn smoke_fifty_wus() {
    let built = Scheduler::builder()
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .with(NoStores)
        .build(store_for_fifty(), USize(0));
    if let notko::Outcome::Err(e) = built {
        panic!("fifty-unit registration: {e:?}");
    }
}

// ---------------------------------------------------------------------
// WuSatisfied uncap stress: a single WU with 16 stores in its Read
// set. Validates the recursive HoldsAll proof handles realistic
// store-count depth via the read! macro emitting Cons cells.
// ---------------------------------------------------------------------

// Each `Sn` is registered as a `Resource` value below, so each must be
// `ColumnValue` (`Copy + 'static`). They are bare markers with no `Seq`/`Map`
// collection members, so their L1 morsel footprint is zero.
macro_rules! markers {
    ($($name:ident),+) => {
        $(
            #[derive(Copy, Clone)]
            struct $name;

            impl ResourceFootprint for $name {
                const L1_BYTES: USize = USize(0);
            }
        )+
    };
}
markers!(
    S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12, S13, S14, S15
);

struct SixteenStores;

impl BuilderInput for SixteenStores {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for SixteenStores {
    type Ctx<'frame> = TestCtx;
    type Hint = (Immediate, Atomic, Normal);
    type Read = read![
        Resource<S0>,
        Resource<S1>,
        Resource<S2>,
        Resource<S3>,
        Resource<S4>,
        Resource<S5>,
        Resource<S6>,
        Resource<S7>,
        Resource<S8>,
        Resource<S9>,
        Resource<S10>,
        Resource<S11>,
        Resource<S12>,
        Resource<S13>,
        Resource<S14>,
        Resource<S15>,
    ];
    type Write = write![];

    fn execute<'frame>(&self, _ctx: &TestCtx) {}
}

#[test]
fn smoke_wu_with_sixteen_stores() {
    let built = Scheduler::builder()
        .with(Resource::new(S0))
        .with(Resource::new(S1))
        .with(Resource::new(S2))
        .with(Resource::new(S3))
        .with(Resource::new(S4))
        .with(Resource::new(S5))
        .with(Resource::new(S6))
        .with(Resource::new(S7))
        .with(Resource::new(S8))
        .with(Resource::new(S9))
        .with(Resource::new(S10))
        .with(Resource::new(S11))
        .with(Resource::new(S12))
        .with(Resource::new(S13))
        .with(Resource::new(S14))
        .with(Resource::new(S15))
        .with(SixteenStores)
        .build(store(), USize(0));
    assert!(built.is_ok(), "the builder must accept this registration");
}

// ---------------------------------------------------------------------
// Depth compile-time assertion using the Cons<H, R> impl.
// ---------------------------------------------------------------------

type Cons1<T> = Cons<NoStores, T>;
type Cons5<T> = Cons1<Cons1<Cons1<Cons1<Cons1<T>>>>>;
type Cons10<T> = Cons5<Cons5<T>>;
type Cons50 = Cons10<Cons10<Cons10<Cons10<Cons10<Empty>>>>>;

const _: () = assert!(<Cons50 as Depth>::D.0 == 50);
