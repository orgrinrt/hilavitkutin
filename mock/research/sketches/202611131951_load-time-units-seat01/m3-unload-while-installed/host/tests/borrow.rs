//! Seat 01 reply, M3 option b in its type-level form. Question: can the
//! occupant carry the library's lifetime, so that the borrow checker refuses
//! a drop while the occupant is installed, through the trunk's store path?
//!
//! The occupant borrows the `Library` it came from (`'lib`), exactly as
//! `Symbol<'lib, T>` does. It is registered as an ordinary `Resource`. If
//! this file does not compile, the type-level tie has no expressible form on
//! the trunk's store path, and the compiler output is the result. Not
//! restructured to build.

use core::marker::PhantomData;

use arvo::{Bool, USize};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Resource;
use hilavitkutin_linking::Library;
use hilavitkutin_providers::ArenaColumnStorage;
use m3_host::{Kernel, host_fallback};

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

/// An occupant that cannot outlive the library it was resolved from.
#[derive(Copy, Clone)]
struct BorrowedOccupant<'lib> {
    kernel: Kernel,
    _lib:   PhantomData<&'lib Library>,
}
impl hilavitkutin_api::store::Replaceable for BorrowedOccupant<'_> {}
impl hilavitkutin_api::footprint::ResourceFootprint for BorrowedOccupant<'_> {
    const L1_BYTES: USize = USize(0);
}

fn install<'lib>(_lib: &'lib Library) {
    let occ = BorrowedOccupant::<'lib> {
        kernel: host_fallback,
        _lib:   PhantomData,
    };
    let _s = Scheduler::builder()
        .with(Resource::new(occ))
        .build(store(ZeroedHeap), USize(0))
        .unwrap_or_else(|_| panic!("build"));
}

#[test]
fn an_occupant_borrowing_its_library_is_a_resource() {
    let _ = install;
}
