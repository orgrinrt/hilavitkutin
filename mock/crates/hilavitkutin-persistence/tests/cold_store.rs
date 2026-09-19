//! `ColdStore`: what a freshly opened store is, and the contracts its
//! file-backed operations owe.
//!
//! The first group holds for the skeleton and for the file-backed store
//! alike: a store opened over a context that has flushed nothing has an empty
//! table directory and string table, has nothing to load, and hands back the
//! context it was opened with.
//!
//! The second group is catalogued red, in the shape `adapt_perf_contracts.rs`
//! set in the engine: each states what the design's Flush coordination,
//! Manifest and Backup sections promise, is `#[ignore]`d so the gate stays green, and
//! fails when run until the file-backed round lands. The `FIXME:` on each stub
//! in `src/cold_store.rs` names the same gap.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin_api::MemoryProviderApi;
use hilavitkutin_persistence::{ColdStore, PersistenceContext, PersistenceError};
use hilavitkutin_str::{ArenaInterner, StringInterner};
use notko::{Maybe, Outcome};

/// A memory provider handing out slices of one owned buffer, so a
/// file-backed store has real memory to map into.
struct BumpMemory<const N: usize> {
    buf:  UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}

impl<const N: usize> BumpMemory<N> {
    fn new() -> Self {
        Self {
            buf:  UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
            used: Cell::new(0),
        }
    }
}

unsafe impl<const N: usize> Send for BumpMemory<N> {}
unsafe impl<const N: usize> Sync for BumpMemory<N> {}

impl<const N: usize> MemoryProviderApi for BumpMemory<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = self.buf.get() as *mut u8;
        let align = align.0.max(1);
        let start = self.used.get().div_ceil(align) * align;
        if start + len.0 > N {
            return core::ptr::null_mut();
        }
        self.used.set(start + len.0);
        // SAFETY: `start + len <= N`, inside the owned buffer.
        unsafe { base.add(start) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

struct NoArena;

impl ArenaInterner for NoArena {
    fn arena_intern(&self, _s: &str) -> u32 {
        0
    }

    fn arena_resolve(&self, _id: u32) -> &str {
        ""
    }
}

type Memory = BumpMemory<65536>;

fn open<'a>(
    memory: &'a Memory,
    interner: &'a StringInterner<NoArena>,
) -> ColdStore<'a, Memory, NoArena> {
    match ColdStore::open(PersistenceContext::new(memory, interner)) {
        Outcome::Ok(store) => store,
        Outcome::Err(e) => panic!("open over a fresh context failed: {e:?}"),
    }
}

// ---- what holds now and after the file-backed round -----------------------

#[test]
fn a_fresh_store_has_no_tables() {
    let memory = Memory::new();
    let interner = StringInterner::new(NoArena);
    let store = open(&memory, &interner);
    assert_eq!(store.manifest().count.0, USize(0));
}

#[test]
fn a_fresh_store_has_an_empty_string_table() {
    let memory = Memory::new();
    let interner = StringInterner::new(NoArena);
    let store = open(&memory, &interner);
    let table = store.string_table();
    assert!(table.entries.is_empty());
    assert!(table.buffer.is_empty());
}

#[test]
fn a_fresh_store_resolves_no_content_hash() {
    let memory = Memory::new();
    let interner = StringInterner::new(NoArena);
    let store = open(&memory, &interner);
    for raw in [0u64, 1, 0xDEAD_BEEF, u64::MAX] {
        let hash = arvo_hash::ContentHash::from_raw(raw);
        assert!(
            matches!(store.string_table().lookup(hash), Maybe::Isnt),
            "hash {raw:#x} resolved in an empty string table"
        );
    }
}

#[test]
fn a_fresh_store_has_nothing_to_load() {
    let memory = Memory::new();
    let interner = StringInterner::new(NoArena);
    let mut store = open(&memory, &interner);
    assert!(matches!(
        store.load(),
        Outcome::Err(PersistenceError::Missing)
    ));
}

#[test]
fn a_store_hands_back_the_context_it_was_opened_with() {
    let memory = Memory::new();
    let interner = StringInterner::new(NoArena);
    let store = open(&memory, &interner);
    assert!(core::ptr::eq(store.context().memory(), &memory));
    assert!(core::ptr::eq(store.context().interner(), &interner));
}

// ---- catalogued contracts of the file-backed round -------------------------

/// CONTRACT: what a store flushed is what a store reopened over the same
/// context loads. Design, Flush coordination: `flush` serialises dirty data to
/// disk and `load` maps it back.
#[test]
#[ignore = "catalogue: flush writes nothing, so a reopened store finds nothing to load; needs the mmap-backed file I/O; tracked: BACKLOG mmap-backed file I/O"]
fn a_flushed_store_is_loaded_by_the_next_open() {
    let memory = Memory::new();
    let interner = StringInterner::new(NoArena);
    {
        let mut first = open(&memory, &interner);
        assert!(matches!(first.flush(), Outcome::Ok(())));
    }
    let mut second = open(&memory, &interner);
    assert!(
        matches!(second.load(), Outcome::Ok(())),
        "a store reopened after a flush finds what was flushed"
    );
}

/// CONTRACT: `open` reads the manifest a previous flush wrote, so the table
/// directory survives a close and reopen. Design, File layout and Manifest:
/// the data directory holds `manifest.rkyv`, and the store opens from it.
#[test]
#[ignore = "catalogue: open reads no manifest; needs the mmap-backed file I/O and a way to register a table; tracked: BACKLOG mmap-backed file I/O"]
fn open_reads_the_manifest_a_flush_wrote() {
    unimplemented!(
        "contract: register a table, flush, drop the store, reopen over the same \
         context, and the reopened manifest's count and table metadata equal the \
         flushed ones. Fill when the file-backed round lands a table registration \
         and the manifest read. tracked: BACKLOG mmap-backed file I/O"
    );
}

/// CONTRACT: `snapshot` copies every file of the data directory, so a store
/// opened over the copy loads what the original held. Design, Backup.
#[test]
#[ignore = "catalogue: snapshot copies nothing; needs the backup / snapshot mechanism; tracked: BACKLOG Backup / snapshot mechanism"]
fn a_snapshot_opens_as_the_store_it_copied() {
    unimplemented!(
        "contract: flush a store, snapshot it to a target, open a store over the \
         target, and it loads what the original held. Fill when snapshot takes a \
         target and copies the data directory. tracked: BACKLOG Backup / snapshot mechanism"
    );
}
