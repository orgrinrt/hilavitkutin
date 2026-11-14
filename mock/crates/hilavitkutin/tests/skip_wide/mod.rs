//! The wide skip fixture: every worker path `run_parallel` has, judged on one
//! history.
//!
//! Six producers each read a resource of their own and write a column of their
//! own, so the first phase holds six trunk roots and trunk ownership, which is
//! round-robin over the cores, places them differently at 1, 2, 3, 4 and
//! `MAX_CORES` workers. A consumer reads the first producer's column and writes
//! a sink, two hops down that producer's cone. The fixture is built three ways
//! by [`wide_scheduler!`]: over more records than one window, which is the
//! windowed walk; over no records, which is the record-less walk; and with an
//! accumulator writer reading the second producer's column, which puts the
//! whole frame on the unit-outer walk.
//!
//! Each unit bumps its own counter on every `execute`, and a trace records how
//! many times each unit ran in each frame, not only whether it ran, so a unit
//! dispatched twice is seen. The counters are read only through a [`Serial`],
//! which holds the one lock every test of a binary takes before it runs a unit.
//!
//! Included by name (`#[macro_use] mod skip_wide;`), beside
//! `mod stable_provider;`, which it does not include.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, MutexGuard};

use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons,
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    SnapCons,
    SnapNil,
};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi,
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasAccumWriter,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
    HasResourceProvider,
    ResourceProviderApi,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Accum, Column, Resource, Started, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

/// Wrap a provider in the default-capacity arena store.
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

/// The producers, each a trunk root of the first phase.
pub const ROOTS: usize = 6;
/// The consumer of the first producer's column.
pub const CONSUMER: usize = ROOTS;
/// The accumulator writer, present in the accumulator build only.
pub const ACCUMULATOR: usize = ROOTS + 1;
/// Every unit's counter, the accumulator writer's included.
pub const UNITS: usize = ROOTS + 2;
/// Records for the windowed and accumulator builds: more than twice the widest
/// window the plan bakes for the fixture's fibers (3072 records for the first
/// producer's, 6144 for the others'), so every fiber walks several, and not a
/// multiple of `MAX_CORES`, so the unit-outer tiling leaves a core idle there.
pub const RECORDS: usize = 13000;
/// The memory every build takes: eight columns and the accumulator of
/// [`RECORDS`] `u32` records, with room for their padding.
pub const BYTES: usize = 10 * RECORDS * 4;

/// The seed the host writes at record `i` before the cold frame. Every
/// producer adds it to its input, so the bytes say which record a write
/// landed at, and a unit stays a function of its reads alone.
pub const fn seed(i: usize) -> u32 {
    3 * i as u32 + 1
}

/// The input of producer `K`.
#[derive(Copy, Clone)]
pub struct In<const K: usize>(pub u32);
// The inputs carry no Seq or Map members, so their footprint is zero.
impl<const K: usize> hilavitkutin_api::footprint::ResourceFootprint for In<K> {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}

/// The output of producer `K`. Transparent, since the snapshot reads it
/// through a `u32` pointer.
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
pub struct Out<const K: usize>(pub u32);
/// The consumer's output. Transparent, read as the producers' are.
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
pub struct Sink(pub u32);
/// The column every producer reads beside its input, written by the host
/// through a `u32` pointer, so transparent.
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
pub struct Seed(pub u32);
/// The accumulator's record.
#[derive(Copy, Clone)]
pub struct Kept(pub u32);

static RAN: [AtomicU32; UNITS] = [const { AtomicU32::new(0) }; UNITS];
static LOCK: Mutex<()> = Mutex::new(());

fn bump(unit: usize) {
    RAN[unit].fetch_add(1, Ordering::Relaxed);
}

