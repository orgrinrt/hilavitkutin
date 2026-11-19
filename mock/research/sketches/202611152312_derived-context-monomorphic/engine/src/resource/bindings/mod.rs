//! Resource bindings: the type-keyed pointer index over the registered
//! store values, resolved against the unified `ColumnStorage`.
//!
//! The bindings are a cons-list keyed on the builder's `StoreValues`
//! list, not on the `Stores` access set. Each `.with`-registered store
//! value contributes one binding cons-cell: a `Resource<T>` carrier
//! (`StagedResource<T>`) contributes a `ResourceBinding<T, _>` holding
//! the moved-in value's `ResourcePtr<T>`; a column, plain or buffered,
//! comes as a `Started<S>` carrier and contributes a `ColumnBinding`
//! holding the reserved column buffer base (`ColumnPtr<T>`, sized to the
//! build-time record count) and the count, its start written to every record;
//! a `Virtual<T>` marker contributes a `VirtualBinding` (no backing
//! storage); an `Accum<T>` contributes an `AccumBinding` over its capacity
//! buffer.
//!
//! Keying on `StoreValues` (rather than `Stores`) keeps the drain a
//! trivial single-list walk and sidesteps the Kit case: a Kit's owned
//! stores enter the `Stores` access set via `Concat` with no
//! `.with`-value, so they contribute no `StoreValues` node and no binding
//! here. The drain handles the explicitly registered store values only.
//!
//! `DrainStores` walks the value list, reserving a one-record column per
//! `Resource<T>` and a `record_count`-record column per `Column<T>`
//! through the supplied `ColumnStorage`, and recording each column base
//! pointer. A `Buffered<T, M>` reserves two such columns, its frozen and
//! its live side. Resources and column values are `ColumnValue` (`Copy +
//! 'static`), so the bindings hold only raw pointers: the store owns the
//! bytes and frees them on its own `Drop`, and there is no per-binding
//! destructor to run. A zero-sized resource (or zero-record column)
//! occupies no bytes, so it reserves nothing and records a dangling,
//! well-aligned pointer.
//!
//! Each kind of binding sits in a module of its own with its list mapping,
//! its drain and its frame-start reset: `resource`, `column`, `virt` and
//! `accum`.

use arvo::USize;
use hilavitkutin_api::access::{AccessSet, Cons, Empty};
use hilavitkutin_api::member::Member;
use hilavitkutin_api::store::Started;
use hilavitkutin_api::store_values::{StoreValues, Sv, SvEmpty};
use hilavitkutin_api::{ColumnStorage, ColumnValue};

use crate::scheduler::BuildError;

mod accum;
mod close;
mod column;
mod per_core;
mod refusal;
mod resource;
mod virt;

pub use accum::AccumBinding;
pub use close::{BringIn, PlaceIn};
use column::drain_column;
pub use column::{ColKind, ColumnBinding, Flip, Kept, MarkPtr, Plain};
pub(crate) use per_core::accum_slice;
pub use per_core::{CollectAccumCounts, MergeAccums, PerCoreCounts, RebaseBindings};
pub use refusal::FirstRefusal;
pub use resource::ResourceBinding;
pub use virt::VirtualBinding;

mod sealed {
    pub trait BindingsSealed {}
}

/// The empty bindings list (matches the empty value list).
pub struct BindingNil;

