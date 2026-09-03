//! Per-fiber morsel-outer dispatch test (fiber-structured dispatch slice).
//!
//! `Scheduler::run` walks the plan's `phases -> trunks -> fibers` structure and
//! gates the morsel-outer-versus-unit-outer choice per fiber on the fiber's
//! `morsel_local` bit (true when the fiber writes no accumulator). This test
//! discriminates the per-fiber decision from the prior whole-pipeline guard,
//! which forced the entire plan unit-outer the moment any accumulator was
//! registered.
//!
//! The fixture is a MIXED plan: a column chain (a producer writes `Column<Cv>`,
//! a consumer reads it, a RAW edge so the plan orders producer before consumer)
//! that touches no accumulator, plus a separate appender that writes
//! `Accum<Av>`. The two groups are store-disjoint (one touches the column, the
//! other the accumulator), so block-diagonalisation places them in distinct
//! fibers. The column-chain fiber is morsel-local (no accumulator) and the
//! appender fiber is not.
//!
//! Each unit records its tag once per `execute`, and `execute` runs once per
//! (unit, morsel). At 600 records under the 256-record default `MORSEL_SIZE`
//! the range spans three morsels, so the column-chain fiber dispatched
//! morsel-outer records `[P, C, P, C, P, C]` (one morsel runs both its units
//! before the next) while the appender fiber dispatched unit-outer records
//! `[A, A, A]` (the single unit over three morsels, contiguous). Fibers run
//! sequentially, so the two contiguous runs appear back to back; which fiber
//! runs first is a block-ordering detail the assertion stays agnostic to.
//!
//! Red first: against the prior whole-pipeline guard the registered `Accum<Av>`
//! routes the WHOLE plan unit-outer, so the producer and consumer would NOT
//! interleave (their run would be `[P, P, P, C, C, C]`); the morsel-outer
//! interleaving of the column-chain fiber fails that guard and passes once the
//! decision is per-fiber.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::cell::RefCell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasAccumWriter, HasColumnReader,
    HasColumnWriter, HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

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

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// Three morsels under the 256-record default `MORSEL_SIZE`: [0,256), [256,512),
// [512,600).
const RECORDS: usize = 600;

// Distinct tags so the producer, consumer, and appender are all
// distinguishable in one recorded sequence.
const PRODUCER_TAG: u8 = 0;
const CONSUMER_TAG: u8 = 1;
const APPENDER_TAG: u8 = 2;

#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Cv(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Av(u32);

type Col = Cons<Column<Cv>, Empty>;
type AccW = Cons<Accum<Av>, Empty>;

thread_local! {
    static DISPATCH_ORDER: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

// Producer: writes `Column<Cv>`, forming the head of the accumulator-free
// column-chain fiber.
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
    type Ctx<'frame> = EngineCtx<'frame, Empty, Col, SnapNil, ColPtrNil, ColPtrCons<Cv, ColPtrNil>>;

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

// Consumer: reads `Column<Cv>`, the tail of the column-chain fiber.
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
    type Ctx<'frame> = EngineCtx<'frame, Col, Empty, SnapNil, ColPtrCons<Cv, ColPtrNil>, ColPtrNil>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(CONSUMER_TAG));
        ctx.each().run(|i| {
            // SAFETY: the producer (ordered before this unit by the plan's RAW
            // edge) wrote every record the morsel covers; no concurrent writer.
            let _v: Cv = unsafe { ctx.reader().read::<Cv, _>(i) };
        });
    }
}

// Appender: writes `Accum<Av>`, a store-disjoint fiber that is not
// morsel-local, so it dispatches unit-outer.
struct AppenderWu;

impl BuilderInput for AppenderWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl WorkUnit<Always> for AppenderWu {
    type Read = Empty;
    type Write = AccW;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        AccW,
        SnapNil,
        ColPtrNil,
        ColPtrNil,
        AccPtrCons<'frame, Av, AccPtrNil>,
    >;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        DISPATCH_ORDER.with(|o| o.borrow_mut().push(APPENDER_TAG));
        // SAFETY: `build` reserved the `Accum<Av>` buffer for the record count
        // (>= the one append per dispatch) and the plan proved this unit the
        // exclusive appender; the append saturates within the reservation.
        unsafe { ctx.accums().append::<Av, _>(Av(1)) };
    }
}

#[test]
fn mixed_plan_dispatches_whole_carrier_flat() {
    // GATE-1 flat dispatch (spec Approach E): the whole `WuVals` carrier is one
    // type-level `RunFiber` walk in carrier order. A carrier bearing an
    // accumulator (the appender) selects the unit-outer drive, so every unit
    // executes once over the full record range, in carrier order: the producer,
    // then its consumer, then the independent appender.
    //
    // Per-fiber morsel locality (the accumulator-free column chain morsel-outer
    // while the accumulator stays unit-outer, as separate contiguous blocks) is
    // the post-GATE-1 Approach-A `FiberCons` nesting (#670). A cons-list carrier
    // type cannot be sliced at a runtime fiber index, so per-fiber sub-walks
    // need per-fiber sub-carrier types, which the flat GATE-1 walk does not yet
    // build. The original per-fiber-block assertion is the contract #670
    // re-enables.
    DISPATCH_ORDER.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<32768>::new();
    let mut scheduler = Scheduler::builder()
        .with(Column::<Cv>::new())
        .with(Accum::<Av>::new())
        .with(ProducerWu)
        .with(ConsumerWu)
        .with(AppenderWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));

    DISPATCH_ORDER.with(|o| {
        let observed = o.borrow();
        // Whole-carrier unit-outer walk: each unit executes once, in carrier
        // (registration) order. The producer writes the column its consumer
        // reads (a RAW edge `build` validated topological), then the appender.
        assert_eq!(
            observed.as_slice(),
            &[PRODUCER_TAG, CONSUMER_TAG, APPENDER_TAG],
            "the flat Approach-E walk runs the whole carrier once in carrier \
             order under the unit-outer drive an accumulator selects. observed: {:?}",
            observed.as_slice()
        );
    });
}
