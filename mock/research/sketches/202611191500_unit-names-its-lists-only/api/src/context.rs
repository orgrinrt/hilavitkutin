//! Provider API and accessor traits for the WU context.
//!
//! The API traits cover the surface a WU `execute` body can touch:
//! column read/write, accumulator append and read, resource fetch,
//! virtual fire, each/batch/reduce iteration. Each API trait is
//! access-set-parameterised so call sites prove membership via
//! `Holds<...>` at compile time.
//!
//! The `HasX` accessor traits come from `hilavitkutin-ctx`'s
//! `provider_generic!` / `provider_generic2!` macros. They do not
//! emit `Context<P>` delegations: consumers that want sugar wrap
//! the provider tuple in their own newtype.

use arvo::USize;
use hilavitkutin_ctx::{provider_generic, provider_generic2};

use crate::access::{AccessSet, Holds};
use crate::column_value::ColumnValue;
use crate::store::{Accum, Column, Resource, Virtual};
use crate::values::{ReadSet, ReadVals, WriteSet, WriteVals};
use crate::write_at::WriteAt;

// ---------------------------------------------------------------------
// Keyed-resolution bridge traits.
//
// The keyed accessors (`read` / `write` / `resource`) look the store
// type `T` up over the scheduler-generated context's per-WU projected
// pointer bundle. The bundle is a heterogeneous cons-list, so the
// lookup needs a per-element index witness; a lookup keyed on `T` alone
// over a generic cons-list would otherwise require `specialization`.
//
// Instead the accessor carries an inferred index generic `I` and bounds
// `Self` on the matching bridge trait. The context implements each
// bridge by delegating to its bundle's index witness, where `I` selects
// the matching node. At a concrete call site the bundle is concrete, so
// exactly one `I` applies and inference resolves it; `I` never appears
// at the call site. These bridges name no engine types, so the contract
// stays in the api crate.
// ---------------------------------------------------------------------

/// Resolve a borrow of resource `T` at the inferred bundle index `I`.
pub trait ResolveResource<T: 'static, I> {
    /// Borrow the projected resource value of type `T`.
    fn resolve_resource(&self) -> &T;
}

/// Resolve a column read of `T` at the inferred bundle index `I`.
pub trait ResolveColumnRead<T: ColumnValue, I> {
    /// Read the record at index `i` from the projected column `T`.
    ///
    /// # Safety
    ///
    /// Caller must hold the column slot for `T` at `i`. Plan-time DAG
    /// analysis proves this; WU bodies should not re-check.
    unsafe fn resolve_read(&self, i: USize) -> T;
}

/// Resolve a column write of `T` at the inferred bundle index `I`.
pub trait ResolveColumnWrite<T: ColumnValue, I> {
    /// Write `v` to the record at index `i` of the projected column `T`, and
    /// count the store as changed.
    ///
    /// # Safety
    ///
    /// Caller must hold the exclusive writer slot for `T` at `i`.
    unsafe fn resolve_write(&self, i: USize, v: T);

    /// The value the record at index `i` holds, for a member that compares.
    ///
    /// # Safety
    ///
    /// Caller must hold the exclusive writer slot for `T` at `i`.
    unsafe fn resolve_held(&self, i: USize) -> T;
}

/// Resolve an accumulator append of `T` at the inferred bundle index `I`.
///
/// # Safety
///
/// Safe to call and `unsafe` to implement. The implementor holds that the
/// context a unit is handed reaches one thread at a time, that a runner
/// splitting a unit over threads gives each thread a binding of its own, and
/// that each binding is the one appender of its region.
pub unsafe trait ResolveAccumAppend<T: ColumnValue, I> {
    /// Append `v` at the accumulator's live offset, or refuse it.
    ///
    /// Every call is counted as asked. The record is written and the live
    /// length advances only while nothing has been refused through this binding
    /// this frame and the live length is below its capacity; otherwise nothing
    /// is written, and every later append through the binding in the same frame
    /// is refused too. On one thread the accumulator is one binding, so the
    /// records kept are a prefix of the appends asked. Under `run_parallel` each
    /// core appends through its own binding over its own region, so one core
    /// can refuse while another still accepts: the records kept depend on the
    /// worker count, and the `AccumRefused` value does not. The run returns the
    /// first refusing accumulator as an `AccumRefused` through `RunCfg::Out`.
    fn resolve_append(&self, v: T);

    /// The current live length of the projected accumulator `T`.
    fn resolve_len(&self) -> USize;
}

/// Resolve a read of everything appended to accumulator `T` this pass, at
/// the inferred bundle index `I`.
///
/// # Safety
///
/// Safe to call and `unsafe` to implement. The implementor holds that the
/// reader is handed only regions no appender is still writing.
pub unsafe trait ResolveAccumRead<T: ColumnValue, I> {
    /// Call `f` once for every value appended so far in this pass.
    fn resolve_each<F: FnMut(T)>(&self, f: F);
}

