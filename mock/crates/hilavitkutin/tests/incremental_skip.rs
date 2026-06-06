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
//! Lives under `tests/` so the bare numeric record values and counters do
//! not trip the src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::cell::RefCell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrCons, PtrNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
    HasResourceProvider, ResourceProviderApi,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Column, Resource};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

/// Wrap a provider in the default-capacity arena store (`D = Dim<256>`).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Stack-backed test memory provider (mirrors tests/morsel_outer.rs).
struct BumpProvider<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}

impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self {
            buf: UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
            used: Cell::new(0),
        }
    }
}

unsafe impl<const N: usize> Send for BumpProvider<N> {}
unsafe impl<const N: usize> Sync for BumpProvider<N> {}

impl<const N: usize> MemoryProviderApi for BumpProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = self.buf.get() as *mut u8;
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > N {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: `aligned + len <= N`, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for ProducerWu {
    type Read = ReadA;
    type Write = WriteCa;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, ReadA, WriteCa, PtrCons<InA, PtrNil>, ColPtrNil, ColPtrCons<Ca, ColPtrNil>>;

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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for ConsumerWu {
    type Read = ReadCa;
    type Write = WriteCb;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadCa,
        WriteCb,
        PtrNil,
        ColPtrCons<Ca, ColPtrNil>,
        ColPtrCons<Cb, ColPtrNil>,
    >;

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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for UnrelatedWu {
    type Read = ReadC;
    type Write = WriteCc;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, ReadC, WriteCc, PtrCons<InC, PtrNil>, ColPtrNil, ColPtrCons<Cc, ColPtrNil>>;

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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for SecondConsumerWu {
    type Read = ReadCb;
    type Write = WriteCd;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadCb,
        WriteCd,
        PtrNil,
        ColPtrCons<Cb, ColPtrNil>,
        ColPtrCons<Cd, ColPtrNil>,
    >;

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
    let provider = BumpProvider::<32768>::new();
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
    let provider = BumpProvider::<32768>::new();
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
