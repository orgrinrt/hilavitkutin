//! `CtxFor`: the computed per-WorkUnit Context type (P0.2), and the four
//! bundle-fold traits it is built from.
//!
//! Split out of `dispatch/engine_ctx.rs` (file-size lint). No behaviour
//! change.

use hilavitkutin_api::Always;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::store::{Accum, Column, Resource, Virtual};

use super::selectors::MetaPtrFor;
use super::{
    AccPtrCons,
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    SnapCons,
    SnapNil,
    VirtCons,
    VirtNil,
};

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

/// Folds a Read access set into its `SnapCons` snapshot bundle (`SnapNil` leaf).
pub trait ResourceBundleOf {
    /// The projected resource pointer bundle for this access set.
    type Out;
}
impl ResourceBundleOf for Empty {
    type Out = SnapNil;
}
impl<T, Tail: ResourceBundleOf> ResourceBundleOf for Cons<Resource<T>, Tail> {
    type Out = SnapCons<T, Tail::Out>;
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
