//! ColdStore skeleton method surface.

use arvo::newtype::{Bool, USize};
use hilavitkutin_api::MemoryProviderApi;
use hilavitkutin_persistence::{ColdStore, PersistenceContext, PersistenceError};
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

fn make_store<'a>(
    memory: &'a StubMemory,
    interner: &'a StringInterner<StubArena>,
) -> ColdStore<'a, StubMemory, StubArena> {
    let ctx = PersistenceContext::new(memory, interner);
    ColdStore::open(ctx).expect("open ok")
}

#[test]
fn open_returns_store_with_default_manifest() {
    let memory = StubMemory;
    let interner = StringInterner::new(StubArena);
    let store = make_store(&memory, &interner);
    assert_eq!(store.manifest().count, 0);
}

#[test]
fn open_string_table_is_empty() {
    let memory = StubMemory;
    let interner = StringInterner::new(StubArena);
    let store = make_store(&memory, &interner);
    assert_eq!(store.string_table().entries.len(), 0);
    assert_eq!(store.string_table().buffer.len(), 0);
}

#[test]
fn flush_is_noop_ok() {
    let memory = StubMemory;
    let interner = StringInterner::new(StubArena);
    let mut store = make_store(&memory, &interner);
    assert_eq!(store.flush(), Ok(()));
}

#[test]
fn snapshot_is_noop_ok() {
    let memory = StubMemory;
    let interner = StringInterner::new(StubArena);
    let store = make_store(&memory, &interner);
    assert_eq!(store.snapshot(), Ok(()));
}

#[test]
fn load_returns_missing() {
    let memory = StubMemory;
    let interner = StringInterner::new(StubArena);
    let mut store = make_store(&memory, &interner);
    assert_eq!(store.load(), Err(PersistenceError::Missing));
}
