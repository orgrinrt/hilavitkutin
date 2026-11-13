//! The walk at the boundary between two passes.
//!
//! It reads every column store's changed mark. A store the pass changed has
//! its live side brought in, which for a plain column is nothing at all, and
//! has its position set in the mask the next pass seeds its dirty units from.
//! A store nothing changed is left alone: no copy, no exchange, no bit.

use arvo::USize;
use arvo_tensor::Capacity;

use super::{AccumBinding, BindingNil, ColKind, ColumnBinding, ResourceBinding, VirtualBinding};
use crate::plan::AccessMask;

/// Bring in what the pass changed and report which stores those were.
///
/// A node's position in the bindings is its store's position in the access
/// masks, since both lists are built from the same registrations in the same
/// sequence. `at` is the position of the node the walk stands on.
pub trait BringIn {
    /// Clear every mark, bring in each marked store, and return `changed`
    /// with the marked stores' positions set.
    ///
    /// # Safety
    ///
    /// No context bound from these bindings, or from a per-core copy of
    /// them, is alive: the last unit of the pass has returned and the
    /// workers are parked.
    unsafe fn bring_in<C: Capacity>(&self, at: USize, changed: AccessMask<C>) -> AccessMask<C>;
}

/// The position after `at`.
#[inline(always)]
const fn next(at: USize) -> USize {
    USize(at.0 + 1) // lint:allow(no-bare-numeric) reason: store position step along the bindings; tracked: #121
}

impl BringIn for BindingNil {
    #[inline]
    unsafe fn bring_in<C: Capacity>(&self, _at: USize, changed: AccessMask<C>) -> AccessMask<C> {
        changed
    }
}

impl<T, Tail: BringIn> BringIn for ResourceBinding<T, Tail> {
    #[inline]
    unsafe fn bring_in<C: Capacity>(&self, at: USize, changed: AccessMask<C>) -> AccessMask<C> {
        // SAFETY: the caller's guarantee covers the whole list.
        unsafe { self.tail.bring_in(next(at), changed) }
    }
}

impl<T, Tail: BringIn> BringIn for AccumBinding<T, Tail> {
    #[inline]
    unsafe fn bring_in<C: Capacity>(&self, at: USize, changed: AccessMask<C>) -> AccessMask<C> {
        // SAFETY: the caller's guarantee covers the whole list.
        unsafe { self.tail.bring_in(next(at), changed) }
    }
}

impl<T, Tail: BringIn> BringIn for VirtualBinding<T, Tail> {
    #[inline]
    unsafe fn bring_in<C: Capacity>(&self, at: USize, changed: AccessMask<C>) -> AccessMask<C> {
        // SAFETY: the caller's guarantee covers the whole list.
        unsafe { self.tail.bring_in(next(at), changed) }
    }
}

impl<T, Tail: BringIn, K: ColKind> BringIn for ColumnBinding<T, Tail, K> {
    #[inline]
    unsafe fn bring_in<C: Capacity>(&self, at: USize, changed: AccessMask<C>) -> AccessMask<C> {
        let changed = if self.mark.take().0 {
            // SAFETY: the caller holds that nothing reads or writes the store
            // now, and both sides were reserved for `count` records.
            unsafe { K::bring_in(self.ptr, &self.side, self.count) };
            changed.set(at)
        } else {
            changed
        };
        // SAFETY: the caller's guarantee covers the whole list.
        unsafe { self.tail.bring_in(next(at), changed) }
    }
}
