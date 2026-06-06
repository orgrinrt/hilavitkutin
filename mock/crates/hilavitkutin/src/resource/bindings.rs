//! Resource bindings: the type-keyed pointer index over the registered
//! store values, resolved against the unified `ColumnStorage`.
//!
//! The bindings are a cons-list keyed on the builder's `StoreValues`
//! list, not on the `Stores` access set. Each `.with`-registered store
//! value contributes one binding cons-cell: a `Resource<T>` carrier
//! (`StagedResource<T>`) contributes a `ResourceBinding<T, _>` holding
//! the moved-in value's `ResourcePtr<T>`; a `Column<T>` marker
//! contributes a `ColumnBinding` holding the reserved column buffer base
//! (`ColumnPtr<T>`, sized to the build-time record count) and the count;
//! a `Virtual<T>` marker contributes a `VirtualBinding` (no backing
//! storage).
//!
//! Keying on `StoreValues` (rather than `Stores`) keeps the drain a
//! trivial single-list walk and sidesteps the Kit case: a Kit's owned
//! stores enter the `Stores` access set via `Concat` with NO
//! `.with`-value (their values come from `HasTrivialCtor`, a later
//! round), so they contribute no `StoreValues` node and no binding
//! here. That is the correct behaviour: the drain handles the explicitly
//! registered store values only.
//!
//! `DrainStores` walks the value list, reserving a one-record column per
//! `Resource<T>` and a `record_count`-record column per `Column<T>`
//! through the supplied `ColumnStorage`, and recording each column base
//! pointer. Resources and column values are `ColumnValue` (`Copy +
//! 'static`), so the bindings hold only raw pointers: the store owns the
//! bytes and frees them on its own `Drop`, and there is no per-binding
//! destructor to run. A zero-sized resource (or zero-record column)
//! occupies no bytes, so it reserves nothing and records a dangling,
//! well-aligned pointer.

use core::cell::Cell;
use core::marker::PhantomData;
use core::mem::size_of;

use arvo::USize;
use hilavitkutin_api::access::{AccessSet, Cons, Empty};
use hilavitkutin_api::store::{Accum, Column, Resource, StagedResource, Virtual};
use hilavitkutin_api::store_values::{StoreValues, Sv, SvEmpty};
use hilavitkutin_api::{ColumnStorage, ColumnValue, StoreId};

use crate::resource::provenance::{ColumnPtr, ResourcePtr};
use crate::scheduler::BuildError;

mod sealed {
    pub trait Sealed {}
}

/// The empty bindings list (matches the empty value list).
pub struct BindingNil;

/// Bindings cons-cell for one registered `Resource<T>`.
///
/// Holds the `ResourcePtr<T>` for the moved-in value (pointing into the
/// store-reserved column, or a dangling pointer for a ZST) and the tail
/// node for the remaining store values.
pub struct ResourceBinding<T, Tail> {
    pub(crate) ptr: ResourcePtr<T>,
    pub(crate) tail: Tail,
}

impl<T, Tail> ResourceBinding<T, Tail> {
    /// The recorded resource pointer. Hidden test accessor: lets tests
    /// deref the moved-in value. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __ptr(&self) -> ResourcePtr<T> {
        self.ptr
    }

    /// The tail node. Hidden test accessor; not supported surface.
    #[doc(hidden)]
    pub fn __tail(&self) -> &Tail {
        &self.tail
    }
}

/// Bindings cons-cell for one registered `Column<T>`.
///
/// Holds the reserved `ColumnPtr<T>` (base of the column buffer sized by
/// the build-time record count) and the record count, read through
/// hidden accessors mirroring `ResourceBinding`. A zero-record (or
/// zero-sized) column records a dangling, well-aligned pointer (never
/// dereferenced, the morsel is empty).
pub struct ColumnBinding<T, Tail> {
    pub(crate) ptr: ColumnPtr<T>,
    pub(crate) count: USize,
    pub(crate) tail: Tail,
}

