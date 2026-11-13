//! Incremental-skip integration test (domain 16, canonical Step 9, E7).
//!
//! `Scheduler::run` re-runs only the units whose input cone changed since
//! the prior frame; the rest are skipped, producing identical output to
//! running them. The skip is seeded per store: `mark_dirty::<S>()` flags a
//! store the consumer mutated, every unit reading that store is seeded
//! dirty, and the dirty flag propagates forward over the predecessor masks
//! in carrier (topological) order so a unit downstream of a dirty unit also
//! re-runs.
//!
//! Three units across two independent chains exercise the three behaviours:
//! a producer reads `Resource<InA>` and writes `Column<Ca>`; a consumer
//! reads `Column<Ca>` and writes `Column<Cb>` (a RAW edge, so the producer
//! is its predecessor); an unrelated unit reads `Resource<InC>` and writes
//! `Column<Cc>`. Each unit bumps a per-unit execution counter on every
//! `execute`, so the counter is the direct skip observable.
//!
//! Three frames pin the contract. The cold first frame runs every unit
//! (`first_frame` seeds all dirty), so the counts are `[1, 1, 1]`. A second
//! frame with no `mark_dirty` finds an empty change seed, so every unit is
//! clean and skipped: the counts stay `[1, 1, 1]`. A third frame marks only
//! `Resource<InA>`: the producer is seeded (it reads `InA`), the consumer
//! inherits the producer's dirt over its predecessor mask, and the unrelated
//! `InC` chain stays clean. So the producer and consumer run again and the
//! unrelated unit is skipped: the counts become `[2, 2, 1]`.
//!
//! Red first: against a `run` that does not gate (every unit runs every
//! frame), the second frame's counts would be `[2, 2, 2]` and the third's
//! `[3, 3, 3]`, so both post-cold assertions fail. The gate is what makes
//! the clean units skip.
//!
//! The counts alone cannot tell a skip that is correct from one that is stale,
//! so a differential test drives a skipping scheduler and an every-store-dirty
//! scheduler through one input history and asserts equal column bytes at every
//! frame, with the every-store-dirty run also checked against a model of the
//! chain. A twin test changes an input without `mark_dirty` and asserts the two
//! runs differ.
//!
//! Lives under `tests/` so the bare numeric record values and counters do
//! not trip the src-tree primitive lints.

mod stable_provider;

use std::cell::RefCell;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapCons, SnapNil};
use hilavitkutin::scheduler::Scheduler;
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
use hilavitkutin_api::store::{Column, Resource};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use stable_provider::StableProvider;

/// Wrap a provider in the default-capacity arena store (`D = Dim<256>`).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// A few records, one morsel under the 256 default: an accumulator-free
// carrier with records > 0 dispatches morsel-outer, the gated path.
const RECORDS: usize = 4;

// Resource value newtypes (root inputs). Copy so they store as resources.
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct InA(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct InC(u32);

// Column value newtypes: a Copy newtype over u32 picks up the blanket
// `ColumnValue`. The payloads only give the units something to write and
// read (the RAW edge that orders the producer before the consumer); the
// test asserts execution counts, not values.
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Ca(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Cb(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Cc(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Cd(u32);

type ReadA = Cons<Resource<InA>, Empty>;
type WriteCa = Cons<Column<Ca>, Empty>;
type ReadCa = Cons<Column<Ca>, Empty>;
type WriteCb = Cons<Column<Cb>, Empty>;
type ReadCb = Cons<Column<Cb>, Empty>;
type WriteCd = Cons<Column<Cd>, Empty>;
type ReadC = Cons<Resource<InC>, Empty>;
type WriteCc = Cons<Column<Cc>, Empty>;

thread_local! {
    // Per-unit execution counts: [producer, consumer, unrelated, second_consumer].
    static EXEC: RefCell<[u32; 4]> = const { RefCell::new([0, 0, 0, 0]) };
}

const PRODUCER: usize = 0;
const CONSUMER: usize = 1;
const UNRELATED: usize = 2;
const SECOND_CONSUMER: usize = 3;

// Producer: reads Resource<InA>, writes Column<Ca>. The InA read is what
// `mark_dirty::<Resource<InA>>()` seeds.
struct ProducerWu;

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
        EXEC.with(|e| e.borrow_mut()[PRODUCER] += 1);
        let seed: &InA = ctx.resources().resource();
        let base = seed.0;
        ctx.each().run(|i| {
            // SAFETY: `build` reserved the `Column<Ca>` buffer for the record
            // count and the plan proved this unit the exclusive writer.
            unsafe { ctx.writer().write::<Ca, _>(i, Ca(base + i.0 as u32)) };
        });
    }
}

// Consumer: reads Column<Ca>, writes Column<Cb>. The RAW edge on Ca makes
// the producer its predecessor, so the consumer inherits the producer's dirt.
struct ConsumerWu;

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
        EXEC.with(|e| e.borrow_mut()[CONSUMER] += 1);
        ctx.each().run(|i| {
            // SAFETY: the producer (ordered before this unit by the plan's RAW
            // edge) wrote every record the morsel covers; no concurrent writer.
            let v: Ca = unsafe { ctx.reader().read::<Ca, _>(i) };
            // SAFETY: as above, for the exclusively-written `Column<Cb>` buffer.
            unsafe { ctx.writer().write::<Cb, _>(i, Cb(v.0)) };
        });
    }
}

