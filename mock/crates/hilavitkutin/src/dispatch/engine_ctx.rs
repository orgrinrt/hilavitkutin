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

use core::cell::Cell;
use core::marker::PhantomData;

use arvo::strategy::Identity;
use arvo::USize;
use hilavitkutin_api::access::{AccessSet, Cons, Contains, Empty};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::context::{
    AccumWriterApi, BatchApi, ColumnReaderApi, ColumnWriterApi, EachApi, ReduceApi,
    ResolveAccumAppend, ResolveColumnRead, ResolveColumnWrite, ResolveResource, ResourceProviderApi,
    VirtualFirerApi,
};
use hilavitkutin_api::store::{Accum, Column, Resource, Virtual};

use crate::dispatch::morsel::MorselRange;
use crate::resource::bindings::{AccumBinding, ColumnBinding, ResourceBinding, VirtualBinding};
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
// Projected accumulator node + bundle.
//
// `AccumColPtr<'frame, T>` is the projected accumulator handle: the capacity
// buffer base (a `Copy` `ColumnPtr<T>`) plus a `'frame` borrow of the live
// length cell. Unlike `ResourcePtr` / `ColumnPtr` (copied by value with no
// retained borrow), the accumulator handle holds the borrow, so the append
// accessor can advance the live length under `&self`. The borrow makes the
// whole bundle lifetime-tied; that is why the accumulator projection runs over
// the `'frame` bindings source (not the shorter-lived column source).
//
// `AccPtrNil` / `AccPtrCons` carry the projected `AccumColPtr<'frame, T>` for
// the accumulator members of `W`, a distinct cons-list shape from the resource
// (`PtrCons`) and column (`ColPtrCons`) bundles.
// ---------------------------------------------------------------------

/// Projected accumulator handle: capacity base, borrowed live-length cell, and
/// the reserved capacity the append saturates at.
pub struct AccumColPtr<'frame, T> {
    base: ColumnPtr<T>,
    len: &'frame Cell<USize>,
    cap: USize,
}

impl<'frame, T> Copy for AccumColPtr<'frame, T> {}
impl<'frame, T> Clone for AccumColPtr<'frame, T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

/// Empty accumulator pointer bundle (tail leaf).
pub struct AccPtrNil;

/// One projected accumulator handle `head` of type `AccumColPtr<'frame, H>`,
/// followed by the rest, `tail`.
pub struct AccPtrCons<'frame, H, Tail> {
    pub(crate) head: AccumColPtr<'frame, H>,
    pub(crate) tail: Tail,
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

// Pass-through over column and virtual nodes: a resource declared after a
// column (or virtual) in the registration order is reachable by recursing
// the tail. Without these, `Selector` traversed only resource nodes, so a
// resource behind a column was unreachable (the resource-after-column gap).

impl<T, U, Tail, I> Selector<T, There<I>> for ColumnBinding<U, Tail>
where
    Tail: Selector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> ResourcePtr<T> {
        self.__tail().get()
    }
}

impl<T, U, Tail, I> Selector<T, There<I>> for VirtualBinding<U, Tail>
where
    Tail: Selector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> ResourcePtr<T> {
        self.__tail().get()
    }
}

impl<T, U, Tail, I> Selector<T, There<I>> for AccumBinding<U, Tail>
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

// Over the scheduler bindings nodes (Shape A): the same bindings cons-list
// that resolves resources via `Selector` resolves columns via `ColSelector`,
// so one witness list into the one `ColumnStorage` serves both. `Here`
// matches a `ColumnBinding<T, _>`; `There<I>` recurses the tail over any node
// kind (a column behind a resource, column, or virtual node).

impl<T, Tail> ColSelector<T, Here> for ColumnBinding<T, Tail> {
    #[inline(always)]
    fn get(&self) -> ColumnPtr<T> {
        self.__ptr()
    }
}

impl<T, U, Tail, I> ColSelector<T, There<I>> for ColumnBinding<U, Tail>
where
    Tail: ColSelector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> ColumnPtr<T> {
        self.__tail().get()
    }
}

