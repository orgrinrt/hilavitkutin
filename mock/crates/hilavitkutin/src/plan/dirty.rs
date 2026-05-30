//! Dirty mask: incremental-skip propagation (domain 16).
//!
//! Tracks which stores changed since last frame so downstream
//! fibers can skip when their inputs are clean. Same bit layout
//! as `AccessMask`; kept as a distinct type to avoid accidental
//! cross-wiring.
//!
//! Skeleton uses a `USize` backing; swap for arvo-bitmask once const-
//! generic bitmask support lands (BACKLOG). Target variant depends
//! on the store capacity: Mask64 for ≤ 64, Mask256 for ≤ 256, const-
//! generic `Mask<N>` for larger (tracked as arvo BACKLOG).

use core::marker::PhantomData;

use arvo::{Bool, USize};
use arvo::strategy::Identity;
use arvo_tensor::{cap_size, Capacity};

/// Per-store dirty bit. Same shape as `AccessMask`: kept distinct
/// so `overlaps`-vs-access checks and `union_with`-vs-dirty checks
/// don't silently interchange. `C` is a phantom store-capacity marker.
pub struct DirtyMask<C: Capacity> {
    bits: USize,
    _cap: PhantomData<C>,
}

impl<C: Capacity> core::fmt::Debug for DirtyMask<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DirtyMask").field("bits", &self.bits.0).finish()
    }
}

impl<C: Capacity> DirtyMask<C> {
    // Skeleton ceiling: the `USize` backing is one 64-bit word, so
    // any store capacity > 64 would silently drop dirty bits past
    // index 63. The arvo-bitmask multi-container swap (BACKLOG)
    // lifts this; until then, fail at compile time rather than
    // running with partial coverage. Associated consts only evaluate
    // on monomorphisation when referenced, so every constructor
    // discharges the assert with `let _ = Self::_ASSERT_FITS_IN_USIZE`.
    const _ASSERT_FITS_IN_USIZE: () = assert!( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-context size assertion; tracked: #429
        cap_size(C::CAP) <= 64,
        "DirtyMask: store capacity > 64 is not supported by the skeleton USize backing. Once arvo-bitmask ships multi-container Mask<W>, this assert lifts and DirtyMask widens.",
    );

    /// Empty mask: nothing dirty.
    pub const fn empty() -> Self {
        let _ = Self::_ASSERT_FITS_IN_USIZE;
        Self { bits: USize::ZERO, _cap: PhantomData }
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
            _cap: PhantomData,
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

// Manual `Copy` / `Clone` / `PartialEq` / `Eq`: deriving would demand
// `C: Copy`, but `C` is a phantom store-capacity marker. The real state
// is the `USize` bits, `Copy` for any `C`.
impl<C: Capacity> Copy for DirtyMask<C> {}

impl<C: Capacity> Clone for DirtyMask<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Capacity> PartialEq for DirtyMask<C> {
    fn eq(&self, other: &Self) -> bool { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: std-trait method signature; core::cmp::PartialEq::eq is fixed to return bool by the trait (no-bare-primitives.md exception 5); tracked: #207
        self.bits == other.bits
    }
}

impl<C: Capacity> Eq for DirtyMask<C> {}

impl<C: Capacity> Default for DirtyMask<C> {
    fn default() -> Self {
        Self::empty()
    }
}

/// Per-fiber dirty masks: which stores changed since last frame, per
/// fiber. Drives incremental-skip propagation: a fiber whose inputs
/// are entirely clean (its access set disjoint from the running
/// dirty mask) can skip dispatch this frame.
///
/// Plan-stage output of the fused upward-rank + dirty step (step 8).
/// `CF` is the fiber capacity (array length); `CS` is the store
/// capacity (the inner `DirtyMask` phantom).
pub struct DirtyMasks<CF: Capacity, CS: Capacity> {
    pub per_fiber: <CF as Capacity>::Array<DirtyMask<CS>>,
}

impl<CF: Capacity, CS: Capacity> DirtyMasks<CF, CS> {
    pub fn new() -> Self {
        Self { per_fiber: <CF as Capacity>::filled(DirtyMask::empty()) }
    }
}

// Manual `Copy` / `Clone`: the GAT array is `Copy` when its element is,
// and `DirtyMask<CS>` is `Copy`; deriving would over-constrain on
// `CF: Copy`.
impl<CF: Capacity, CS: Capacity> Copy for DirtyMasks<CF, CS> where
    <CF as Capacity>::Array<DirtyMask<CS>>: Copy
{
}

impl<CF: Capacity, CS: Capacity> Clone for DirtyMasks<CF, CS>
where
    <CF as Capacity>::Array<DirtyMask<CS>>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<CF: Capacity, CS: Capacity> Default for DirtyMasks<CF, CS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<CF: Capacity, CS: Capacity> core::fmt::Debug for DirtyMasks<CF, CS> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DirtyMasks").finish_non_exhaustive()
    }
}
