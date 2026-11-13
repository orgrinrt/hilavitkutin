// A consumer asks for `replace_value` on a registered `Resource<InA>` and is
// refused: the method takes the store marker `Resource<InA>` as its argument,
// so it names no value to install, and its `Replaceable` bound cannot be met by
// a consumer, because both `Replaceable` and `Resource` are foreign to it and
// the orphan rule refuses the impl the diagnostic suggests. The twin
// `replace_value_unreachable_pass.rs` differs only in the call: `mark_dirty`.
// This records what a consumer can call today, not what the design says
// (`DESIGN.md.tmpl` has the resource swapped); the catalogue row for ticket
// `147e3897011f` holds the intended test.

use core::cell::Cell;

use arvo::{Bool, USize};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Resource;
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

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

#[derive(Copy, Clone)]
struct InA(u32);

impl hilavitkutin_api::footprint::ResourceFootprint for InA {
    const L1_BYTES: USize = USize(0);
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
        .with(Resource::new(InA(1)))
        .build(store(provider), USize(4))
        .unwrap_or_else(|_| panic!("build should succeed"));
    scheduler.replace_value(Resource::<InA>::default());
}
