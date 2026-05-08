//! Provider API and accessor traits for the WU context.
//!
//! Seven API traits cover the surface a WU `execute` body can touch:
//! column read/write, resource fetch, virtual fire, each/batch/reduce
//! iteration. Each API trait is access-set-parameterised so call
//! sites prove membership via `Contains<...>` at compile time.
//!
//! The `HasX` accessor traits come from `hilavitkutin-ctx`'s
//! `provider_generic!` / `provider_generic2!` macros. They do not
//! emit `Context<P>` delegations: consumers that want sugar wrap
//! the provider tuple in their own newtype.

use arvo::USize;
use hilavitkutin_ctx::{provider_generic, provider_generic2};

use crate::access::{AccessSet, Contains};
use crate::column_value::ColumnValue;
use crate::store::{Column, Resource, Virtual};

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
    unsafe fn read<T: ColumnValue>(&self, i: USize) -> T
    where
        R: Contains<Column<T>>;
}

/// Write access to columns declared in `W`.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide column-write API for set `{W}`",
    note = "Implemented by the scheduler-generated context. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `HasColumnWriter<W>` for the write set the WU declares."
)]
pub trait ColumnWriterApi<W: AccessSet> {
    /// Write `v` to the record at index `i` of column `T`.
    ///
    /// # Safety
    ///
    /// Caller must hold the exclusive writer slot for `T` at `i`.
    unsafe fn write<T: ColumnValue>(&self, i: USize, v: T)
    where
        W: Contains<Column<T>>;
}

/// Shared-resource fetch for resources declared in `R`.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide resource API for set `{R}`",
    note = "Implemented by the scheduler-generated context. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `HasResourceProvider<R>` for the read set the WU declares."
)]
pub trait ResourceProviderApi<R: AccessSet> {
    /// Borrow the resource value of type `T`.
    fn resource<T: 'static>(&self) -> &T
    where
        R: Contains<Resource<T>>;
}

/// Fire a virtual flag declared in `W`.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not provide virtual-fire API for set `{W}`",
    note = "Implemented by the scheduler-generated context. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `HasVirtualFirer<W>` for the write set the WU declares."
)]
pub trait VirtualFirerApi<W: AccessSet> {
    /// Fire the virtual `V`. Consumers bound to `On<V>` run next pass.
    fn fire<V: 'static>(&self)
    where
        W: Contains<Virtual<V>>;
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
provider_generic!(<R: AccessSet> ResourceProviderApi as HasResourceProvider => resources);
provider_generic!(<W: AccessSet> VirtualFirerApi as HasVirtualFirer => virtuals);
provider_generic2!(<R: AccessSet, W: AccessSet> EachApi as HasEach => each);
provider_generic2!(<R: AccessSet, W: AccessSet> BatchApi as HasBatch => batch);
provider_generic2!(<R: AccessSet, W: AccessSet> ReduceApi as HasReduce => reduce);
