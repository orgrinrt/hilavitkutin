// `MemoryProviderApi` is an unsafe trait: the engine reads and writes through
// the blocks a provider returns after the provider has moved, so the promises
// that make that sound are the implementer's. A plain `impl` makes none of
// them and is refused. Every provider in the tree is an `unsafe impl` with the
// promises argued beside it; `StaticBumpMemory` in `hilavitkutin-providers` is
// the one the tests build on.

use arvo::{Bool, USize};
use hilavitkutin_api::platform::MemoryProviderApi;

struct Inline {
    _bytes: [u8; 64],
}

impl MemoryProviderApi for Inline {
    unsafe fn allocate(&self, _len: USize, _align: USize) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

fn main() {}
