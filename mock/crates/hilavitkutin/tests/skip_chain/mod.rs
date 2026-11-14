//! The four-unit skip fixture, shared by every test that judges the skip.
//!
//! A producer reads `Resource<InA>` and writes `Column<Ca>`; a consumer reads
//! `Column<Ca>` and writes `Column<Cb>`; a second consumer reads `Column<Cb>`
//! and writes `Column<Cd>`, two hops from the producer; an unrelated unit reads
//! `Resource<InC>` and writes `Column<Cc>`. Each unit bumps its own execution
//! counter on every `execute`, so the counters say which units ran in a frame.
//!
//! The counters are atomics, so a worker thread of `run_parallel` bumps the
//! same counter the test reads. They are read only through a [`Serial`], which
//! holds the one lock every test of a binary takes before it runs a unit, so a
//! count never carries another test's executions.
//!
//! [`HISTORY`] is the input history the differentials drive: marked input
//! changes, clean frames, a `replace_value` of the producer's input and a
//! `replace_resource` of the unrelated unit's input, each with a clean frame
//! after it. [`trace!`] runs a scheduler through it and records, per frame,
//! the column bytes and the set of units that ran. The comparisons return the
//! first difference rather than asserting, so a test can also hold them to a
//! runner known to be wrong.
//!
//! Included by name from each test that needs it (`#[macro_use] mod
//! skip_chain;`), beside `mod stable_provider;`, which it does not include.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, MutexGuard};

use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapCons, SnapNil};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
    HasResourceProvider,
    ResourceProviderApi,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, Resource, Started, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

/// Wrap a provider in the default-capacity arena store (`D = Dim<256>`).
pub fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

/// Register a column of `T` with its default at every record.
pub fn start<T>() -> Started<Column<T>>
where
    Column<T>: StartsFromDefault,
{
    Column::<T>::start()
}

/// A few records, one morsel under the 256 default: an accumulator-free
/// carrier with records > 0 dispatches morsel-outer, the gated path.
pub const RECORDS: usize = 4;

/// The producer's input. A value swap replaces it (`replace_value`).
#[derive(Copy, Clone)]
pub struct InA(pub u32);
impl hilavitkutin_api::store::Replaceable for InA {}

/// The unrelated unit's input. A plan swap replaces it (`replace_resource`).
#[derive(Copy, Clone)]
pub struct InC(pub u32);
impl hilavitkutin_api::run_cfg::PlanAffecting for InC {}

/// The producer's output.
#[derive(Copy, Clone, Default)]
pub struct Ca(pub u32);
/// The consumer's output.
#[derive(Copy, Clone, Default)]
pub struct Cb(pub u32);
/// The unrelated unit's output.
#[derive(Copy, Clone, Default)]
pub struct Cc(pub u32);
/// The second consumer's output.
#[derive(Copy, Clone, Default)]
pub struct Cd(pub u32);

