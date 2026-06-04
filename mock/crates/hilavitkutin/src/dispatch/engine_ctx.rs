//! Per-WorkUnit projected Context (B3).
//!
//! `EngineCtx<'frame, R, W>` is the value a WU's `execute` body
//! touches. It carries only the projected pointers for the stores the
//! WU declares in `R` (read) and `W` (write), enforcing the access
//! scope physically: a WU cannot reach an undeclared store because its
//! Context does not hold that pointer.
//!
//! The projection reuses a frunk-style index witness. A `Selector<T,
//! Index>` resolves a type-keyed lookup over a heterogeneous cons-list
//! without specialization: the two impls (head-match and tail-recurse)
//! are keyed on distinct `Index` types (`Here` / `There<I>`), so they
//! never overlap, and the index infers at the call site. `Project<R,
//! Indices>` carries a parallel `Indices` cons-list so each element
//! index is a trait type parameter (constrained by "this trait is
//! implemented"), dodging E0207; the free `project_reads::<R, _, _>`
//! helper pins `R` by turbofish while inference fills the index list.
//!
//! Resources project out of the scheduler bindings (`ResourceBinding`,
//! real storage from B2a). Columns project out of a per-frame column
//! pointer bundle passed in at construction, because the B2a bindings
//! column nodes are dangling placeholders (column buffers are sized by
//! the per-run record count and belong to the run-loop / plan phase).
//!
//! Accessors take `&self`, never `&mut self`, so LLVM does not reorder
//! writes across fused WUs. The unsafe read / write aliasing obligation
//! is the scheduler's: plan-time DAG analysis proves no concurrent
//! write-overlap, and WU bodies do not re-check.

use core::marker::PhantomData;

use arvo::USize;
use hilavitkutin_api::access::{AccessSet, Cons, Contains, Empty};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::context::{
    BatchApi, ColumnReaderApi, ColumnWriterApi, EachApi, ReduceApi, ResolveColumnRead,
    ResolveColumnWrite, ResolveResource, ResourceProviderApi, VirtualFirerApi,
};
use hilavitkutin_api::store::{Column, Resource, Virtual};

use crate::dispatch::morsel::MorselRange;
use crate::resource::bindings::ResourceBinding;
use crate::resource::provenance::{ColumnPtr, ResourcePtr};

// ---------------------------------------------------------------------
// Index witnesses.
//
// `Here` and `There<I>` are the disjoint index types that key the two
// `Selector` impls. Because they are distinct concrete types, the
// head-match impl (`Here`) and the tail-recurse impl (`There<I>`) never
// overlap, so the lookup compiles without specialization.
// ---------------------------------------------------------------------

/// Index witness: the matching node is the head of the list.
pub struct Here;

/// Index witness: the matching node is `I` steps into the tail.
pub struct There<I>(PhantomData<I>);

// ---------------------------------------------------------------------
// Projected pointer bundles.
//
// `PtrCons` / `PtrNil` carry the projected `ResourcePtr<T>` for the
// resource members of `R`. `ColPtrCons` / `ColPtrNil` carry the
// projected `ColumnPtr<T>` for the column members of `R` union `W`.
// Two distinct cons-list shapes keep resource and column provenance
// classes separate.
// ---------------------------------------------------------------------

/// Empty resource pointer bundle (tail leaf).
pub struct PtrNil;

/// One projected resource pointer `head` of type `ResourcePtr<H>`,
/// followed by the rest, `tail`.
pub struct PtrCons<H, Tail> {
    pub(crate) head: ResourcePtr<H>,
    pub(crate) tail: Tail,
}

impl<H, Tail> PtrCons<H, Tail> {
    /// Construct a resource bundle node. Hidden test accessor: the
    /// run-loop builds the resource bundle via `project_reads`; tests may
    /// build it by hand. Not part of the supported surface.
    #[doc(hidden)]
    #[inline]
    pub fn __new(head: ResourcePtr<H>, tail: Tail) -> Self {
        Self { head, tail }
    }
}

/// Empty column pointer bundle (tail leaf).
pub struct ColPtrNil;

/// One projected column pointer `head` of type `ColumnPtr<H>`,
/// followed by the rest, `tail`.
pub struct ColPtrCons<H, Tail> {
    pub(crate) head: ColumnPtr<H>,
    pub(crate) tail: Tail,
}

