// A unit that is a pure per-record map (it reads one column, writes another and
// keeps no accumulator) is run through `run_fused`, but it does not declare its
// map: it does not implement `RecordOp`. The fused entry folds the carrier into
// a chain of `RecordOp` maps, so the carrier is refused at compile time, and
// that missing implementation is the only error. The twin
// `unmapped_unit_fused_pass.rs` differs only in the `impl RecordOp` block. The
// refusal in the other direction, a unit that declares its map and also
// appends, is the pair `appending_unit_fused_{fail,pass}.rs`.

use core::cell::Cell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
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
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

struct Provider {
    base: *mut u8,
    used: Cell<usize>,
}

unsafe impl Send for Provider {}
unsafe impl Sync for Provider {}

impl MemoryProviderApi for Provider {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let align = align.0.max(1);
        let addr = self.base as usize + self.used.get();
        let aligned = (addr + align - 1) / align * align - self.base as usize;
        if aligned + len.0 > 8192 {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        unsafe { self.base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

#[derive(Copy, Clone)]
struct Xa(u32);

#[derive(Copy, Clone, PartialEq)]
struct Ya(u32);

type ReadX = Cons<Column<Xa>, Empty>;
type WriteY = Cons<Column<Ya>, Empty>;

#[derive(Copy, Clone)]
struct MapWu;

impl BuilderInput for MapWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for MapWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadX,
        WriteY,
        SnapNil,
        ColPtrCons<Xa, ColPtrNil>,
        ColPtrCons<Ya, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadX;
    type Write = WriteY;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: `Xa` is read only here and `Ya` is this unit's exclusive
            // write column, both reserved for the record range.
            let x = unsafe { ctx.reader().read::<Xa, _>(i) };
            unsafe { ctx.writer().write::<Ya, _>(i, Ya(x.0)) };
        });
    }
}

fn store(provider: Provider) -> ArenaColumnStorage<Provider> {
    ArenaColumnStorage::new(provider)
}

fn main() {
    let base = std::vec![0u8; 8192].leak().as_mut_ptr();
    let provider = Provider {
        base,
        used: Cell::new(0),
    };
    let mut scheduler = Scheduler::builder()
        .with(Column::<Xa>::new())
        .with(Column::<Ya>::new())
        .with(MapWu)
        .build(store(provider), USize(4))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let outcome = scheduler.run_fused();
    assert!(matches!(outcome, notko::Outcome::Ok(())));
}
