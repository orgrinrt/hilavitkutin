//! Spike, seat 02 phase two. A zeroing std-heap provider for the trunk store.

use std::alloc::{Layout, alloc_zeroed, dealloc};

use arvo::{Bool, USize};
use hilavitkutin_api::MemoryProviderApi;

pub struct ZeroedHeap;

unsafe impl Send for ZeroedHeap {}
unsafe impl Sync for ZeroedHeap {}

impl MemoryProviderApi for ZeroedHeap {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        unsafe { alloc_zeroed(Layout::from_size_align(len.0, align.0).unwrap()) }
    }

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize) {
        unsafe { dealloc(ptr, Layout::from_size_align(len.0, align.0).unwrap()) }
    }

    unsafe fn protect(&self, _p: *mut u8, _l: USize, _r: Bool, _w: Bool) {}
}