/// Maps a `StoreValues` list to its concrete bindings shape.
///
/// Sealed: the arms (`SvEmpty`, and `Sv` headed by `StagedResource<T>`,
/// a `Started<S>` carrier of a column, plain or buffered, `Virtual<T>` or
/// `Accum<T>`) are the only
/// store-value shapes the builder places on the list. The heads are
/// distinct concrete types, so the arms are non-overlapping.
pub trait BindingsFor: sealed::BindingsSealed {
    /// The concrete bindings cons-list for this value list.
    ///
    /// The bindings hold only `Copy` pointers; the store frees resource
    /// memory on its own `Drop`, so the scheduler runs no bindings walk
    /// on drop and `Bindings` carries no destructor bound.
    type Bindings: BringIn;

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

impl sealed::BindingsSealed for SvEmpty {}
impl BindingsFor for SvEmpty {
    type Bindings = BindingNil;
    type Markers = Empty;
}

impl<S: Kept + Member + 'static, L: StoreValues + BindingsFor> sealed::BindingsSealed
    for Sv<Started<S>, L>
{
}
impl<S: Kept + Member + 'static, L: StoreValues + BindingsFor> BindingsFor for Sv<Started<S>, L> {
    type Bindings = ColumnBinding<S::Value, <L as BindingsFor>::Bindings, S::Kind>;
    // As with a resource, the value list carries the carrier and the access
    // sets name the marker.
    type Markers = Cons<S, <L as BindingsFor>::Markers>;
}

/// Reserves and populates the bindings by consuming a `StoreValues` list.
///
/// Implemented on the value list. Per `Resource<T>` value: reserve a
/// one-record column through the store, write the carrier's value in,
/// record the column base pointer. A zero-sized `T` reserves no column
/// and records a dangling pointer. Per column value, plain or buffered, which
/// comes as a `Started<S>` carrier: reserve its records and write its start to
/// every record of every reservation. A `Virtual<T>` marker carries no storage
/// here.
///
/// Sealed: only the arms below and in the binding modules inhabit it.
pub trait DrainStores: BindingsFor + sealed::BindingsSealed {
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

impl<S: Kept + Member + 'static, L> DrainStores for Sv<Started<S>, L>
where
    S::Value: ColumnValue,
    L: StoreValues + BindingsFor + DrainStores,
{
    fn drain<CS: ColumnStorage>(
        self,
        cs: &mut CS,
        next_id: &mut USize,
        record_count: USize,
    ) -> notko::Outcome<Self::Bindings, BuildError> {
        // A column of either kind. A plain one is one reservation of
        // `record_count` records. A buffered one is two of one layout, the
        // live side and the frozen one, at two storage ids one after the
        // other, and one store in every access mask all the same. Every
        // record of every reservation holds the value the store was
        // registered with once the drain returns, so no read of a pass
        // reaches a record nothing wrote.
        let (carrier, rest) = self.into_parts();
        let start = carrier.into_inner();
        let drained =
            drain_column::<S::Value, S::Kind, CS, _>(cs, next_id, record_count, |cs, next_id| {
                <L as DrainStores>::drain(rest, cs, next_id, record_count)
            });
        if let notko::Outcome::Ok(binding) = &drained {
            binding.start_from(start);
        }
        drained
    }
}

/// Zeroes every accumulator live length and `wanted` count at frame start.
///
/// The schedule-once-reuse model runs one built scheduler across many frames.
/// An `AccumBinding` holds a live length and a `wanted` count the append
/// accessor advances during a frame; without a per-frame reset the next frame
/// would append from the prior frame's offset and inherit its refusal. Every
/// run entry calls this at frame start so each frame appends into a fresh
/// buffer with nothing refused.
///
/// The walk is the bindings-cons-list mirror of `DrainStores`: a no-op at every
/// non-accumulator node and two `Cell` writes per `AccumBinding`, recursing
/// into the tail. The reset is `&self`: both counts are interior-mutable
/// (`Cell`), so no `&mut` is needed. For an accumulator-free carrier the walk
/// visits no `AccumBinding` and compiles to nothing.
pub trait ResetAccumulators {
    /// Zero every accumulator live-length and `wanted` cell in this list.
    fn reset_accumulators(&self);
}

impl ResetAccumulators for BindingNil {
    #[inline]
    fn reset_accumulators(&self) {}
}

impl<T, Tail: ResetAccumulators, K: ColKind> ResetAccumulators for ColumnBinding<T, Tail, K> {
    #[inline]
    fn reset_accumulators(&self) {
        self.tail.reset_accumulators();
    }
}
