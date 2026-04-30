//! Morsel record range (domain 17).
//!
//! One morsel's slice of the global record index space.

use arvo::{Bool, USize};

/// Half-open `[start, start + len)` record range for one morsel.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MorselRange {
    /// First record index.
    pub start: USize,
    /// Count of records in this morsel.
    pub len: USize,
}

impl MorselRange {
    /// Construct a morsel range with `start` and `len`.
    pub const fn new(start: USize, len: USize) -> Self {
        Self { start, len }
    }

    /// One past the last record index (`start + len`).
    pub const fn end(&self) -> USize {
        USize(self.start.0 + self.len.0)
    }

    /// True iff `len == 0`.
    pub const fn is_empty(&self) -> Bool {
        Bool(self.len.0 == 0)
    }
}

impl Default for MorselRange {
    fn default() -> Self {
        Self {
            start: USize(0),
            len: USize(0),
        }
    }
}
