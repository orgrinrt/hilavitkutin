//! ColdStore: file-backed cold store handle.
//!
//! A skeleton: `open` returns a default Manifest + empty
//! StringTable. `flush` / `snapshot` are no-op `Ok(())`. `load`
//! returns `PersistenceError::Missing`. There is no mmap or rkyv
//! implementation yet; one waits on rkyv's no_std + no_alloc story
//! being vetted against `MemoryProviderApi`.

use hilavitkutin_api::MemoryProviderApi;
use hilavitkutin_str::ArenaInterner;
use notko::Outcome;

use crate::context::PersistenceContext;
use crate::error::PersistenceError;
use crate::manifest::Manifest;
use crate::string_table::StringTable;

/// File-backed cold store.
pub struct ColdStore<'a, M: MemoryProviderApi, A: ArenaInterner> {
    context:      PersistenceContext<'a, M, A>,
    manifest:     Manifest,
    string_table: StringTable,
}

impl<'a, M: MemoryProviderApi, A: ArenaInterner> ColdStore<'a, M, A> {
    /// Open a cold store from the given context.
    ///
    /// Returns a default-Manifest instance with an empty StringTable.
    /// It does not yet mmap-load `manifest.rkyv` +
    /// `strings/runtime.rkyv` via `ctx.memory()`.
    pub fn open(ctx: PersistenceContext<'a, M, A>) -> Outcome<Self, PersistenceError> {
        // FIXME(hilavitkutin_persistence_5_unmarked_placeholders): reads no manifest; the file-backed open maps `manifest.rkyv` and `strings/runtime.rkyv` through `ctx.memory()` (BACKLOG, mmap-backed file I/O).
        Outcome::Ok(Self {
            context:      ctx,
            manifest:     Manifest::new(),
            string_table: StringTable::empty(),
        })
    }

    /// Borrow the context.
    pub fn context(&self) -> &PersistenceContext<'a, M, A> {
        &self.context
    }

    /// Borrow the manifest.
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// Borrow the string table.
    pub fn string_table(&self) -> &StringTable {
        &self.string_table
    }

    /// Flush dirty data to disk.
    ///
    /// A no-op today. It does not yet serialise dirty tables via rkyv
    /// and write through `MemoryProvider`.
    pub fn flush(&mut self) -> Outcome<(), PersistenceError> {
        // FIXME(hilavitkutin_persistence_5_unmarked_placeholders): writes nothing and reports success; a flushed store's data is not yet what a later `load` finds (BACKLOG, mmap-backed file I/O).
        Outcome::Ok(())
    }

    /// Load a table from disk into the hot store.
    ///
    /// Returns `Missing` today. It does not yet mmap the table file
    /// and hand the archived view off to the consumer.
    pub fn load(&mut self) -> Outcome<(), PersistenceError> {
        // FIXME(hilavitkutin_persistence_5_unmarked_placeholders): reports `Missing` whatever was flushed; the real load maps the table file (BACKLOG, mmap-backed file I/O).
        Outcome::Err(PersistenceError::Missing)
    }

    /// Snapshot the cold store for backup.
    ///
    /// A no-op today. It does not yet enumerate files in the data
    /// directory and atomically copy them to the target path.
    pub fn snapshot(&self) -> Outcome<(), PersistenceError> {
        // FIXME(hilavitkutin_persistence_5_unmarked_placeholders): copies nothing and reports success (BACKLOG, backup / snapshot mechanism).
        Outcome::Ok(())
    }
}