// Unrelated: reads Resource<InC>, writes Column<Cc>. No edge to the InA chain,
// so marking InA leaves this unit clean.
struct UnrelatedWu;

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
        EXEC.with(|e| e.borrow_mut()[UNRELATED] += 1);
        let seed: &InC = ctx.resources().resource();
        let base = seed.0;
        ctx.each().run(|i| {
            // SAFETY: `build` reserved the `Column<Cc>` buffer for the record
            // count and the plan proved this unit the exclusive writer.
            unsafe { ctx.writer().write::<Cc, _>(i, Cc(base + i.0 as u32)) };
        });
    }
}

// SecondConsumer: reads Column<Cb>, writes Column<Cd>. Its RAW edge on Cb makes
// the consumer its predecessor, so it sits two hops downstream of the producer
// (InA -> Ca -> Cb -> Cd). Marking InA must propagate dirt to it transitively.
struct SecondConsumerWu;

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
        EXEC.with(|e| e.borrow_mut()[SECOND_CONSUMER] += 1);
        ctx.each().run(|i| {
            // SAFETY: the consumer (ordered before by the plan's RAW edge on Cb)
            // wrote every record the morsel covers; no concurrent writer.
            let v: Cb = unsafe { ctx.reader().read::<Cb, _>(i) };
            // SAFETY: as above, for the exclusively-written `Column<Cd>` buffer.
            unsafe { ctx.writer().write::<Cd, _>(i, Cd(v.0)) };
        });
    }
}

fn counts() -> [u32; 4] {
    EXEC.with(|e| *e.borrow())
}

#[test]
fn incremental_skip_runs_only_the_changed_cone() {
    EXEC.with(|e| *e.borrow_mut() = [0, 0, 0, 0]);
    let provider = StableProvider::new(32768);
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(InA(100)))
        .with(Resource::new(InC(200)))
        .with(Column::<Ca>::new())
        .with(Column::<Cb>::new())
        .with(Column::<Cc>::new())
        .with(ProducerWu)
        .with(ConsumerWu)
        .with(UnrelatedWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Cold frame: `first_frame` seeds every unit dirty, so all three run. The
    // fourth slot (SecondConsumer) is unused here; this test registers only
    // three units, so it stays zero throughout.
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        counts(),
        [1, 1, 1, 0],
        "the cold first frame runs every unit (all seeded dirty)"
    );

    // Clean frame: no `mark_dirty`, so the change seed is empty and every unit
    // is skipped. A non-gating `run` would re-run all three here ([2, 2, 2]).
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        counts(),
        [1, 1, 1, 0],
        "a frame with no marked store skips every unit; the counts do not move"
    );

    // Mark the producer's input only. The producer reads InA so it is seeded;
    // the consumer inherits the producer's dirt over its predecessor mask; the
    // unrelated InC chain stays clean and is skipped.
    scheduler.mark_dirty::<Resource<InA>, _>();
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        counts(),
        [2, 2, 1, 0],
        "marking InA re-runs the producer and its dependent consumer; the \
         unrelated InC chain stays clean and skipped"
    );
}