impl<T, U, Tail, I> ColSelector<T, There<I>> for ResourceBinding<U, Tail>
where
    Tail: ColSelector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> ColumnPtr<T> {
        self.__tail().get()
    }
}

impl<T, U, Tail, I> ColSelector<T, There<I>> for VirtualBinding<U, Tail>
where
    Tail: ColSelector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> ColumnPtr<T> {
        self.__tail().get()
    }
}

impl<T, U, Tail, I> ColSelector<T, There<I>> for AccumBinding<U, Tail>
where
    Tail: ColSelector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> ColumnPtr<T> {
        self.__tail().get()
    }
}

// ---------------------------------------------------------------------
// Accumulator selector: type-keyed lookup over bindings nodes and over
// the projected accumulator bundle.
//
// `AccumSelector<T, Index>` yields the `AccumColPtr<'_, T>` for `T`, the
// handle whose borrowed live-length cell the append accessor advances. `Here`
// matches an `AccumBinding<T, _>` (or the projected `AccPtrCons<_, T, _>`
// head); `There<I>` recurses the tail over any node kind, so an accumulator
// declared after a resource, column, virtual, or another accumulator resolves.
// The returned handle's lifetime ties to `&self`, threading the binding borrow
// down the tail.
// ---------------------------------------------------------------------

/// Type-keyed lookup yielding the `AccumColPtr<'_, T>` for `T` in the list.
pub trait AccumSelector<T, Index> {
    /// The projected accumulator handle for `T`, borrowing `self`.
    fn get(&self) -> AccumColPtr<'_, T>;
}

// Over the scheduler bindings nodes: `Here` matches an `AccumBinding<T, _>`,
// reading its base pointer and borrowing its live-length cell.

impl<T, Tail> AccumSelector<T, Here> for AccumBinding<T, Tail> {
    #[inline(always)]
    fn get(&self) -> AccumColPtr<'_, T> {
        AccumColPtr {
            base: self.__ptr(),
            len: self.__len_cell(),
            cap: self.__cap(),
        }
    }
}

impl<T, U, Tail, I> AccumSelector<T, There<I>> for AccumBinding<U, Tail>
where
    Tail: AccumSelector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> AccumColPtr<'_, T> {
        self.__tail().get()
    }
}

impl<T, U, Tail, I> AccumSelector<T, There<I>> for ResourceBinding<U, Tail>
where
    Tail: AccumSelector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> AccumColPtr<'_, T> {
        self.__tail().get()
    }
}

impl<T, U, Tail, I> AccumSelector<T, There<I>> for ColumnBinding<U, Tail>
where
    Tail: AccumSelector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> AccumColPtr<'_, T> {
        self.__tail().get()
    }
}

impl<T, U, Tail, I> AccumSelector<T, There<I>> for VirtualBinding<U, Tail>
where
    Tail: AccumSelector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> AccumColPtr<'_, T> {
        self.__tail().get()
    }
}

// Over the projected accumulator bundle (`AccPtrCons` / `AccPtrNil`).

impl<'f, T, Tail> AccumSelector<T, Here> for AccPtrCons<'f, T, Tail> {
    #[inline(always)]
    fn get(&self) -> AccumColPtr<'_, T> {
        self.head
    }
}

impl<'f, T, U, Tail, I> AccumSelector<T, There<I>> for AccPtrCons<'f, U, Tail>
where
    Tail: AccumSelector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> AccumColPtr<'_, T> {
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

// Accum head: no resource node, recurse on the tail.
impl<A, T, RTail, Indices> Project<Cons<Accum<T>, RTail>, Indices> for A
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

