//! PersistenceContext accessor roundtrip via test-local stubs.

use arvo::newtype::{Bool, USize};
use hilavitkutin_api::MemoryProviderApi;
use hilavitkutin_persistence::PersistenceContext;
use hilavitkutin_str::{ArenaInterner, StringInterner};

struct StubMemory;

impl MemoryProviderApi for StubMemory {
    unsafe fn allocate(&self, _len: USize, _align: USize) -> *mut u8 {
        core::ptr::null_mut()
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

struct StubArena;

impl ArenaInterner for StubArena {
    fn arena_intern(&self, _s: &str) -> u32 {
        0
    }
    fn arena_resolve(&self, _id: u32) -> &str {
        ""
    }
}

#[test]
fn context_exposes_memory_and_interner() {
    let memory = StubMemory;
    let interner = StringInterner::new(StubArena);
    let ctx = PersistenceContext::new(&memory, &interner);

    // Exercise the accessors — they must return references to the
    // same objects we constructed with.
    let _m: &StubMemory = ctx.memory();
    let _i: &StringInterner<StubArena> = ctx.interner();
}

#[test]
fn context_memory_pointer_identity() {
    let memory = StubMemory;
    let interner = StringInterner::new(StubArena);
    let ctx = PersistenceContext::new(&memory, &interner);

    let a: *const StubMemory = ctx.memory();
    let b: *const StubMemory = &memory;
    assert_eq!(a, b);
}
