//! Persistence-local domain newtypes.
//!
//! Wrap arvo primitives (USize for count/offset semantics; raw u32
//! or u64 where the bit width is load-bearing, as with
//! SchemaVersion's ordinal) under semantic names used in the
//! persistence pub API. ContentHash is imported from arvo_hash;
//! this module ships the persistence-specific family only.
//!
//! Traits derivable on arvo's USize are Debug, Clone, Copy,
//! PartialEq, Eq. USize does not yet implement PartialOrd / Ord /
//! Hash / Default. USize-wrapping aliases inherit that constraint;
//! bare-integer-wrapping aliases get the fuller set.

use arvo::USize;

/// Byte offset into a persistence buffer (string table, column
/// data). Non-arithmetic across API boundaries; arithmetic within
/// a buffer is on the inner USize.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferOffset(pub USize);

/// Byte length of a persistence buffer span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferLen(pub USize);

/// Row count of a persisted table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowCount(pub USize);

/// Column count of a persisted table; also usable as a
/// table-level entry count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnCount(pub USize);

/// Distinct-value count for a column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cardinality(pub USize);

/// Ordinal table-schema version. `next()` increments; no
/// arithmetic beyond that.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SchemaVersion(pub u32);

impl SchemaVersion {
    pub const fn new(v: u32) -> Self {
        Self(v)
    }
    pub const fn bits(self) -> u32 {
        self.0
    }
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

/// SieveCache eviction-priority weight. Comparable, opaque.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EvictionWeight(pub u64);

impl EvictionWeight {
    pub const fn new(w: u64) -> Self {
        Self(w)
    }
    pub const fn bits(self) -> u64 {
        self.0
    }
}

/// Column bit-width declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BitWidth(pub u32);

impl BitWidth {
    pub const fn new(b: u32) -> Self {
        Self(b)
    }
    pub const fn bits(self) -> u32 {
        self.0
    }
}