// Accum head: no column node, recurse on the tail (accumulators project
// through `AccumProject`, not `ColProject`).
impl<C, T, STail, Indices> ColProject<Cons<Accum<T>, STail>, Indices> for C
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
// AccumProject: build the projected accumulator bundle from the bindings.
//
// `AccumProject<'s, Set, Indices>` recurses on the `Accum<T>` members of an
// access set `Set`, pulling each matching `AccumColPtr<'s, T>` out of a source
// via `AccumSelector`. The accumulator analogue of `ColProject`, with one
// difference: the projected bundle retains a `'s` borrow of the source (the
// live-length cells), so the trait carries the lifetime `'s` and projects via
// `&'s self`. Carrying `'s` at the trait (rather than a GAT `Out<'s>`) keeps
// `Out` a plain associated type, so the `project` constructor can tie it to
// the Context's `WAccum` parameter through an `Out = WAccum` equality bound;
// the de-risk sketch used a free function that named the GAT directly, which a
// method on the `WAccum`-generic `EngineCtx` cannot.
//
// `Indices` is a parallel cons-list of per-member selector indices, carried as
// a trait type parameter to constrain each index (dodging E0207). `Resource<T>`
// / `Column<T>` / `Virtual<T>` members produce no accumulator-bundle node.
// ---------------------------------------------------------------------

/// Project the `Accum<T>` members of `Set` out of a source `C` into an
/// accumulator pointer bundle borrowing `C` for `'s`.
pub trait AccumProject<'s, Set, Indices> {
    /// The projected accumulator bundle (borrows the source for `'s`).
    type Out;

    /// Build the projected bundle by pulling each accumulator handle.
    fn acc_project(&'s self) -> Self::Out;
}

impl<'s, C> AccumProject<'s, Empty, Empty> for C {
    type Out = AccPtrNil;

    #[inline(always)]
    fn acc_project(&'s self) -> AccPtrNil {
        AccPtrNil
    }
}

// Accum head: pull the handle, recurse on the tail.
impl<'s, C, T, I, STail, ITail> AccumProject<'s, Cons<Accum<T>, STail>, Cons<I, ITail>> for C
where
    C: AccumSelector<T, I>,
    C: AccumProject<'s, STail, ITail>,
{
    type Out = AccPtrCons<'s, T, <C as AccumProject<'s, STail, ITail>>::Out>;

    #[inline(always)]
    fn acc_project(&'s self) -> Self::Out {
        AccPtrCons {
            head: <C as AccumSelector<T, I>>::get(self),
            tail: <C as AccumProject<'s, STail, ITail>>::acc_project(self),
        }
    }
}

// Resource head: no accumulator node, recurse on the tail with the same
// index list (resources do not consume an accumulator selector index).
impl<'s, C, T, STail, Indices> AccumProject<'s, Cons<Resource<T>, STail>, Indices> for C
where
    C: AccumProject<'s, STail, Indices>,
{
    type Out = <C as AccumProject<'s, STail, Indices>>::Out;

    #[inline(always)]
    fn acc_project(&'s self) -> Self::Out {
        <C as AccumProject<'s, STail, Indices>>::acc_project(self)
    }
}

// Column head: no accumulator node, recurse on the tail.
impl<'s, C, T, STail, Indices> AccumProject<'s, Cons<Column<T>, STail>, Indices> for C
where
    C: AccumProject<'s, STail, Indices>,
{
    type Out = <C as AccumProject<'s, STail, Indices>>::Out;

    #[inline(always)]
    fn acc_project(&'s self) -> Self::Out {
        <C as AccumProject<'s, STail, Indices>>::acc_project(self)
    }
}

