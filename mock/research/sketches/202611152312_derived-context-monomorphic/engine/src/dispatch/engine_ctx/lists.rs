//! Projected bundle carrier types: the cons-list nodes a projection builds
//! to carry resource, column, accumulator, write-virtual and meta pointers
//! into an `EngineCtx`.
//!
//! Split out of `dispatch/engine_ctx.rs` (file-size lint). No behaviour
//! change. `AccumColPtr`'s fields and `MetaRef`'s inner field are
//! `pub(super)` rather than private: `scheduler::selectors` (which builds an
//! `AccumColPtr` literal in its `AccumSelector` impl for `AccumBinding`) and
//! `scheduler::ctx_writers` / `scheduler::ctx_meta_resource` (which read
//! them back) are siblings of this module rather than descendants, so plain
//! module-privacy does not reach across.

use core::cell::Cell;
use core::marker::PhantomData;

use arvo::{Bool, USize};

use crate::meta::MetaBlock;
use crate::resource::bindings::{ColKind, MarkPtr, Plain};
use crate::resource::provenance::ColumnPtr;

// ---------------------------------------------------------------------
// Projected bundles.
//
// `SnapCons` / `SnapNil` carry the projected resource VALUES for the
// resource members of `R`: the projection copies each value out of the
// canonical blob (through the binding's backcast pointer) into the
// Context itself. The Context lives on the dispatch stack frame, so
// the value bundle IS the domain-19 stack-local snapshot; `resource()`
// borrows it at stack provenance and the morsel hot loop touches no
// resource memory. `ColPtrCons` / `ColPtrNil` carry the projected
// `ColumnPtr<T>` for the column members of `R` union `W`. The distinct
// shapes keep the snapshot (stack) and column (heap) provenance
// classes separate.
// ---------------------------------------------------------------------

/// Empty resource snapshot bundle (tail leaf).
pub struct SnapNil;

/// One snapshot resource value `head` of type `H`, followed by the
/// rest, `tail`.
pub struct SnapCons<H, Tail> {
    pub(crate) head: H,
    pub(crate) tail: Tail,
}

impl<H, Tail> SnapCons<H, Tail> {
    /// Construct a snapshot bundle node. Hidden test accessor: the
    /// run-loop builds the resource bundle via the projection; tests may
    /// build it by hand. Not part of the supported surface.
    #[doc(hidden)]
    #[inline]
    pub fn __new(head: H, tail: Tail) -> Self {
        Self {
            head,
            tail,
        }
    }
}

/// Empty column pointer bundle (tail leaf).
pub struct ColPtrNil;

/// One column store as a context reaches it.
///
/// `read` is the base every read of the pass sees and `write` the base every
/// write lands on. For a plain column they are the one base; for a buffered
/// column they are its frozen side and its live side. `mark` is the store's
/// changed mark and `count` its record count, which bounds a read with a
/// reach.
pub struct ColRef<T> {
    pub(super) read:  ColumnPtr<T>,
    pub(super) write: ColumnPtr<T>,
    pub(super) mark:  MarkPtr,
    pub(super) count: USize,
}

impl<T> ColRef<T> {
    /// The base every read of the pass sees. Hidden accessor for a host that
    /// reaches a column through the bindings: for a plain column it is the
    /// one base the column has, for a buffered column its frozen side. Not
    /// part of the supported surface.
    #[doc(hidden)]
    #[inline(always)]
    pub const fn __read(&self) -> ColumnPtr<T> {
        self.read
    }

    /// The base every write of the pass lands on: the one base of a plain
    /// column, the live side of a buffered one. Hidden accessor, as `__read`.
    /// Not part of the supported surface.
    #[doc(hidden)]
    #[inline(always)]
    pub const fn __write(&self) -> ColumnPtr<T> {
        self.write
    }
}

impl<T> Copy for ColRef<T> {}
impl<T> Clone for ColRef<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

/// One projected column `head` of value type `H` and kind `K`, followed by
/// the rest, `tail`.
///
/// `K` is `Plain` for a `Column<H>` and the buffer mode for a `Buffered<H,
/// M>`, so the two never answer for each other in a lookup by type.
pub struct ColPtrCons<H, Tail, K: ColKind = Plain> {
    pub(crate) head: ColRef<H>,
    pub(crate) tail: Tail,
    pub(crate) _k:   PhantomData<K>,
}

impl<H, Tail> ColPtrCons<H, Tail> {
    /// Construct a plain column bundle node over `count` records at `head`.
    /// Hidden test accessor: the dispatch builds the column bundle from the
    /// bindings, and a test may build one by hand. The node's changed mark is
    /// one nothing reads. Not part of the supported surface.
    #[doc(hidden)]
    #[inline]
    pub fn __new(head: ColumnPtr<H>, count: USize, tail: Tail) -> Self {
        Self {
            head: ColRef {
                read: head,
                write: head,
                mark: MarkPtr::unread(),
                count,
            },
            tail,
            _k: PhantomData,
        }
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
// (`SnapCons`) and column (`ColPtrCons`) bundles.
// ---------------------------------------------------------------------

/// Projected accumulator handle: capacity base, the borrowed live-length and
/// `wanted` cells, and the reserved capacity past which an append is refused.
pub struct AccumColPtr<'frame, T> {
    pub(super) base:   ColumnPtr<T>,
    pub(super) len:    &'frame Cell<USize>,
    pub(super) wanted: &'frame Cell<USize>,
    pub(super) cap:    USize,
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
    pub(crate) _h:   PhantomData<H>,
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
pub struct MetaRef<'frame>(pub(super) &'frame MetaBlock);

/// Whether a context is a maintenance unit's, read off its meta pointer.
///
/// A maintenance unit's write to a plain column is compared and marked, and
/// an ordinary unit's is neither. The branch is on this constant, so each
/// context compiles the one arm it takes.
pub trait MetaRank {
    /// `TRUE` on a maintenance unit's context.
    const MAINTAINS: Bool;
}

impl MetaRank for MetaNil {
    const MAINTAINS: Bool = Bool::FALSE;
}

impl<'frame> MetaRank for MetaRef<'frame> {
    const MAINTAINS: Bool = Bool::TRUE;
}

/// What a context holds of an accumulator its unit reads: the base of the
/// buffer and how many records it held when the context was bound.
///
/// It sits in the snapshot bundle beside the resource values. Only a
/// maintenance unit's context has an accessor that reads it.
pub struct AccSnap<T> {
    pub(super) base: ColumnPtr<T>,
    pub(super) len:  USize,
}

impl<T> Copy for AccSnap<T> {}
impl<T> Clone for AccSnap<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}
