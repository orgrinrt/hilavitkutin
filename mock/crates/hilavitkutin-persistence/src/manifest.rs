//! Manifest, TableMeta, ColumnMeta: const-sized directory of
//! table metadata for the cold store.
//!
//! Const arrays are the skeleton's chosen representation: they
//! compile under `#![no_std]` with no alloc, and shape the API that
//! consumers observe today. Real widening (or a dynamic fallback)
//! lands with a later round if consumer pressure surfaces it.

use arvo::USize;
use arvo::strategy::Identity;
use arvo_hash::ContentHash;

use crate::primitives::{BitWidth, Cardinality, ColumnCount, RowCount, SchemaVersion};

/// Maximum number of tables a single Manifest can hold.
///
/// Bare `usize` required by Rust array-size const-eval.
pub const MAX_TABLES: usize = 256; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: array-size const must be usize (rust grammar); tracked: #121

/// Maximum number of columns a single TableMeta can hold.
///
/// Bare `usize` required by Rust array-size const-eval.
pub const MAX_COLUMNS_PER_TABLE: usize = 64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: array-size const must be usize (rust grammar); tracked: #121

/// Per-column metadata.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ColumnMeta {
    /// Content hash of the column name (28-bit FNV).
    pub name_hash: ContentHash,
    /// Bit width reported by the underlying ColumnValue impl.
    pub bit_width: BitWidth,
    /// Observed cardinality (distinct-value count, rough or exact).
    pub cardinality: Cardinality,
}

impl ColumnMeta {
    /// Default empty-column metadata.
    pub const EMPTY: Self = Self {
        name_hash: ContentHash::from_raw(0),
        bit_width: BitWidth::new(0),
        cardinality: Cardinality(USize::ZERO),
    };
}

impl Default for ColumnMeta {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// Per-table metadata. `columns` is a fixed-size array; only the
/// first `column_count` entries are valid.
#[derive(Debug, Clone, Copy)]
pub struct TableMeta {
    /// Content hash of the table name (28-bit FNV).
    pub name_hash: ContentHash,
    /// Schema version; consumer-defined meaning.
    pub version: SchemaVersion,
    /// Total row count at last flush.
    pub row_count: RowCount,
    /// Column metadata slots; `columns[..column_count]` is live.
    pub columns: [ColumnMeta; MAX_COLUMNS_PER_TABLE],
    /// Number of populated column slots.
    pub column_count: ColumnCount,
}

impl TableMeta {
    /// Default empty-table metadata.
    pub const EMPTY: Self = Self {
        name_hash: ContentHash::from_raw(0),
        version: SchemaVersion::new(0),
        row_count: RowCount(USize::ZERO),
        columns: [ColumnMeta::EMPTY; MAX_COLUMNS_PER_TABLE],
        column_count: ColumnCount(USize::ZERO),
    };
}

impl Default for TableMeta {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// Table directory. `tables` is a fixed-size array; only the first
/// `count` entries are valid.
#[derive(Debug, Clone, Copy)]
pub struct Manifest {
    /// Table metadata slots; `tables[..count]` is live.
    pub tables: [TableMeta; MAX_TABLES],
    /// Number of populated table slots.
    pub count: ColumnCount,
}

impl Manifest {
    /// Default empty manifest.
    pub const EMPTY: Self = Self {
        tables: [TableMeta::EMPTY; MAX_TABLES],
        count: ColumnCount(USize::ZERO),
    };

    /// Construct an empty manifest.
    pub const fn new() -> Self {
        Self::EMPTY
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Self::EMPTY
    }
}