impl<T, Tail> ColumnBinding<T, Tail> {
    /// The recorded column base pointer. Hidden accessor used by the
    /// column `ColSelector` and by tests. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __ptr(&self) -> ColumnPtr<T> {
        self.ptr
    }

    /// The reserved record count. Hidden test accessor; not supported surface.
    #[doc(hidden)]
    pub fn __count(&self) -> USize {
        self.count
    }

    /// The tail node. Hidden accessor used by the pass-through selectors
    /// and by tests. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __tail(&self) -> &Tail {
        &self.tail
    }
}

/// Bindings cons-cell for one registered `Accum<T>`.
///
/// Holds the reserved `ColumnPtr<T>` (base of the capacity buffer, sized by
/// the build-time record count this round) and a `Cell<USize>` live-length
/// that the `&self` append accessor advances through interior mutability. The
/// `Cell` is non-atomic: correct single-core, swapped for an atomic when
/// multi-core lands. A zero-record (or zero-sized) accumulator records a
/// dangling, well-aligned pointer (never dereferenced, the morsel is empty).
pub struct AccumBinding<T, Tail> {
    pub(crate) ptr: ColumnPtr<T>,
    pub(crate) len: Cell<USize>,
    pub(crate) cap: USize,
    pub(crate) tail: Tail,
}

impl<T, Tail> AccumBinding<T, Tail> {
    /// The reserved capacity-buffer base pointer. Hidden accessor used by the
    /// `AccumSelector` and by tests. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __ptr(&self) -> ColumnPtr<T> {
        self.ptr
    }

    /// The reserved record capacity. Hidden accessor used by the
    /// `AccumSelector` (the append saturates at this bound) and by tests. Not
    /// part of the supported surface.
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

    /// The tail node. Hidden accessor used by the pass-through selectors and by
    /// tests. Not part of the supported surface.
    #[doc(hidden)]
    pub fn __tail(&self) -> &Tail {
        &self.tail
    }
}

/// Bindings cons-cell for one registered `Virtual<T>`.
///
/// Carries no pointer: a `Virtual<T>` store is a DAG-edge marker with
/// no backing storage.
pub struct VirtualBinding<T, Tail> {
    pub(crate) _marker: PhantomData<T>,
    pub(crate) tail: Tail,
}

impl<T, Tail> VirtualBinding<T, Tail> {
    /// The tail node. Hidden accessor used by the pass-through selectors
    /// (a resource or column behind a virtual node). Not supported surface.
    #[doc(hidden)]
    pub fn __tail(&self) -> &Tail {
        &self.tail
    }
}

/// Maps a `StoreValues` list to its concrete bindings shape.
///
/// Sealed: the four arms (`SvEmpty`, and `Sv` headed by
/// `StagedResource<T>` / `Column<T>` / `Virtual<T>`) are the only
/// store-value shapes the builder places on the list. The heads are
/// distinct concrete types, so the arms are non-overlapping.
#[allow(private_bounds)]
pub trait BindingsFor: sealed::Sealed {
    /// The concrete bindings cons-list for this value list.
    ///
    /// The bindings hold only `Copy` pointers; the store frees resource
    /// memory on its own `Drop`, so the scheduler runs no bindings walk
    /// on drop and `Bindings` carries no destructor bound.
    type Bindings;

    /// The store-marker `AccessSet` this value list registers, in the same
    /// types and order as the builder's `Stores` param. `StoreDispatch<S>`
    /// prepends `Cons<S, Stores>` to the marker list while `Sv<S, _>` carries
    /// the same `S` on the value list, so `Markers` reconstructs exactly the
    /// `Stores` the builder used: the dispatch-order machinery (`MaskProject` /
    /// `Locate` in `dispatch::order`) Locates over `Markers` and gets the same
    /// bit positions `build` used, without retaining `Stores` as a `Scheduler`
    /// type param.
    type Markers: AccessSet;
}

impl sealed::Sealed for SvEmpty {}
impl BindingsFor for SvEmpty {
    type Bindings = BindingNil;
    type Markers = Empty;
}