// Virtual head: no accumulator node, recurse on the tail.
impl<'s, C, T, STail, Indices> AccumProject<'s, Cons<Virtual<T>, STail>, Indices> for C
where
    C: AccumProject<'s, STail, Indices>,
{
    type Out = <C as AccumProject<'s, STail, Indices>>::Out;

    #[inline(always)]
    fn acc_project(&'s self) -> Self::Out {
        <C as AccumProject<'s, STail, Indices>>::acc_project(self)
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
/// The Context is its own provider for every accessor: the eight `HasX`
/// traits resolve `type Provider = Self`.
///
/// `WAccum` (the projected write-set accumulator bundle) defaults to
/// `AccPtrNil`, the empty accumulator bundle. Accumulators are an opt-in, so
/// the default keeps an accum-free WU's `Ctx` declaration at the six prior
/// bundle params; an accum-bearing WU spells the seventh explicitly. The
/// default never masks a mismatch: `project` and `CollectFiber` force `WAccum`
/// to the real projection of `W`, so a WU that declares an accumulator but
/// omits the bundle fails to compile at the projection tie.
pub struct EngineCtx<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum = AccPtrNil> {
    reads: RBundle,
    read_cols: RCols,
    write_cols: WCols,
    write_accums: WAccum,
    morsel: MorselRange,
    _frame: PhantomData<&'frame ()>,
    _sets: PhantomData<(R, W)>,
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum>
    EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
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
        write_accums: WAccum,
        morsel: MorselRange,
    ) -> Self {
        Self {
            reads,
            read_cols,
            write_cols,
            write_accums,
            morsel,
            _frame: PhantomData,
            _sets: PhantomData,
        }
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum>
    EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
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
    /// The write-set accumulator bundle `WAccum` is projected from the
    /// `bindings` source, not `cols`: an accumulator handle retains a
    /// `'frame` borrow of the binding's live-length cell, so it ties to the
    /// `'frame`-lived bindings, while the column source can stay shorter. The
    /// `AccumProject<'frame, W, WAIdx, Out = WAccum>` bound forces `WAccum` to
    /// be the real projection of `W`, so the Context's accumulator parameter
    /// cannot be mismatched.
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
    pub fn project<A, C, RIdx, RCIdx, WCIdx, WAIdx>(
        bindings: &'frame A,
        cols: &C,
        morsel: MorselRange,
    ) -> EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
    where
        A: Project<R, RIdx, Out = RBundle>,
        C: ColProject<R, RCIdx, Out = RCols>,
        C: ColProject<W, WCIdx, Out = WCols>,
        A: AccumProject<'frame, W, WAIdx, Out = WAccum>,
    {
        let reads = <A as Project<R, RIdx>>::project(bindings);
        let read_cols = <C as ColProject<R, RCIdx>>::col_project(cols);
        let write_cols = <C as ColProject<W, WCIdx>>::col_project(cols);
        // The accumulator bundle projects from the `'frame` bindings (it
        // retains a borrow of each live-length cell), not the column source.
        let write_accums = <A as AccumProject<'frame, W, WAIdx>>::acc_project(bindings);
        EngineCtx::from_projected(reads, read_cols, write_cols, write_accums, morsel)
    }
}

// ResourceProviderApi: resolve `&T` via the resource bundle Selector.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum> ResourceProviderApi<R>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
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
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, T: 'static, I> ResolveResource<T, I>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum> ColumnReaderApi<R>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
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
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, T: ColumnValue, I> ResolveColumnRead<T, I>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum> ColumnWriterApi<W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
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
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, T: ColumnValue, I> ResolveColumnWrite<T, I>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
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

// AccumWriterApi: resolve the accumulator handle, append at the live offset,
// advance the live length. The append is a self-relative grow, not a
// morsel-indexed write: the offset is the accumulator's own live count, not
// `morsel.start + i`.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum> AccumWriterApi<W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    #[inline]
    unsafe fn append<T: ColumnValue, I>(&self, v: T)
    where
        W: Contains<Accum<T>>,
        Self: ResolveAccumAppend<T, I>,
    {
        // SAFETY: forwarded to the bridge; the caller's obligation (the engine
        // proved exclusive-appender ownership at plan time, and the live length
        // is within the reserved capacity) carries through.
        unsafe { <Self as ResolveAccumAppend<T, I>>::resolve_append(self, v) }
    }

    #[inline]
    fn len<T: ColumnValue, I>(&self) -> USize
    where
        W: Contains<Accum<T>>,
        Self: ResolveAccumAppend<T, I>,
    {
        <Self as ResolveAccumAppend<T, I>>::resolve_len(self)
    }
}

// ResolveAccumAppend: resolve the `AccumColPtr<T>` through the projected
// accumulator bundle's `AccumSelector<T, I>` witness, write at the live offset,
// advance the live length through the borrowed `Cell`.
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, T: ColumnValue, I>
    ResolveAccumAppend<T, I> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
where
    WAccum: AccumSelector<T, I>,
{
    #[inline]
    unsafe fn resolve_append(&self, v: T) {
        let acc = <WAccum as AccumSelector<T, I>>::get(&self.write_accums);
        let live = acc.len.get();
        // Saturate at the reserved capacity: once the live length reaches the
        // capacity the append is dropped, so the write and the advance never
        // run past the reserved buffer. A WorkUnit's appends are not bounded by
        // the plan the way a column write's morsel index is, so this checked
        // bound is the soundness guard.
        if live.0 >= acc.cap.0 {
            return;
        }
        // B3 treats the capacity buffer as `[T]`-shaped at stride
        // `size_of::<T>()`; sub-byte bitpacking is a later round.
        // SAFETY: the accumulator bundle holds an `AccumColPtr<T>` at the
        // witnessed index `I` only because `W: Contains<Accum<T>>` placed it
        // there. The engine's plan-time DAG analysis proves this WU holds the
        // exclusive appender slot for `T`; no concurrent appender aliases it.
        // The capacity check above keeps `live` strictly within the reserved
        // record count, so the write lands in the buffer. `&self` (not
        // `&mut self`) keeps LLVM from reordering the write across fused WUs.
        // Valid for `'frame`.
        unsafe { core::ptr::write(acc.base.as_ptr().add(live.0), v) };
        acc.len.set(USize(live.0 + 1));
    }

    #[inline]
    fn resolve_len(&self) -> USize {
        let acc = <WAccum as AccumSelector<T, I>>::get(&self.write_accums);
        acc.len.get()
    }
}

// VirtualFirerApi: B3 no-op. DAG-edge firing semantics land with the
// run-loop.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum> VirtualFirerApi<W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
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

// EachApi: per-record loop yielding a morsel-relative index `[0, len)`.
// `read` / `write` add `morsel.start` to recover the absolute column index,
// so the body works for any morsel start.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum> EachApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    #[inline]
    fn run<F>(&self, mut f: F)
    where
        F: FnMut(USize),
    {
        let mut i = USize::ZERO;
        let len = self.morsel.len;
        while i.0 < len.0 {
            f(i);
            i = USize(i.0 + 1);
        }
    }
}

