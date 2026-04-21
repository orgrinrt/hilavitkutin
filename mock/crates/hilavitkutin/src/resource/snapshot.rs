//! Stack-local resource snapshot.
//!
//! Snapshot accessed resources to stack before a morsel loop,
//! redirect res_ptr. LLVM proves stack doesn't alias heap column
//! data → promotes to registers. Zero resource memory ops in
//! hot loop.

use arvo::newtype::{FBits, IBits};
use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo::USize;

/// Placeholder slot payload. Real storage lives in per-width
/// variants landing with 5a2 / 5a3 once the WU data layout needs
/// surface.
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
pub struct Slot(pub UFixed<{ IBits(64) }, { FBits::ZERO }, Hot>);

/// Const-sized stack-local snapshot of up to `N` resource slots.
#[derive(Copy, Clone)]
pub struct ResourceSnapshot<const N: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    slots: [Slot; N],
}

impl<const N: usize> Default for ResourceSnapshot<N> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    #[inline(always)]
    fn default() -> Self {
        Self {
            slots: [Slot(UFixed::from_raw(0)); N],
        }
    }
}

impl<const N: usize> ResourceSnapshot<N> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            slots: [Slot(UFixed::from_raw(0)); N],
        }
    }

    #[inline(always)]
    pub fn get(&self, i: USize) -> Slot {
        self.slots[*i]
    }

    #[inline(always)]
    pub fn set(&mut self, i: USize, v: Slot) {
        self.slots[*i] = v;
    }
}
