//! Access mask: which stores a WU touches (domain 11).
//!
//! Skeleton uses a `USize` backing; swap for arvo-bitmask once const-
//! generic bitmask support lands (BACKLOG). Target variant depends
//! on `MAX_STORES`:
//!   - `MAX_STORES <= 64`  → `arvo_bitmask::Mask64`
//!   - `MAX_STORES <= 256` → `arvo_bitmask::Mask256`
//!   - `MAX_STORES > 256`  → needs a const-generic `Mask<N>` in
//!     arvo-bitmask (arvo BACKLOG).
//! Callers see a stable surface (`empty` / `set` / `contains` /
//! `overlaps` / `union_with`) across that swap.

use arvo::{Bool, USize};

/// Bit pattern identifying which stores (indexed 0..MAX_STORES) a
/// WU reads or writes. Skeleton supports MAX_STORES ≤ 64.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AccessMask<const MAX_STORES: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    bits: USize,
}

impl<const MAX_STORES: usize> AccessMask<MAX_STORES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// Empty mask — touches no stores.
    pub const fn empty() -> Self {
        Self { bits: USize(0) }
    }

    /// True iff no store is touched.
    pub const fn is_empty(&self) -> Bool {
        Bool(self.bits.0 == 0)
    }

    /// Return a new mask with `idx` added. No-op if `idx` ≥ 64
    /// (skeleton limitation, documented above).
    pub const fn set(self, idx: USize) -> Self {
        if idx.0 >= 64 {
            return self;
        }
        Self {
            bits: USize(self.bits.0 | (1usize << idx.0)), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bit-literal shift operand; tracked: #72
        }
    }

    /// True iff `idx` is set. False if `idx` ≥ 64.
    pub const fn contains(&self, idx: USize) -> Bool {
        if idx.0 >= 64 {
            return Bool::FALSE;
        }
        Bool((self.bits.0 & (1usize << idx.0)) != 0) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bit-literal shift operand; tracked: #72
    }

    /// True iff this mask and `other` share any set bit.
    pub const fn overlaps(&self, other: &Self) -> Bool {
        Bool((self.bits.0 & other.bits.0) != 0)
    }

    /// In-place union with `other`.
    pub fn union_with(&mut self, other: &Self) {
        self.bits = USize(self.bits.0 | other.bits.0);
    }

    /// Raw bit pattern accessor for downstream rounds that need it.
    pub const fn raw(&self) -> USize {
        self.bits
    }
}

impl<const MAX_STORES: usize> Default for AccessMask<MAX_STORES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    fn default() -> Self {
        Self::empty()
    }
}
