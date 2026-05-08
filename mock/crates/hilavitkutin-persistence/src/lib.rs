//! `hilavitkutin-persistence`: `#![no_std]`, no alloc cold-store
//! bridge for the hilavitkutin ecosystem.
//!
//! Skeleton round: wires the type surface described in DESIGN.md
//! (Manifest, TableMeta, ColumnMeta, PersistenceContext,
//! StringTable, SieveCache, ColdStore). `evict_str` / `inject_str`
//! ship as real bit-layout logic on `Str`. Everything touching files
//! (`ColdStore::flush` / `load` / `snapshot`) is a stub this round;
//! real rkyv / mmap impls land in a follow-up round once rkyv's
//! no_std + no_alloc story has been vetted against the
//! `MemoryProviderApi` surface.

#![no_std]

pub mod archive_str;
pub mod cold_store;
pub mod context;
pub mod error;
pub mod manifest;
pub mod primitives;
pub mod sieve;
pub mod string_table;

pub use archive_str::{evict_str, inject_str};
pub use cold_store::ColdStore;
pub use context::PersistenceContext;
pub use error::PersistenceError;
pub use manifest::{ColumnMeta, Manifest, TableMeta, MAX_COLUMNS_PER_TABLE, MAX_TABLES};
pub use primitives::{
    BitWidth, BufferLen, BufferOffset, Cardinality, ColumnCount, EvictionWeight, RowCount,
    SchemaVersion,
};
pub use sieve::SieveCache;
pub use string_table::{StringTable, StringTableEntry}; // lint:allow(no-alloc) reason: `StringTable` / `StringTableEntry` are persistence string header types, not std `String`; tracked: #72
