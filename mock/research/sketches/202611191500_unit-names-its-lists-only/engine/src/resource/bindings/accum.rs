//! The binding of one registered `Accum<T>`: its cons-cell, its place in the
//! value-list mapping, its drain and its frame-start reset.

use core::cell::Cell;

use arvo::USize;
use hilavitkutin_api::access::Cons;
use hilavitkutin_api::store::Accum;
use hilavitkutin_api::store_values::{StoreValues, Sv};
use hilavitkutin_api::{ColumnStorage, ColumnValue};

use super::column::reserve_records;
use super::{BindingsFor, DrainStores, ResetAccumulators, sealed};
use crate::resource::provenance::ColumnPtr;
use crate::scheduler::BuildError;

/// Bindings cons-cell for one registered `Accum<T>`.
///
/// Holds the reserved `ColumnPtr<T>` (base of the capacity buffer, sized by
/// the build-time record count), a `Cell<USize>` live-length and a
/// `Cell<USize>` `wanted` count, both advanced by the `&self` append accessor
/// through interior mutability. `wanted` counts every append the frame asked
/// of the accumulator; the live length counts the ones written. So `wanted`
/// is never below the live length, and the accumulator has refused this frame
/// exactly when `wanted` is above it. The cells are non-atomic: each binding
/// is written by one thread for the frame (the calling thread, or a worker on
/// its own per-core copy). A zero-record (or zero-sized) accumulator records a
/// dangling, well-aligned pointer (never dereferenced, the morsel is empty).
pub struct AccumBinding<T, Tail> {
    pub(crate) ptr:    ColumnPtr<T>,
    pub(crate) len:    Cell<USize>,
    pub(crate) wanted: Cell<USize>,
    pub(crate) cap:    USize,
    pub(crate) tail:   Tail,
}

impl<T, Tail> AccumBinding<T, Tail> {
    /// The reserved capacity-buffer base pointer. Hidden accessor used by the
    /// `AccumSelector` and by tests. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __ptr(&self) -> ColumnPtr<T> {
        self.ptr
    }

    /// The reserved record capacity. Hidden accessor used by the
    /// `AccumSelector` (an append past this bound is refused) and by tests.
    /// Not part of the supported surface.
    #[doc(hidden)]
    pub fn __cap(&self) -> USize {
        self.cap
    }

    /// The live-length cell. Hidden accessor used by the `AccumSelector` (the
    /// projection borrows it for `'frame`) and by tests. The append accessor
    /// reads, writes, and advances it under `&self`. Not supported surface.
    #[doc(hidden)]
    pub fn __len_cell(&self) -> &Cell<USize> {
        &self.len
    }

    /// The `wanted` cell: every append the frame asked of the accumulator.
    /// Hidden accessor used by the `AccumSelector` (the projection borrows it
    /// for `'frame`) and by tests. Not supported surface.
    #[doc(hidden)]
    pub fn __wanted_cell(&self) -> &Cell<USize> {
        &self.wanted
    }

    /// The tail node. Hidden accessor used by the pass-through selectors and by
    /// tests. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __tail(&self) -> &Tail {
        &self.tail
    }
}

impl<T: 'static, L: StoreValues + BindingsFor> sealed::BindingsSealed for Sv<Accum<T>, L> {}
impl<T: 'static, L: StoreValues + BindingsFor> BindingsFor for Sv<Accum<T>, L> {
    type Bindings = AccumBinding<T, <L as BindingsFor>::Bindings>;
    type Markers = Cons<Accum<T>, <L as BindingsFor>::Markers>;
}

impl<T: ColumnValue, L> DrainStores for Sv<Accum<T>, L>
where
    L: StoreValues + BindingsFor + DrainStores,
{
    fn drain<CS: ColumnStorage>(
        self,
        cs: &mut CS,
        next_id: &mut USize,
        record_count: USize,
    ) -> notko::Outcome<Self::Bindings, BuildError> {
        // Accumulator: reserve a capacity buffer at a `StoreId` continued past
        // the prior columns (capacity equals the record count) and record its
        // base pointer plus a zero live-length. Records are appended by
        // WorkUnits during the frame; the drain reserves and zeroes the live
        // and `wanted` counts, it does not initialise records. An append past
        // `record_count` is refused and counted in `wanted`.
        let (_marker, rest) = self.into_parts();
        let ptr = match reserve_records::<T, CS>(cs, next_id, record_count) {
            notko::Outcome::Ok(ptr) => ptr,
            notko::Outcome::Err(e) => return notko::Outcome::Err(e),
        };
        match <L as DrainStores>::drain(rest, cs, next_id, record_count) {
            notko::Outcome::Ok(tail) => {
                notko::Outcome::Ok(AccumBinding {
                    ptr,
                    len: Cell::new(USize(0)), // lint:allow(no-bare-numeric) reason: zero live-length init on a fresh accumulator; tracked: #345
                    wanted: Cell::new(USize(0)), // lint:allow(no-bare-numeric) reason: zero wanted count on a fresh accumulator; tracked: #345
                    // Capacity equals the reserved record count; the append
                    // accessor refuses past it.
                    cap: record_count,
                    tail,
                })
            },
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}

impl<T, Tail: ResetAccumulators> ResetAccumulators for AccumBinding<T, Tail> {
    #[inline]
    fn reset_accumulators(&self) {
        self.len.set(USize(0)); // lint:allow(no-bare-numeric) reason: zero live-length reset at frame start; tracked: #345
        self.wanted.set(USize(0)); // lint:allow(no-bare-numeric) reason: zero wanted count reset at frame start; tracked: #345
        self.tail.reset_accumulators();
    }
}
