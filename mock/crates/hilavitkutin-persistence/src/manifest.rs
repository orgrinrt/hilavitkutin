//! Manifest, TableMeta, ColumnMeta — const-sized directory of
//! table metadata for the cold store.
//!
//! Const arrays are the skeleton's chosen representation: they
//! compile under `#![no_std]` with no alloc, and shape the API that
//! consumers observe today. Real widening (or a dynamic fallback)
//! lands with a later round if consumer pressure surfaces it.

/// Maximum number of tables a single Manifest can hold.
pub const MAX_TABLES: usize = 256;

/// Maximum number of columns a single TableMeta can hold.
pub const MAX_COLUMNS_PER_TABLE: usize = 64;

/// Per-column metadata.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ColumnMeta {
    /// Content hash of the column name (28-bit FNV, stored in a u32).
    pub name_hash: u32,
    /// Bit width reported by the underlying ColumnValue impl.
    pub bit_width: u32,
    /// Observed cardinality (distinct-value count, rough or exact).
    pub cardinality: u64,
}

impl ColumnMeta {
    /// Default empty-column metadata.
    pub const EMPTY: Self = Self {
        name_hash: 0,
        bit_width: 0,
        cardinality: 0,
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
    /// Content hash of the table name (28-bit FNV, stored in a u32).
    pub name_hash: u32,
    /// Schema version; consumer-defined meaning.
    pub version: u32,
    /// Total row count at last flush.
    pub row_count: u64,
    /// Column metadata slots; `columns[..column_count]` is live.
    pub columns: [ColumnMeta; MAX_COLUMNS_PER_TABLE],
    /// Number of populated column slots.
    pub column_count: usize,
}

impl TableMeta {
    /// Default empty-table metadata.
    pub const EMPTY: Self = Self {
        name_hash: 0,
        version: 0,
        row_count: 0,
        columns: [ColumnMeta::EMPTY; MAX_COLUMNS_PER_TABLE],
        column_count: 0,
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
    pub count: usize,
}

impl Manifest {
    /// Default empty manifest.
    pub const EMPTY: Self = Self {
        tables: [TableMeta::EMPTY; MAX_TABLES],
        count: 0,
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
