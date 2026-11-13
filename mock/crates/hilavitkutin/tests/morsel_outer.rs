//! Morsel-outer dispatch nesting test (single-core dispatch completion, slice 1).
//!
//! `Scheduler::run` dispatches an accumulator-free pipeline morsel-outer: one
//! morsel runs the whole unit sequence before the next morsel. This test
//! discriminates the nesting from the prior unit-outer (each unit completes its
//! whole record range before the next unit runs).
//!
//! Two units form an accumulator-free column chain: a producer writes a
//! `Column<Cv>`, a consumer reads it (a RAW edge, so the plan dispatches the
//! producer before the consumer). Each unit records its tag once per `execute`
//! call, and `execute` is called once per (unit, morsel). The record count
//! (7000) spans two morsels under the fiber's plan-baked 6144-record L1
//! window (A2b). The recorded sequence is therefore the dispatch order across
//! the (unit, morsel) calls: morsel-outer is `[P, C, P, C]` (one morsel runs
//! both units before the next), unit-outer is `[P, P, C, C]` (a unit
//! completes all morsels before the next unit).
//!
//! Red first: against the unit-outer dispatch the recorded order is
//! `[P, P, C, C]`, so the morsel-outer assertion fails; once the nesting
//! flips for accumulator-free pipelines it passes. A per-record result check
//! cannot discriminate the nesting (each record is independent, so both
//! nestings compute the same column), which is why the test observes dispatch
//! order directly.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

mod stable_provider;

use std::cell::RefCell;

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
use hilavitkutin_api::context::{
    AccumWriterApi,
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasAccumWriter,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Accum, Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use stable_provider::StableProvider;

/// Wrap a provider in the default-capacity arena store (`D = Dim<256>`).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Two morsels under the fiber's plan-baked L1 window (one 4-byte write column
// under the default budget windows at 24576 / 4 = 6144): [0,6144), [6144,7000).
const RECORDS: usize = 7000;

// Tag pushed once per `execute` call: producer is 0, consumer is 1.
const PRODUCER_TAG: u8 = 0;
const CONSUMER_TAG: u8 = 1;

// Column value: a Copy newtype over u32, so the blanket `ColumnValue` applies.
// The payload exists only to give the producer something to write and the
// consumer something to read (the RAW edge that orders them); the test asserts
// dispatch order, not the value, so the field is intentionally not inspected.
#[derive(Copy, Clone, Default)]
#[allow(dead_code)]
struct Cv(u32);

type Col = Cons<Column<Cv>, Empty>;

thread_local! {
    // The dispatch order across (unit, morsel) calls: each unit pushes its tag
    // once per `execute`.
    static DISPATCH_ORDER: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

// Producer: writes Column<Cv>, reads nothing. Records its tag once per dispatch.
struct ProducerWu;

impl BuilderInput for ProducerWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for ProducerWu {
    type Ctx<'frame> = EngineCtx<'frame, Empty, Col, SnapNil, ColPtrNil, ColPtrCons<Cv, ColPtrNil>>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = Empty;
    type Write = Col;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(PRODUCER_TAG));
        ctx.each().run(|i| {
            // SAFETY: `build` reserved the `Column<Cv>` buffer for the record
            // count and the plan proved this unit the exclusive writer; the
            // morsel covers only reserved records.
            unsafe { ctx.writer().write::<Cv, _>(i, Cv(i.0 as u32)) };
        });
    }
}

// Consumer: reads Column<Cv>, writes nothing. Records its tag once per dispatch.
struct ConsumerWu;

impl BuilderInput for ConsumerWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for ConsumerWu {
    type Ctx<'frame> = EngineCtx<'frame, Col, Empty, SnapNil, ColPtrCons<Cv, ColPtrNil>, ColPtrNil>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = Col;
    type Write = Empty;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(CONSUMER_TAG));
        ctx.each().run(|i| {
            // SAFETY: the producer (ordered before this unit by the plan's RAW
            // edge) wrote every record the morsel covers under both nestings; no
            // concurrent writer.
            let _v: Cv = unsafe { ctx.reader().read::<Cv, _>(i) };
        });
    }
}

