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
//! call, and `execute` is called once per (unit, morsel). The record count (600)
//! spans three morsels under the 256 default `MORSEL_SIZE`. The recorded
//! sequence is therefore the dispatch order across the six (unit, morsel) calls:
//! morsel-outer is `[P, C, P, C, P, C]` (one morsel runs both units before the
//! next), unit-outer is `[P, P, P, C, C, C]` (a unit completes all morsels
//! before the next unit).
//!
//! Red first: against the unit-outer dispatch the recorded order is
//! `[P, P, P, C, C, C]`, so the morsel-outer assertion fails; once the nesting
//! flips for accumulator-free pipelines it passes. A per-record result check
//! cannot discriminate the nesting (each record is independent, so both
//! nestings compute the same column), which is why the test observes dispatch
//! order directly.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::cell::RefCell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, PtrNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasAccumWriter, HasColumnReader,
    HasColumnWriter, HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::run_cfg::{DefaultRunCfg, RunCfg};
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

/// Wrap a provider in the default-capacity arena store (`D = Dim<256>`).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Stack-backed test memory provider (mirrors tests/column_dispatch.rs).
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

// Three morsels under the 256-record default `MORSEL_SIZE`: [0,256), [256,512),
// [512,600).
const RECORDS: usize = 600;

// Tag pushed once per `execute` call: producer is 0, consumer is 1.
const PRODUCER_TAG: u8 = 0;
const CONSUMER_TAG: u8 = 1;

// Column value: a Copy newtype over u32, so the blanket `ColumnValue` applies.
// The payload exists only to give the producer something to write and the
// consumer something to read (the RAW edge that orders them); the test asserts
// dispatch order, not the value, so the field is intentionally not inspected.
#[derive(Copy, Clone)]
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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for ProducerWu {
    type Read = Empty;
    type Write = Col;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, Empty, Col, PtrNil, ColPtrNil, ColPtrCons<Cv, ColPtrNil>>;

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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for ConsumerWu {
    type Read = Col;
    type Write = Empty;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, Col, Empty, PtrNil, ColPtrCons<Cv, ColPtrNil>, ColPtrNil>;

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
    let msize = <DefaultRunCfg as RunCfg>::MORSEL_SIZE.0;
    // The record count must exceed the morsel size so the windowing splits into
    // more than one morsel, which is what makes the two nestings distinguishable.
    assert!(RECORDS > msize, "test record count must exceed the morsel size");
    // The record count spans exactly three morsels under the default size.
    let morsels = RECORDS.div_ceil(msize);
    assert_eq!(morsels, 3, "the fixture assumes three morsels");

    DISPATCH_ORDER.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<32768>::new();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Cv>::new())
        .with(ProducerWu)
        .with(ConsumerWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));

    DISPATCH_ORDER.with(|o| {
        let observed = o.borrow();
        // Six dispatch calls: two units over three morsels. Morsel-outer runs
        // both units within each morsel before advancing.
        let expected = [
            PRODUCER_TAG, CONSUMER_TAG, // morsel [0, 256)
            PRODUCER_TAG, CONSUMER_TAG, // morsel [256, 512)
            PRODUCER_TAG, CONSUMER_TAG, // morsel [512, 600)
        ];
        assert_eq!(
            observed.as_slice(),
            &expected,
            "an accumulator-free pipeline dispatches morsel-outer (one morsel \
             runs the whole unit sequence before the next); the unit-outer order \
             would be [P, P, P, C, C, C]"
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
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for AppenderA {
    type Read = Empty;
    type Write = AccWa;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, AccWa, PtrNil, ColPtrNil, ColPtrNil, AccPtrCons<'frame, Av, AccPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(APPENDER_A_TAG));
        // SAFETY: `build` reserved the `Accum<Av>` buffer for the record count
        // (>= the one append per dispatch) and the plan proved this unit the
        // exclusive appender; the append saturates within the reservation.
        unsafe { ctx.accums().append::<Av, _>(Av(1)) };
    }
}

// Appender B: appends to `Accum<Bv>`, records its tag once per dispatch.
struct AppenderB;

impl BuilderInput for AppenderB {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for AppenderB {
    type Read = Empty;
    type Write = AccWb;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, Empty, AccWb, PtrNil, ColPtrNil, ColPtrNil, AccPtrCons<'frame, Bv, AccPtrNil>>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(APPENDER_B_TAG));
        // SAFETY: as above, for the `Accum<Bv>` buffer this unit exclusively
        // appends to.
        unsafe { ctx.accums().append::<Bv, _>(Bv(1)) };
    }
}

#[test]
fn accumulator_pipeline_dispatches_unit_outer() {
    let msize = <DefaultRunCfg as RunCfg>::MORSEL_SIZE.0;
    // Same multi-morsel record count as the morsel-outer test, so the two
    // nestings are distinguishable (a single morsel would make them identical).
    assert!(RECORDS > msize, "test record count must exceed the morsel size");
    let morsels = RECORDS.div_ceil(msize);
    assert_eq!(morsels, 3, "the fixture assumes three morsels");

    DISPATCH_ORDER.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<32768>::new();
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
        // Six dispatch calls: two units over three morsels. The accumulator
        // store makes the pipeline not accumulator-free, so dispatch is
        // unit-outer: each unit completes all three morsels before the next.
        // The recorded sequence is therefore contiguous per unit (three of one
        // tag then three of the other), never interleaved like the morsel-outer
        // `[a, b, a, b, a, b]`. The check is order-agnostic on which unit runs
        // first (both are independent appenders), pinning only the unit-outer
        // shape: a wrong guard routing this pipeline to morsel-outer would
        // interleave the tags and fail here.
        assert_eq!(observed.len(), 6, "two units dispatched over three morsels");
        let first = observed[0];
        let second = observed[3];
        assert_ne!(
            first, second,
            "the two units carry distinct tags, so the two contiguous runs differ"
        );
        assert_eq!(
            observed.as_slice(),
            &[first, first, first, second, second, second],
            "an accumulator pipeline dispatches unit-outer (each unit completes \
             all morsels before the next); a morsel-outer routing would interleave \
             the tags"
        );
    });
}
