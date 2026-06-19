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
use arvo::{Bool, USize};
use hilavitkutin_api::{Always, On, OnMeta};
use hilavitkutin_api::access::{AccessSet, Cons, Contains, Empty};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::context::{
    AccumWriterApi, BatchApi, ColumnReaderApi, ColumnWriterApi, EachApi, ReduceApi,
    ResolveAccumAppend, ResolveColumnRead, ResolveColumnWrite, ResolveResource, ResolveVirtualFire,
    ResourceProviderApi, VirtualFirerApi,
};
use hilavitkutin_api::meta::MetaAccess;
use hilavitkutin_api::store::{Accum, Column, Resource, Virtual};

use crate::dispatch::morsel::MorselRange;
use crate::meta::{MetaBlock, MetaField};
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
/// the reserved capacity the append asserts the live length against (a
/// contract-violating over-append panics before the write).
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
// E4 slice 1: virtual firing.
//
// `VirtNil` / `VirtCons` carry the projected `&'frame Cell<USize>` stamp ref for
// each `Virtual<T>` member of the WU's write set `W`, the firing analogue of the
// accumulator bundle (`AccPtrCons`). `fire<V>` sets the resolved cell to the
// current pass epoch; the `On<V>` consumer's trunk-gate reads the SAME cell from
// the full bindings and gate-opens when `stamp == epoch`. The two reach the cell
// by identity (both resolve through the same `VirtualBinding<T>` nodes), so no
// global virtual index is needed. Proven by sketch
// `202606081800_e4-gate-firer-trait-shapes`.
// ---------------------------------------------------------------------

/// Empty write-virtual bundle (tail leaf).
pub struct VirtNil;

/// One projected stamp-cell ref `head` for `Virtual<H>`, followed by `tail`.
pub struct VirtCons<'frame, H, Tail> {
    pub(crate) head: &'frame Cell<USize>,
    pub(crate) tail: Tail,
    pub(crate) _h: PhantomData<H>,
}

/// Type-keyed lookup yielding the `&Cell<USize>` stamp for `Virtual<V>` in the
/// bindings list. The shared keying primitive: both the firer (via the projected
/// `VirtCons` bundle) and the trunk-gate (via the full bindings) resolve the same
/// cell through this. Mirrors `AccumSelector<T, Index>`: `Here` matches a
/// `VirtualBinding<V, _>` head, `There<I>` recurses any node kind, index infers.
pub trait VirtualStampSelector<V, Index> {
    /// The stamp cell for `Virtual<V>`, borrowing `self`.
    fn vstamp(&self) -> &Cell<USize>;
}

impl<V, Tail> VirtualStampSelector<V, Here> for VirtualBinding<V, Tail> {
    #[inline(always)]
    fn vstamp(&self) -> &Cell<USize> {
        self.__stamp_cell()
    }
}

impl<V, U, Tail, I> VirtualStampSelector<V, There<I>> for VirtualBinding<U, Tail>
where
    Tail: VirtualStampSelector<V, I>,
{
    #[inline(always)]
    fn vstamp(&self) -> &Cell<USize> {
        self.__tail().vstamp()
    }
}

impl<V, U, Tail, I> VirtualStampSelector<V, There<I>> for ResourceBinding<U, Tail>
where
    Tail: VirtualStampSelector<V, I>,
{
    #[inline(always)]
    fn vstamp(&self) -> &Cell<USize> {
        self.__tail().vstamp()
    }
}

impl<V, U, Tail, I> VirtualStampSelector<V, There<I>> for ColumnBinding<U, Tail>
where
    Tail: VirtualStampSelector<V, I>,
{
    #[inline(always)]
    fn vstamp(&self) -> &Cell<USize> {
        self.__tail().vstamp()
    }
}

impl<V, U, Tail, I> VirtualStampSelector<V, There<I>> for AccumBinding<U, Tail>
where
    Tail: VirtualStampSelector<V, I>,
{
    #[inline(always)]
    fn vstamp(&self) -> &Cell<USize> {
        self.__tail().vstamp()
    }
}