#[test]
fn accumulator_free_pipeline_dispatches_morsel_outer() {
    DISPATCH_ORDER.with(|o| o.borrow_mut().clear());
    let provider = StableProvider::new(65536);
    let mut scheduler = Scheduler::builder()
        .with(Column::<Cv>::start())
        .with(ProducerWu)
        .with(ConsumerWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // The dispatch loop windows the fiber by its plan-baked L1 morsel window
    // (A2b); read it off the descriptor so the expectation tracks the plan
    // value (r6_morsel_window_formula pins the formula itself). The record
    // count must exceed the window so the windowing splits into more than one
    // morsel, which is what makes the two nestings distinguishable.
    let window = scheduler.__fiber_morsel_size(USize(0)).0;
    assert!(window > 0, "fiber 0 carries a plan-baked window");
    let morsels = RECORDS.div_ceil(window);
    assert!(
        morsels >= 2,
        "test record count must exceed the morsel window"
    );

    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));

    DISPATCH_ORDER.with(|o| {
        let observed = o.borrow();
        // Two dispatch calls per morsel: morsel-outer runs both units within
        // each morsel before advancing.
        let mut expected = std::vec::Vec::new();
        for _ in 0 .. morsels {
            expected.push(PRODUCER_TAG);
            expected.push(CONSUMER_TAG);
        }
        assert_eq!(
            observed.as_slice(),
            expected.as_slice(),
            "an accumulator-free pipeline dispatches morsel-outer (one morsel \
             runs the whole unit sequence before the next); the unit-outer order \
             would be [P, .., P, C, .., C]"
        );
    });
}

// Accumulator value types. Two distinct accumulators so each appender is the
// exclusive writer of its own store (mirroring the one-appender-per-accum model
// in tests/accum_dispatch.rs); registering either makes the pipeline not
// accumulator-free, which routes dispatch to the unit-outer branch.
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Av(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Bv(u32);

type AccWa = Cons<Accum<Av>, Empty>;
type AccWb = Cons<Accum<Bv>, Empty>;

const APPENDER_A_TAG: u8 = 0;
const APPENDER_B_TAG: u8 = 1;

// Appender A: appends to `Accum<Av>`, records its tag once per dispatch.
struct AppenderA;

impl BuilderInput for AppenderA {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for AppenderA {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        AccWa,
        SnapNil,
        ColPtrNil,
        ColPtrNil,
        AccPtrCons<'frame, Av, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = Empty;
    type Write = AccWa;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(APPENDER_A_TAG));
        // `build` reserved the `Accum<Av>` buffer for the record count
        // (>= the one append per dispatch) and the plan proved this unit the
        // exclusive appender; the append stays within the reservation.
        ctx.accums().append::<Av, _>(Av(1));
    }
}

// Appender B: appends to `Accum<Bv>`, records its tag once per dispatch.
struct AppenderB;

impl BuilderInput for AppenderB {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for AppenderB {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        AccWb,
        SnapNil,
        ColPtrNil,
        ColPtrNil,
        AccPtrCons<'frame, Bv, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = Empty;
    type Write = AccWb;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(APPENDER_B_TAG));
        // As above, for the `Accum<Bv>` buffer this unit exclusively
        // appends to.
        ctx.accums().append::<Bv, _>(Bv(1));
    }
}

#[test]
fn accumulator_pipeline_dispatches_unit_outer() {
    // A carrier bearing an accumulator selects the unit-outer drive: the flat
    // GATE-1 walk (spec Approach E) runs the whole carrier once over the full
    // record range, so each unit executes once (its own `each()` loops the
    // records internally), in carrier (registration) order. This is the
    // cross-record-safe shape (a downstream reader sees the upstream
    // accumulator's complete output), distinct from the morsel-outer
    // interleave `[a, b, a, b, a, b]` an accumulator-free carrier takes.
    //
    // Per-morsel-granular unit-outer (each unit dispatched once per morsel, all
    // morsels for a unit before the next: `[A, A, A, B, B, B]`) is the
    // post-GATE-1 Approach-A `FiberCons` nesting (#670); the flat walk cannot
    // slice the carrier per unit at a runtime morsel index.
    DISPATCH_ORDER.with(|o| o.borrow_mut().clear());
    let provider = StableProvider::new(131072);
    let mut scheduler = Scheduler::builder()
        .with(Accum::<Av>::new())
        .with(Accum::<Bv>::new())
        .with(AppenderA)
        .with(AppenderB)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));

    DISPATCH_ORDER.with(|o| {
        let observed = o.borrow();
        // Two dispatch calls: each appender executes once over the full range,
        // in carrier order. A wrong morsel-outer routing would interleave the
        // tags `[A, B, A, B, ...]`; the full-range unit-outer drive runs each
        // appender exactly once.
        assert_eq!(
            observed.as_slice(),
            &[APPENDER_A_TAG, APPENDER_B_TAG],
            "an accumulator carrier dispatches unit-outer as one full-range walk \
             in carrier order; a morsel-outer routing would interleave the tags"
        );
    });
}
