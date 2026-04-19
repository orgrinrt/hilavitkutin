//! Access mask: which stores a WU touches (domain 11).
//!
//! Skeleton uses `u128` backing; swap for arvo-bitmask once const-
//! generic bitmask support lands (BACKLOG). Callers see a stable
//! surface (`empty` / `set` / `contains` / `overlaps` / `union_with`)
//! across that swap.

/// Bit pattern identifying which stores (indexed 0..MAX_STORES) a
/// WU reads or writes. Skeleton supports MAX_STORES ≤ 128.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct AccessMask<const MAX_STORES: usize> {
    bits: u128,
}

impl<const MAX_STORES: usize> AccessMask<MAX_STORES> {
    /// Empty mask — touches no stores.
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// True iff no store is touched.
    pub const fn is_empty(&self) -> bool {
        self.bits == 0
    }

    /// Return a new mask with `idx` added. No-op if `idx` ≥ 128
    /// (skeleton limitation, documented above).
    pub const fn set(self, idx: u32) -> Self {
        if idx >= 128 {
            return self;
        }
        Self {
            bits: self.bits | (1u128 << idx),
        }
    }

    /// True iff `idx` is set. False if `idx` ≥ 128.
    pub const fn contains(&self, idx: u32) -> bool {
        if idx >= 128 {
            return false;
        }
        (self.bits & (1u128 << idx)) != 0
    }

    /// True iff this mask and `other` share any set bit.
    pub const fn overlaps(&self, other: &Self) -> bool {
        (self.bits & other.bits) != 0
    }

    /// In-place union with `other`.
    pub fn union_with(&mut self, other: &Self) {
        self.bits |= other.bits;
    }

    /// Raw bit pattern accessor for downstream rounds that need it.
    pub const fn raw(&self) -> u128 {
        self.bits
    }
}
