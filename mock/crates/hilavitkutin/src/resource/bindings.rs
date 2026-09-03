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

use arvo::{Bool, USize};
use hilavitkutin_api::access::{AccessSet, Cons, Empty};
use hilavitkutin_api::store::{Accum, Column, Resource, StagedResource, Virtual};
use hilavitkutin_api::store_values::{StoreValues, Sv, SvEmpty};
use hilavitkutin_api::{ColumnStorage, ColumnValue, StoreId};

use crate::resource::provenance::{ColumnPtr, ErasedResourcePtr, ResourcePtr};
use crate::resource::shape::ValueShape;
use crate::scheduler::BuildError;

mod sealed {
    pub trait Sealed {}
}

/// The empty bindings list (matches the empty value list).
pub struct BindingNil;

/// Bindings cons-cell for one registered `Resource<T>`.
///
/// Records the moved-in value's one-record blob base in erased form
/// (`ErasedResourcePtr`, pointing into the store-reserved column, or a
/// dangling base for a ZST) plus the value's static shape, and the tail
/// node for the remaining store values. The typed `ResourcePtr<T>` is
/// recovered by backcast at projection time; the binding's type
/// parameter is the backcast witness and carries the value type's
/// Send/Sync gating through `PhantomData<T>`.
pub struct ResourceBinding<T, Tail> {
    pub(crate) erased: ErasedResourcePtr,
    pub(crate) shape: ValueShape,
    pub(crate) _ty: PhantomData<T>,
    pub(crate) tail: Tail,
}

impl<T, Tail> ResourceBinding<T, Tail> {
    /// The typed resource pointer, backcast from the erased base. Hidden
    /// accessor used by the projection and by tests. Not part of the
    /// supported surface.
    #[doc(hidden)]
    pub fn __ptr(&self) -> ResourcePtr<T> {
        // Forward-looking guard: tautological in-process (this binding's
        // own `T` recorded the shape), a real check only at a future
        // extension boundary where a foreign shape is supplied.
        debug_assert_eq!(self.shape, ValueShape::of::<T>());
        // SAFETY: the binding's type parameter witnesses that the erased
        // base was recorded for a `T` at drain (the drain writes the
        // staged `T` and erases the same pointer).
        unsafe { self.erased.typed::<T>() }
    }