impl<H, Tail> ColPtrCons<H, Tail> {
    /// Construct a column bundle node. Hidden test/run-loop accessor: the
    /// run-loop builds the column bundle from per-frame buffers; tests
    /// build it by hand. Not part of the supported surface.
    #[doc(hidden)]
    #[inline]
    pub fn __new(head: ColumnPtr<H>, tail: Tail) -> Self {
        Self { head, tail }
    }
}

// ---------------------------------------------------------------------
// Resource selector: type-keyed lookup over bindings nodes and over the
// projected resource bundle.
// ---------------------------------------------------------------------

/// Type-keyed lookup yielding the `ResourcePtr<T>` for `T` in the list.
///
/// `Index` is `Here` (the head matches) or `There<I>` (recurse `I`
/// steps into the tail). The two impls never overlap because the
/// indices are distinct types.
pub trait Selector<T, Index> {
    /// The recorded resource pointer for `T`.
    fn get(&self) -> ResourcePtr<T>;
}

// Over the bindings nodes (resources project from the real B2a bindings).

impl<T, Tail> Selector<T, Here> for ResourceBinding<T, Tail> {
    #[inline(always)]
    fn get(&self) -> ResourcePtr<T> {
        self.__ptr()
    }
}

impl<T, U, Tail, I> Selector<T, There<I>> for ResourceBinding<U, Tail>
where
    Tail: Selector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> ResourcePtr<T> {
        self.__tail().get()
    }
}

// Over the projected resource bundle (`PtrCons` / `PtrNil`).

impl<T, Tail> Selector<T, Here> for PtrCons<T, Tail> {
    #[inline(always)]
    fn get(&self) -> ResourcePtr<T> {
        self.head
    }
}

impl<T, U, Tail, I> Selector<T, There<I>> for PtrCons<U, Tail>
where
    Tail: Selector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> ResourcePtr<T> {
        self.tail.get()
    }
}

// ---------------------------------------------------------------------
// Column selector: type-keyed lookup over the projected column bundle.
// ---------------------------------------------------------------------

/// Type-keyed lookup yielding the `ColumnPtr<T>` for `T` in the bundle.
pub trait ColSelector<T, Index> {
    /// The recorded column pointer for `T`.
    fn get(&self) -> ColumnPtr<T>;
}

impl<T, Tail> ColSelector<T, Here> for ColPtrCons<T, Tail> {
    #[inline(always)]
    fn get(&self) -> ColumnPtr<T> {
        self.head
    }
}

impl<T, U, Tail, I> ColSelector<T, There<I>> for ColPtrCons<U, Tail>
where
    Tail: ColSelector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> ColumnPtr<T> {
        self.tail.get()
    }
}

// ---------------------------------------------------------------------
// Project: build the projected resource bundle from the bindings.
//
// `Project<R, Indices>` recurses on the `Resource<T>` members of the
// access set `R`, pulling each matching `ResourcePtr<T>` out of the
// bindings via `Selector`. `Indices` is a parallel cons-list whose
// elements are the per-member selector indices; carrying it as a trait
// type parameter constrains each index (dodging E0207).
//
// `Column<T>` and `Virtual<T>` members of `R` produce no resource-
// bundle node here: only the resource members contribute. The free
// `project_reads::<R, _, _>(bindings)` helper pins `R` by turbofish.
// ---------------------------------------------------------------------

/// Project the `Resource<T>` members of `R` out of a source `A` into a
/// resource pointer bundle.
pub trait Project<R, Indices> {
    /// The projected resource bundle.
    type Out;

    /// Build the projected bundle by pulling each resource pointer.
    fn project(&self) -> Self::Out;
}

impl<A> Project<Empty, Empty> for A {
    type Out = PtrNil;

    #[inline(always)]
    fn project(&self) -> PtrNil {
        PtrNil
    }
}