/// Apply a value function over the records of the bound morsel.
///
/// The bridge `EachApi::map` resolves through. A runner implements it on
/// its context for the read and write lists it can gather and store: it
/// forms the read product at each record, applies `f`, and stores what `f`
/// returns (a column value at the record, a present accumulator value by an
/// append, a set virtual by firing it).
pub trait ResolveMap<R: ReadSet, W: WriteSet> {
    /// Apply `f` at every record of the morsel and store its results.
    fn resolve_map<F>(&self, f: F)
    where
        F: for<'a> Fn(&ReadVals<'a, R>) -> WriteVals<W>;
}

/// Resolve the fire for virtual `V` through the projected write-virtual bundle
/// at index witness `I`. The virtual-fire analogue of `ResolveAccumAppend`: the
/// engine impl resolves the `V` stamp cell via a type-keyed selector over the
/// bundle (so `I` infers, dodging E0207) and stamps it with the current epoch.
pub trait ResolveVirtualFire<V, I> {
    /// Stamp the projected `Virtual<V>` cell with the current pass epoch.
    fn resolve_fire(&self);
}

/// Read access to columns declared in `R`.
///
/// The `read` method's where-clause enforces that the column type
/// appears in the WU's `Read` set. `&self` receiver prevents LLVM
/// from re-emitting noalias metadata that would reorder writes
/// across fused WUs. `unsafe` pinky-swears the engine proved
/// ownership at plan time.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide column-read API for set `{R}`",
    note = "Implemented by the scheduler-generated context. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `HasColumnReader<R>` for the read set the WU declares."
)]
pub trait ColumnReaderApi<R: AccessSet> {
    /// Read the record at index `i` from a column `T`.
    ///
    /// # Safety
    ///
    /// Caller must hold the column slot for `T` at `i`. Plan-time
    /// DAG analysis proves this; WU bodies should not re-check.
    unsafe fn read<T: ColumnValue, I>(&self, i: USize) -> T
    where
        R: Holds<Column<T>>,
        Self: ResolveColumnRead<T, I>;
}

/// Write access to columns declared in `W`.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide column-write API for set `{W}`",
    note = "Implemented by the scheduler-generated context. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `HasColumnWriter<W>` for the write set the WU declares."
)]
pub trait ColumnWriterApi<W: AccessSet> {
    /// Write `v` to the record at index `i` of column `T`.
    ///
    /// Reaches a column the unit lists bare, under `Written` or under
    /// `Differs`, and does what the member says: under `Differs` the value is
    /// compared with the one the record holds, and an equal one is neither
    /// stored nor counted as a change.
    ///
    /// # Safety
    ///
    /// Caller must hold the exclusive writer slot for `T` at `i`: `i` is
    /// below the record count of the window, and the plan gave this unit
    /// the record.
    #[inline(always)]
    unsafe fn write<T: ColumnValue, I>(&self, i: USize, v: T)
    where
        (W, Self): WriteAt<T, I, Ctx = Self>,
    {
        // SAFETY: the caller's obligation, passed on unchanged.
        unsafe { <(W, Self) as WriteAt<T, I>>::write_at(self, i, v) }
    }
}

/// Append access to accumulators declared in `W`.
///
/// The `append` method's where-clause enforces that the accumulator type
/// appears in the WU's `Write` set. The `&self` receiver advances the live
/// length through interior mutability, so appends never need `&mut`; this
/// keeps LLVM from reordering the write across fused WUs. `len` reads the
/// current live length without mutating.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide accumulator-append API for set `{W}`",
    note = "Implemented by the scheduler-generated context. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `HasAccumWriter<W>` for the write set the WU declares."
)]
pub trait AccumWriterApi<W: AccessSet> {
    /// Append `v` to accumulator `T`, or refuse it past the reserved capacity,
    /// as `ResolveAccumAppend::resolve_append` states; a refusal comes back
    /// from the run, not from this call.
    ///
    /// Drive appends from the per-record loop (`each` / `reduce` / `batch`) so
    /// the total across the frame equals the record count: `execute` runs once
    /// per morsel, so a fixed number of appends issued directly in `execute`
    /// repeats per morsel when the record range spans more than one.
    ///
    /// Safe, and takes no position: an append names no record. It lands at
    /// the accumulator's own live length, which the plumbing keeps.
    #[inline(always)]
    fn append<T: ColumnValue, I>(&self, v: T)
    where
        W: Holds<Accum<T>>,
        Self: ResolveAccumAppend<T, I>,
    {
        self.resolve_append(v)
    }

    /// The live length of the binding the body appends to accumulator `T`
    /// through.
    #[inline(always)]
    fn len<T: ColumnValue, I>(&self) -> USize
    where
        W: Holds<Accum<T>>,
        Self: ResolveAccumAppend<T, I>,
    {
        self.resolve_len()
    }
}

