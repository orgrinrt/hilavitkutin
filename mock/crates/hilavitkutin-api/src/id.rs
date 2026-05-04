//! Store identity.
//!
//! `StoreId` is a dense plan-time index into the store table. Bitmask
//! shapes used by the engine live at `hilavitkutin::plan::access::AccessMask`.

use arvo::USize;
use arvo::strategy::Identity;

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
        StoreId(USize::ZERO)
    }
}

impl core::hash::Hash for StoreId {
    #[inline(always)]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.0.hash(state);
    }
}