// Resource head: pull the pointer, recurse on the tail.
impl<A, T, I, RTail, ITail> Project<Cons<Resource<T>, RTail>, Cons<I, ITail>> for A
where
    A: Selector<T, I>,
    A: Project<RTail, ITail>,
{
    type Out = PtrCons<T, <A as Project<RTail, ITail>>::Out>;

    #[inline(always)]
    fn project(&self) -> Self::Out {
        PtrCons {
            head: <A as Selector<T, I>>::get(self),
            tail: <A as Project<RTail, ITail>>::project(self),
        }
    }
}

// Column head: no resource node, recurse on the tail with the same
// index list (columns do not consume a resource selector index).
impl<A, T, RTail, Indices> Project<Cons<Column<T>, RTail>, Indices> for A
where
    A: Project<RTail, Indices>,
{
    type Out = <A as Project<RTail, Indices>>::Out;

    #[inline(always)]
    fn project(&self) -> Self::Out {
        <A as Project<RTail, Indices>>::project(self)
    }
}

// Virtual head: no resource node, recurse on the tail.
impl<A, T, RTail, Indices> Project<Cons<Virtual<T>, RTail>, Indices> for A
where
    A: Project<RTail, Indices>,
{
    type Out = <A as Project<RTail, Indices>>::Out;

    #[inline(always)]
    fn project(&self) -> Self::Out {
        <A as Project<RTail, Indices>>::project(self)
    }
}

/// Project the resource members of `R` out of `bindings` into a bundle.
///
/// Pins `R` by turbofish at the call site; inference fills the parallel
/// `Indices` list and the source type `A`.
#[inline(always)]
pub fn project_reads<R, A, Indices>(bindings: &A) -> <A as Project<R, Indices>>::Out
where
    A: Project<R, Indices>,
{
    bindings.project()
}

// ---------------------------------------------------------------------
// ColProject: build the projected column bundle from a column source.
//
// `ColProject<Set, Indices>` recurses on the `Column<T>` members of an
// access set `Set`, pulling each matching `ColumnPtr<T>` out of a column
// source via `ColSelector`. `Indices` is a parallel cons-list whose
// elements are the per-member selector indices; carrying it as a trait
// type parameter constrains each index (dodging E0207). This is the
// column analogue of `Project`: it forces the projected column bundle to
// be the projection of the access set over the supplied source, so a
// caller cannot hand a mismatched bundle.
//
// `Resource<T>` and `Virtual<T>` members produce no column-bundle node:
// only the column members contribute.
// ---------------------------------------------------------------------

/// Project the `Column<T>` members of `Set` out of a column source `C`
/// into a column pointer bundle.
pub trait ColProject<Set, Indices> {
    /// The projected column bundle.
    type Out;

    /// Build the projected bundle by pulling each column pointer.
    fn col_project(&self) -> Self::Out;
}

impl<C> ColProject<Empty, Empty> for C {
    type Out = ColPtrNil;

    #[inline(always)]
    fn col_project(&self) -> ColPtrNil {
        ColPtrNil
    }
}

// Column head: pull the pointer, recurse on the tail.
impl<C, T, I, STail, ITail> ColProject<Cons<Column<T>, STail>, Cons<I, ITail>> for C
where
    C: ColSelector<T, I>,
    C: ColProject<STail, ITail>,
{
    type Out = ColPtrCons<T, <C as ColProject<STail, ITail>>::Out>;

    #[inline(always)]
    fn col_project(&self) -> Self::Out {
        ColPtrCons {
            head: <C as ColSelector<T, I>>::get(self),
            tail: <C as ColProject<STail, ITail>>::col_project(self),
        }
    }
}

// Resource head: no column node, recurse on the tail with the same
// index list (resources do not consume a column selector index).
impl<C, T, STail, Indices> ColProject<Cons<Resource<T>, STail>, Indices> for C
where
    C: ColProject<STail, Indices>,
{
    type Out = <C as ColProject<STail, Indices>>::Out;

    #[inline(always)]
    fn col_project(&self) -> Self::Out {
        <C as ColProject<STail, Indices>>::col_project(self)
    }
}

// Virtual head: no column node, recurse on the tail.
impl<C, T, STail, Indices> ColProject<Cons<Virtual<T>, STail>, Indices> for C
where
    C: ColProject<STail, Indices>,
{
    type Out = <C as ColProject<STail, Indices>>::Out;

    #[inline(always)]
    fn col_project(&self) -> Self::Out {
        <C as ColProject<STail, Indices>>::col_project(self)
    }
}

