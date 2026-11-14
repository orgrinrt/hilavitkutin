//! The binding of one registered `Virtual<T>`: its cons-cell, its place in
//! the value-list mapping, its drain and its frame-start reset.

use core::cell::Cell;
use core::marker::PhantomData;

use arvo::{Bool, USize};
use hilavitkutin_api::ColumnStorage;
use hilavitkutin_api::access::Cons;
use hilavitkutin_api::store::Virtual;
use hilavitkutin_api::store_values::{StoreValues, Sv};

use super::{BindingsFor, DrainStores, ResetAccumulators, sealed};
use crate::scheduler::BuildError;

/// Bindings cons-cell for one registered `Virtual<T>`.
///
/// Carries no pointer: a `Virtual<T>` store is a DAG-edge marker with
/// no backing storage.
pub struct VirtualBinding<T, Tail> {
    pub(crate) _marker: PhantomData<T>,
    /// Fired-flag stamp (domain 10): the epoch value at which `T` last
    /// fired. "Set this pass" means `stamp == current_epoch`; a per-pass
    /// epoch increment auto-clears stale stamps with no memset. The firer
    /// (`EngineCtx::fire`) and the `On<V>` dispatch gate both reach this
    /// cell (the firer via the write-virtual projection, the gate via a
    /// `Locate` witness over the full bindings), so they agree by cell
    /// identity with no global index. A `Cell<USize>` per virtual; packing
    /// the stamps into `Bits` words for hierarchical zero-skip is a later
    /// refinement.
    pub(crate) stamp:   Cell<USize>,
    pub(crate) tail:    Tail,
}

impl<T, Tail> VirtualBinding<T, Tail> {
    /// Record a fire at `epoch`: stamp the cell so `__is_set(epoch)` reads true
    /// until the epoch advances. Non-atomic: a fire is internal to the frame.
    #[doc(hidden)]
    pub fn __fire(&self, epoch: USize) {
        self.stamp.set(epoch);
    }

    /// True if `T` fired this pass: `stamp == epoch`. The per-pass epoch
    /// increment makes a stale stamp read false without a clear.
    #[doc(hidden)]
    pub fn __is_set(&self, epoch: USize) -> Bool {
        Bool(self.stamp.get().0 == epoch.0)
    }

    /// The fired-stamp cell. Hidden accessor the write-virtual projection
    /// borrows for `'frame` (mirrors `AccumBinding::__len_cell`). Not surface.
    #[doc(hidden)]
    pub fn __stamp_cell(&self) -> &Cell<USize> {
        &self.stamp
    }

    /// The tail node. Hidden accessor used by the pass-through selectors
    /// (a resource or column behind a virtual node). Not supported surface.
    #[doc(hidden)]
    pub fn __tail(&self) -> &Tail {
        &self.tail
    }
}

impl<T: 'static, L: StoreValues + BindingsFor> sealed::BindingsSealed for Sv<Virtual<T>, L> {}
impl<T: 'static, L: StoreValues + BindingsFor> BindingsFor for Sv<Virtual<T>, L> {
    type Bindings = VirtualBinding<T, <L as BindingsFor>::Bindings>;
    type Markers = Cons<Virtual<T>, <L as BindingsFor>::Markers>;
}

impl<T: 'static, L> DrainStores for Sv<Virtual<T>, L>
where
    L: StoreValues + BindingsFor + DrainStores,
{
    fn drain<CS: ColumnStorage>(
        self,
        cs: &mut CS,
        next_id: &mut USize,
        record_count: USize,
    ) -> notko::Outcome<Self::Bindings, BuildError> {
        let (_marker, rest) = self.into_parts();
        match <L as DrainStores>::drain(rest, cs, next_id, record_count) {
            notko::Outcome::Ok(tail) => {
                notko::Outcome::Ok(VirtualBinding {
                    _marker: PhantomData,
                    stamp: Cell::new(USize(0)),
                    tail,
                })
            },
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}

impl<T, Tail: ResetAccumulators> ResetAccumulators for VirtualBinding<T, Tail> {
    #[inline]
    fn reset_accumulators(&self) {
        self.tail.reset_accumulators();
    }
}
