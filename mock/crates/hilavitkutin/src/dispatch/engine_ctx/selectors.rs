//! Type-keyed lookups over bindings nodes and projected bundles: the
//! `Selector` family, the E4 virtual-firing gate machinery, and the
//! E4 slice-3 meta-pointer keying.
//!
//! Split out of `dispatch/engine_ctx.rs` (file-size lint). No behaviour
//! change.

use core::cell::Cell;

use arvo::{Bool, USize};
use hilavitkutin_api::{Always, On, OnMeta};

use super::{
    AccPtrCons,
    AccumColPtr,
    ColPtrCons,
    Here,
    MetaNil,
    MetaRef,
    SnapCons,
    There,
    VirtCons,
};
use crate::meta::MetaBlock;
use crate::resource::bindings::{AccumBinding, ColumnBinding, ResourceBinding, VirtualBinding};
use crate::resource::provenance::{ColumnPtr, ResourcePtr};

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
        Bool(<A as VirtualStampSelector<V, GI>>::vstamp(bindings).get().0 == epoch.0) // lint:allow(no-bare-numeric) reason: epoch-stamp equality compare; tracked: #121
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

// ---------------------------------------------------------------------
// Snapshot selector: type-keyed lookup over the projected value bundle
// (`SnapCons` / `SnapNil`). Distinct from `Selector` (which yields the
// canonical `ResourcePtr<T>` off the bindings and feeds the projection):
// this one borrows the snapshot value the projection copied into the
// Context, so `resource()` reads at stack provenance with no unsafe.
// ---------------------------------------------------------------------

/// Type-keyed lookup yielding `&T` from the snapshot bundle.
pub trait SnapSelector<T, Index> {
    /// The snapshot value for `T`.
    fn get(&self) -> &T;
}

impl<T, Tail> SnapSelector<T, Here> for SnapCons<T, Tail> {
    #[inline(always)]
    fn get(&self) -> &T {
        &self.head
    }
}

impl<T, U, Tail, I> SnapSelector<T, There<I>> for SnapCons<U, Tail>
where
    Tail: SnapSelector<T, I>,
{
    #[inline(always)]
    fn get(&self) -> &T {
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
            len:  self.__len_cell(),
            cap:  self.__cap(),
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
// E4 slice 3: the engine-to-meta bridge (keying half; the carrier types
// `MetaNil` / `MetaRef` live in `super::lists`).
// ---------------------------------------------------------------------

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
