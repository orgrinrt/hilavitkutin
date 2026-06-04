//! Column dispatch round-trip integration test (column data plane, round 1).
//!
//! A producer WorkUnit writes a `Column<Cv>` over `[0, N)`; a consumer
//! WorkUnit reads the column back. The producer is registered first, so the
//! builder's prepend puts the consumer ahead in the retained list; a plain
//! registration walk would run the consumer (reading uninitialised records)
//! before the producer. The plan adds a RAW edge producer to consumer (the
//! producer writes what the consumer reads), so topological dispatch reorders
//! them: the producer runs first and fills the column, the consumer reads back
//! exactly what was written. This pins the round-1 contract that column-bearing
//! units dispatch, that the column buffer is reserved real (sized by the
//! build-time record count), and that the bindings serve as the column source.
//!
//! Red first: before the dispatch bound is lifted, a column-bearing unit cannot
//! satisfy the resource-only `CollectFiber` bound and the file does not compile;
//! once lifted but before the drain reserves a real buffer, the readback reads a
//! dangling placeholder. Both precede the green round-trip.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use std::cell::RefCell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

/// Wrap a provider in the default-capacity arena store (`D = Dim<256>`).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Stack-backed test memory provider (mirrors tests/scheduler_run_walk.rs).
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

// The record count for the test column.
const N: usize = 4;

// Column value: a Copy newtype over u32, so the blanket `ColumnValue` applies.
#[derive(Copy, Clone)]
struct Cv(u32);

type Col = Cons<Column<Cv>, Empty>;

thread_local! {
    static OBSERVED: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}

// Producer: writes Column<Cv>, reads nothing. Writes `Cv(i * 10)` at each
// record index in its morsel.
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
        ctx.each().run(|i| {
            // SAFETY: `build` reserved the `Column<Cv>` buffer for the record
            // count and the plan proved this unit the exclusive writer; the
            // morsel covers only reserved records.
            unsafe { ctx.writer().write::<Cv, _>(i, Cv(i.0 as u32 * 10)) };
        });
    }
}

// Consumer: reads Column<Cv>, writes nothing. Pushes each record it reads.
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
        ctx.each().run(|i| {
            // SAFETY: the producer (ordered before this unit by the plan's RAW
            // edge) wrote every record the morsel covers; no concurrent writer.
            let v: Cv = unsafe { ctx.reader().read::<Cv, _>(i) };
            OBSERVED.with(|o| o.borrow_mut().push(v.0));
        });
    }
}

#[test]
fn column_round_trips_producer_to_consumer_through_run() {
    OBSERVED.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<8192>::new();
    // Register the column, then the producer, then the consumer. The builder
    // prepends, so the retained value list is [consumer, producer]: a plain
    // registration walk runs the consumer first (reading uninitialised
    // records). The plan adds a RAW edge producer to consumer, so topological
    // dispatch runs the producer first. The consumer then reads back what the
    // producer wrote.
    let mut scheduler = Scheduler::builder()
        .with(Column::<Cv>::new())
        .with(ProducerWu)
        .with(ConsumerWu)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // The drain reserved the column sized by the record count.
    assert_eq!(scheduler.__bindings().__count(), USize(N));

    let result = scheduler.run();

    assert!(matches!(result, Outcome::Ok(())));
    OBSERVED.with(|o| {
        assert_eq!(
            o.borrow().as_slice(),
            &[0u32, 10u32, 20u32, 30u32],
            "the consumer read back exactly what the producer wrote, in record \
             order, because the plan ran the producer first"
        );
    });
}
