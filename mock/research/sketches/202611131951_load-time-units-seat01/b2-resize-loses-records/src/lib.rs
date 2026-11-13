//! A std-heap `MemoryProviderApi` for the spike, zeroing every block it hands
//! out so a test that reads a fresh block reads zeros and never garbage. The
//! zeroing is what makes the red test deterministic: a resize that kept the
//! records reads them back, a resize that did not reads zeros.

use std::alloc::{Layout, alloc_zeroed, dealloc};

use arvo::{Bool, USize};
use hilavitkutin_api::MemoryProviderApi;

pub struct ZeroedHeap;

unsafe impl Send for ZeroedHeap {}
unsafe impl Sync for ZeroedHeap {}

impl MemoryProviderApi for ZeroedHeap {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let layout = Layout::from_size_align(len.0, align.0).expect("layout");
        unsafe { alloc_zeroed(layout) }
    }

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize) {
        let layout = Layout::from_size_align(len.0, align.0).expect("layout");
        unsafe { dealloc(ptr, layout) }
    }

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}