impl<T: 'static, L: StoreValues + BindingsFor> sealed::Sealed for Sv<StagedResource<T>, L> {}
impl<T: 'static, L: StoreValues + BindingsFor> BindingsFor for Sv<StagedResource<T>, L> {
    type Bindings = ResourceBinding<T, <L as BindingsFor>::Bindings>;
    // The value list carries the `StagedResource<T>` carrier, but the builder's
    // `Stores` access set lists `Resource<T>` (`StagedResource<T>` dispatches as
    // `StoreDispatch<Resource<T>>`). Recover the access marker, not the carrier,
    // so `MaskProject`/`Locate` over `Markers` find the same bit positions the
    // WU access sets (which reference `Resource<T>`) use.
    type Markers = Cons<Resource<T>, <L as BindingsFor>::Markers>;
}

impl<T: 'static, L: StoreValues + BindingsFor> sealed::Sealed for Sv<Column<T>, L> {}
impl<T: 'static, L: StoreValues + BindingsFor> BindingsFor for Sv<Column<T>, L> {
    type Bindings = ColumnBinding<T, <L as BindingsFor>::Bindings>;
    type Markers = Cons<Column<T>, <L as BindingsFor>::Markers>;
}

impl<T: 'static, L: StoreValues + BindingsFor> sealed::Sealed for Sv<Virtual<T>, L> {}
impl<T: 'static, L: StoreValues + BindingsFor> BindingsFor for Sv<Virtual<T>, L> {
    type Bindings = VirtualBinding<T, <L as BindingsFor>::Bindings>;
    type Markers = Cons<Virtual<T>, <L as BindingsFor>::Markers>;
}

impl<T: 'static, L: StoreValues + BindingsFor> sealed::Sealed for Sv<Accum<T>, L> {}
impl<T: 'static, L: StoreValues + BindingsFor> BindingsFor for Sv<Accum<T>, L> {
    type Bindings = AccumBinding<T, <L as BindingsFor>::Bindings>;
    type Markers = Cons<Accum<T>, <L as BindingsFor>::Markers>;
}

/// Reserves and populates the bindings by consuming a `StoreValues` list.
///
/// Implemented on the value list. Per `Resource<T>` value: reserve a
/// one-record column through the store, write the carrier's value in,
/// record the column base pointer. A zero-sized `T` reserves no column
/// and records a dangling pointer. Per `Column<T>` / `Virtual<T>` value:
/// no reservation (the column buffer round and virtual markers carry no
/// storage here).
///
/// Sealed: only the arms below inhabit it.
#[allow(private_bounds)]
pub trait DrainStores: BindingsFor + sealed::Sealed {
    /// Reserve and populate the bindings, consuming the value list.
    ///
    /// `next_id` is the running drain-order column id, advanced once per
    /// non-zero-sized resource and once per column. `record_count` is the
    /// per-frame record count every input column is reserved to. On
    /// reservation failure the store frees every column reserved so far
    /// when it is dropped (the resources are `Copy`, so no destructor is
    /// skipped), and `Err(BuildError::AllocationFailed)` returns.
    fn drain<CS: ColumnStorage>(
        self,
        cs: &mut CS,
        next_id: &mut USize,
        record_count: USize,
    ) -> notko::Outcome<Self::Bindings, BuildError>;
}

impl DrainStores for SvEmpty {
    #[inline]
    fn drain<CS: ColumnStorage>(
        self,
        _cs: &mut CS,
        _next_id: &mut USize,
        _record_count: USize,
    ) -> notko::Outcome<Self::Bindings, BuildError> {
        notko::Outcome::Ok(BindingNil)
    }
}