// The inputs carry no Seq or Map members, so their footprint is zero.
impl hilavitkutin_api::footprint::ResourceFootprint for InA {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for InC {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}

type ReadA = Cons<Resource<InA>, Empty>;
type WriteCa = Cons<Column<Ca>, Empty>;
type ReadCa = Cons<Column<Ca>, Empty>;
type WriteCb = Cons<Column<Cb>, Empty>;
type ReadCb = Cons<Column<Cb>, Empty>;
type WriteCd = Cons<Column<Cd>, Empty>;
type ReadC = Cons<Resource<InC>, Empty>;
type WriteCc = Cons<Column<Cc>, Empty>;

/// Index of each unit in a count and in a ran set.
pub const PRODUCER: usize = 0;
/// See [`PRODUCER`].
pub const CONSUMER: usize = 1;
/// See [`PRODUCER`].
pub const UNRELATED: usize = 2;
/// See [`PRODUCER`].
pub const SECOND_CONSUMER: usize = 3;

static RAN: [AtomicU32; 4] = [const { AtomicU32::new(0) }; 4];
static LOCK: Mutex<()> = Mutex::new(());

fn bump(unit: usize) {
    RAN[unit].fetch_add(1, Ordering::Relaxed);
}

/// The lock every test of a binary holds while it runs a unit of the fixture.
///
/// Taking it resets the counters, and the counters are read only through it.
pub struct Serial(MutexGuard<'static, ()>);

/// Take the fixture's lock and reset the counters.
pub fn serial() -> Serial {
    // A test that panicked while holding the lock poisons it; the counters are
    // reset below, so the next test starts clean whatever the last one did.
    let guard = LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    for counter in &RAN {
        counter.store(0, Ordering::Relaxed);
    }
    Serial(guard)
}

impl Serial {
    /// How many times each unit has run since the lock was taken.
    pub fn counts(&self) -> [u32; 4] {
        let _ = &self.0;
        [0, 1, 2, 3].map(|unit| RAN[unit].load(Ordering::Relaxed))
    }

    /// Which units ran since `before`, a value of [`Serial::counts`].
    pub fn ran_since(&self, before: [u32; 4]) -> Ran {
        let now = self.counts();
        [0, 1, 2, 3].map(|unit| now[unit] != before[unit])
    }
}

// Producer: reads Resource<InA>, writes Column<Ca>.
pub struct ProducerWu;

impl BuilderInput for ProducerWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for ProducerWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadA,
        WriteCa,
        SnapCons<InA, SnapNil>,
        ColPtrNil,
        ColPtrCons<Ca, ColPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadA;
    type Write = WriteCa;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        bump(PRODUCER);
        let seed: &InA = ctx.resources().resource();
        let base = seed.0;
        ctx.each().run(|i| {
            // SAFETY: `build` reserved the `Column<Ca>` buffer for the record
            // count and the plan proved this unit the exclusive writer.
            unsafe { ctx.writer().write::<Ca, _>(i, Ca(base + i.0 as u32)) };
        });
    }
}

// Consumer: reads Column<Ca>, writes Column<Cb>. The RAW edge on Ca makes the
// producer its predecessor, so the consumer inherits the producer's dirt.
pub struct ConsumerWu;

impl BuilderInput for ConsumerWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for ConsumerWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadCa,
        WriteCb,
        SnapNil,
        ColPtrCons<Ca, ColPtrNil>,
        ColPtrCons<Cb, ColPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadCa;
    type Write = WriteCb;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        bump(CONSUMER);
        ctx.each().run(|i| {
            // SAFETY: the producer (ordered before this unit by the plan's RAW
            // edge) wrote every record the morsel covers; no concurrent writer.
            let v: Ca = unsafe { ctx.reader().read::<Ca, _>(i) };
            // SAFETY: as above, for the exclusively-written `Column<Cb>` buffer.
            unsafe { ctx.writer().write::<Cb, _>(i, Cb(v.0)) };
        });
    }
}

// Unrelated: reads Resource<InC>, writes Column<Cc>. No edge to the InA cone,
// so marking InA leaves this unit clean.
pub struct UnrelatedWu;

impl BuilderInput for UnrelatedWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for UnrelatedWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadC,
        WriteCc,
        SnapCons<InC, SnapNil>,
        ColPtrNil,
        ColPtrCons<Cc, ColPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadC;
    type Write = WriteCc;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        bump(UNRELATED);
        let seed: &InC = ctx.resources().resource();
        let base = seed.0;
        ctx.each().run(|i| {
            // SAFETY: `build` reserved the `Column<Cc>` buffer for the record
            // count and the plan proved this unit the exclusive writer.
            unsafe { ctx.writer().write::<Cc, _>(i, Cc(base + i.0 as u32)) };
        });
    }
}

// SecondConsumer: reads Column<Cb>, writes Column<Cd>, two hops downstream of
// the producer (InA -> Ca -> Cb -> Cd).
pub struct SecondConsumerWu;

impl BuilderInput for SecondConsumerWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for SecondConsumerWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadCb,
        WriteCd,
        SnapNil,
        ColPtrCons<Cb, ColPtrNil>,
        ColPtrCons<Cd, ColPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadCb;
    type Write = WriteCd;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        bump(SECOND_CONSUMER);
        ctx.each().run(|i| {
            // SAFETY: the consumer (ordered before by the plan's RAW edge on Cb)
            // wrote every record the morsel covers; no concurrent writer.
            let v: Cb = unsafe { ctx.reader().read::<Cb, _>(i) };
            // SAFETY: as above, for the exclusively-written `Column<Cd>` buffer.
            unsafe { ctx.writer().write::<Cd, _>(i, Cd(v.0)) };
        });
    }
}

