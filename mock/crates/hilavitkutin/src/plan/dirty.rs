//! Dirty mask: incremental-skip propagation (domain 16).
//!
//! Tracks which stores changed since last frame so downstream
//! fibers can skip when their inputs are clean. Same bit layout
//! as `AccessMask`; kept as a distinct type to avoid accidental
//! cross-wiring.
//!
//! Skeleton uses `u128` backing; swap for arvo-bitmask once const-
//! generic bitmask support lands (BACKLOG). Target variant depends
//! on `MAX_STORES`: Mask64 for ≤ 64, Mask256 for ≤ 256, const-
//! generic `Mask<N>` for larger (tracked as arvo BACKLOG).

/// Per-store dirty bit. Same shape as `AccessMask` — kept distinct
/// so `overlaps`-vs-access checks and `union_with`-vs-dirty checks
/// don't silently interchange.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct DirtyMask<const MAX_STORES: usize> {
    bits: u128,
}

impl<const MAX_STORES: usize> DirtyMask<MAX_STORES> {
    /// Empty mask — nothing dirty.
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// True iff nothing is dirty.
    pub const fn is_empty(&self) -> bool {
        self.bits == 0
    }

    /// Return a new mask with `idx` marked dirty. No-op if
    /// `idx` ≥ 128 (skeleton limitation).
    pub const fn set(self, idx: u32) -> Self {
        if idx >= 128 {
            return self;
        }
        Self {
            bits: self.bits | (1u128 << idx),
        }
    }

    /// True iff `idx` is dirty. False if `idx` ≥ 128.
    pub const fn contains(&self, idx: u32) -> bool {
        if idx >= 128 {
            return false;
        }
        (self.bits & (1u128 << idx)) != 0
    }

    /// In-place union with `other`.
    pub fn union_with(&mut self, other: &Self) {
        self.bits |= other.bits;
    }

    /// Raw bit pattern accessor for downstream rounds.
    pub const fn raw(&self) -> u128 {
        self.bits
    }
}