#[test]
fn incremental_skip_propagates_two_hops() {
    EXEC.with(|e| *e.borrow_mut() = [0, 0, 0, 0]);
    let provider = StableProvider::new(32768);
    // Producer -> Consumer -> SecondConsumer is a depth-two RAW chain
    // (InA -> Ca -> Cb -> Cd); the unrelated unit reads InC -> Cc. Registration
    // order is topological (each writer before its reader), which `build`
    // requires.
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(InA(100)))
        .with(Resource::new(InC(200)))
        .with(Column::<Ca>::new())
        .with(Column::<Cb>::new())
        .with(Column::<Cc>::new())
        .with(Column::<Cd>::new())
        .with(ProducerWu)
        .with(ConsumerWu)
        .with(SecondConsumerWu)
        .with(UnrelatedWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Cold frame: all four run.
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        counts(),
        [1, 1, 1, 1],
        "the cold first frame runs every unit (all seeded dirty)"
    );

    // Clean frame: every unit is skipped.
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        counts(),
        [1, 1, 1, 1],
        "a frame with no marked store skips every unit; the counts do not move"
    );

    // Mark only the root input InA. The producer is seeded (reads InA); the
    // consumer inherits over its Ca predecessor mask; the second consumer
    // inherits over its Cb predecessor mask, two hops from the marked root.
    // The unrelated InC chain stays clean. A propagation that stopped at depth
    // one would leave the second consumer at 1 here ([2, 2, 1, 1]); transitive
    // propagation re-runs it ([2, 2, 1, 2]).
    scheduler.mark_dirty::<Resource<InA>, _>();
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        counts(),
        [2, 2, 1, 2],
        "marking InA propagates dirt two hops: producer, consumer, and second \
         consumer all re-run; the unrelated InC chain stays clean and skipped"
    );
}

// Differential: a skipping scheduler and an every-store-dirty scheduler are
// driven through the same input history on the single-core entry (`run`), and
// every column holds equal bytes after every frame.
//
// Registration order InA, InC, Ca, Cb, Cc, Cd puts the last registered store at
// the bindings head, so Cd is the head and each registered store before it sits
// one tail deeper: Cc 1, Cb 2, Ca 3, InC 4, InA 5.
macro_rules! four_chain_scheduler {
    ($provider:expr) => {
        Scheduler::builder()
            .with(Resource::new(InA(100)))
            .with(Resource::new(InC(200)))
            .with(Column::<Ca>::new())
            .with(Column::<Cb>::new())
            .with(Column::<Cc>::new())
            .with(Column::<Cd>::new())
            .with(ProducerWu)
            .with(ConsumerWu)
            .with(SecondConsumerWu)
            .with(UnrelatedWu)
            .build(store($provider), USize(RECORDS))
            .unwrap_or_else(|_| panic!("build should succeed"))
    };
}

macro_rules! set_inputs {
    ($sched:expr, $a:expr, $c:expr) => {{
        // SAFETY: both resources were moved in at build and live in the
        // store-reserved one-record blobs for the scheduler's lifetime; no
        // frame is running while the host writes.
        unsafe {
            *$sched
                .__bindings()
                .__tail()
                .__tail()
                .__tail()
                .__tail()
                .__tail()
                .__ptr()
                .as_ptr() = InA($a);
            *$sched
                .__bindings()
                .__tail()
                .__tail()
                .__tail()
                .__tail()
                .__ptr()
                .as_ptr() = InC($c);
        }
    }};
}

macro_rules! snapshot {
    ($sched:expr) => {{
        let b = $sched.__bindings();
        let ca = b.__tail().__tail().__tail().__ptr().as_ptr() as *const u32;
        let cb = b.__tail().__tail().__ptr().as_ptr() as *const u32;
        let cc = b.__tail().__ptr().as_ptr() as *const u32;
        let cd = b.__ptr().as_ptr() as *const u32;
        let mut out = [[0u32; RECORDS]; 4];
        for i in 0 .. RECORDS {
            // SAFETY: each column holds RECORDS reserved records of a
            // Copy newtype over u32; the scheduler (hence storage) is alive.
            unsafe {
                out[0][i] = *ca.add(i);
                out[1][i] = *cb.add(i);
                out[2][i] = *cc.add(i);
                out[3][i] = *cd.add(i);
            }
        }
        out
    }};
}

