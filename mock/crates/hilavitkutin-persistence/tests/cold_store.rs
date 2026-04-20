//! ColdStore skeleton method surface.

use arvo::newtype::{Bool, USize};
use hilavitkutin_api::MemoryProviderApi;
use hilavitkutin_persistence::{ColdStore, ColumnCount, PersistenceContext, PersistenceError};
use hilavitkutin_str::{ArenaInterner, StringInterner};
use notko::Outcome;

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
    match ColdStore::open(ctx) {
        Outcome::Ok(s) => s,
        Outcome::Err(_) => panic!("open ok"),
    }
}

#[test]
fn open_returns_store_with_default_manifest() {
    let memory = StubMemory;
    let interner = StringInterner::new(StubArena);
    let store = make_store(&memory, &interner);
    assert_eq!(store.manifest().count, ColumnCount(USize(0)));
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
    match store.flush() {
        Outcome::Ok(()) => {}
        Outcome::Err(_) => panic!("flush ok"),
    }
}

#[test]
fn snapshot_is_noop_ok() {
    let memory = StubMemory;
    let interner = StringInterner::new(StubArena);
    let store = make_store(&memory, &interner);
    match store.snapshot() {
        Outcome::Ok(()) => {}
        Outcome::Err(_) => panic!("snapshot ok"),
    }
}

#[test]
fn load_returns_missing() {
    let memory = StubMemory;
    let interner = StringInterner::new(StubArena);
    let mut store = make_store(&memory, &interner);
    match store.load() {
        Outcome::Err(e) => assert_eq!(e, PersistenceError::Missing),
        Outcome::Ok(()) => panic!("expected missing"),
    }
}
