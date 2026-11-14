// A unit declares its value function (`RecordOp`) and its write list holds an
// accumulator beside the column, so every record appends one value. The fused
// entry folds a carrier of units that read one column and write one column
// into a chain that threads values in registers, and a chain has nowhere to
// append. The carrier is refused at compile time. The twin
// `appending_unit_fused_pass.rs` differs only in the entry it calls: `run`
// executes the unit's own body, which appends.

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
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Accum, Column, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{ReadVals, RecordOp, WriteVals, rec};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Maybe;

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

#[derive(Copy, Clone, Default)]
struct Xa(u32);

#[derive(Copy, Clone, PartialEq, Default)]
struct Ya(u32);

#[derive(Copy, Clone)]
struct Log(u32);

type ReadX = Cons<Column<Xa>, Empty>;
type WriteBoth = Cons<Column<Ya>, Cons<Accum<Log>, Empty>>;

#[derive(Copy, Clone)]
struct MapAndLogWu;

impl BuilderInput for MapAndLogWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for MapAndLogWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        ReadX,
        WriteBoth,
        SnapNil,
        ColPtrCons<Xa, ColPtrNil>,
        ColPtrCons<Ya, ColPtrNil>,
        AccPtrCons<'frame, Log, AccPtrNil>,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadX;
    type Write = WriteBoth;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|read| self.apply(read));
    }
}

impl RecordOp for MapAndLogWu {
    fn apply(&self, read: &ReadVals<'_, ReadX>) -> WriteVals<WriteBoth> {
        let x = read.get(Column::<Xa>::new());
        rec![Ya(x.0 + 1), Maybe::Is(Log(x.0 + 7))]
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
    // `Log` is registered last, so its accumulator is the head of the bindings.
    let mut scheduler = Scheduler::builder()
        .with(Column::<Xa>::start())
        .with(Column::<Ya>::start())
        .with(Accum::<Log>::new())
        .with(MapAndLogWu)
        .build(store(provider), USize(4))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let outcome = scheduler.run_fused();
    assert!(matches!(outcome, notko::Outcome::Ok(())));
    assert_eq!(scheduler.__bindings().__len_cell().get(), USize(4));
    let logged = scheduler.__bindings().__ptr().as_ptr();
    // SAFETY: four values were appended, so the first of them is initialised.
    assert_eq!(unsafe { core::ptr::read(logged) }.0, 7);
}
