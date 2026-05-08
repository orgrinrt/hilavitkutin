//! `Depth` trait for total cons-list element count.
//!
//! Plan-stage code uses `Depth::D` for total recursive counts.
//! `AccessSet::LEN` reports immediate-tuple-arity (always 2 for
//! cons-list cells); the two consts coexist with distinct contracts.
//!
//! Round 4 deleted the four sealed builder bridges (`Buildable`,
//! `WuSatisfied`, `BuilderExtending`, `BuilderResource`). Their
//! roles are subsumed by `WorkUnitBundle` (in `work_unit`) +
//! `StoreBundle` (in `store`) + `ContainsAll<L>` (in `access`). See
//! `mock/design_rounds/202605042200_changelist.doc.lock.md`.

use arvo::USize;
use arvo::strategy::Identity;

use crate::access::{Cons, Empty};

mod depth_sealed {
    pub trait Sealed {}
}

/// Total cons-list element count.
///
/// `<()>::D == USize::ZERO` and `<Empty>::D == USize::ZERO`.
/// `<(H, R)>::D == R::D + USize::ONE` and
/// `<Cons<H, R>>::D == R::D + USize::ONE`. Both flat-tuple and
/// `Cons<H, R>` shapes impl `Depth`. Flat tuples of arity 3+
/// deliberately do not impl `Depth`.
///
/// Sealed; consumers cannot impl directly.
#[allow(private_bounds)]
pub trait Depth: depth_sealed::Sealed {
    const D: USize;
}

// ---------------------------------------------------------------------
// Depth: total cons-list element count. Two pairs of impls (flat
// tuple () + (H, R) and HList Empty + Cons<H, R>), all disjoint.
// ---------------------------------------------------------------------

impl depth_sealed::Sealed for () {}
impl<H, R: Depth> depth_sealed::Sealed for (H, R) {}

impl Depth for () {
    const D: USize = USize::ZERO;
}
impl<H, R: Depth> Depth for (H, R) {
    const D: USize = R::D + USize::ONE;
}

impl depth_sealed::Sealed for Empty {}
impl<H, R: Depth> depth_sealed::Sealed for Cons<H, R> {}

impl Depth for Empty {
    const D: USize = USize::ZERO;
}
impl<H, R: Depth> Depth for Cons<H, R> {
    const D: USize = R::D + USize::ONE;
}
