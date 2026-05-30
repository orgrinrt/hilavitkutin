//! Store-value routing for the scheduler builder.
//!
//! The builder keeps a single unified `.with(value)` verb, yet routes
//! the registered value to one of two retained lists, or drops it.
//! Store-registration values (the `Resource<T>` carrier) ride a
//! `StoreValues` list until `build()` moves them into the arena.
//! WorkUnit instances ride a `WuCons` value list the run walk consumes.
//! Platform and run-config values are dropped at the call site (their
//! TYPE is still tracked in the typestate accumulators).
//!
//! Routing at the type level without an overlapping impl uses a
//! `RouterKind` tag on each dispatch router plus a `Place<P>` view keyed
//! on the tag-as-`Self`. The tags (`StoreKind` / `WorkUnitKind` /
//! `UnitKind` / `PlatformKind`) are disjoint `Self` types, so the trait
//! solver sees no overlap and no specialization is needed. One `Place`
//! call routes the single registered value onto both lists at once:
//! `StoreKind` prepends the store list, `WorkUnitKind` prepends the WU
//! list, `UnitKind` (run-config, kit) and `PlatformKind` pass both
//! through (drop). The value is consumed once and lands on exactly one
//! list.
//!
//! Mechanism validated by the standalone sketch
//! `mock/research/sketches/202605290002_builder-kind-dispatch.md`
//! (compiles on stable, no specialization, no `alloc`).

use crate::builder_input::{PlatformDispatch, StoreDispatch, UnitDispatch};
use crate::run_cfg::RunCfgDispatch;
use crate::work_unit_values::WuCons;

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

/// Disjoint kind tag for WorkUnit-registration inputs.
///
/// Routes a registered WorkUnit instance onto the WorkUnit-value list
/// (prepend), passing the store-value list through. The engine's run
/// walk consumes the WorkUnit-value list.
pub struct WorkUnitKind;

/// The kind tag a dispatch router carries.
///
/// Reads as "which placement does this router's input take".
/// `StoreDispatch` routes to `StoreKind` (the value rides the store
/// list); `UnitDispatch` routes to `WorkUnitKind` (the instance rides
/// the WorkUnit-value list); `PlatformDispatch` routes to
/// `PlatformKind` and `RunCfgDispatch` to `UnitKind` (the value is
/// dropped, its type tracked elsewhere).
pub trait RouterKind {
    /// The disjoint placement tag.
    type Kind;
}

impl<S> RouterKind for StoreDispatch<S> {
    type Kind = StoreKind;
}

impl<W> RouterKind for UnitDispatch<W> {
    type Kind = WorkUnitKind;
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

/// Kind-conditional value placement, keyed on the tag as `Self`.
///
/// Four non-overlapping impls (`StoreKind` / `WorkUnitKind` /
/// `UnitKind` / `PlatformKind`). A registered value is consumed once and
/// lands on exactly one of two lists: the store-value list (the arena
/// drain reads it) or the WorkUnit-value list (the run walk reads it).
/// `place` routes onto both at once, so the single move is unambiguous.
/// `StoreKind` prepends the store list and passes the WU list through;
/// `WorkUnitKind` prepends the WU list and passes the store list
/// through; `UnitKind` and `PlatformKind` pass both through (drop the
/// value, its type stays tracked in the builder typestate).
///
/// The store-list GAT parameter is named `S` rather than `Sv`: `Sv` is
/// the cons-cell struct, and a parameter named `Sv` would shadow it.
pub trait Place<P> {
    /// The store-value list after placing `P` onto store list `S`.
    type NextStores<S: StoreValues>: StoreValues;

    /// The WorkUnit-value list after placing `P` onto WU list `W`.
    type NextWus<W>;

    /// Route `provider` onto the store list `sv` and the WU list `wv`,
    /// producing the next pair.
    fn place<S: StoreValues, W>(
        provider: P,
        sv: S,
        wv: W,
    ) -> (Self::NextStores<S>, Self::NextWus<W>);
}

impl<P> Place<P> for StoreKind {
    type NextStores<S: StoreValues> = Sv<P, S>;
    type NextWus<W> = W;

    #[inline]
    fn place<S: StoreValues, W>(provider: P, sv: S, wv: W) -> (Sv<P, S>, W) {
        (
            Sv {
                head: provider,
                tail: sv,
            },
            wv,
        )
    }
}

impl<P> Place<P> for WorkUnitKind {
    type NextStores<S: StoreValues> = S;
    type NextWus<W> = WuCons<P, W>;

    #[inline]
    fn place<S: StoreValues, W>(provider: P, sv: S, wv: W) -> (S, WuCons<P, W>) {
        (
            sv,
            WuCons {
                head: provider,
                tail: wv,
            },
        )
    }
}

impl<P> Place<P> for UnitKind {
    type NextStores<S: StoreValues> = S;
    type NextWus<W> = W;

    #[inline]
    fn place<S: StoreValues, W>(_provider: P, sv: S, wv: W) -> (S, W) {
        (sv, wv)
    }
}

impl<P> Place<P> for PlatformKind {
    type NextStores<S: StoreValues> = S;
    type NextWus<W> = W;

    #[inline]
    fn place<S: StoreValues, W>(_provider: P, sv: S, wv: W) -> (S, W) {
        (sv, wv)
    }
}
