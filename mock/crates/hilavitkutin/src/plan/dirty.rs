//! Dirty mask: incremental-skip propagation (domain 16).
//!
//! Tracks which stores changed since last frame so downstream
//! fibers can skip when their inputs are clean. Same bit layout
//! as `AccessMask`; kept as a distinct type to avoid accidental
//! cross-wiring.
//!
//! Skeleton uses a `USize` backing; swap for arvo-bitmask once const-
//! generic bitmask support lands (BACKLOG). Target variant depends
//! on `MAX_STORES`: Mask64 for ≤ 64, Mask256 for ≤ 256, const-
//! generic `Mask<N>` for larger (tracked as arvo BACKLOG).

use arvo::{Bool, USize};
use arvo::strategy::Identity;

/// Per-store dirty bit. Same shape as `AccessMask` — kept distinct
/// so `overlaps`-vs-access checks and `union_with`-vs-dirty checks
/// don't silently interchange.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DirtyMask<const MAX_STORES: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    bits: USize,
}

impl<const MAX_STORES: usize> DirtyMask<MAX_STORES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// Empty mask — nothing dirty.
    pub const fn empty() -> Self {
        Self { bits: USize::ZERO }
    }

    /// True iff nothing is dirty.
    pub const fn is_empty(&self) -> Bool {
        Bool(self.bits.0 == 0)
    }

    /// Return a new mask with `idx` marked dirty. No-op if
    /// `idx` ≥ 64 (skeleton limitation).
    pub const fn set(self, idx: USize) -> Self {
        if idx.0 >= 64 {
            return self;
        }
        Self {
            bits: USize(self.bits.0 | (1usize << idx.0)), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bit-literal shift operand; tracked: #72
        }
    }

    /// True iff `idx` is dirty. False if `idx` ≥ 64.
    pub const fn contains(&self, idx: USize) -> Bool {
        if idx.0 >= 64 {
            return Bool::FALSE;
        }
        Bool((self.bits.0 & (1usize << idx.0)) != 0) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bit-literal shift operand; tracked: #72
    }

    /// In-place union with `other`.
    pub fn union_with(&mut self, other: &Self) {
        self.bits = USize(self.bits.0 | other.bits.0);
    }

    /// Raw bit pattern accessor for downstream rounds.
    pub const fn raw(&self) -> USize {
        self.bits
    }
}

impl<const MAX_STORES: usize> Default for DirtyMask<MAX_STORES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    fn default() -> Self {
        Self::empty()
    }
}