// ---------------------------------------------------------------------
// EngineCtx: the per-WU projected Context.
// ---------------------------------------------------------------------

/// Per-WorkUnit projected Context.
///
/// Holds only the projected resource and column pointers for the
/// stores the WU declares, plus the morsel range it iterates. `'frame`
/// ties the borrowed pointers to the scheduler-owned storage that lives
/// for the dispatch frame. `R` is the WU's read set, `W` its write set.
///
/// The Context is its own provider for every accessor: the seven `HasX`
/// traits resolve `type Provider = Self`.
pub struct EngineCtx<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols> {
    reads: RBundle,
    read_cols: RCols,
    write_cols: WCols,
    morsel: MorselRange,
    _frame: PhantomData<&'frame ()>,
    _sets: PhantomData<(R, W)>,
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols>
    EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    /// Construct a Context from pre-built projected bundles.
    ///
    /// Crate-internal only. The bundle types are caller-chosen here, so
    /// this constructor does not by itself prove that the bundles are the
    /// projection of `R` / `W`. The public `project` constructor derives
    /// them from the access sets and is the only way an external caller
    /// builds a Context; this internal entry exists so `project` (and the
    /// run-loop) can assemble the value once the tie is established. Never
    /// make this `pub`: a `pub` bundle-taking constructor would let a
    /// caller pair a non-empty access set with a mismatched bundle,
    /// satisfying the `Contains` proof while resolving through an
    /// unrelated bundle and hitting the nil base-case panic.
    #[inline]
    pub(crate) fn from_projected(
        reads: RBundle,
        read_cols: RCols,
        write_cols: WCols,
        morsel: MorselRange,
    ) -> Self {
        Self {
            reads,
            read_cols,
            write_cols,
            morsel,
            _frame: PhantomData,
            _sets: PhantomData,
        }
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols>
    EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    /// Project a Context from the scheduler bindings and a per-frame column
    /// source.
    ///
    /// This is the public constructor. The projected bundles are not
    /// caller-chosen: `RBundle` is forced to be the resource projection of
    /// `R` over the `bindings`; `RCols` is the column projection of `R` and
    /// `WCols` the column projection of `W`, both over the `cols` source. A
    /// caller therefore cannot pair a non-empty access set with an empty or
    /// mismatched source; the `Project` / `ColProject` bounds are
    /// unsatisfiable when the source lacks a declared store, so the
    /// construction fails at compile time rather than panicking at the nil
    /// base case during a later accessor call.
    ///
    /// Columns are projected per side, not over `R union W`: the read
    /// accessor resolves over `RCols` (the columns in `R`), the write
    /// accessor over `WCols` (the columns in `W`). A column that is both
    /// read and written appears once in each bundle, both pulling the same
    /// pointer from the `cols` source. Projecting per side keeps each
    /// bundle free of duplicate column types, so the type-keyed index
    /// witness resolves uniquely; a single `R union W` bundle would list a
    /// read-write column twice and make the inferred index ambiguous.
    ///
    /// The projection tie is enforced by the type system. A Read set
    /// containing `Resource<u32>` cannot be projected from an empty
    /// source: the `Project` bound is unsatisfiable, so the construction
    /// is rejected at compile time rather than reaching the nil
    /// base-case panic during a later accessor call.
    ///
    /// ```compile_fail
    /// use hilavitkutin::dispatch::engine_ctx::{EngineCtx, PtrNil, ColPtrNil};
    /// use hilavitkutin::dispatch::morsel::MorselRange;
    /// use hilavitkutin_api::access::{Cons, Empty};
    /// use hilavitkutin_api::store::Resource;
    /// use arvo::USize;
    ///
    /// type ReadU32 = Cons<Resource<u32>, Empty>;
    ///
    /// // The Read set declares `Resource<u32>`, but the resource source
    /// // is empty (`PtrNil`). `PtrNil: Project<ReadU32, _>` does not
    /// // hold (no `Selector<u32, _>` on `PtrNil`), so this does not
    /// // compile.
    /// let _ctx: EngineCtx<'_, ReadU32, Empty, _, _, _> =
    ///     EngineCtx::project(&PtrNil, &ColPtrNil, MorselRange::new(USize::ZERO, USize::ZERO));
    /// ```
    #[inline]
    pub fn project<A, C, RIdx, RCIdx, WCIdx>(
        bindings: &'frame A,
        cols: &C,
        morsel: MorselRange,
    ) -> EngineCtx<'frame, R, W, RBundle, RCols, WCols>
    where
        A: Project<R, RIdx, Out = RBundle>,
        C: ColProject<R, RCIdx, Out = RCols>,
        C: ColProject<W, WCIdx, Out = WCols>,
    {
        let reads = <A as Project<R, RIdx>>::project(bindings);
        let read_cols = <C as ColProject<R, RCIdx>>::col_project(cols);
        let write_cols = <C as ColProject<W, WCIdx>>::col_project(cols);
        EngineCtx::from_projected(reads, read_cols, write_cols, morsel)
    }
}

// ResourceProviderApi: resolve `&T` via the resource bundle Selector.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols> ResourceProviderApi<R>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    #[inline]
    fn resource<T: 'static, I>(&self) -> &T
    where
        R: Contains<Resource<T>>,
        Self: ResolveResource<T, I>,
    {
        <Self as ResolveResource<T, I>>::resolve_resource(self)
    }
}