/// Read access to what the pass appended to the accumulators declared in
/// `R`.
///
/// A runner implements it on the context of a maintenance unit and on no
/// other, so an ordinary unit's context cannot name it.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide accumulator-read API for set `{R}`",
    note = "Only the context of a maintenance unit (one scheduled `OnMeta<V>`) reads what a pass appended. An ordinary unit appends, and a unit at the close reads and applies."
)]
pub trait AccumReaderApi<R: AccessSet> {
    /// Call `f` once for every value appended to accumulator `T` in this
    /// pass so far.
    ///
    /// On one thread the values come in the sequence they were appended.
    /// After a parallel run they come region by region, one region for each
    /// core, so their sequence depends on the worker count.
    #[inline(always)]
    fn each<T: ColumnValue, I, F: FnMut(T)>(&self, f: F)
    where
        R: Holds<Accum<T>>,
        Self: ResolveAccumRead<T, I>,
    {
        self.resolve_each(f)
    }
}

/// Shared-resource fetch for resources declared in `R`.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide resource API for set `{R}`",
    note = "Implemented by the scheduler-generated context. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `HasResourceProvider<R>` for the read set the WU declares."
)]
pub trait ResourceProviderApi<R: AccessSet> {
    /// Borrow the resource value of type `T`.
    fn resource<T: 'static, I>(&self) -> &T
    where
        R: Holds<Resource<T>>,
        Self: ResolveResource<T, I>;
}

/// Fire a virtual flag declared in `W`.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide virtual-fire API for set `{W}`",
    note = "Implemented by the scheduler-generated context. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `HasVirtualFirer<W>` for the write set the WU declares."
)]
pub trait VirtualFirerApi<W: AccessSet> {
    /// Fire the virtual `V`. Consumers bound to `On<V>` run this pass (once the
    /// firing producer is ordered before them) or next pass.
    ///
    /// `I` is the index witness for `V` in the projected write-virtual bundle,
    /// inferred at the call site (`ctx.fire::<V>()` needs no second turbofish).
    fn fire<V: 'static, I>(&self)
    where
        W: Holds<Virtual<V>>,
        Self: ResolveVirtualFire<V, I>;
}

/// Per-element loop over the WU's morsel slice.
///
/// Fusible: consecutive `each` calls coalesce into one loop at
/// compile time when morsel boundaries align.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide Each API for sets `{R}` / `{W}`",
    note = "Implemented by the scheduler-generated context. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `HasEach<R, W>` for the read and write sets the WU declares."
)]
pub trait EachApi<R: AccessSet, W: AccessSet> {
    /// Run `f` once per element. Implementors may fuse consecutive
    /// each-calls when the scheduler proves alignment.
    fn run<F>(&self, f: F)
    where
        F: FnMut(USize);

    /// Apply the value function `f` at every record of the morsel.
    ///
    /// `f` takes the values of the read list at one record and returns the
    /// values of the write list at that record. The runner stores what it
    /// returns: a column value at the record, a present accumulator value by
    /// an append, a set virtual by firing it. `f` must be a pure function of
    /// its reads: it may be applied to the records in any sequence and on
    /// any thread.
    #[inline(always)]
    fn map<F>(&self, f: F)
    where
        R: ReadSet,
        W: WriteSet,
        Self: ResolveMap<R, W>,
        F: for<'a> Fn(&ReadVals<'a, R>) -> WriteVals<W>,
    {
        self.resolve_map(f)
    }
}

/// Morsel-slice loop; non-fusible.
///
/// Hands the full slice to `f` in one call. Use when the body
/// processes records in bulk (SIMD, BLAS-style kernels).
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide Batch API for sets `{R}` / `{W}`",
    note = "Implemented by the scheduler-generated context. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `HasBatch<R, W>` for the read and write sets the WU declares."
)]
pub trait BatchApi<R: AccessSet, W: AccessSet> {
    /// Run `f` once with the morsel index range.
    fn run<F>(&self, f: F)
    where
        F: FnMut(USize, USize);
}

/// Reduce over morsel slice with an accumulator.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide Reduce API for sets `{R}` / `{W}`",
    note = "Implemented by the scheduler-generated context. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `HasReduce<R, W>` for the read and write sets the WU declares."
)]
pub trait ReduceApi<R: AccessSet, W: AccessSet> {
    /// Fold `init` with `f` over each index in the morsel slice.
    fn run<A, F>(&self, init: A, f: F) -> A
    where
        A: 'static,
        F: FnMut(A, USize) -> A;
}

// Accessor traits: one per API trait, generated by ctx macros.
// These declare `HasX<...>` as trait aliases that expose the
// provider-tuple entry point.

provider_generic!(<R: AccessSet> ColumnReaderApi as HasColumnReader => reader);
provider_generic!(<W: AccessSet> ColumnWriterApi as HasColumnWriter => writer);
provider_generic!(<W: AccessSet> AccumWriterApi as HasAccumWriter => accums);
// Maintenance contexts only.
provider_generic!(<R: AccessSet> AccumReaderApi as HasAccumReader => appended);
provider_generic!(<R: AccessSet> ResourceProviderApi as HasResourceProvider => resources);
provider_generic!(<W: AccessSet> VirtualFirerApi as HasVirtualFirer => virtuals);
provider_generic2!(<R: AccessSet, W: AccessSet> EachApi as HasEach => each);
provider_generic2!(<R: AccessSet, W: AccessSet> BatchApi as HasBatch => batch);
provider_generic2!(<R: AccessSet, W: AccessSet> ReduceApi as HasReduce => reduce);
