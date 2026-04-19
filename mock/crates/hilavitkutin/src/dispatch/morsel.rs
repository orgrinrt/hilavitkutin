//! Morsel record range (domain 17).
//!
//! One morsel's slice of the global record index space.

/// Half-open `[start, start + len)` record range for one morsel.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct MorselRange {
    /// First record index.
    pub start: u64,
    /// Count of records in this morsel.
    pub len: u32,
}

impl MorselRange {
    /// Construct a morsel range with `start` and `len`.
    pub const fn new(start: u64, len: u32) -> Self {
        Self { start, len }
    }

    /// One past the last record index (`start + len`).
    pub const fn end(&self) -> u64 {
        self.start + self.len as u64
    }

    /// True iff `len == 0`.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}
