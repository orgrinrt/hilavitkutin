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
//! Resources project out of the scheduler arena (`ArenaResourceNode`,
//! real storage from B2a). Columns project out of a per-frame column
//! pointer bundle passed in at construction, because the B2a arena
//! column nodes are dangling placeholders (column buffers are sized by
//! the per-run record count and belong to the run-loop / plan phase).
//!
//! Accessors take `&self`, never `&mut self`, so LLVM does not reorder
//! writes across fused WUs. The unsafe read / write aliasing obligation
//! is the scheduler's: plan-time DAG analysis proves no concurrent
//! write-overlap, and WU bodies do not re-check.

use core::marker::PhantomData;

use arvo::USize;
use hilavitkutin_api::access::{AccessSet, Concat, Cons, Contains, Empty};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::context::{
    BatchApi, ColumnReaderApi, ColumnWriterApi, EachApi, ReduceApi, ResourceProviderApi,
    VirtualFirerApi,
};
use hilavitkutin_api::store::{Column, Resource, Virtual};

use crate::dispatch::morsel::MorselRange;
use crate::resource::arena::ArenaResourceNode;
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
// Resource selector: type-keyed lookup over arena nodes and over the
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

// Over the arena nodes (resources project from the real B2a arena).

impl<T, Tail> Selector<T, Here> for ArenaResourceNode<T, Tail> {
    #[inline(always)]
    fn get(&self) -> ResourcePtr<T> {
        self.__ptr()
    }
}

impl<T, U, Tail, I> Selector<T, There<I>> for ArenaResourceNode<U, Tail>
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
// Project: build the projected resource bundle from the arena.
//
// `Project<R, Indices>` recurses on the `Resource<T>` members of the
// access set `R`, pulling each matching `ResourcePtr<T>` out of the
// arena via `Selector`. `Indices` is a parallel cons-list whose
// elements are the per-member selector indices; carrying it as a trait
// type parameter constrains each index (dodging E0207).
//
// `Column<T>` and `Virtual<T>` members of `R` produce no resource-
// bundle node here: only the resource members contribute. The free
// `project_reads::<R, _, _>(arena)` helper pins `R` by turbofish.
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

