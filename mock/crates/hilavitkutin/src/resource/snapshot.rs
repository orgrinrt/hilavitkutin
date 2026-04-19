//! Stack-local resource snapshot.
//!
//! Snapshot accessed resources to stack before a morsel loop,
//! redirect res_ptr. LLVM proves stack doesn't alias heap column
//! data → promotes to registers. Zero resource memory ops in
//! hot loop.

/// Placeholder slot payload. Real storage lives in per-width
/// variants landing with 5a2 / 5a3 once the WU data layout needs
/// surface.
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
pub struct Slot(pub u64);

/// Const-sized stack-local snapshot of up to `N` resource slots.
#[derive(Copy, Clone)]
pub struct ResourceSnapshot<const N: usize> {
    slots: [Slot; N],
}

impl<const N: usize> Default for ResourceSnapshot<N> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            slots: [Slot(0); N],
        }
    }
}

impl<const N: usize> ResourceSnapshot<N> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            slots: [Slot(0); N],
        }
    }

    #[inline(always)]
    pub fn get(&self, i: usize) -> Slot {
        self.slots[i]
    }

    #[inline(always)]
    pub fn set(&mut self, i: usize, v: Slot) {
        self.slots[i] = v;
    }
}