/// Per-member schedule gate (E4 slice 1): does this unit's schedule open this
/// pass, given the full bindings and the current epoch?
///
/// Dispatched on the unit's `<W as HasSchedule>::Sched`. `Always` pins `GI =
/// Here` and returns `true` (const-foldable, so the gate DCE's away for every
/// existing Always WU, preserving the devirt / ASM properties). `On<V>` resolves
/// the `Virtual<V>` stamp cell from the FULL carrier bindings `A` via the shared
/// `VirtualStampSelector` (the same cell the firer stamps through its projected
/// bundle, so a fire is observed here) and opens when `stamp == epoch`.
///
/// `GI` is the bindings-side selector index for `V`, carried in the per-unit
/// `RunFiber` witness tuple (a constrained position alongside the projection
/// indices), so neither impl trips E0207 and `GI` infers with them. Proven by
/// sketch `202606081800_e4-gate-firer-trait-shapes`.
pub trait GateWith<A, GI> {
    /// True if this schedule opens this pass.
    fn open(bindings: &A, epoch: USize) -> Bool;
}

impl<A> GateWith<A, Here> for Always {
    #[inline(always)]
    fn open(_bindings: &A, _epoch: USize) -> Bool {
        Bool::TRUE
    }
}

impl<A, V, GI> GateWith<A, GI> for On<V>
where
    A: VirtualStampSelector<V, GI>,
{
    #[inline(always)]
    fn open(bindings: &A, epoch: USize) -> Bool {
        // lint:allow(no-bare-numeric) reason: epoch-stamp equality compare; tracked: #121
        Bool(<A as VirtualStampSelector<V, GI>>::vstamp(bindings).get().0 == epoch.0)
    }
}

// E4 slice 2: a meta work unit's `OnMeta<V>` gate is const-open, like `Always`.
// The lifecycle conditional (PlanStage runs only on a plan-dirty frame) is not a
// per-unit stamp gate; it is the kernel's phase-band skip in `dispatch_trunks`,
// which simply does not dispatch the plan band's phases on a clean frame. So when
// an `OnMeta<V>` unit's phase IS dispatched, it always runs. GI is `Here` (no
// bindings read), so it infers exactly as `Always` does in the witness tuple.
impl<A, V> GateWith<A, Here> for OnMeta<V> {
    #[inline(always)]
    fn open(_bindings: &A, _epoch: USize) -> Bool {
        Bool::TRUE
    }
}

/// Type-keyed fire over the projected write-virtual bundle: set the stamp cell
/// for `Virtual<T>` to `epoch`. Mirrors `AccumSelector` over the projected
/// accumulator bundle; index infers at the `fire<V>` call site.
pub trait VirtualFire<T, Index> {
    /// Set the stamp for `Virtual<T>` to `epoch`.
    fn fire(&self, epoch: USize);
}

impl<'f, T, Tail> VirtualFire<T, Here> for VirtCons<'f, T, Tail> {
    #[inline(always)]
    fn fire(&self, epoch: USize) {
        self.head.set(epoch);
    }
}

impl<'f, T, U, Tail, I> VirtualFire<T, There<I>> for VirtCons<'f, U, Tail>
where
    Tail: VirtualFire<T, I>,
{
    #[inline(always)]
    fn fire(&self, epoch: USize) {
        self.tail.fire(epoch);
    }
}

// ---------------------------------------------------------------------
// VirtualProject: build the projected write-virtual bundle from the bindings.
//
// The virtual analogue of `AccumProject`: recurse on the `Virtual<T>` members of
// the write set `W`, pulling each matching stamp cell ref out of the bindings via
// `VirtualStampSelector`. `Resource<T>` / `Column<T>` / `Accum<T>` members
// contribute no virtual-bundle node. `Indices` is the parallel selector-index
// list, inferred at the call site (dodging E0207). The projected bundle borrows
// `&'s`, so it ties to the `'frame`-lived bindings, like the accumulator bundle.
// ---------------------------------------------------------------------

/// Project the `Virtual<T>` members of `W` out of a source `A` into a stamp-cell
/// bundle.
pub trait VirtualProject<'s, Set, Indices> {
    /// The projected write-virtual bundle.
    type Out;

    /// Build the projected bundle by pulling each stamp cell ref.
    fn virt_project(&'s self) -> Self::Out;
}