// ResolveResource: resolve the `ResourcePtr<T>` through the projected
// resource bundle's `Selector<T, I>` witness, then borrow. `I` is the
// per-`T` bundle index, inferred at the concrete WU call site (the bundle
// is a concrete cons-list there, so exactly one index applies). This is
// the spec-free replacement for the old type-equality `fetch<T>`.
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, T: 'static, I> ResolveResource<T, I>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
where
    RBundle: Selector<T, I>,
{
    #[inline]
    fn resolve_resource(&self) -> &T {
        let ptr = <RBundle as Selector<T, I>>::get(&self.reads);
        // SAFETY: the projected bundle holds a `ResourcePtr<T>` at the
        // witnessed index `I` only because `R: Contains<Resource<T>>`
        // placed it there at projection time. The pointer was written to
        // scheduler-owned storage that lives for `'frame`; the returned
        // `&T` is tied to `&self`, which cannot outlive `'frame`.
        // Read-only access; the scheduler's plan-time analysis proves no
        // concurrent write.
        unsafe { &*ptr.as_ptr() }
    }
}

// ColumnReaderApi: resolve the column pointer, read at the morsel
// offset. B3 treats the column buffer as `[T]`-shaped at stride
// `size_of::<T>()`; sub-byte bitpacking is a later round.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols> ColumnReaderApi<R>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    #[inline]
    unsafe fn read<T: ColumnValue, I>(&self, i: USize) -> T
    where
        R: Contains<Column<T>>,
        Self: ResolveColumnRead<T, I>,
    {
        // SAFETY: forwarded to the bridge; the caller's obligation (the
        // engine proved slot ownership at plan time) carries through.
        unsafe { <Self as ResolveColumnRead<T, I>>::resolve_read(self, i) }
    }
}

// ResolveColumnRead: resolve the `ColumnPtr<T>` through the projected
// column bundle's `ColSelector<T, I>` witness, read at the morsel offset.
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, T: ColumnValue, I> ResolveColumnRead<T, I>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
where
    RCols: ColSelector<T, I>,
{
    #[inline]
    unsafe fn resolve_read(&self, i: USize) -> T {
        let ptr = <RCols as ColSelector<T, I>>::get(&self.read_cols);
        let idx = USize(self.morsel.start.0 + i.0);
        // B3 treats the column buffer as `[T]`-shaped at stride
        // `size_of::<T>()`; sub-byte bitpacking (using `T::BIT_WIDTH`)
        // is a later round.
        // SAFETY: the column bundle holds a `ColumnPtr<T>` at the
        // witnessed index `I` only because `R: Contains<Column<T>>`
        // placed it there. The caller (the engine, via plan-time DAG
        // analysis) guarantees the slot at `idx` is initialised and the
        // buffer is at least `start + len` records long. Valid for
        // `'frame`.
        unsafe { core::ptr::read(ptr.as_ptr().add(idx.0)) }
    }
}

