// A unit that appends to an `Accum<Av>` is run through `run_fused`. The fused
// entry folds the carrier into one chain and a chain writes no accumulator, so
// the carrier is refused at compile time. The twin `append_fused_pass.rs`
// differs only in the entry called: `run` instead of `run_fused`.

use core::cell::Cell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{AccPtrCons, AccPtrNil, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, HasAccumWriter};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Accum;
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
        let aligned = (self.used.get() + align - 1) / align * align;
        if aligned + len.0 > 8192 {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        unsafe { self.base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

#[derive(Copy, Clone)]
struct Av(u32);

type AccW = Cons<Accum<Av>, Empty>;

#[derive(Copy, Clone)]
struct AppendWu;

impl BuilderInput for AppendWu {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for AppendWu {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Empty,
        AccW,
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
    type Write = AccW;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        unsafe { ctx.accums().append::<Av, _>(Av(700)) };
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
        .with(Accum::<Av>::new())
        .with(AppendWu)
        .build(store(provider), USize(4))
        .unwrap_or_else(|_| panic!("build should succeed"));
    let _ = scheduler.run_fused();
}
