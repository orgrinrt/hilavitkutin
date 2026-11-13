// A unit reads `Resource<InA>` but the builder never registers it. `build`
// must refuse: the unit's read set is not contained in the registered stores.
// The twin `unregistered_store_pass.rs` differs only by the `.with(Resource::new(InA(1)))` line.

use core::cell::Cell;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrNil, EngineCtx, SnapCons, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Resource;
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
        if aligned + len.0 > 4096 {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        unsafe { self.base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

#[derive(Copy, Clone)]
struct InA(u32);

impl hilavitkutin_api::footprint::ResourceFootprint for InA {
    const L1_BYTES: USize = USize(0);
}

type ReadA = Cons<Resource<InA>, Empty>;

struct ReadsInA;

impl BuilderInput for ReadsInA {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for ReadsInA {
    type Ctx<'frame> =
        EngineCtx<'frame, ReadA, Empty, SnapCons<InA, SnapNil>, ColPtrNil, ColPtrNil>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = ReadA;
    type Write = Empty;

    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {}
}

fn store(provider: Provider) -> ArenaColumnStorage<Provider> {
    ArenaColumnStorage::new(provider)
}

fn main() {
    let base = std::vec![0u8; 4096].leak().as_mut_ptr();
    let provider = Provider {
        base,
        used: Cell::new(0),
    };
    let _ = Scheduler::builder()
        .with(Resource::new(InA(1)))
        .with(ReadsInA)
        .build(store(provider), USize(0));
}
