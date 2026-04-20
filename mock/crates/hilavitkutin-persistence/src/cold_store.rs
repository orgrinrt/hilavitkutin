//! ColdStore — file-backed cold store handle.
//!
//! Skeleton round: `open` returns a default Manifest + empty
//! StringTable. `flush` / `snapshot` are no-op `Ok(())`. `load`
//! returns `PersistenceError::Missing`. Real mmap / rkyv impls land
//! in a follow-up round once rkyv's no_std + no_alloc story has been
//! vetted against `MemoryProviderApi`.

use hilavitkutin_api::MemoryProviderApi;
use hilavitkutin_str::ArenaInterner;
use notko::Outcome;

use crate::context::PersistenceContext;
use crate::error::PersistenceError;
use crate::manifest::Manifest;
use crate::string_table::StringTable;

/// File-backed cold store.
pub struct ColdStore<'a, M: MemoryProviderApi, A: ArenaInterner> {
    context: PersistenceContext<'a, M, A>,
    manifest: Manifest,
    string_table: StringTable,
}

impl<'a, M: MemoryProviderApi, A: ArenaInterner> ColdStore<'a, M, A> {
    /// Open a cold store from the given context.
    ///
    /// Skeleton: returns a default-Manifest instance with an empty
    /// StringTable. Follow-up round mmap-loads `manifest.rkyv` +
    /// `strings/runtime.rkyv` via `ctx.memory()`.
    pub fn open(ctx: PersistenceContext<'a, M, A>) -> Outcome<Self, PersistenceError> {
        Outcome::Ok(Self {
            context: ctx,
            manifest: Manifest::new(),
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
    /// Skeleton: no-op. Follow-up round serialises dirty tables via
    /// rkyv and writes through `MemoryProvider`.
    pub fn flush(&mut self) -> Outcome<(), PersistenceError> {
        Outcome::Ok(())
    }

    /// Load a table from disk into the hot store.
    ///
    /// Skeleton: returns `Missing`. Follow-up round mmaps the table
    /// file and hands the archived view off to the consumer.
    pub fn load(&mut self) -> Outcome<(), PersistenceError> {
        Outcome::Err(PersistenceError::Missing)
    }

    /// Snapshot the cold store for backup.
    ///
    /// Skeleton: no-op. Follow-up round enumerates files in the data
    /// directory and atomically copies them to the target path.
    pub fn snapshot(&self) -> Outcome<(), PersistenceError> {
        Outcome::Ok(())
    }
}
