//! Access mask: which stores a WU touches (domain 11).
//!
//! Skeleton uses a `USize` backing; swap for arvo-bitmask once const-
//! generic bitmask support lands (BACKLOG). Target variant depends
//! on the store capacity:
//!   - stores <= 64  → `arvo_bitmask::Mask64`
//!   - stores <= 256 → `arvo_bitmask::Mask256`
//!   - stores > 256  → needs a const-generic `Mask<N>` in
//!     arvo-bitmask (arvo BACKLOG).
//! Callers see a stable surface (`empty` / `set` / `contains` /
//! `overlaps` / `union_with`) across that swap.

use core::marker::PhantomData;

use arvo::{Bool, USize};
use arvo::strategy::Identity;
use arvo_tensor::Capacity;

/// Bit pattern identifying which stores (indexed within the store
/// capacity `C`) a WU reads or writes. Skeleton supports up to 64
/// stores. `C` is a phantom capacity marker: the store width is a
/// type, while the bits live in a single `USize` word.
pub struct AccessMask<C: Capacity> {
    bits: USize,
    _cap: PhantomData<C>,
}

impl<C: Capacity> core::fmt::Debug for AccessMask<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AccessMask").field("bits", &self.bits.0).finish()
    }
}

impl<C: Capacity> AccessMask<C> {
    /// Empty mask: touches no stores.
    pub const fn empty() -> Self {
        Self { bits: USize::ZERO, _cap: PhantomData }
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
            _cap: PhantomData,
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

// Manual `Copy` / `Clone`: deriving would demand `C: Copy`, but `C` is
// a phantom capacity marker (`Dim<N>` / `DefaultPlanDims` are not
// `Copy`). The real state is the `USize` bits plus a zero-size
// `PhantomData`, both `Copy` for any `C`.
impl<C: Capacity> Copy for AccessMask<C> {}

impl<C: Capacity> Clone for AccessMask<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Capacity> PartialEq for AccessMask<C> {
    fn eq(&self, other: &Self) -> bool { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: std-trait method signature; core::cmp::PartialEq::eq is fixed to return bool by the trait (no-bare-primitives.md exception 5); tracked: #207
        self.bits == other.bits
    }
}

impl<C: Capacity> Eq for AccessMask<C> {}

impl<C: Capacity> Default for AccessMask<C> {
    fn default() -> Self {
        Self::empty()
    }
}