impl<T: ColumnValue, L> DrainStores for Sv<StagedResource<T>, L>
where
    L: StoreValues + BindingsFor + DrainStores,
{
    fn drain<CS: ColumnStorage>(
        self,
        cs: &mut CS,
        next_id: &mut USize,
        record_count: USize,
    ) -> notko::Outcome<Self::Bindings, BuildError> {
        let (carrier, rest) = self.into_parts();
        let value = carrier.into_inner();
        // lint:allow(no-bare-numeric) reason: size_of returns usize by contract; tracked: #345
        let ptr = if size_of::<T>() == 0 {
            // Zero-sized resource: no bytes to store. A column reserve
            // would hand back a null base pointer (nothing allocated),
            // and `ResourcePtr` is non-null by construction. Record a
            // dangling, well-aligned pointer; writing and reading a ZST
            // through any aligned non-null pointer touches no memory.
            let dangling = core::ptr::NonNull::<T>::dangling().as_ptr();
            // SAFETY: a ZST write touches no memory; `dangling` is the
            // type's alignment, which is a valid address for a ZST.
            unsafe {
                core::ptr::write(dangling, value);
            }
            // SAFETY: `NonNull::dangling` is non-null.
            unsafe { ResourcePtr::new_unchecked(dangling) }
        } else {
            let id = StoreId(*next_id);
            // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal drain-order counter; tracked: #72
            *next_id = USize((*next_id).0 + 1);
            // Reserve one record's worth of column for this resource.
            // lint:allow(no-bare-numeric) reason: one-record column reservation count; tracked: #345
            match cs.reserve::<T>(id, USize(1)) {
                notko::Outcome::Ok(()) => {}
                notko::Outcome::Err(_) => return notko::Outcome::Err(BuildError::AllocationFailed),
            }
            // SAFETY: `id` names a column just reserved for `T`; the store
            // returns its 64-byte-aligned base pointer sized for one `T`.
            let typed = unsafe { cs.column_ptr_mut::<T>(id) };
            if typed.is_null() {
                return notko::Outcome::Err(BuildError::AllocationFailed);
            }
            // SAFETY: `typed` is non-null (checked), suitably aligned, and
            // sized for one `T` (just reserved); the write initialises it
            // without reading or dropping prior (uninitialised) contents.
            unsafe {
                core::ptr::write(typed, value);
            }
            // SAFETY: non-null checked above.
            unsafe { ResourcePtr::new_unchecked(typed) }
        };
        match <L as DrainStores>::drain(rest, cs, next_id, record_count) {
            notko::Outcome::Ok(tail) => notko::Outcome::Ok(ResourceBinding { ptr, tail }),
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}

impl<T: ColumnValue, L> DrainStores for Sv<Column<T>, L>
where
    L: StoreValues + BindingsFor + DrainStores,
{
    fn drain<CS: ColumnStorage>(
        self,
        cs: &mut CS,
        next_id: &mut USize,
        record_count: USize,
    ) -> notko::Outcome<Self::Bindings, BuildError> {
        // Input column: reserve a buffer sized by the record count at a
        // `StoreId` continued past the resource columns, and record the real
        // base pointer plus the count. Records are written by producer
        // WorkUnits during the frame; the drain reserves, it does not
        // initialise.
        let (_marker, rest) = self.into_parts();
        let id = StoreId(*next_id);
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal drain-order counter; tracked: #72
        *next_id = USize((*next_id).0 + 1);
        match cs.reserve::<T>(id, record_count) {
            notko::Outcome::Ok(()) => {}
            notko::Outcome::Err(_) => return notko::Outcome::Err(BuildError::AllocationFailed),
        }
        // SAFETY: `id` names a column just reserved for `record_count`
        // records of `T`; the store returns its 64-byte-aligned base pointer.
        let typed = unsafe { cs.column_ptr_mut::<T>(id) };
        let ptr = if typed.is_null() {
            // A zero-record (or zero-sized `T`) column reserves no bytes and
            // hands back a null base. The morsel is then empty, so the column
            // is never read or written; record a dangling, well-aligned,
            // non-null pointer to satisfy `ColumnPtr`'s invariant.
            // SAFETY: `NonNull::dangling` is non-null and `T`-aligned; never
            // dereferenced (the empty morsel skips every access).
            unsafe { ColumnPtr::new_unchecked(core::ptr::NonNull::<T>::dangling().as_ptr()) }
        } else {
            // SAFETY: `typed` is non-null (checked), 64-byte aligned, and sized
            // for `record_count` records (just reserved).
            unsafe { ColumnPtr::new_unchecked(typed) }
        };
        match <L as DrainStores>::drain(rest, cs, next_id, record_count) {
            notko::Outcome::Ok(tail) => notko::Outcome::Ok(ColumnBinding {
                ptr,
                count: record_count,
                tail,
            }),
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
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
            notko::Outcome::Ok(tail) => notko::Outcome::Ok(VirtualBinding {
                _marker: PhantomData,
                tail,
            }),
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}

/// Zeroes every accumulator live-length in a bindings list at frame start.
///
/// The schedule-once-reuse model runs one built scheduler across many frames.
/// An `AccumBinding` holds a `Cell<USize>` live-length the append accessor
/// advances during a frame; without a per-frame reset the next frame would
/// append from the prior frame's offset (and saturate once it reaches the
/// reserved capacity). `Scheduler::run` calls this at frame start so each frame
/// appends into a fresh buffer.
///
/// The walk is the bindings-cons-list mirror of `DrainStores`: a no-op at every
/// non-accumulator node and one `Cell` write per `AccumBinding`, recursing into
/// the tail. The reset is `&self`: the live-length is interior-mutable
/// (`Cell`), so no `&mut` is needed. For an accumulator-free carrier the walk
/// visits no `AccumBinding` and compiles to nothing.
pub trait ResetAccumulators {
    /// Zero every accumulator live-length cell in this bindings list.
    fn reset_accumulators(&self);
}

impl ResetAccumulators for BindingNil {
    #[inline]
    fn reset_accumulators(&self) {}
}

impl<T, Tail: ResetAccumulators> ResetAccumulators for ResourceBinding<T, Tail> {
    #[inline]
    fn reset_accumulators(&self) {
        self.tail.reset_accumulators();
    }
}

impl<T, Tail: ResetAccumulators> ResetAccumulators for ColumnBinding<T, Tail> {
    #[inline]
    fn reset_accumulators(&self) {
        self.tail.reset_accumulators();
    }
}

impl<T, Tail: ResetAccumulators> ResetAccumulators for VirtualBinding<T, Tail> {
    #[inline]
    fn reset_accumulators(&self) {
        self.tail.reset_accumulators();
    }
}

impl<T, Tail: ResetAccumulators> ResetAccumulators for AccumBinding<T, Tail> {
    #[inline]
    fn reset_accumulators(&self) {
        // lint:allow(no-bare-numeric) reason: zero live-length reset at frame start; tracked: #345
        self.len.set(USize(0));
        self.tail.reset_accumulators();
    }
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
        // the prior columns (capacity equals the record count this round) and
        // record its base pointer plus a zero live-length. Records are appended
        // by WorkUnits during the frame; the drain reserves and zeroes the live
        // count, it does not initialise records. Appending past `record_count`
        // is out of contract this round (the plan does not yet bound it).
        let (_marker, rest) = self.into_parts();
        let id = StoreId(*next_id);
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-arith on USize internal drain-order counter; tracked: #72
        *next_id = USize((*next_id).0 + 1);
        match cs.reserve::<T>(id, record_count) {
            notko::Outcome::Ok(()) => {}
            notko::Outcome::Err(_) => return notko::Outcome::Err(BuildError::AllocationFailed),
        }
        // SAFETY: `id` names a column just reserved for `record_count` records
        // of `T`; the store returns its 64-byte-aligned base pointer.
        let typed = unsafe { cs.column_ptr_mut::<T>(id) };
        let ptr = if typed.is_null() {
            // A zero-record (or zero-sized `T`) accumulator reserves no bytes and
            // hands back a null base. Nothing is ever appended (an empty frame),
            // so record a dangling, well-aligned, non-null pointer to satisfy
            // `ColumnPtr`'s invariant.
            // SAFETY: `NonNull::dangling` is non-null and `T`-aligned; never
            // dereferenced (no append touches an empty buffer).
            unsafe { ColumnPtr::new_unchecked(core::ptr::NonNull::<T>::dangling().as_ptr()) }
        } else {
            // SAFETY: `typed` is non-null (checked), 64-byte aligned, and sized
            // for `record_count` records (just reserved).
            unsafe { ColumnPtr::new_unchecked(typed) }
        };
        match <L as DrainStores>::drain(rest, cs, next_id, record_count) {
            notko::Outcome::Ok(tail) => notko::Outcome::Ok(AccumBinding {
                ptr,
                // lint:allow(no-bare-numeric) reason: zero live-length init on a fresh accumulator; tracked: #345
                len: Cell::new(USize(0)),
                // Capacity equals the reserved record count this round; the
                // append accessor saturates at it.
                cap: record_count,
                tail,
            }),
            notko::Outcome::Err(e) => notko::Outcome::Err(e),
        }
    }
}