impl<'s, C> VirtualProject<'s, Empty, Empty> for C {
    type Out = VirtNil;

    #[inline]
    fn virt_project(&'s self) -> VirtNil {
        VirtNil
    }
}

impl<'s, C, T, I, STail, ITail> VirtualProject<'s, Cons<Virtual<T>, STail>, Cons<I, ITail>> for C
where
    C: VirtualStampSelector<T, I>,
    C: VirtualProject<'s, STail, ITail>,
{
    type Out = VirtCons<'s, T, <C as VirtualProject<'s, STail, ITail>>::Out>;

    #[inline]
    fn virt_project(&'s self) -> Self::Out {
        VirtCons {
            head: <C as VirtualStampSelector<T, I>>::vstamp(self),
            tail: <C as VirtualProject<'s, STail, ITail>>::virt_project(self),
            _h: PhantomData,
        }
    }
}

// Skip non-virtual members of the write set (resource / column / accumulator).

impl<'s, C, T, STail, Indices> VirtualProject<'s, Cons<Resource<T>, STail>, Indices> for C
where
    C: VirtualProject<'s, STail, Indices>,
{
    type Out = <C as VirtualProject<'s, STail, Indices>>::Out;

    #[inline]
    fn virt_project(&'s self) -> Self::Out {
        <C as VirtualProject<'s, STail, Indices>>::virt_project(self)
    }
}

impl<'s, C, T, STail, Indices> VirtualProject<'s, Cons<Column<T>, STail>, Indices> for C
where
    C: VirtualProject<'s, STail, Indices>,
{
    type Out = <C as VirtualProject<'s, STail, Indices>>::Out;

    #[inline]
    fn virt_project(&'s self) -> Self::Out {
        <C as VirtualProject<'s, STail, Indices>>::virt_project(self)
    }
}

impl<'s, C, T, STail, Indices> VirtualProject<'s, Cons<Accum<T>, STail>, Indices> for C
where
    C: VirtualProject<'s, STail, Indices>,
{
    type Out = <C as VirtualProject<'s, STail, Indices>>::Out;

    #[inline]
    fn virt_project(&'s self) -> Self::Out {
        <C as VirtualProject<'s, STail, Indices>>::virt_project(self)
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
// E4 slice 3: the engine-to-meta bridge.
//
// Mutable meta state is engine-owned (a `MetaBlock` on the scheduler), not a
// consumer `Resource` (consumer resources are `Copy` read-only). An `OnMeta`
// work unit reads it through a `meta::<T>()` accessor present ONLY on a Ctx
// carrying a `MetaRef`. The meta pointer is the 9th `EngineCtx` parameter,
// defaulted `MetaNil` so consumer Ctx aliases are unchanged (mirrors the slice-1
// `WVirt = VirtNil` default); the dispatch walk wires a real `MetaRef` only for
// `OnMeta` units, via `MetaPtrFor` (keyed on the schedule) and `BuildMetaPtr`.
// Proven by sketch `202606090300_e4-slice3-meta-bridge-accessor`.
// ---------------------------------------------------------------------

/// Consumer Ctx meta pointer: no meta reference. The default 9th `EngineCtx`
/// parameter, so consumer Ctx aliases need no change.
#[derive(Clone, Copy)]
pub struct MetaNil;

/// `OnMeta` Ctx meta pointer: a borrow of the engine-owned meta block for the
/// dispatch frame. The `meta::<T>()` accessor exists only on a Ctx carrying it.
#[derive(Clone, Copy)]
pub struct MetaRef<'frame>(&'frame MetaBlock);

/// Build the per-unit meta pointer from the engine-owned block.
///
/// `MetaNil` ignores the block (consumer units); `MetaRef` captures it (`OnMeta`
/// units). The dispatch walk calls this once per unit at Ctx construction.
pub trait BuildMetaPtr<'frame> {
    /// Produce the meta pointer for this unit from the engine-owned block.
    fn build(block: &'frame MetaBlock) -> Self;
}

impl<'frame> BuildMetaPtr<'frame> for MetaNil {
    #[inline(always)]
    fn build(_block: &'frame MetaBlock) -> Self {
        MetaNil
    }
}

impl<'frame> BuildMetaPtr<'frame> for MetaRef<'frame> {
    #[inline(always)]
    fn build(block: &'frame MetaBlock) -> Self {
        MetaRef(block)
    }
}