/// Project the resource members of `R` out of `arena` into a bundle.
///
/// Pins `R` by turbofish at the call site; inference fills the parallel
/// `Indices` list and the source type `A`.
#[inline(always)]
pub fn project_reads<R, A, Indices>(arena: &A) -> <A as Project<R, Indices>>::Out
where
    A: Project<R, Indices>,
{
    arena.project()
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
pub struct EngineCtx<'frame, R: AccessSet, W: AccessSet, RBundle, WCols> {
    reads: RBundle,
    cols: WCols,
    morsel: MorselRange,
    _frame: PhantomData<&'frame ()>,
    _sets: PhantomData<(R, W)>,
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, WCols>
    EngineCtx<'frame, R, W, RBundle, WCols>
{
    /// Construct a Context from pre-built projected bundles.
    ///
    /// Crate-internal only. The bundle types are caller-chosen here, so
    /// this constructor does not by itself prove that `RBundle` / `WCols`
    /// are the projection of `R` / `W`. The public `project` constructor
    /// derives the bundles from the access sets and is the only way an
    /// external caller builds a Context; this internal entry exists so
    /// `project` (and the run-loop) can assemble the value once the tie
    /// is established. Never make this `pub`: a `pub` bundle-taking
    /// constructor would let a caller pair a non-empty access set with a
    /// mismatched bundle, satisfying the `Contains` proof while resolving
    /// through an unrelated bundle and hitting the nil base-case panic.
    #[inline]
    pub(crate) fn from_projected(reads: RBundle, cols: WCols, morsel: MorselRange) -> Self {
        Self {
            reads,
            cols,
            morsel,
            _frame: PhantomData,
            _sets: PhantomData,
        }
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, WCols>
    EngineCtx<'frame, R, W, RBundle, WCols>
{
    /// Project a Context from the scheduler arena and a per-frame column
    /// source.
    ///
    /// This is the public constructor. The projected bundles are not
    /// caller-chosen: `RBundle` is forced to be the resource projection
    /// of `R` over the `arena`, and `WCols` is forced to be the column
    /// projection of `R` union `W` over the `cols` source. A caller
    /// therefore cannot pair a non-empty access set with an empty or
    /// mismatched source; the `Project` / `ColProject` bounds are
    /// unsatisfiable when the source lacks a declared store, so the
    /// construction fails at compile time rather than panicking at the
    /// nil base case during a later accessor call.
    ///
    /// The resource bundle projects out of the scheduler arena (real
    /// B2a storage). The column bundle projects out of the per-frame
    /// column source supplied by the run-loop (or by hand in tests),
    /// since column buffers are sized by the per-run record count and
    /// are not part of the build-time arena. Columns are projected over
    /// `R union W` because a column may be read (in `R`), written (in
    /// `W`), or both, and the bundle must hold a pointer for every such
    /// column.
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
    /// let _ctx: EngineCtx<'_, ReadU32, Empty, _, _> =
    ///     EngineCtx::project(&PtrNil, &ColPtrNil, MorselRange::new(USize::ZERO, USize::ZERO));
    /// ```
    #[inline]
    pub fn project<A, C, RIdx, CIdx>(
        arena: &'frame A,
        cols: &C,
        morsel: MorselRange,
    ) -> EngineCtx<'frame, R, W, RBundle, WCols>
    where
        R: Concat<W>,
        A: Project<R, RIdx, Out = RBundle>,
        C: ColProject<<R as Concat<W>>::Out, CIdx, Out = WCols>,
    {
        let reads = <A as Project<R, RIdx>>::project(arena);
        let projected_cols = <C as ColProject<<R as Concat<W>>::Out, CIdx>>::col_project(cols);
        EngineCtx::from_projected(reads, projected_cols, morsel)
    }
}

// ResourceProviderApi: resolve `&T` via the resource bundle Selector.

impl<'frame, R: AccessSet, W: AccessSet, RBundle: ResourceBundle, WCols> ResourceProviderApi<R>
    for EngineCtx<'frame, R, W, RBundle, WCols>
{
    #[inline]
    fn resource<T: 'static>(&self) -> &T
    where
        R: Contains<Resource<T>>,
    {
        let ptr = self.reads.fetch::<T>();
        // SAFETY: the projected bundle holds a `ResourcePtr<T>` only
        // because `R: Contains<Resource<T>>` placed it there at
        // projection time. The pointer was written to scheduler-owned
        // storage that lives for `'frame`; the returned `&T` is tied to
        // `&self`, which cannot outlive `'frame`. Read-only access; the
        // scheduler's plan-time analysis proves no concurrent write.
        unsafe { &*ptr.as_ptr() }
    }
}

// ColumnReaderApi: resolve the column pointer, read at the morsel
// offset. B3 treats the column buffer as `[T]`-shaped at stride
// `size_of::<T>()`; sub-byte bitpacking is a later round.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, WCols: ColumnBundle> ColumnReaderApi<R>
    for EngineCtx<'frame, R, W, RBundle, WCols>
{
    #[inline]
    unsafe fn read<T: ColumnValue>(&self, i: USize) -> T
    where
        R: Contains<Column<T>>,
    {
        let ptr = self.cols.fetch::<T>();
        let idx = USize(self.morsel.start.0 + i.0);
        // B3 treats the column buffer as `[T]`-shaped at stride
        // `size_of::<T>()`; sub-byte bitpacking (using `T::BIT_WIDTH`)
        // is a later round.
        // SAFETY: the column bundle holds a `ColumnPtr<T>` only because
        // `R: Contains<Column<T>>` placed it there. The caller (the
        // engine, via plan-time DAG analysis) guarantees the slot at
        // `idx` is initialised and the buffer is at least
        // `start + len` records long. The pointer is valid for `'frame`.
        unsafe { core::ptr::read(ptr.as_ptr().add(idx.0)) }
    }
}

// ColumnWriterApi: resolve the column pointer, write at the morsel
// offset. Same stride simplification as the reader.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, WCols: ColumnBundle> ColumnWriterApi<W>
    for EngineCtx<'frame, R, W, RBundle, WCols>
{
    #[inline]
    unsafe fn write<T: ColumnValue>(&self, i: USize, v: T)
    where
        W: Contains<Column<T>>,
    {
        let ptr = self.cols.fetch::<T>();
        let idx = USize(self.morsel.start.0 + i.0);
        // B3 treats the column buffer as `[T]`-shaped at stride
        // `size_of::<T>()`; sub-byte bitpacking (using `T::BIT_WIDTH`)
        // is a later round.
        // SAFETY: the column bundle holds a `ColumnPtr<T>` only because
        // `W: Contains<Column<T>>` placed it there. The engine's
        // plan-time DAG analysis proves this WU holds the exclusive
        // writer slot for `T` at `idx`; no concurrent reader or writer
        // aliases it. `&self` (not `&mut self`) keeps LLVM from
        // reordering the write across fused WUs. Valid for `'frame`.
        unsafe { core::ptr::write(ptr.as_ptr().add(idx.0), v) }
    }
}

// VirtualFirerApi: B3 no-op. DAG-edge firing semantics land with the
// run-loop.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, WCols> VirtualFirerApi<W>
    for EngineCtx<'frame, R, W, RBundle, WCols>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle, WCols> EachApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, WCols>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle, WCols> BatchApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, WCols>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle, WCols> ReduceApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, WCols>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle: ResourceBundle, WCols: ColumnBundle>
    HasColumnReader<R> for EngineCtx<'frame, R, W, RBundle, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn reader(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle: ResourceBundle, WCols: ColumnBundle>
    HasColumnWriter<W> for EngineCtx<'frame, R, W, RBundle, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn writer(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle: ResourceBundle, WCols: ColumnBundle>
    HasResourceProvider<R> for EngineCtx<'frame, R, W, RBundle, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn resources(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle: ResourceBundle, WCols: ColumnBundle>
    HasVirtualFirer<W> for EngineCtx<'frame, R, W, RBundle, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn virtuals(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle: ResourceBundle, WCols: ColumnBundle>
    HasEach<R, W> for EngineCtx<'frame, R, W, RBundle, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn each(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle: ResourceBundle, WCols: ColumnBundle>
    HasBatch<R, W> for EngineCtx<'frame, R, W, RBundle, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn batch(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle: ResourceBundle, WCols: ColumnBundle>
    HasReduce<R, W> for EngineCtx<'frame, R, W, RBundle, WCols>
{
    type Provider = Self;
    #[inline(always)]
    fn reduce(&self) -> &Self::Provider {
        self
    }
}
// ---------------------------------------------------------------------
// Type-keyed resolution over the projected bundle.
//
// The accessor methods (`resource<T>`, `read<T>`, `write<T>`) are fixed
// by the `hilavitkutin-api` traits: not generic over a selector index,
// so they cannot name the witness `Selector<T, Index>` the projection
// uses. They need a lookup keyed on `T` alone, callable with only a
// `ResourceBundle` / `ColumnBundle` bound on the bundle (no per-`T` impl
// bound, which is not expressible).
//
// `TryHead<T>` is the equality test: a default impl returns `Isnt` for
// any head type; a specialising impl returns `Is(head)` when the head IS
// `T`. `feature(specialization)` resolves the overlap by type equality.
// The bundle's `fetch<T>` tries the head, and on `Isnt` recurses the
// tail. Totality over the cons chain plus the nil terminal makes
// `fetch<T>` callable for every `T: 'static`; the accessor's
// `Contains<...>` where-clause is what gates an undeclared store at
// compile time, so the nil panic arm is dead for any admitted `T`.
// ---------------------------------------------------------------------

/// Head-equality test for a projected resource node, keyed on `T`.
pub trait TryHeadResource<T> {
    /// `Is(ptr)` when this node's head is `ResourcePtr<T>`, else `Isnt`.
    fn try_head(&self) -> notko::Maybe<ResourcePtr<T>>;
}

// Default: head is some other type, no match.
impl<T: 'static, H: 'static, Tail> TryHeadResource<T> for PtrCons<H, Tail> {
    #[inline(always)]
    default fn try_head(&self) -> notko::Maybe<ResourcePtr<T>> {
        notko::Maybe::Isnt
    }
}

// Specialisation: head IS `ResourcePtr<T>`.
impl<T: 'static, Tail> TryHeadResource<T> for PtrCons<T, Tail> {
    #[inline(always)]
    fn try_head(&self) -> notko::Maybe<ResourcePtr<T>> {
        notko::Maybe::Is(self.head)
    }
}

/// Head-equality test for a projected column node, keyed on `T`.
pub trait TryHeadColumn<T> {
    /// `Is(ptr)` when this node's head is `ColumnPtr<T>`, else `Isnt`.
    fn try_head(&self) -> notko::Maybe<ColumnPtr<T>>;
}

impl<T: 'static, H: 'static, Tail> TryHeadColumn<T> for ColPtrCons<H, Tail> {
    #[inline(always)]
    default fn try_head(&self) -> notko::Maybe<ColumnPtr<T>> {
        notko::Maybe::Isnt
    }
}

impl<T: 'static, Tail> TryHeadColumn<T> for ColPtrCons<T, Tail> {
    #[inline(always)]
    fn try_head(&self) -> notko::Maybe<ColumnPtr<T>> {
        notko::Maybe::Is(self.head)
    }
}

/// A projected resource pointer bundle (`PtrCons` chain over `PtrNil`).
///
/// `fetch<T>` resolves the recorded `ResourcePtr<T>` by trying the head
/// (via `TryHeadResource`, an equality test broken by specialisation)
/// and recursing the tail on a miss. The accessor calls it with only a
/// `ResourceBundle` bound; the `Contains<Resource<T>>` proof on the
/// accessor guarantees the requested `T` is present, so the nil panic
/// arm is dead.
pub trait ResourceBundle {
    /// Fetch the recorded `ResourcePtr<T>` for a contained `T`.
    fn fetch<T: 'static>(&self) -> ResourcePtr<T>;
}

impl ResourceBundle for PtrNil {
    #[inline(always)]
    fn fetch<T: 'static>(&self) -> ResourcePtr<T> {
        // Genuinely unreachable. The only public constructor (`project`)
        // forces this bundle to be `<A as Project<R, _>>::Out`, the
        // resource projection of the access set `R`. The accessor's
        // `R: Contains<Resource<T>>` proof then guarantees `R` carries a
        // `Resource<T>` member, so the projection placed a `PtrCons<T,
        // _>` node before this nil leaf and `fetch::<T>` matches it on
        // the head-try walk. There is no `pub` path to a Context whose
        // bundle is not the projection of its access set.
        unreachable!("resource bundle has no node for the requested type")
    }
}