// Marks every registered store dirty, so every unit is seeded directly and no
// result leans on the predecessor propagation.
macro_rules! mark_every_store {
    ($sched:expr) => {{
        $sched.mark_dirty::<Resource<InA>, _>();
        $sched.mark_dirty::<Resource<InC>, _>();
        $sched.mark_dirty::<Column<Ca>, _>();
        $sched.mark_dirty::<Column<Cb>, _>();
        $sched.mark_dirty::<Column<Cc>, _>();
        $sched.mark_dirty::<Column<Cd>, _>();
    }};
}

// One frame of the input history: the new value of each resource, or `None`
// to leave it as the previous frame had it.
type Step = (Option<u32>, Option<u32>);

const HISTORY: [Step; 7] = [
    (None, None),
    (None, None),
    (Some(7), None),
    (None, Some(9)),
    (Some(11), Some(13)),
    (None, None),
    (Some(11), None),
];

#[test]
fn skipping_run_matches_every_store_dirty_run_at_every_frame() {
    let provider_a = StableProvider::new(32768);
    let provider_b = StableProvider::new(32768);
    let mut skipping = four_chain_scheduler!(provider_a);
    let mut every = four_chain_scheduler!(provider_b);
    let (mut a, mut c) = (100u32, 200u32);

    // Cold frame: both run everything.
    assert!(matches!(skipping.run(), Outcome::Ok(())));
    assert!(matches!(every.run(), Outcome::Ok(())));
    assert_eq!(snapshot!(skipping), snapshot!(every), "cold frame");
    assert_eq!(snapshot!(every), model(a, c), "cold frame: model");

    for (frame, (na, nc)) in HISTORY.iter().enumerate() {
        if let Some(v) = na {
            a = *v;
            skipping.mark_dirty::<Resource<InA>, _>();
        }
        if let Some(v) = nc {
            c = *v;
            skipping.mark_dirty::<Resource<InC>, _>();
        }
        set_inputs!(skipping, a, c);
        set_inputs!(every, a, c);
        mark_every_store!(every);
        assert!(matches!(skipping.run(), Outcome::Ok(())));
        assert!(matches!(every.run(), Outcome::Ok(())));
        assert_eq!(
            snapshot!(skipping),
            snapshot!(every),
            "frame {frame}: skipping and every-store-dirty runs must hold equal column bytes"
        );
        // The every-store-dirty run is itself checked against a model of the
        // chain, so two runs that are equally stale cannot pass.
        assert_eq!(snapshot!(every), model(a, c), "frame {frame}: model");
    }
}

// What the four units compute from the two inputs, independent of the engine.
fn model(a: u32, c: u32) -> [[u32; RECORDS]; 4] {
    let mut out = [[0u32; RECORDS]; 4];
    for i in 0 .. RECORDS {
        out[0][i] = a + i as u32;
        out[1][i] = a + i as u32;
        out[2][i] = c + i as u32;
        out[3][i] = a + i as u32;
    }
    out
}

#[test]
fn skipping_run_diverges_when_a_mutated_store_is_not_marked() {
    let provider_a = StableProvider::new(32768);
    let provider_b = StableProvider::new(32768);
    let mut skipping = four_chain_scheduler!(provider_a);
    let mut every = four_chain_scheduler!(provider_b);
    assert!(matches!(skipping.run(), Outcome::Ok(())));
    assert!(matches!(every.run(), Outcome::Ok(())));

    // The host changes InA but forgets `mark_dirty`: the skipping run keeps the
    // stale cone, which is what the differential above must be able to see.
    set_inputs!(skipping, 7, 200);
    set_inputs!(every, 7, 200);
    mark_every_store!(every);
    assert!(matches!(skipping.run(), Outcome::Ok(())));
    assert!(matches!(every.run(), Outcome::Ok(())));
    assert_ne!(snapshot!(skipping), snapshot!(every));
}

// A3b: test-local resource values carry no Seq/Map members; zero footprint.
impl hilavitkutin_api::footprint::ResourceFootprint for InA {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for InC {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
