// A unit writes `Differs<Column<Height>>` and reads `Reach<Column<Slot>, ONE>`,
// and the builder registers neither column. `build` must refuse, and name the
// stores under the wrappers, since a wrapper is not what is registered.
// The twin `unregistered_wrapped_store_pass.rs` differs only by the two
// `.with(Column::<..>::start_from(..))` lines, which it carries, and runs the unit to a
// value read back from `Height`.

use core::cell::Cell;

use arvo::{Bool, Cap, USize};
use hilavitkutin::dispatch::engine_ctx::{AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{Differs, Reach, rec};
use hilavitkutin_providers::ArenaColumnStorage;

const ONE: Cap = arvo_tensor::cap(1);
const RECORDS: usize = 4;

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

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

#[derive(Copy, Clone, PartialEq)]
struct Slot(u32);

#[derive(Copy, Clone, PartialEq)]
struct Height(u32);

type Around = Cons<Reach<Column<Slot>, ONE>, Empty>;
type Changed = Cons<Differs<Column<Height>>, Empty>;

struct Lift;

impl BuilderInput for Lift {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit<Always> for Lift {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Around,
        Changed,
        SnapNil,
        ColPtrCons<Slot, ColPtrNil>,
        ColPtrCons<Height, ColPtrNil>,
        AccPtrNil,
    >;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Read = Around;
    type Write = Changed;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|read| {
            let w = read.get(Reach::<Column<Slot>, ONE>::new());
            rec![Height(w.here().0)]
        });
    }
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
    let mut scheduler = Scheduler::builder()
        .with(Lift)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build refused a unit whose wrapped stores are registered"));
    assert!(matches!(scheduler.run(), notko::Outcome::Ok(())));
    // The column registered last is the bindings head.
    let height = scheduler.__bindings().__ptr().as_ptr();
    for i in 0 .. RECORDS {
        // SAFETY: `Height` holds `RECORDS` reserved records and no pass is running.
        let got: Height = unsafe { height.add(i).read() };
        assert_eq!(
            got.0, 5,
            "record {i} holds the value read through the wrapper"
        );
    }
}