/// Build the four-unit scheduler over `$provider`. Taking a [`Serial`] first
/// makes the lock a precondition of building one.
///
/// Registration order InA, InC, Ca, Cb, Cc, Cd puts the last registered store
/// at the bindings head, so Cd is the head and each registered store before it
/// sits one tail deeper: Cc 1, Cb 2, Ca 3, InC 4, InA 5.
macro_rules! four_chain_scheduler {
    ($serial:expr, $provider:expr) => {{
        let _: &$crate::skip_chain::Serial = &$serial;
        hilavitkutin::scheduler::Scheduler::builder()
            .with(hilavitkutin_api::store::Resource::new(
                $crate::skip_chain::InA($crate::skip_chain::START.0),
            ))
            .with(hilavitkutin_api::store::Resource::new(
                $crate::skip_chain::InC($crate::skip_chain::START.1),
            ))
            .with($crate::skip_chain::start::<$crate::skip_chain::Ca>())
            .with($crate::skip_chain::start::<$crate::skip_chain::Cb>())
            .with($crate::skip_chain::start::<$crate::skip_chain::Cc>())
            .with($crate::skip_chain::start::<$crate::skip_chain::Cd>())
            .with($crate::skip_chain::ProducerWu)
            .with($crate::skip_chain::ConsumerWu)
            .with($crate::skip_chain::SecondConsumerWu)
            .with($crate::skip_chain::UnrelatedWu)
            .build(
                $crate::skip_chain::store($provider),
                arvo::USize($crate::skip_chain::RECORDS),
            )
            .unwrap_or_else(|_| panic!("build should succeed"))
    }};
}

/// Write both inputs in place, with no mark.
macro_rules! set_inputs {
    ($sched:expr, $a:expr, $c:expr) => {{
        let (a, c): (u32, u32) = ($a, $c);
        let b = $sched.__bindings();
        let in_a = b
            .__tail()
            .__tail()
            .__tail()
            .__tail()
            .__tail()
            .__ptr()
            .as_ptr() as *mut $crate::skip_chain::InA;
        let in_c =
            b.__tail().__tail().__tail().__tail().__ptr().as_ptr() as *mut $crate::skip_chain::InC;
        // SAFETY: both resources were moved in at build and live in the
        // store-reserved one-record blobs for the scheduler's lifetime; no
        // frame is running while the host writes.
        unsafe {
            *in_a = $crate::skip_chain::InA(a);
            *in_c = $crate::skip_chain::InC(c);
        }
    }};
}

/// The four columns' bytes, read through their value types.
macro_rules! snapshot {
    ($sched:expr) => {{
        let b = $sched.__bindings();
        let ca = b.__tail().__tail().__tail().__ptr().as_ptr() as *const $crate::skip_chain::Ca;
        let cb = b.__tail().__tail().__ptr().as_ptr() as *const $crate::skip_chain::Cb;
        let cc = b.__tail().__ptr().as_ptr() as *const $crate::skip_chain::Cc;
        let cd = b.__ptr().as_ptr() as *const $crate::skip_chain::Cd;
        let mut out: $crate::skip_chain::Bytes = [[0; $crate::skip_chain::RECORDS]; 4];
        for i in 0 .. $crate::skip_chain::RECORDS {
            // SAFETY: each column holds RECORDS reserved records of its value
            // type; the scheduler (hence storage) is alive and no frame runs.
            unsafe {
                out[0][i] = (*ca.add(i)).0;
                out[1][i] = (*cb.add(i)).0;
                out[2][i] = (*cc.add(i)).0;
                out[3][i] = (*cd.add(i)).0;
            }
        }
        out
    }};
}

/// Mark every registered store dirty, so every unit is seeded directly and no
/// result leans on the predecessor propagation.
macro_rules! mark_every_store {
    ($sched:expr) => {{
        $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_chain::InA>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_chain::InC>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_chain::Ca>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_chain::Cb>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_chain::Cc>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_chain::Cd>, _>();
    }};
}

#[macro_use]
mod history;
pub use history::*;
