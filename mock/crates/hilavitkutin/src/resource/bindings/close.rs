//! The walk at the boundary between two passes, and the walk that tells each
//! column store where it sits in the store list.
//!
//! The boundary reads every column store's changed mark. A store the pass
//! changed has its live side brought in, which for a plain column is nothing
//! at all, and has its position set in the mask the next pass seeds its dirty
//! units from. A store nothing changed is left alone: no copy, no exchange,
//! no bit.

use arvo_tensor::Capacity;
use hilavitkutin_api::access::{Cons, Empty};

use super::{AccumBinding, BindingNil, ColKind, ColumnBinding, ResourceBinding, VirtualBinding};
use crate::plan::AccessMask;
use crate::plan::project::{Locate, WitnessIndex};

/// Give every column store its position in the store list `Stores`.
///
/// `Witnesses` is one position witness per column store, in the sequence the
/// bindings hold them, and is inferred where `build` calls the walk. The
/// position is looked up by the marker the store was registered under, with
/// the same `Locate` the access masks are projected by, so the bit the
/// boundary reports for a store is the bit a unit's read mask has for it.
pub trait PlaceIn<Stores, Witnesses> {
    /// Record each column store's position on its node.
    fn place_in(&mut self);
}

impl<Stores> PlaceIn<Stores, Empty> for BindingNil {
    #[inline]
    fn place_in(&mut self) {}
}

impl<Stores, W, T, Tail: PlaceIn<Stores, W>> PlaceIn<Stores, W> for ResourceBinding<T, Tail> {
    #[inline]
    fn place_in(&mut self) {
        self.tail.place_in();
    }
}

impl<Stores, W, T, Tail: PlaceIn<Stores, W>> PlaceIn<Stores, W> for AccumBinding<T, Tail> {
    #[inline]
    fn place_in(&mut self) {
        self.tail.place_in();
    }
}

impl<Stores, W, T, Tail: PlaceIn<Stores, W>> PlaceIn<Stores, W> for VirtualBinding<T, Tail> {
    #[inline]
    fn place_in(&mut self) {
        self.tail.place_in();
    }
}

impl<Stores, I, W, T, Tail, K> PlaceIn<Stores, Cons<I, W>> for ColumnBinding<T, Tail, K>
where
    K: ColKind,
    Stores: Locate<K::Marker<T>, I>,
    I: WitnessIndex,
    Tail: PlaceIn<Stores, W>,
{
    #[inline]
    fn place_in(&mut self) {
        self.pos = I::INDEX;
        self.tail.place_in();
    }
}

/// Bring in what the pass changed and report which stores those were.
pub trait BringIn {
    /// Clear every mark, bring in each marked store, and return `changed`
    /// with the marked stores' positions set.
    ///
    /// # Safety
    ///
    /// No context bound from these bindings, or from a per-core copy of
    /// them, is alive: the last unit of the pass has returned and the
    /// workers are parked.
    unsafe fn bring_in<C: Capacity>(&self, changed: AccessMask<C>) -> AccessMask<C>;
}

impl BringIn for BindingNil {
    #[inline]
    unsafe fn bring_in<C: Capacity>(&self, changed: AccessMask<C>) -> AccessMask<C> {
        changed
    }
}

impl<T, Tail: BringIn> BringIn for ResourceBinding<T, Tail> {
    #[inline]
    unsafe fn bring_in<C: Capacity>(&self, changed: AccessMask<C>) -> AccessMask<C> {
        // SAFETY: the caller's guarantee covers the whole list.
        unsafe { self.tail.bring_in(changed) }
    }
}

impl<T, Tail: BringIn> BringIn for AccumBinding<T, Tail> {
    #[inline]
    unsafe fn bring_in<C: Capacity>(&self, changed: AccessMask<C>) -> AccessMask<C> {
        // SAFETY: the caller's guarantee covers the whole list.
        unsafe { self.tail.bring_in(changed) }
    }
}

impl<T, Tail: BringIn> BringIn for VirtualBinding<T, Tail> {
    #[inline]
    unsafe fn bring_in<C: Capacity>(&self, changed: AccessMask<C>) -> AccessMask<C> {
        // SAFETY: the caller's guarantee covers the whole list.
        unsafe { self.tail.bring_in(changed) }
    }
}

impl<T, Tail: BringIn, K: ColKind> BringIn for ColumnBinding<T, Tail, K> {
    #[inline]
    unsafe fn bring_in<C: Capacity>(&self, changed: AccessMask<C>) -> AccessMask<C> {
        let changed = if self.mark.take().0 {
            // SAFETY: the caller holds that nothing reads or writes the store
            // now, and both sides were reserved for `count` records.
            unsafe { K::bring_in(self.ptr, &self.side, self.count) };
            changed.set(self.pos)
        } else {
            changed
        };
        // SAFETY: the caller's guarantee covers the whole list.
        unsafe { self.tail.bring_in(changed) }
    }
}