// ColumnWriterApi: resolve the column pointer, write at the morsel
// offset. Same stride simplification as the reader.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols> ColumnWriterApi<W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    #[inline]
    unsafe fn write<T: ColumnValue, I>(&self, i: USize, v: T)
    where
        W: Contains<Column<T>>,
        Self: ResolveColumnWrite<T, I>,
    {
        // SAFETY: forwarded to the bridge; the caller's obligation (the
        // engine proved exclusive-writer ownership at plan time) carries
        // through.
        unsafe { <Self as ResolveColumnWrite<T, I>>::resolve_write(self, i, v) }
    }
}

// ResolveColumnWrite: resolve the `ColumnPtr<T>` through the projected
// column bundle's `ColSelector<T, I>` witness, write at the morsel offset.
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, T: ColumnValue, I> ResolveColumnWrite<T, I>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
where
    WCols: ColSelector<T, I>,
{
    #[inline]
    unsafe fn resolve_write(&self, i: USize, v: T) {
        let ptr = <WCols as ColSelector<T, I>>::get(&self.write_cols);
        let idx = USize(self.morsel.start.0 + i.0);
        // B3 treats the column buffer as `[T]`-shaped at stride
        // `size_of::<T>()`; sub-byte bitpacking (using `T::BIT_WIDTH`)
        // is a later round.
        // SAFETY: the column bundle holds a `ColumnPtr<T>` at the
        // witnessed index `I` only because `W: Contains<Column<T>>`
        // placed it there. The engine's plan-time DAG analysis proves
        // this WU holds the exclusive writer slot for `T` at `idx`; no
        // concurrent reader or writer aliases it. `&self` (not
        // `&mut self`) keeps LLVM from reordering the write across fused
        // WUs. Valid for `'frame`.
        unsafe { core::ptr::write(ptr.as_ptr().add(idx.0), v) }
    }
}

// VirtualFirerApi: B3 no-op. DAG-edge firing semantics land with the
// run-loop.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols> VirtualFirerApi<W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    #[inline]
    fn fire<V: 'static>(&self)
    where
        W: Contains<Virtual<V>>,
    {
        // B3 no-op: the virtual is declared in `W` (the where-clause
        // proves it), but the DAG-edge firing that schedules `On<V>`
        // consumers next pass lands with the run-loop.
    }
}

// EachApi: per-record loop over the morsel.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols> EachApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    #[inline]
    fn run<F>(&self, mut f: F)
    where
        F: FnMut(USize),
    {
        let mut i = self.morsel.start;
        let end = self.morsel.end();
        while i.0 < end.0 {
            f(i);
            i = USize(i.0 + 1);
        }
    }
}

// BatchApi: one call with the full morsel range.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols> BatchApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    #[inline]
    fn run<F>(&self, mut f: F)
    where
        F: FnMut(USize, USize),
    {
        f(self.morsel.start, self.morsel.end());
    }
}

// ReduceApi: fold over the morsel range.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols> ReduceApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    #[inline]
    fn run<A, F>(&self, init: A, mut f: F) -> A
    where
        A: 'static,
        F: FnMut(A, USize) -> A,
    {
        let mut acc = init;
        let mut i = self.morsel.start;
        let end = self.morsel.end();
        while i.0 < end.0 {
            acc = f(acc, i);
            i = USize(i.0 + 1);
        }
        acc
    }
}

// ---------------------------------------------------------------------
// HasX accessor impls: the Context is its own provider for every
// accessor (`type Provider = Self`). The seven `HasX` traits come from
// `hilavitkutin-api`'s `provider_generic!` / `provider_generic2!`
// macros; we satisfy them directly here.
// ---------------------------------------------------------------------

use hilavitkutin_api::context::{
    HasBatch, HasColumnReader, HasColumnWriter, HasEach, HasReduce, HasResourceProvider,
    HasVirtualFirer,
};

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols>
    HasColumnReader<R> for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn reader(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols>
    HasColumnWriter<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn writer(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols>
    HasResourceProvider<R> for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn resources(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols>
    HasVirtualFirer<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn virtuals(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols>
    HasEach<R, W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn each(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols>
    HasBatch<R, W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn batch(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols>
    HasReduce<R, W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn reduce(&self) -> &Self::Provider {
        self
    }
}
