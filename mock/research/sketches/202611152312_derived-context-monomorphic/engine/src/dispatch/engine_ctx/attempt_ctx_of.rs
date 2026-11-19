//! Attempt c1 and c2: the engine supplying the api's `CtxOf` type function,
//! so the api's blanket `WorkUnit` for block units resolves to `CtxFor`.
//!
//! Sketch addition behind the feature `attempt-ctx-of`, which no build turns
//! on except the one that records its output under `diagnostics/`. `CtxOf` is
//! the api's trait and the lists are the api's types, so both spellings name
//! nothing of this crate in the positions the orphan rule reads.

use hilavitkutin_api::access::{AccessSet, Cons, Empty};
use hilavitkutin_api::attempt_blanket::CtxOf;
use hilavitkutin_api::block::{ColList, ColSlots};

use super::bundles::{AccumBundleOf, ColBundleOf, CtxFor, ResourceBundleOf, VirtBundleOf};

// c1: one implementation for every read list.
#[cfg(not(feature = "attempt-ctx-of-per-shape"))]
impl<'frame, R, W> CtxOf<'frame, W> for R
where
    R: AccessSet + ColList + ResourceBundleOf + ColBundleOf,
    W: AccessSet + ColList + ColBundleOf + AccumBundleOf<'frame> + VirtBundleOf<'frame>,
    <R as ColBundleOf>::Out: ColSlots<R>,
    <W as ColBundleOf>::Out: ColSlots<W>,
{
    type Ctx = CtxFor<'frame, R, W>;
}

// c2: one implementation per list shape, so no bare parameter is the
// implementing type.
#[cfg(feature = "attempt-ctx-of-per-shape")]
impl<'frame, W> CtxOf<'frame, W> for Empty
where
    W: AccessSet + ColList + ColBundleOf + AccumBundleOf<'frame> + VirtBundleOf<'frame>,
    <W as ColBundleOf>::Out: ColSlots<W>,
{
    type Ctx = CtxFor<'frame, Empty, W>;
}

#[cfg(feature = "attempt-ctx-of-per-shape")]
impl<'frame, H, T, W> CtxOf<'frame, W> for Cons<H, T>
where
    Cons<H, T>: AccessSet + ColList + ResourceBundleOf + ColBundleOf,
    W: AccessSet + ColList + ColBundleOf + AccumBundleOf<'frame> + VirtBundleOf<'frame>,
    <Cons<H, T> as ColBundleOf>::Out: ColSlots<Cons<H, T>>,
    <W as ColBundleOf>::Out: ColSlots<W>,
{
    type Ctx = CtxFor<'frame, Cons<H, T>, W>;
}
