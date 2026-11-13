//! A test memory provider whose addresses survive moves of the scheduler.
//!
//! The scheduler records the base address of every column and accumulator it
//! reserves at build. A provider that keeps its buffer inline (an array in the
//! struct) moves with the scheduler, so those recorded bases dangle after the
//! move and a green test rests on a stack slot happening to survive. This
//! provider owns a leaked heap buffer instead, and aligns the address it hands
//! out (`base + used`), not the offset into a buffer whose own alignment is one.
//!
//! Included by name from each test that needs it (`mod stable_provider;`).
//! Cargo does not build a directory under `tests/` as a test target, so this is
//! not one. The trybuild fixtures under `tests/ui/` compile alone and cannot
//! include it, so they keep a copy each.

use core::cell::Cell;

use arvo::{Bool, USize};
use hilavitkutin_api::platform::MemoryProviderApi;

/// Heap-backed bump provider with a fixed capacity.
pub struct StableProvider {
    base: *mut u8,
    cap:  usize,
    used: Cell<usize>,
}

impl StableProvider {
    /// A provider over a fresh leaked buffer of `cap` bytes.
    pub fn new(cap: usize) -> Self {
        let leaked: &'static mut [u8] = std::vec![0u8; cap].leak();
        Self {
            base: leaked.as_mut_ptr(),
            cap,
            used: Cell::new(0),
        }
    }
}

// SAFETY: the buffer is leaked and never freed, and the bump cursor is only
// touched from the thread that builds the scheduler.
unsafe impl Send for StableProvider {}
unsafe impl Sync for StableProvider {}

impl MemoryProviderApi for StableProvider {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let align = align.0.max(1);
        let addr = self.base as usize + self.used.get();
        let aligned = (addr + align - 1) / align * align - self.base as usize;
        if aligned + len.0 > self.cap {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: `aligned + len <= cap`, inside the leaked buffer, and the
        // returned address is a multiple of `align`.
        unsafe { self.base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}
