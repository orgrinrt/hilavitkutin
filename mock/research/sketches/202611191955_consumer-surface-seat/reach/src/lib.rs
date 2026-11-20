//! Can a unit reach the default interner through `Resource<T>`?
//!
//! Arm `direct`: the unit reads `Resource<StringInterner<MemoryArena<..>>>`,
//! the holding the providers crate's own doc names (src/interner.rs:239-241).
//! Arm `newtype`: the consumer wraps the interner in a local type so it can
//! give it the resource footprint the build asks for. Build one arm at a time.

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{ColPtrNil, EngineCtx, SnapCons, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{HasResourceProvider, ResourceProviderApi};
use hilavitkutin_api::store::Resource;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::{ArenaColumnStorage, InternerApi, MemoryArena, default_interner};
use hilavitkutin_str::StringInterner;

#[cfg(feature = "direct")]
pub type Held = StringInterner<MemoryArena<256, 16>>;

#[cfg(feature = "newtype")]
pub struct Held(pub StringInterner<MemoryArena<256, 16>>);

#[cfg(feature = "newtype")]
impl hilavitkutin_api::footprint::ResourceFootprint for Held {
    const L1_BYTES: USize = USize(0);
}

type ReadInterner = Cons<Resource<Held>, Empty>;

pub struct InternsAName;

impl BuilderInput for InternsAName {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for InternsAName {
    type Ctx<'frame> =
        EngineCtx<'frame, ReadInterner, Empty, SnapCons<Held, SnapNil>, ColPtrNil, ColPtrNil>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadInterner;
    type Write = Empty;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let held: &Held = ctx.resources().resource();
        #[cfg(feature = "direct")]
        let _ = InternerApi::intern(held, "from-a-unit");
        #[cfg(feature = "newtype")]
        let _ = InternerApi::intern(&held.0, "from-a-unit");
    }
}

pub fn build_and_run<M: hilavitkutin_api::platform::MemoryProviderApi>(provider: M) {
    #[cfg(feature = "direct")]
    let value: Held = default_interner::<256, 16>();
    #[cfg(feature = "newtype")]
    let value = Held(default_interner::<256, 16>());
    let mut scheduler = match Scheduler::builder()
        .with(Resource::new(value))
        .with(InternsAName)
        .build(ArenaColumnStorage::new(provider), USize(4))
    {
        notko::Outcome::Ok(s) => s,
        notko::Outcome::Err(_) => panic!("build failed"),
    };
    let _ = scheduler.run();
}
