//! Store identity and access mask.
//!
//! `StoreId` is a dense plan-time index into the store table. The
//! scheduler operates on `AccessMask` bitwords; one bit per store.

use arvo::USize;
use arvo_bitmask::Mask64;

/// Dense store index assigned at plan time.
///
/// Wraps `arvo::USize` so boundary types stay inside the arvo
/// newtype family. Transparent repr means no wrapping cost at
/// runtime.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct StoreId(pub USize);

impl Default for StoreId {
    #[inline(always)]
    fn default() -> Self {
        StoreId(USize(0))
    }
}

impl core::hash::Hash for StoreId {
    #[inline(always)]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.0.hash(state);
    }
}

/// Access mask: bit per store, up to 64 stores per mask word.
///
/// Read/write set bitmasks at plan time use this shape. Single-
/// instruction bitwise set ops.
pub type AccessMask = Mask64;