impl<H: 'static, Tail: ResourceBundle> ResourceBundle for PtrCons<H, Tail> {
    #[inline(always)]
    fn fetch<T: 'static>(&self) -> ResourcePtr<T> {
        match <Self as TryHeadResource<T>>::try_head(self) {
            notko::Maybe::Is(ptr) => ptr,
            notko::Maybe::Isnt => self.tail.fetch::<T>(),
        }
    }
}

/// A projected column pointer bundle (`ColPtrCons` chain over
/// `ColPtrNil`).
///
/// Same head-try / tail-recurse shape as `ResourceBundle`, for column
/// pointers.
pub trait ColumnBundle {
    /// Fetch the recorded `ColumnPtr<T>` for a contained `T`.
    fn fetch<T: 'static>(&self) -> ColumnPtr<T>;
}

impl ColumnBundle for ColPtrNil {
    #[inline(always)]
    fn fetch<T: 'static>(&self) -> ColumnPtr<T> {
        // Genuinely unreachable. The only public constructor (`project`)
        // forces this bundle to be the column projection of `R union W`
        // over the column source. A column accessor proves `R:
        // Contains<Column<T>>` (read) or `W: Contains<Column<T>>`
        // (write); either way `Column<T>` is a member of `R union W`, so
        // the projection placed a `ColPtrCons<T, _>` node before this nil
        // leaf. There is no `pub` path to a Context whose column bundle
        // is not the projection of its access set.
        unreachable!("column bundle has no node for the requested type")
    }
}

impl<H: 'static, Tail: ColumnBundle> ColumnBundle for ColPtrCons<H, Tail> {
    #[inline(always)]
    fn fetch<T: 'static>(&self) -> ColumnPtr<T> {
        match <Self as TryHeadColumn<T>>::try_head(self) {
            notko::Maybe::Is(ptr) => ptr,
            notko::Maybe::Isnt => self.tail.fetch::<T>(),
        }
    }
}
