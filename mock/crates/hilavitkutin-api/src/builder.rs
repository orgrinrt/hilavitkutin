//! Builder support traits for the scheduler builder.
//!
//! Four sealed contracts ship in api so the engine's
//! `SchedulerBuilder<MAX_*, Wus, Stores>` can carry where-clauses on
//! `.build()` and on `add_kit`. They live here, not in the engine,
//! because consumers reference them at WU declarations and at
//! `.build()` call sites.
//!
//! `Buildable<Stores>` proves every WorkUnit in `Wus` has its
//! `Read` / `Write` access sets satisfied by `Stores`. Two impls:
//! base `()` and recursive `(H, R)`. No per-arity cap. Linear
//! trait-solver depth in the size of `Wus`.
//!
//! `WuSatisfied<A>` proves every member of `A` is present in
//! `Self`. `A` is a cons-list shape produced by the [`crate::read`]
//! / [`crate::write`] macros from a consumer's flat-tuple syntax.
//! Two impls: base `()` and recursive `(H, R)`. No per-arity cap.
//!
//! `BuilderExtending<B>` constrains `Kit::Output` so a Kit cannot
//! drop prior registrations. `Wus` must match the input builder's;
//! `NewStores` must satisfy `WuSatisfied<OldStores>`. The single
//! impl lives in the engine; the trait + sealing module live here
//! to keep the consumer-visible bound declarable from api alone.
//!
//! `Depth` reports total cons-list element count. Two impls: `()`
//! and `(H, R: Depth)`. Plan-stage code uses `Depth::D` for
//! recursive total counts. `AccessSet::LEN` reports
//! immediate-tuple-arity (always 2 for cons-list cells); the two
//! consts coexist with distinct contracts.
//!
//! All four traits are sealed via private supertraits; consumers
//! cannot impl them.

use arvo::USize;

use crate::access::{AccessSet, Contains};
use crate::work_unit::WorkUnit;

mod buildable_sealed {
    pub trait Sealed {}
}

mod wu_satisfied_sealed {
    pub trait Sealed<A> {}
}

#[doc(hidden)]
pub mod extending_sealed {
    pub trait Sealed<B> {}
}

mod depth_sealed {
    pub trait Sealed {}
}

/// Proof that every WorkUnit in `Wus` has its `Read` and `Write`
/// access sets satisfied by `Stores`.
///
/// The engine's `SchedulerBuilder::build` carries `Wus:
/// Buildable<Stores>` as its where-clause. Two recursive impls
/// (base `()` and step `(H, R)`) reduce this into per-WU
/// `WuSatisfied<...>` proofs at every cons-list step. No per-arity
/// cap; linear trait-solver depth in the size of `Wus`.
///
/// rustc's default `recursion_limit = 128` accommodates apps up to
/// roughly 30 WUs by 30 stores. Larger workloads need
/// `#![recursion_limit = "512"]` at the consumer crate root. The
/// `hilavitkutin-api`, `hilavitkutin`, and `hilavitkutin-kit` crates
/// already declare 512 internally for their own machinery.
///
/// Sealed; consumers cannot impl directly.
#[allow(private_bounds)]
pub trait Buildable<Stores: AccessSet>: buildable_sealed::Sealed {}

/// Proof that all members of `A` are present in `Self`.
///
/// `Self` is the registered `Stores` tuple; `A` is a WU's `Read`
/// or `Write` access set in cons-list shape (produced by the
/// [`crate::read`] / [`crate::write`] macros). Two recursive
/// impls reduce this into per-store `Self: Contains<H>` proofs at
/// every step. No per-arity cap.
///
/// Sealed; consumers cannot impl directly.
#[allow(private_bounds)]
pub trait WuSatisfied<A: AccessSet>: wu_satisfied_sealed::Sealed<A> {}

/// Proof that `Self` extends `B`: same `Wus`, and the new `Stores`
/// contains every store from the old `Stores`.
///
/// Used as `K::Output: BuilderExtending<Self>` on the engine's
/// `add_kit` to prevent a buggy Kit from wiping prior
/// registrations. The single legal impl lives in the engine; the
/// trait + sealing module live here so consumer-visible bounds
/// stay declarable from api alone.
///
/// Sealed; only the engine's `SchedulerBuilder` impl is permitted.
#[allow(private_bounds)]
pub trait BuilderExtending<B>: extending_sealed::Sealed<B> {}

/// Total cons-list element count.
///
/// `<()>::D == USize(0)`. `<(H, R)>::D == R::D + 1`. Impl'd ONLY
/// on cons-list shapes (`()` and `(H, R: Depth)`). Flat tuples of
/// arity 3+ deliberately do not impl `Depth`.
///
/// Plan-stage code uses `Depth::D` for total recursive counts.
/// [`AccessSet::LEN`] reports immediate-tuple-arity (always 2 for
/// cons-list cells); the two consts coexist with distinct
/// contracts.
///
/// Sealed; consumers cannot impl directly.
#[allow(private_bounds)]
pub trait Depth: depth_sealed::Sealed {
    const D: USize;
}

// ---------------------------------------------------------------------
// Buildable: recursive over the Wus cons-list. No arity cap.
// Coherence trivially holds: () and (H, R) are disjoint shapes.
// ---------------------------------------------------------------------

impl buildable_sealed::Sealed for () {}
impl<Stores: AccessSet> Buildable<Stores> for () {}

impl<H, R> buildable_sealed::Sealed for (H, R) {}
impl<H, R, Stores> Buildable<Stores> for (H, R)
where
    H: WorkUnit,
    R: Buildable<Stores>,
    Stores: AccessSet + WuSatisfied<<H as WorkUnit>::Read> + WuSatisfied<<H as WorkUnit>::Write>,
{
}

// ---------------------------------------------------------------------
// WuSatisfied: recursive over the cons-list shape of A. No arity cap.
// Consumer's WorkUnit::Read / WorkUnit::Write must be cons-list
// (use the read! / write! macros).
// ---------------------------------------------------------------------

impl<S: AccessSet> wu_satisfied_sealed::Sealed<()> for S {}
impl<S: AccessSet> WuSatisfied<()> for S {}

impl<S, H: 'static, R> wu_satisfied_sealed::Sealed<(H, R)> for S
where
    S: WuSatisfied<R>,
    R: AccessSet,
{
}
impl<S, H: 'static, R> WuSatisfied<(H, R)> for S
where
    S: Contains<H> + AccessSet + WuSatisfied<R>,
    R: AccessSet,
{
}

// ---------------------------------------------------------------------
// Depth: total cons-list element count. Two impls, no specialization,
// no marker overlap. () and (H, R) are disjoint shapes.
// ---------------------------------------------------------------------

impl depth_sealed::Sealed for () {}
impl<H, R: Depth> depth_sealed::Sealed for (H, R) {}

impl Depth for () {
    const D: USize = USize(0);
}
impl<H, R: Depth> Depth for (H, R) {
    const D: USize = USize(R::D.0 + 1);
}