    /// The recorded static shape. Hidden test accessor; not supported
    /// surface.
    #[doc(hidden)]
    pub fn __shape(&self) -> ValueShape {
        self.shape
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
    /// Fired-flag stamp (E4 slice-1, domain-10): the epoch value at which
    /// `T` last fired. "Set this pass" means `stamp == current_epoch`; a
    /// per-pass epoch increment auto-clears stale stamps with no memset. The
    /// firer (`EngineCtx::fire`) and the `On<V>` dispatch gate both reach this
    /// cell (the firer via the write-virtual projection, the gate via a
    /// `Locate` witness over the full bindings), so they agree by cell
    /// identity with no global index. Per-virtual `Cell<USize>` is the
    /// correct first shape; bit-packing into `Bits` words (hierarchical
    /// zero-skip) is the slice-1b refinement.
    pub(crate) stamp: Cell<USize>,
    pub(crate) tail: Tail,
}

impl<T, Tail> VirtualBinding<T, Tail> {
    /// Record a fire at `epoch`: stamp the cell so `__is_set(epoch)` reads true
    /// until the epoch advances. Non-atomic (internal fire), per spec `:716`.
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
        let typed_base = if size_of::<T>() == 0 {
            // Zero-sized resource: no bytes to store. A column reserve
            // would hand back a null base pointer (nothing allocated),
            // and the recorded base is non-null by construction. Record a
            // dangling, well-aligned pointer; writing and reading a ZST
            // through any aligned non-null pointer touches no memory.
            let dangling = core::ptr::NonNull::<T>::dangling().as_ptr();
            // SAFETY: a ZST write touches no memory; `dangling` is the
            // type's alignment, which is a valid address for a ZST.
            unsafe {
                core::ptr::write(dangling, value);
            }
            dangling
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
            typed
        };
        // Record the base in erased form plus the value's static shape;
        // the typed view is backcast at projection time (hybrid
        // addressing, round 202606210600).
        // SAFETY: `typed_base` is non-null on both arms (dangling for a
        // ZST, checked for a reserved column).
        let erased = unsafe { ErasedResourcePtr::new_unchecked(typed_base as *mut u8) }; // lint:allow(no-bare-numeric) reason: erased byte base is the addressing contract; tracked: #654
        match <L as DrainStores>::drain(rest, cs, next_id, record_count) {
            notko::Outcome::Ok(tail) => notko::Outcome::Ok(ResourceBinding {
                erased,
                shape: ValueShape::of::<T>(),
                _ty: PhantomData,
                tail,
            }),
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
                stamp: Cell::new(USize(0)),
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

/// Per-core rebase of the bindings for the GATE-2 deviation 9 threaded
/// accumulator path.
///
/// Each worker drives the unit-outer accumulator dispatch over its head+tail
/// record slice `[lo, lo+region_cap)` against a per-core COPY of the bindings:
/// every `AccumBinding` is offset to `base + lo` with a fresh zero live-length
/// cell and capacity `region_cap` (its slice's worst-case room at one append per
/// record), while column / resource / virtual nodes copy their (Copy) pointers
/// unchanged. The per-core bindings then drives the existing `RunFiber::run`
/// unchanged: disjoint per-core regions plus per-core cells mean no shared
/// mutable state, so the append path is sound across workers. The merge
/// (`MergeAccums`) compacts the per-core regions afterwards.
///
/// The walk returns the SAME bindings type (each node maps to itself), so the
/// per-core value is a drop-in for `&self.bindings` at the dispatch call.
pub trait RebaseBindings {
    /// Build a per-core bindings copy with every accumulator offset to `lo` and
    /// capped at `region_cap`, fresh live cells, other nodes copied as-is.
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self;
}

impl RebaseBindings for BindingNil {
    #[inline]
    fn rebase_accums(&self, _lo: USize, _region_cap: USize) -> Self {
        BindingNil
    }
}

impl<T, Tail: RebaseBindings> RebaseBindings for ResourceBinding<T, Tail> {
    #[inline]
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self {
        ResourceBinding {
            erased: self.erased,
            shape: self.shape,
            _ty: PhantomData,
            tail: self.tail.rebase_accums(lo, region_cap),
        }
    }
}

impl<T, Tail: RebaseBindings> RebaseBindings for ColumnBinding<T, Tail> {
    #[inline]
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self {
        ColumnBinding {
            ptr: self.ptr,
            count: self.count,
            tail: self.tail.rebase_accums(lo, region_cap),
        }
    }
}

impl<T, Tail: RebaseBindings> RebaseBindings for VirtualBinding<T, Tail> {
    #[inline]
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self {
        VirtualBinding {
            _marker: PhantomData,
            stamp: Cell::new(self.stamp.get()),
            tail: self.tail.rebase_accums(lo, region_cap),
        }
    }
}

impl<T, Tail: RebaseBindings> RebaseBindings for AccumBinding<T, Tail> {
    #[inline]
    fn rebase_accums(&self, lo: USize, region_cap: USize) -> Self {
        // SAFETY: `ptr` is the reserved-buffer base; `lo` is this core's record-
        // slice start (`lo < total <= reserved capacity`), so `base + lo` lands
        // inside the reserved allocation. `region_cap` caps appends to the
        // slice's worst case (one per record).
        let offset_ptr = unsafe {
            ColumnPtr::new_unchecked(self.ptr.as_ptr().add(lo.0)) // lint:allow(no-bare-numeric) reason: element offset into reserved buffer; tracked: #121
        };
        AccumBinding {
            ptr: offset_ptr,
            len: Cell::new(USize(0)), // lint:allow(no-bare-numeric) reason: fresh per-core live length; tracked: #121
            cap: region_cap,
            tail: self.tail.rebase_accums(lo, region_cap),
        }
    }
}

/// Collect each accumulator's live length into `out` in carrier-accum order.
///
/// After a worker's unit-outer dispatch over its per-core bindings, the merge
/// needs each accumulator's final live count. This walk writes them into
/// `out[idx]`, advancing `idx` past each `AccumBinding`; non-accumulator nodes
/// are no-ops. The caller publishes `out` to the shared merge array.
pub trait CollectAccumLive {
    /// Write each accumulator live length into `out`, advancing `idx`.
    fn collect_accum_live(&self, out: &mut [USize], idx: &mut USize);
}

impl CollectAccumLive for BindingNil {
    #[inline]
    fn collect_accum_live(&self, _out: &mut [USize], _idx: &mut USize) {}
}

impl<T, Tail: CollectAccumLive> CollectAccumLive for ResourceBinding<T, Tail> {
    #[inline]
    fn collect_accum_live(&self, out: &mut [USize], idx: &mut USize) {
        self.tail.collect_accum_live(out, idx);
    }
}

impl<T, Tail: CollectAccumLive> CollectAccumLive for ColumnBinding<T, Tail> {
    #[inline]
    fn collect_accum_live(&self, out: &mut [USize], idx: &mut USize) {
        self.tail.collect_accum_live(out, idx);
    }
}

impl<T, Tail: CollectAccumLive> CollectAccumLive for VirtualBinding<T, Tail> {
    #[inline]
    fn collect_accum_live(&self, out: &mut [USize], idx: &mut USize) {
        self.tail.collect_accum_live(out, idx);
    }
}

impl<T, Tail: CollectAccumLive> CollectAccumLive for AccumBinding<T, Tail> {
    #[inline]
    fn collect_accum_live(&self, out: &mut [USize], idx: &mut USize) {
        out[idx.0] = self.len.get(); // lint:allow(no-bare-numeric) reason: accum index into live array; tracked: #121
        idx.0 += 1; // lint:allow(no-bare-numeric) reason: accum index step; tracked: #121
        self.tail.collect_accum_live(out, idx);
    }
}

/// Merge the per-core accumulator regions into the shared buffer's live prefix.
///
/// After the threaded unit-outer phase, each accumulator's per-core regions sit
/// at `[lo_c, lo_c + live_c)` of the shared reserved buffer (`lo_c = (c*per).min
/// (total)`). This walk forward-compacts them in ascending core order into the
/// shared binding's `[0, sum live)` prefix and sets the binding live length, so
/// downstream readers see the same contiguous prefix single-core `run()` would
/// produce. `live[c * stride + a]` is core `c`'s live count for accumulator `a`;
/// `accum_idx` threads the accumulator position through the walk.
///
/// The write cursor never exceeds `lo_c` (`write_pos = sum of prior live_c <=
/// sum of prior slice sizes = lo_c`), so each `ptr::copy` has `dst <= src` and is
/// forward-safe; order is preserved because cores own ascending record slices.
pub trait MergeAccums {
    /// Forward-compact each accumulator's per-core regions and set its live length.
    fn merge_accums(
        &self,
        per: USize,
        ncores: USize,
        total: USize,
        live: &[USize],
        stride: USize,
        accum_idx: &mut USize,
    );
}

impl MergeAccums for BindingNil {
    #[inline]
    fn merge_accums(
        &self,
        _per: USize,
        _ncores: USize,
        _total: USize,
        _live: &[USize],
        _stride: USize,
        _accum_idx: &mut USize,
    ) {
    }
}

impl<T, Tail: MergeAccums> MergeAccums for ResourceBinding<T, Tail> {
    #[inline]
    fn merge_accums(
        &self,
        per: USize,
        ncores: USize,
        total: USize,
        live: &[USize],
        stride: USize,
        accum_idx: &mut USize,
    ) {
        self.tail
            .merge_accums(per, ncores, total, live, stride, accum_idx);
    }
}

impl<T, Tail: MergeAccums> MergeAccums for ColumnBinding<T, Tail> {
    #[inline]
    fn merge_accums(
        &self,
        per: USize,
        ncores: USize,
        total: USize,
        live: &[USize],
        stride: USize,
        accum_idx: &mut USize,
    ) {
        self.tail
            .merge_accums(per, ncores, total, live, stride, accum_idx);
    }
}

impl<T, Tail: MergeAccums> MergeAccums for VirtualBinding<T, Tail> {
    #[inline]
    fn merge_accums(
        &self,
        per: USize,
        ncores: USize,
        total: USize,
        live: &[USize],
        stride: USize,
        accum_idx: &mut USize,
    ) {
        self.tail
            .merge_accums(per, ncores, total, live, stride, accum_idx);
    }
}

impl<T, Tail: MergeAccums> MergeAccums for AccumBinding<T, Tail> {
    #[inline]
    fn merge_accums(
        &self,
        per: USize,
        ncores: USize,
        total: USize,
        live: &[USize],
        stride: USize,
        accum_idx: &mut USize,
    ) {
        let a = accum_idx.0; // lint:allow(no-bare-numeric) reason: this accumulator's position; tracked: #121
        let base = self.ptr.as_ptr();
        let mut write_pos = 0; // lint:allow(no-bare-numeric) reason: compaction cursor; tracked: #121
        let mut c = 0; // lint:allow(no-bare-numeric) reason: core index; tracked: #121
        while c < ncores.0 {
            let lo = (c * per.0).min(total.0); // lint:allow(no-bare-numeric) reason: core slice start; tracked: #121
            let live_ca = live[c * stride.0 + a]; // lint:allow(no-bare-numeric) reason: per-(core,accum) live count; tracked: #121
            if live_ca.0 > 0 && lo != write_pos {
                // SAFETY: `base + lo` and `base + write_pos` are both within the
                // reserved buffer (`lo, write_pos < total <= capacity`), the two
                // ranges of `live_ca` elements are in-bounds, and `write_pos <=
                // lo` makes the forward copy non-destructive of unread source.
                unsafe {
                    core::ptr::copy(base.add(lo), base.add(write_pos), live_ca.0); // lint:allow(no-bare-numeric) reason: compaction memmove offsets/len; tracked: #121
                }
            }
            write_pos += live_ca.0; // lint:allow(no-bare-numeric) reason: cursor advance; tracked: #121
            c += 1; // lint:allow(no-bare-numeric) reason: core index step; tracked: #121
        }
        self.len.set(USize(write_pos));
        accum_idx.0 += 1; // lint:allow(no-bare-numeric) reason: accum position step; tracked: #121
        self.tail
            .merge_accums(per, ncores, total, live, stride, accum_idx);
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
