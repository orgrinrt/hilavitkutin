//! Unified columnar storage contract.
//!
//! All engine runtime state reaches one storage floor: a column of
//! column-sized scalars. `ColumnStorage` owns those columns over a
//! `MemoryProviderApi`; `Decompose` maps an aggregate type to the
//! scalar column leaves it occupies. `Resource`, `Column`, `Virtual`,
//! `Field`, `Seq`, and `Map` are access-and-intent views over this one
//! store, not independent or arena-backed mechanisms.
//!
//! This is the contract surface (domain 07). The arena-backed default
//! impl ships separately; bespoke per-subsystem arenas are forbidden.

use arvo::USize;
use notko::Outcome;

use crate::column_value::ColumnValue;
use crate::id::StoreId;
use crate::store::StoreBundle;

/// Owning contract for columns of `ColumnValue` scalars.
///
/// Sits between `MemoryProviderApi` (the raw allocator the consumer
/// supplies) below and the per-morsel Context accessors
/// (`ColumnReaderApi` / `ColumnWriterApi`) above. It owns column memory
/// and hands out base pointers the dispatch codegen turns into
/// per-morsel raw reads.
///
/// Access is by raw pointer, not slice: a slice borrow taken alongside
/// a resource pointer from the same context defeats LLVM's `noalias`
/// and forces a reload every iteration (the T6 bench, 1.28 to 1.40x
/// overhead; resolution R6). An impl reserves each column 64-byte
/// aligned with type-native stride (`size_of::<T>()` per column, no
/// universal stride).
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not implement the ColumnStorage contract",
    note = "ColumnStorage owns columns of ColumnValue scalars over a MemoryProvider. Use the arena-backed default impl, or supply your own; bespoke per-subsystem arenas are forbidden (see the unified-storage principle)."
)]
pub trait ColumnStorage {
    /// Failure type returned by `reserve` (provider exhausted, column
    /// not reserved, length overflow).
    type Error;

    /// Reserve column `id` for `len` records of scalar `T`.
    ///
    /// Allocates through the bound `MemoryProvider`, 64-byte aligned,
    /// stride `size_of::<T>()`. Re-reserving an existing `id` resizes
    /// it. Returns `Outcome::Err` when the backing allocation fails.
    fn reserve<T: ColumnValue>(&mut self, id: StoreId, len: USize) -> Outcome<(), Self::Error>;

    /// Base pointer of column `id`. Stride is `size_of::<T>()`.
    ///
    /// Raw, not a slice (R6). The pointer is valid for `count(id)`
    /// records until a matching `release` or a resizing `reserve`.
    ///
    /// # Safety
    ///
    /// `id` must name a column reserved for `T`; the caller proves the
    /// record index stays below `count(id)`.
    unsafe fn column_ptr<T: ColumnValue>(&self, id: StoreId) -> *const T;

    /// Mutable base pointer of column `id`. See [`column_ptr`].
    ///
    /// # Safety
    ///
    /// Same obligations as [`column_ptr`], plus the caller proves no
    /// aliasing read pointer to the same column is live.
    ///
    /// [`column_ptr`]: ColumnStorage::column_ptr
    unsafe fn column_ptr_mut<T: ColumnValue>(&self, id: StoreId) -> *mut T;

    /// Record count of column `id`.
    fn count(&self, id: StoreId) -> USize;

    /// Advisory release of column `id`.
    ///
    /// Reader-count model: an impl decrements on fiber completion and
    /// frees the column at zero. A no-op on the naive placeholder; the
    /// columnar engine reclaims arena space.
    fn release(&mut self, id: StoreId);
}

/// Maps an aggregate type to its scalar column leaves.
///
/// The struct-of-arrays expansion: a multi-field struct yields one leaf
/// per field, recursing through nested structs until every leaf is a
/// `ColumnValue` scalar. `Field<S>` is the base case the recursion
/// stops at, so scalar promotion is preserved as the fundamental. A
/// `Seq` or `Map` element expands the same way when it is not itself a
/// scalar.
///
/// Trait-only here; the `#[derive(Decompose)]` that computes `Leaves`
/// from a struct's fields is the deferred ergonomic shape.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not implement Decompose",
    note = "Decompose maps an aggregate to its scalar column leaves (a cons-list of Field<S>). Hand-write the impl, or derive it once the derive ships."
)]
pub trait Decompose {
    /// Cons-list of this aggregate's scalar column leaves: a `Field<S>`
    /// per scalar leaf, in field order. `Empty` for a unit aggregate.
    type Leaves: StoreBundle;
}
