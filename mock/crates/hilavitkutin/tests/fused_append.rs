//! Two-frame append through the single-core entry and the fused entry.
//!
//! One unit maps a record column (`RecordOp`) and also appends one value per
//! record to an accumulator. `run` executes the unit's body, so each frame
//! leaves one appended value per record in the accumulator. `run_fused` folds
//! the carrier into a chain of `RecordOp` maps and runs only the maps: the
//! unit's body, with its appends, never runs, and nothing refuses the carrier
//! at compile time because `FuseCarrier` bounds a unit on `RecordOp` and `Copy`
//! and not on its write set.
//!
//! The first test holds the behaviour the second is compared against, so the
//! second cannot fail for the harness's sake. The second asserts the same live
//! length through `run_fused` and fails today; it is ignored with ticket
//! `0fb716c5a463`, which owns making the fused entry refuse the carrier or run
//! the body. Drop the `#[ignore]` when that ticket closes (a refusal turns this
//! into a compile-fail pair).
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::Cell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons,
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::RecordOp;
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
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Heap-backed bump provider whose address survives moves of the scheduler.
struct StableProvider {
    base: *mut u8,
    cap:  usize,
    used: Cell<usize>,
}

impl StableProvider {
    fn new(cap: usize) -> Self {
        let leaked: &'static mut [u8] = std::vec![0u8; cap].leak();
        Self {
            base: leaked.as_mut_ptr(),
            cap,
            used: Cell::new(0),
        }
    }
}

unsafe impl Send for StableProvider {}
unsafe impl Sync for StableProvider {}

impl MemoryProviderApi for StableProvider {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let align = align.0.max(1);
        let addr = self.base as usize + self.used.get();
        let aligned = (addr + align - 1) / align * align - self.base as usize;
        if aligned + len.0 > self.cap {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: `aligned + len <= cap`, inside the leaked buffer, and the
        // returned address is a multiple of `align`.
        unsafe { self.base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// Records per frame, and the accumulator's reserved capacity (the record
// count, which is what `build` reserves).
const RECORDS: usize = 2;

#[derive(Copy, Clone)]
struct InV(u32);
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct OutV(u32);
#[derive(Copy, Clone)]
struct Log(u32);

type ReadIn = Cons<Column<InV>, Empty>;
type WriteBoth = Cons<Column<OutV>, Cons<Accum<Log>, Empty>>;

// Maps `InV` to `OutV` and appends one `Log` per record.
#[derive(Copy, Clone)]
struct MapAndLogWu;

impl BuilderInput for MapAndLogWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for MapAndLogWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadIn,
        WriteBoth,
        SnapNil,
        ColPtrCons<InV, ColPtrNil>,
        ColPtrCons<OutV, ColPtrNil>,
        AccPtrCons<'frame, Log, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadIn;
    type Write = WriteBoth;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: `InV` is reserved for the record range and read only
            // here; `OutV` is this unit's exclusive write column; the unit holds
            // the exclusive appender slot for `Log`, and the accumulator is
            // reserved for the record count, one append per record.
            let v = unsafe { ctx.reader().read::<InV, _>(i) };
            unsafe { ctx.writer().write::<OutV, _>(i, OutV(v.0 + 1)) };
            unsafe { ctx.accums().append::<Log, _>(Log(i.0 as u32)) };
        });
    }
}

impl RecordOp for MapAndLogWu {
    type In = InV;
    type Out = OutV;

    fn apply(&self, x: InV) -> OutV {
        OutV(x.0 + 1)
    }
}

macro_rules! scheduler {
    () => {{
        let provider = StableProvider::new(16384);
        // `Log` is registered last, so its accumulator is the bindings head.
        Scheduler::builder()
            .with(Column::<InV>::new())
            .with(Column::<OutV>::new())
            .with(Accum::<Log>::new())
            .with(MapAndLogWu)
            .build(store(provider), USize(RECORDS))
            .unwrap_or_else(|_| panic!("build should succeed"))
    }};
}

#[test]
fn run_appends_one_value_per_record_each_frame() {
    let mut scheduler = scheduler!();
    for frame in 0 .. 2 {
        assert!(matches!(scheduler.run(), Outcome::Ok(())));
        assert_eq!(
            scheduler.__bindings().__len_cell().get(),
            USize(RECORDS),
            "frame {frame}: one append per record through `run`",
        );
        let base = scheduler.__bindings().__ptr().as_ptr();
        // SAFETY: `RECORDS` values were appended this frame into a buffer reset
        // at frame start; records `[0, RECORDS)` are initialised.
        let logged =
            [unsafe { core::ptr::read(base.add(0)) }, unsafe { core::ptr::read(base.add(1)) }];
        assert_eq!(
            [logged[0].0, logged[1].0],
            [0u32, 1u32],
            "frame {frame}: the appended values land in record order",
        );
    }
}

#[test]
#[ignore = "catalogue: run_fused runs only the map and never the body of a RecordOp unit that also appends; needs the fused entry to refuse the carrier or run the body; tracked #run_fused_does_not_refuse_a_recordop_unit_that_writes_an_acc"]
fn run_fused_appends_one_value_per_record_each_frame() {
    let mut scheduler = scheduler!();
    for frame in 0 .. 2 {
        let _ = scheduler.run_fused();
        assert_eq!(
            scheduler.__bindings().__len_cell().get(),
            USize(RECORDS),
            "frame {frame}: one append per record through `run_fused`, as through `run`",
        );
    }
}