// BatchApi: one call with the morsel-relative half-open range `[0, len)`.
// A body looping that range and calling `write(i)` lands at `morsel.start + i`.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum> BatchApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    #[inline]
    fn run<F>(&self, mut f: F)
    where
        F: FnMut(USize, USize),
    {
        f(USize::ZERO, self.morsel.len);
    }
}

// ReduceApi: fold yielding a morsel-relative index `[0, len)`, matching
// `EachApi`. `read` / `write` add `morsel.start` for the absolute index.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum> ReduceApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    #[inline]
    fn run<A, F>(&self, init: A, mut f: F) -> A
    where
        A: 'static,
        F: FnMut(A, USize) -> A,
    {
        let mut acc = init;
        let mut i = USize::ZERO;
        let len = self.morsel.len;
        while i.0 < len.0 {
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
    HasAccumWriter, HasBatch, HasColumnReader, HasColumnWriter, HasEach, HasReduce,
    HasResourceProvider, HasVirtualFirer,
};

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum>
    HasColumnReader<R> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    type Provider = Self;
    #[inline(always)]
    fn reader(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum>
    HasColumnWriter<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    type Provider = Self;
    #[inline(always)]
    fn writer(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum>
    HasResourceProvider<R> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    type Provider = Self;
    #[inline(always)]
    fn resources(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum>
    HasVirtualFirer<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    type Provider = Self;
    #[inline(always)]
    fn virtuals(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum>
    HasEach<R, W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    type Provider = Self;
    #[inline(always)]
    fn each(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum>
    HasBatch<R, W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    type Provider = Self;
    #[inline(always)]
    fn batch(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum>
    HasReduce<R, W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    type Provider = Self;
    #[inline(always)]
    fn reduce(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum>
    HasAccumWriter<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum>
{
    type Provider = Self;
    #[inline(always)]
    fn accums(&self) -> &Self::Provider {
        self
    }
}
