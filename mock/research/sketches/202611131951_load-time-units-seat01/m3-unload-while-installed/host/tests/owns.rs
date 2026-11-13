//! Seat 01 reply, M3 option b in its ownership form. Question: can the
//! occupant own the library, so that the image lives exactly as long as the
//! occupant does, through the trunk's store path?
//!
//! `Library` has a `Drop` impl (hilavitkutin-linking/src/lib.rs:110-118), so
//! a type that owns one cannot be `Copy`. It is declared here without `Copy`
//! and registered as a resource of a built scheduler. `Resource::new` itself
//! takes any `T` (hilavitkutin-api/src/store.rs:46); the question is whether
//! `build` does. If this file does not compile, an owning occupant has no
//! expressible form as a resource, and a store kind that owns its occupant is
//! a store kind the trunk does not have. Not restructured to build.

use arvo::{Bool, USize};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Resource;
use hilavitkutin_linking::Library;
use hilavitkutin_providers::ArenaColumnStorage;
use m3_host::Kernel;

struct ZeroedHeap;
unsafe impl Send for ZeroedHeap {}
unsafe impl Sync for ZeroedHeap {}
impl MemoryProviderApi for ZeroedHeap {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        unsafe {
            std::alloc::alloc_zeroed(std::alloc::Layout::from_size_align(len.0, align.0).unwrap())
        }
    }

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize) {
        unsafe {
            std::alloc::dealloc(
                ptr,
                std::alloc::Layout::from_size_align(len.0, align.0).unwrap(),
            )
        }
    }

    unsafe fn protect(&self, _p: *mut u8, _l: USize, _r: Bool, _w: Bool) {}
}

fn store<M: MemoryProviderApi>(m: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(m)
}

/// An occupant that owns the image its kernel points into.
struct OwningOccupant {
    kernel: Kernel,
    lib:    Library,
}
impl hilavitkutin_api::store::Replaceable for OwningOccupant {}
impl hilavitkutin_api::footprint::ResourceFootprint for OwningOccupant {
    const L1_BYTES: USize = USize(0);
}

fn install(occ: OwningOccupant) {
    let _s = Scheduler::builder()
        .with(Resource::new(occ))
        .build(store(ZeroedHeap), USize(0))
        .unwrap_or_else(|_| panic!("build"));
}

#[test]
fn an_occupant_owning_its_library_is_a_resource() {
    let _ = install;
}
