//! Store-value routing for the scheduler builder.
//!
//! The builder keeps a single unified `.with(value)` verb, yet only
//! store-registration values (the `Resource<T>` carrier) need to ride
//! a value list until `build()`. WorkUnit and platform values are
//! dropped at the call site (their TYPE is still tracked in the `Wus`
//! and `Platform` typestate accumulators).
//!
//! Routing store-vs-nonstore at the type level without an overlapping
//! impl uses a `RouterKind` tag on each dispatch router plus a
//! `Place<P>` view keyed on the tag-as-`Self`. The three tags
//! (`StoreKind` / `UnitKind` / `PlatformKind`) are disjoint `Self`
//! types, so the trait solver sees no overlap and no specialization is
//! needed. `StoreKind` prepends the registered value onto a
//! `Stores`-aligned `StoreValues` cons-list; `UnitKind` and
//! `PlatformKind` are identity and drop the value.
//!
//! Mechanism validated by the standalone sketch
//! `mock/research/sketches/202605290002_builder-kind-dispatch.md`
//! (compiles on stable, no specialization, no `alloc`).

use crate::builder_input::{PlatformDispatch, StoreDispatch, UnitDispatch};
use crate::run_cfg::RunCfgDispatch;

mod sealed {
    pub trait Sealed {}
}

/// A value-carrying registration list aligned with the `Stores`
/// typestate.
///
/// Sealed: only `SvEmpty` and `Sv` inhabit it, so the builder's
/// store-value accumulator cannot be forged by a consumer.
pub trait StoreValues: sealed::Sealed {}

/// The empty store-value list, the builder's initial store-value
/// state.
pub struct SvEmpty;

/// One store value `head` of type `H`, followed by the rest, `tail`.
///
/// `StoreKind::place` prepends a node per store registration. The node
/// owns `head` so the registered value stays alive until `build()`,
/// where the arena drain moves it into scheduler-owned storage.
#[allow(dead_code)] // head/tail are moved into the arena by the drain and read by the retention test; the node owns the value to keep it alive
pub struct Sv<H, T: StoreValues> {
    pub(crate) head: H,
    pub(crate) tail: T,
}

impl sealed::Sealed for SvEmpty {}
impl StoreValues for SvEmpty {}

impl<H, T: StoreValues> sealed::Sealed for Sv<H, T> {}
impl<H, T: StoreValues> StoreValues for Sv<H, T> {}

impl<H, T: StoreValues> Sv<H, T> {
    /// Borrow the head value. Hidden test accessor; not supported
    /// surface. The arena drain owns reading the head value at build.
    #[doc(hidden)]
    pub fn __head(&self) -> &H {
        &self.head
    }

    /// Borrow the tail list. Hidden test accessor; not supported
    /// surface.
    #[doc(hidden)]
    pub fn __tail(&self) -> &T {
        &self.tail
    }

    /// Consume the node into its head value and tail list.
    ///
    /// The arena drain walks the value list by repeatedly splitting off
    /// the head (the registered store value) and recursing on the tail.
    #[inline]
    pub fn into_parts(self) -> (H, T) {
        (self.head, self.tail)
    }
}

/// Disjoint kind tag for store-registration inputs.
pub struct StoreKind;

/// Disjoint kind tag for WorkUnit-registration inputs.
pub struct UnitKind;

/// Disjoint kind tag for platform-registration inputs.
pub struct PlatformKind;

/// The kind tag a dispatch router carries.
///
/// Reads as "which store-value placement does this router's input
/// take". `StoreDispatch` routes to `StoreKind` (the value is
/// retained); `UnitDispatch` / `PlatformDispatch` / `RunCfgDispatch`
/// route to `UnitKind` / `PlatformKind` (the value is dropped, the
/// type is tracked elsewhere).
pub trait RouterKind {
    /// The disjoint placement tag.
    type Kind;
}

impl<S> RouterKind for StoreDispatch<S> {
    type Kind = StoreKind;
}

impl<W> RouterKind for UnitDispatch<W> {
    type Kind = UnitKind;
}

impl<P> RouterKind for PlatformDispatch<P> {
    type Kind = PlatformKind;
}

// The RunCfg value is read via the type-level store accumulator
// (`RunCfgDispatch` prepends the cfg type onto `Stores`), not via the
// value list. Route it to `UnitKind` so its registered value is
// dropped rather than retained on the store-value list.
impl<C> RouterKind for RunCfgDispatch<C> {
    type Kind = UnitKind;
}

/// Kind-conditional store-value placement, keyed on the tag as `Self`.
///
/// Three non-overlapping impls (`StoreKind` / `UnitKind` /
/// `PlatformKind`). `Next<L>` names the store-value list shape after
/// placing one input of this kind onto the current list `L`.
///
/// The GAT parameter is named `L` rather than `Sv`: `Sv` is the
/// cons-cell struct, and a parameter named `Sv` would shadow it.
pub trait Place<P> {
    /// The store-value list after placing `P` onto a list `L`.
    type Next<L: StoreValues>: StoreValues;

    /// Place `provider` onto `sv`, producing the next list.
    fn place<L: StoreValues>(provider: P, sv: L) -> Self::Next<L>;
}

impl<P> Place<P> for StoreKind {
    type Next<L: StoreValues> = Sv<P, L>;

    #[inline]
    fn place<L: StoreValues>(provider: P, sv: L) -> Self::Next<L> {
        Sv {
            head: provider,
            tail: sv,
        }
    }
}

impl<P> Place<P> for UnitKind {
    type Next<L: StoreValues> = L;

    #[inline]
    fn place<L: StoreValues>(_provider: P, sv: L) -> Self::Next<L> {
        sv
    }
}

impl<P> Place<P> for PlatformKind {
    type Next<L: StoreValues> = L;

    #[inline]
    fn place<L: StoreValues>(_provider: P, sv: L) -> Self::Next<L> {
        sv
    }
}
