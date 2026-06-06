//! Phase-sequential dispatch integration test (HILA-RUNTIME, Phase E).
//!
//! `Scheduler::run` dispatches units in the plan's phase-sequential order (the
//! plan flattened `phases -> trunks -> fibers -> units`, derived at build),
//! not the flat `unit_meta` topological permutation. This test exercises a
//! three-unit linear chain through that order.
//!
//! A producer writes `Column<Av>`; a transform reads `Av` and writes
//! `Column<Bv>`; a consumer reads `Bv`. The two RAW edges (`Av`:
//! producer->transform, `Bv`: transform->consumer) force the dependency order.
//! The columns are registered first, then the producer, then the transform,
//! then the consumer, so the builder's append keeps that topological
//! registration order in the carrier. The build-time topological-registration
//! gate requires this order; an anti-topological registration (consumer first)
//! is rejected at build with `BuildError::NonTopologicalRegistration`. The
//! plan's phase structure dispatches producer -> transform -> consumer, and the
//! phase flatten visits all three units, so the consumer reads back the full
//! chain.
//!
//! This validates the phase flatten: a dropped unit (an incomplete partition)
//! leaves the consumer reading uninitialised records, and a mis-ordered phase
//! walk runs a reader before its writer. It guards that the flatten preserves
//! dependency order and visits every unit.
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

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

const N: usize = 4;

#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);

type ColA = Cons<Column<Av>, Empty>;
type ColB = Cons<Column<Bv>, Empty>;

thread_local! {
    static OBSERVED: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}

// Producer: writes Av(i * 10) at each record.
struct ProducerWu;
impl BuilderInput for ProducerWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ProducerWu {
    type Read = Empty;
    type Write = ColA;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, Empty, ColA, PtrNil, ColPtrNil, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: build reserved the column; the plan proved this the
            // exclusive writer; the morsel covers reserved records.
            unsafe { ctx.writer().write::<Av, _>(i, Av(i.0 as u32 * 10)) };
        });
    }
}

// Transform: reads Av, writes Bv = Av + 1.
struct TransformWu;
impl BuilderInput for TransformWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for TransformWu {
    type Read = ColA;
    type Write = ColB;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, ColA, ColB, PtrNil, ColPtrCons<Av, ColPtrNil>, ColPtrCons<Bv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: the producer (ordered before this unit by the plan's RAW
            // edge on `Av`) wrote every record the morsel covers.
            let a: Av = unsafe { ctx.reader().read::<Av, _>(i) };
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(a.0 + 1)) };
        });
    }
}

// Consumer: reads Bv, records it.
struct ConsumerWu;
impl BuilderInput for ConsumerWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for ConsumerWu {
    type Read = ColB;
    type Write = Empty;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<'frame, ColB, Empty, PtrNil, ColPtrCons<Bv, ColPtrNil>, ColPtrNil>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: the transform (ordered before this unit by the plan's RAW
            // edge on `Bv`) wrote every record the morsel covers.
            let b: Bv = unsafe { ctx.reader().read::<Bv, _>(i) };
            OBSERVED.with(|o| o.borrow_mut().push(b.0));
        });
    }
}

#[test]
fn phase_sequential_dispatch_runs_the_chain_in_order() {
    OBSERVED.with(|o| o.borrow_mut().clear());
    let provider = BumpProvider::<16384>::new();
    // Register the two columns, then producer, transform, consumer. The builder
    // appends, so the retained carrier is [producer, transform, consumer] in
    // registration order: a valid topological order of the chain's RAW edges Av
    // (producer->transform) and Bv (transform->consumer). The build-time
    // topological-registration gate requires this; an anti-topological
    // registration (consumer first) is rejected with
    // BuildError::NonTopologicalRegistration. The phase flatten dispatches the
    // chain in dependency order and visits every unit.
    let mut scheduler = Scheduler::builder()
        .with(Column::<Av>::new())
        .with(Column::<Bv>::new())
        .with(ProducerWu)
        .with(TransformWu)
        .with(ConsumerWu)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));

    OBSERVED.with(|o| {
        assert_eq!(
            o.borrow().as_slice(),
            &[1u32, 11u32, 21u32, 31u32],
            "the consumer read back Av(i*10) + 1 for every record, so the phase \
             flatten dispatched producer -> transform -> consumer in order and \
             visited all three units"
        );
    });
}