/// The lock every test of a binary holds while it runs a unit of the fixture.
pub struct Serial(MutexGuard<'static, ()>);

/// Take the fixture's lock and reset the counters.
pub fn serial() -> Serial {
    // A test that panicked holding the lock poisons it; the counters are reset
    // below, so the next test starts clean whatever the last one did.
    let guard = LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    for counter in &RAN {
        counter.store(0, Ordering::Relaxed);
    }
    Serial(guard)
}

/// How many times each unit ran.
pub type Counts = [u32; UNITS];

impl Serial {
    /// How many times each unit has run since the lock was taken.
    pub fn counts(&self) -> Counts {
        let _ = &self.0;
        core::array::from_fn(|unit| RAN[unit].load(Ordering::Relaxed))
    }

    /// How many times each unit ran since `before`, a value of [`Serial::counts`].
    pub fn counts_since(&self, before: Counts) -> Counts {
        let now = self.counts();
        core::array::from_fn(|unit| now[unit] - before[unit])
    }
}

type Hint = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

type ProducerRead<const K: usize> = Cons<Resource<In<K>>, Cons<Column<Seed>, Empty>>;

/// Producer `K`: reads `Resource<In<K>>` and `Column<Seed>`, writes
/// `Column<Out<K>>` as the input plus the seed.
pub struct Producer<const K: usize>;

impl<const K: usize> BuilderInput for Producer<K> {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl<const K: usize> WorkUnit<Always> for Producer<K> {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ProducerRead<K>,
        Cons<Column<Out<K>>, Empty>,
        SnapCons<In<K>, SnapNil>,
        ColPtrCons<Seed, ColPtrNil>,
        ColPtrCons<Out<K>, ColPtrNil>,
    >;
    type Hint = Hint;
    type Read = ProducerRead<K>;
    type Write = Cons<Column<Out<K>>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        bump(K);
        let base = ctx.resources().resource::<In<K>, _>().0;
        ctx.each().run(|i| {
            // SAFETY: the host wrote the seed at every record before the cold
            // frame and nothing writes it after.
            let seed: Seed = unsafe { ctx.reader().read::<Seed, _>(i) };
            // SAFETY: `build` reserved the column for the record count and the
            // plan proved this unit its one writer.
            unsafe { ctx.writer().write::<Out<K>, _>(i, Out::<K>(base + seed.0)) };
        });
    }
}

/// The consumer: reads `Column<Out<0>>`, writes `Column<Sink>` as a copy.
pub struct Consumer;

impl BuilderInput for Consumer {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Consumer {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Cons<Column<Out<0>>, Empty>,
        Cons<Column<Sink>, Empty>,
        SnapNil,
        ColPtrCons<Out<0>, ColPtrNil>,
        ColPtrCons<Sink, ColPtrNil>,
    >;
    type Hint = Hint;
    type Read = Cons<Column<Out<0>>, Empty>;
    type Write = Cons<Column<Sink>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        bump(CONSUMER);
        ctx.each().run(|i| {
            // SAFETY: the first producer, ordered before by the plan's edge on
            // its column, wrote every record the morsel covers.
            let v: Out<0> = unsafe { ctx.reader().read::<Out<0>, _>(i) };
            // SAFETY: as above, for the exclusively written sink.
            unsafe { ctx.writer().write::<Sink, _>(i, Sink(v.0)) };
        });
    }
}

/// The accumulator writer: reads `Column<Out<1>>` and appends each odd value,
/// at most one append per record and in record order.
pub struct AccumulatorWriter;

impl BuilderInput for AccumulatorWriter {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for AccumulatorWriter {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Cons<Column<Out<1>>, Empty>,
        Cons<Accum<Kept>, Empty>,
        SnapNil,
        ColPtrCons<Out<1>, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Kept, AccPtrNil>,
    >;
    type Hint = Hint;
    type Read = Cons<Column<Out<1>>, Empty>;
    type Write = Cons<Accum<Kept>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        bump(ACCUMULATOR);
        ctx.each().run(|i| {
            // SAFETY: the second producer, ordered before by the plan's edge on
            // its column, wrote every record the slice covers.
            let v: Out<1> = unsafe { ctx.reader().read::<Out<1>, _>(i) };
            if v.0 % 2 == 1 {
                // The accumulator reserves one record per record, and the plan
                // proved this unit its one appender.
                ctx.accums().append::<Kept, _>(Kept(v.0));
            }
        });
    }
}

/// The binding `$depth` tails below the head, one `t` per tail.
macro_rules! at {
    ($b:expr;) => {
        $b
    };
    ($b:expr; t $($rest:tt)*) => {
        at!($b.__tail(); $($rest)*)
    };
}

/// Build the wide scheduler over `$records` records, `plain` or with the
/// accumulator writer, and write the seed. Taking a [`Serial`] first makes the
/// lock a precondition of building one.
///
/// The last registered store is the bindings head and each one before it sits
/// one tail deeper: Sink 0, `Out<5>` 1 to `Out<0>` 6, `In<5>` 7 to `In<0>` 12,
/// the seed 13, and in the accumulator build `Accum<Kept>` 14, registered
/// first.
macro_rules! wide_scheduler {
    ($serial:expr, $records:expr, plain) => {{
        let sched = wide_scheduler!(@units $serial, hilavitkutin::scheduler::Scheduler::builder())
            .build(
                $crate::skip_wide::store($crate::skip_wide::stable_bytes()),
                arvo::USize($records),
            )
            .unwrap_or_else(|_| panic!("build should succeed"));
        wide_scheduler!(@seed sched, $records);
        sched
    }};
    ($serial:expr, $records:expr, accumulator) => {{
        let sched = wide_scheduler!(
            @units $serial,
            hilavitkutin::scheduler::Scheduler::builder()
                .with(hilavitkutin_api::store::Accum::<$crate::skip_wide::Kept>::new())
        )
        .with($crate::skip_wide::AccumulatorWriter)
        .build(
            $crate::skip_wide::store($crate::skip_wide::stable_bytes()),
            arvo::USize($records),
        )
        .unwrap_or_else(|_| panic!("build should succeed"));
        wide_scheduler!(@seed sched, $records);
        sched
    }};
    (@seed $sched:expr, $records:expr) => {{
        let seed = at!($sched.__bindings(); t t t t t t t t t t t t t).__ptr().as_ptr() as *mut u32;
        for i in 0 .. $records {
            // SAFETY: the seed column reserves `$records` one-`u32` records and
            // no frame has run.
            unsafe { *seed.add(i) = $crate::skip_wide::seed(i) };
        }
    }};
    (@units $serial:expr, $builder:expr) => {{
        let _: &$crate::skip_wide::Serial = &$serial;
        let s = $crate::skip_wide::START;
        $builder
            .with($crate::skip_wide::start::<$crate::skip_wide::Seed>())
            .with(hilavitkutin_api::store::Resource::new($crate::skip_wide::In::<0>(s[0])))
            .with(hilavitkutin_api::store::Resource::new($crate::skip_wide::In::<1>(s[1])))
            .with(hilavitkutin_api::store::Resource::new($crate::skip_wide::In::<2>(s[2])))
            .with(hilavitkutin_api::store::Resource::new($crate::skip_wide::In::<3>(s[3])))
            .with(hilavitkutin_api::store::Resource::new($crate::skip_wide::In::<4>(s[4])))
            .with(hilavitkutin_api::store::Resource::new($crate::skip_wide::In::<5>(s[5])))
            .with($crate::skip_wide::start::<$crate::skip_wide::Out<0>>())
            .with($crate::skip_wide::start::<$crate::skip_wide::Out<1>>())
            .with($crate::skip_wide::start::<$crate::skip_wide::Out<2>>())
            .with($crate::skip_wide::start::<$crate::skip_wide::Out<3>>())
            .with($crate::skip_wide::start::<$crate::skip_wide::Out<4>>())
            .with($crate::skip_wide::start::<$crate::skip_wide::Out<5>>())
            .with($crate::skip_wide::start::<$crate::skip_wide::Sink>())
            .with($crate::skip_wide::Producer::<0>)
            .with($crate::skip_wide::Producer::<1>)
            .with($crate::skip_wide::Producer::<2>)
            .with($crate::skip_wide::Producer::<3>)
            .with($crate::skip_wide::Producer::<4>)
            .with($crate::skip_wide::Producer::<5>)
            .with($crate::skip_wide::Consumer)
    }};
}

/// Write every input in place, with no mark.
macro_rules! set_inputs {
    ($sched:expr, $v:expr) => {{
        let v: [u32; $crate::skip_wide::ROOTS] = $v;
        let b = $sched.__bindings();
        // SAFETY: every input was moved in at build and lives in its
        // store-reserved one-record blob for the scheduler's lifetime; no frame
        // is running while the host writes.
        unsafe {
            *(at!(b; t t t t t t t t t t t t).__ptr().as_ptr() as *mut $crate::skip_wide::In<0>) =
                $crate::skip_wide::In(v[0]);
            *(at!(b; t t t t t t t t t t t).__ptr().as_ptr() as *mut $crate::skip_wide::In<1>) =
                $crate::skip_wide::In(v[1]);
            *(at!(b; t t t t t t t t t t).__ptr().as_ptr() as *mut $crate::skip_wide::In<2>) =
                $crate::skip_wide::In(v[2]);
            *(at!(b; t t t t t t t t t).__ptr().as_ptr() as *mut $crate::skip_wide::In<3>) =
                $crate::skip_wide::In(v[3]);
            *(at!(b; t t t t t t t t).__ptr().as_ptr() as *mut $crate::skip_wide::In<4>) =
                $crate::skip_wide::In(v[4]);
            *(at!(b; t t t t t t t).__ptr().as_ptr() as *mut $crate::skip_wide::In<5>) =
                $crate::skip_wide::In(v[5]);
        }
    }};
}

/// The seven columns over `$records` records, `Out<0>` to `Out<5>` then the
/// sink, and the accumulator's live records in the accumulator build.
macro_rules! snapshot {
    ($sched:expr, $records:expr, plain) => {
        $crate::skip_wide::Bytes {
            cols:  snapshot!(@cols $sched, $records),
            accum: Vec::new(),
        }
    };
    ($sched:expr, $records:expr, accumulator) => {{
        let b = $sched.__bindings();
        let acc = at!(b; t t t t t t t t t t t t t t);
        let live = acc.__len_cell().get().0;
        let base = acc.__ptr().as_ptr();
        $crate::skip_wide::Bytes {
            cols:  snapshot!(@cols $sched, $records),
            // SAFETY: the first `live` records were written by this frame's
            // appends, and no frame runs.
            accum: (0 .. live).map(|k| unsafe { (*base.add(k)).0 }).collect(),
        }
    }};
    (@cols $sched:expr, $records:expr) => {{
        let b = $sched.__bindings();
        let ptrs: [*const u32; 7] = [
            at!(b; t t t t t t).__ptr().as_ptr() as *const u32,
            at!(b; t t t t t).__ptr().as_ptr() as *const u32,
            at!(b; t t t t).__ptr().as_ptr() as *const u32,
            at!(b; t t t).__ptr().as_ptr() as *const u32,
            at!(b; t t).__ptr().as_ptr() as *const u32,
            at!(b; t).__ptr().as_ptr() as *const u32,
            at!(b;).__ptr().as_ptr() as *const u32,
        ];
        ptrs.iter()
            // SAFETY: each column holds `$records` reserved records of a
            // one-`u32` value type; the scheduler is alive and no frame runs.
            .map(|&p| (0 .. $records).map(|i| unsafe { *p.add(i) }).collect())
            .collect::<Vec<Vec<u32>>>()
    }};
}

/// Mark every registered store dirty, so every unit is seeded directly.
macro_rules! mark_every_store {
    ($sched:expr) => {{
        $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<0>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<1>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<2>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<3>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<4>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<5>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_wide::Out<0>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_wide::Out<1>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_wide::Out<2>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_wide::Out<3>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_wide::Out<4>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_wide::Out<5>>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_wide::Sink>, _>();
        $sched.mark_dirty::<hilavitkutin_api::store::Column<$crate::skip_wide::Seed>, _>();
    }};
}

/// A fresh leaked buffer of [`BYTES`] bytes, each of them 0xA5, so no result
/// leans on memory that happens to be zero.
pub fn stable_bytes() -> hilavitkutin_providers::StaticBumpMemory {
    hilavitkutin_providers::StaticBumpMemory::new(
        std::vec![core::mem::MaybeUninit::new(0xA5u8); BYTES].leak(),
    )
}

#[macro_use]
mod history;
pub use history::*;
