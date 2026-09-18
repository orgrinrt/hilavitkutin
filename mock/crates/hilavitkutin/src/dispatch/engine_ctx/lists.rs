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

use arvo::USize;

use crate::meta::MetaBlock;
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
        Self {
            head,
            tail,
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

/// Projected accumulator handle: capacity base, borrowed live-length cell, and
/// the reserved capacity the append asserts the live length against (a
/// contract-violating over-append panics before the write).
pub struct AccumColPtr<'frame, T> {
    pub(super) base: ColumnPtr<T>,
    pub(super) len:  &'frame Cell<USize>,
    pub(super) cap:  USize,
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