/// The meta pointer a schedule's Ctx carries.
///
/// `MetaNil` for consumer schedules (`Always`, `On<V>`), `MetaRef<'frame>` for
/// meta schedules (`OnMeta<V>`). The dispatch walk computes the 9th `EngineCtx`
/// parameter from each unit's schedule through this, so consumer Ctx aliases
/// default to `MetaNil` and only `OnMeta` units gain a meta reference.
pub trait MetaPtrFor<'frame> {
    /// The meta pointer type for this schedule.
    type Ptr: BuildMetaPtr<'frame>;
}

impl<'frame> MetaPtrFor<'frame> for Always {
    type Ptr = MetaNil;
}

impl<'frame, V> MetaPtrFor<'frame> for On<V> {
    type Ptr = MetaNil;
}

impl<'frame, V> MetaPtrFor<'frame> for OnMeta<V> {
    type Ptr = MetaRef<'frame>;
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
/// default never masks a mismatch: `project` and `RunFiber` force `WAccum`
/// to the real projection of `W`, so a WU that declares an accumulator but
/// omits the bundle fails to compile at the projection tie.
pub struct EngineCtx<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum = AccPtrNil, WVirt = VirtNil, MP = MetaNil> {
    reads: RBundle,
    read_cols: RCols,
    write_cols: WCols,
    write_accums: WAccum,
    // E4 slice 1: the projected write-virtual bundle (one `&'frame Cell<USize>`
    // stamp ref per `Virtual<T>` in `W`), and the current pass epoch. `fire<V>`
    // sets the resolved stamp to `epoch`; an `On<V>` consumer's gate (trunk_gate)
    // reads the SAME cell from the full bindings and compares to its epoch.
    // Defaults `WVirt = VirtNil`: a WU writing no virtual carries an empty bundle.
    write_virtuals: WVirt,
    // E4 slice 3: the per-unit meta pointer. `MetaNil` for consumer units (no
    // meta reference); `MetaRef<'frame>` for `OnMeta` units (a borrow of the
    // engine-owned `MetaBlock`). The `meta::<T>()` accessor exists only on a Ctx
    // whose `MP = MetaRef<'frame>`. Defaults `MP = MetaNil`.
    meta_ptr: MP,
    epoch: USize,
    morsel: MorselRange,
    _frame: PhantomData<&'frame ()>,
    _sets: PhantomData<(R, W)>,
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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
        write_virtuals: WVirt,
        meta_ptr: MP,
        epoch: USize,
        morsel: MorselRange,
    ) -> Self {
        Self {
            reads,
            read_cols,
            write_cols,
            write_accums,
            write_virtuals,
            meta_ptr,
            epoch,
            morsel,
            _frame: PhantomData,
            _sets: PhantomData,
        }
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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
    /// use hilavitkutin::meta::MetaBlock;
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
    ///     EngineCtx::project(&PtrNil, &ColPtrNil, &MetaBlock::default(), USize::ZERO, MorselRange::new(USize::ZERO, USize::ZERO));
    /// ```
    #[inline]
    pub fn project<A, C, RIdx, RCIdx, WCIdx, WAIdx, WVIdx>(
        bindings: &'frame A,
        cols: &C,
        meta_block: &'frame MetaBlock,
        epoch: USize,
        morsel: MorselRange,
    ) -> EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
    where
        A: Project<R, RIdx, Out = RBundle>,
        C: ColProject<R, RCIdx, Out = RCols>,
        C: ColProject<W, WCIdx, Out = WCols>,
        A: AccumProject<'frame, W, WAIdx, Out = WAccum>,
        A: VirtualProject<'frame, W, WVIdx, Out = WVirt>,
        MP: BuildMetaPtr<'frame>,
    {
        let reads = <A as Project<R, RIdx>>::project(bindings);
        let read_cols = <C as ColProject<R, RCIdx>>::col_project(cols);
        let write_cols = <C as ColProject<W, WCIdx>>::col_project(cols);
        // The accumulator bundle projects from the `'frame` bindings (it
        // retains a borrow of each live-length cell), not the column source.
        let write_accums = <A as AccumProject<'frame, W, WAIdx>>::acc_project(bindings);
        // The write-virtual bundle also projects from the `'frame` bindings: each
        // entry is a borrow of a `VirtualBinding<T>` stamp cell. The same cells
        // the trunk-gate reads, so a fire here is observed by an `On<T>` gate.
        let write_virtuals = <A as VirtualProject<'frame, W, WVIdx>>::virt_project(bindings);
        // E4 slice 3: build the per-unit meta pointer from the engine-owned
        // block. `MetaNil` ignores it (consumer units); `MetaRef` captures it
        // (`OnMeta` units), gaining the gated `meta::<T>()` accessor.
        let meta_ptr = <MP as BuildMetaPtr<'frame>>::build(meta_block);
        EngineCtx::from_projected(
            reads, read_cols, write_cols, write_accums, write_virtuals, meta_ptr, epoch, morsel,
        )
    }
}

// E4 slice 3: the meta accessor, present ONLY on a Ctx carrying a `MetaRef`
// (an `OnMeta` work unit's Ctx). A consumer Ctx (`MP = MetaNil`) has no `meta`
// method, so a consumer cannot reach meta state at compile time. The
// `MetaAccess` enforcement falls out of the gating for free: no negative bound,
// no specialization. Proven by sketch `202606090300`.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt>
    EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MetaRef<'frame>>
{
    /// Read an engine-owned meta resource through the bridge.
    ///
    /// `T` is a meta resource (`MetaAccess`) with a `MetaField` projection out of
    /// the engine-owned `MetaBlock`. Available only on an `OnMeta` work unit's
    /// Ctx; a consumer Ctx does not have this method (compile-time `MetaAccess`
    /// enforcement).
    ///
    /// ```compile_fail
    /// use hilavitkutin::dispatch::engine_ctx::{EngineCtx, PtrNil, ColPtrNil, MetaNil};
    /// use hilavitkutin_api::access::Empty;
    /// use hilavitkutin_api::meta::SchedulerMetrics;
    ///
    /// // A consumer Ctx (the default `MetaNil` meta pointer) has no `meta`
    /// // accessor: the impl is only on a Ctx carrying `MetaRef`. So a consumer
    /// // cannot reach meta state. This does not compile.
    /// fn consumer_reaches_meta(
    ///     ctx: &EngineCtx<'_, Empty, Empty, PtrNil, ColPtrNil, ColPtrNil>,
    /// ) {
    ///     let _ = ctx.meta::<SchedulerMetrics>();
    /// }
    /// ```
    #[inline]
    pub fn meta<T: MetaAccess + MetaField>(&self) -> &T {
        T::project(self.meta_ptr.0)
    }
}

// ResourceProviderApi: resolve `&T` via the resource bundle Selector.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> ResourceProviderApi<R>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP, T: 'static, I> ResolveResource<T, I>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> ColumnReaderApi<R>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP, T: ColumnValue, I> ResolveColumnRead<T, I>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> ColumnWriterApi<W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP, T: ColumnValue, I> ResolveColumnWrite<T, I>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> AccumWriterApi<W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP, T: ColumnValue, I>
    ResolveAccumAppend<T, I> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
where
    WAccum: AccumSelector<T, I>,
{
    #[inline]
    unsafe fn resolve_append(&self, v: T) {
        let acc = <WAccum as AccumSelector<T, I>>::get(&self.write_accums);
        let live = acc.len.get();
        // Over-appending a fixed-capacity accumulator is a contract violation,
        // not a recoverable condition. Assert the live length is below the
        // reserved capacity and panic before the write: the assert fires ahead of
        // any out-of-bounds access (the soundness floor holds) and a misconfigured
        // capacity fails loudly instead of silently dropping the record. A
        // WorkUnit's appends are not bounded by the plan the way a column write's
        // morsel index is, so the consumer sizes the accumulator for the maximum
        // number of appends a single frame can make.
        assert!(
            live.0 < acc.cap.0,
            "accumulator append exceeded its reserved capacity; size the accumulator for the maximum per-frame appends",
        );
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

// VirtualFirerApi: stamp the projected `Virtual<V>` cell with the current epoch.
// The `On<V>` consumer's trunk-gate reads the same cell from the bindings and
// runs when `stamp == epoch`. Internal fire is non-atomic (spec :716-717).

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> VirtualFirerApi<W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    #[inline]
    fn fire<V: 'static, I>(&self)
    where
        W: Contains<Virtual<V>>,
        Self: ResolveVirtualFire<V, I>,
    {
        <Self as ResolveVirtualFire<V, I>>::resolve_fire(self);
    }
}

// ResolveVirtualFire: resolve the `Virtual<V>` stamp cell through the projected
// write-virtual bundle's `VirtualFire<V, I>` witness and set it to the live
// epoch. Mirrors `ResolveAccumAppend` over the accumulator bundle.
impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP, V: 'static, I>
    ResolveVirtualFire<V, I> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
where
    WVirt: VirtualFire<V, I>,
{
    #[inline]
    fn resolve_fire(&self) {
        <WVirt as VirtualFire<V, I>>::fire(&self.write_virtuals, self.epoch);
    }
}

// EachApi: per-record loop yielding a morsel-relative index `[0, len)`.
// `read` / `write` add `morsel.start` to recover the absolute column index,
// so the body works for any morsel start.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> EachApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> BatchApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> ReduceApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
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

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasColumnReader<R> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;
    #[inline(always)]
    fn reader(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasColumnWriter<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;
    #[inline(always)]
    fn writer(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasResourceProvider<R> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;
    #[inline(always)]
    fn resources(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasVirtualFirer<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;
    #[inline(always)]
    fn virtuals(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasEach<R, W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;
    #[inline(always)]
    fn each(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasBatch<R, W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;
    #[inline(always)]
    fn batch(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasReduce<R, W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;
    #[inline(always)]
    fn reduce(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasAccumWriter<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;
    #[inline(always)]
    fn accums(&self) -> &Self::Provider {
        self
    }
}

// ---------------------------------------------------------------------------
// CtxFor: the computed per-WorkUnit Context type (P0.2).
//
// The six derived `EngineCtx` parameters (the two resource/column read
// bundles, the write-column and write-accumulator and write-virtual bundles,
// and the meta pointer) are pure type functions of a WorkUnit's Read / Write
// access sets and its schedule. Four fold traits compute them by per-store-kind
// dispatch over the cons-list, the same disjoint kind dispatch the `Project` /
// `ColProject` / `AccumProject` / `VirtualProject` value projections use (no
// specialization). Each contributing kind conses its projected node; the other
// three pass the tail, so the output is the kind-filtered subsequence of set
// order, matching the runtime projection value order node for node. The meta
// pointer keys off the schedule via the shipped `MetaPtrFor`. A consumer then
// writes `type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write[, Sched]>`
// instead of hand-spelling the nine `EngineCtx` parameters. Proven by sketch
// 202606111430 (WORKS, identity-asserted, run-proven over the dispatch).

/// Folds a Read access set into its `PtrCons` resource bundle (`PtrNil` leaf).
pub trait ResourceBundleOf {
    /// The projected resource pointer bundle for this access set.
    type Out;
}
impl ResourceBundleOf for Empty {
    type Out = PtrNil;
}
impl<T, Tail: ResourceBundleOf> ResourceBundleOf for Cons<Resource<T>, Tail> {
    type Out = PtrCons<T, Tail::Out>;
}
impl<T, Tail: ResourceBundleOf> ResourceBundleOf for Cons<Column<T>, Tail> {
    type Out = Tail::Out;
}
impl<T, Tail: ResourceBundleOf> ResourceBundleOf for Cons<Accum<T>, Tail> {
    type Out = Tail::Out;
}
impl<T, Tail: ResourceBundleOf> ResourceBundleOf for Cons<Virtual<T>, Tail> {
    type Out = Tail::Out;
}

/// Folds an access set into its `ColPtrCons` column bundle (`ColPtrNil` leaf).
pub trait ColBundleOf {
    /// The projected column pointer bundle for this access set.
    type Out;
}
impl ColBundleOf for Empty {
    type Out = ColPtrNil;
}
impl<T, Tail: ColBundleOf> ColBundleOf for Cons<Column<T>, Tail> {
    type Out = ColPtrCons<T, Tail::Out>;
}
impl<T, Tail: ColBundleOf> ColBundleOf for Cons<Resource<T>, Tail> {
    type Out = Tail::Out;
}
impl<T, Tail: ColBundleOf> ColBundleOf for Cons<Accum<T>, Tail> {
    type Out = Tail::Out;
}
impl<T, Tail: ColBundleOf> ColBundleOf for Cons<Virtual<T>, Tail> {
    type Out = Tail::Out;
}

/// Folds a Write access set into its `AccPtrCons` accumulator bundle
/// (`AccPtrNil` leaf). Carries `'frame` because the accumulator cons nodes
/// borrow the bindings' cells.
pub trait AccumBundleOf<'frame> {
    /// The projected accumulator pointer bundle for this access set.
    type Out;
}
impl<'frame> AccumBundleOf<'frame> for Empty {
    type Out = AccPtrNil;
}
impl<'frame, T, Tail: AccumBundleOf<'frame>> AccumBundleOf<'frame> for Cons<Accum<T>, Tail> {
    type Out = AccPtrCons<'frame, T, Tail::Out>;
}
impl<'frame, T, Tail: AccumBundleOf<'frame>> AccumBundleOf<'frame> for Cons<Resource<T>, Tail> {
    type Out = Tail::Out;
}
impl<'frame, T, Tail: AccumBundleOf<'frame>> AccumBundleOf<'frame> for Cons<Column<T>, Tail> {
    type Out = Tail::Out;
}
impl<'frame, T, Tail: AccumBundleOf<'frame>> AccumBundleOf<'frame> for Cons<Virtual<T>, Tail> {
    type Out = Tail::Out;
}

/// Folds a Write access set into its `VirtCons` write-virtual bundle
/// (`VirtNil` leaf). Carries `'frame` because the virtual cons nodes borrow the
/// bindings' stamp cells.
pub trait VirtBundleOf<'frame> {
    /// The projected write-virtual bundle for this access set.
    type Out;
}
impl<'frame> VirtBundleOf<'frame> for Empty {
    type Out = VirtNil;
}
impl<'frame, V, Tail: VirtBundleOf<'frame>> VirtBundleOf<'frame> for Cons<Virtual<V>, Tail> {
    type Out = VirtCons<'frame, V, Tail::Out>;
}
impl<'frame, T, Tail: VirtBundleOf<'frame>> VirtBundleOf<'frame> for Cons<Resource<T>, Tail> {
    type Out = Tail::Out;
}
impl<'frame, T, Tail: VirtBundleOf<'frame>> VirtBundleOf<'frame> for Cons<Column<T>, Tail> {
    type Out = Tail::Out;
}
impl<'frame, T, Tail: VirtBundleOf<'frame>> VirtBundleOf<'frame> for Cons<Accum<T>, Tail> {
    type Out = Tail::Out;
}

/// The computed per-WorkUnit `Context` type: a consumer assigns this to its
/// `WorkUnit::Ctx<'frame>` GAT instead of hand-spelling the nine `EngineCtx`
/// parameters. `S` defaults to `Always` (mirroring `WorkUnit<Sched = Always>`).
pub type CtxFor<'frame, R, W, S = Always> = EngineCtx<
    'frame,
    R,
    W,
    <R as ResourceBundleOf>::Out,
    <R as ColBundleOf>::Out,
    <W as ColBundleOf>::Out,
    <W as AccumBundleOf<'frame>>::Out,
    <W as VirtBundleOf<'frame>>::Out,
    <S as MetaPtrFor<'frame>>::Ptr,
>;
