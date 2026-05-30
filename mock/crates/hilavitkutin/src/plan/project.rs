//! Project a registered `WorkUnitBundle` into runtime `PlanInputs`.
//!
//! The plan stage consumes a `PlanInputs` (per-unit read/write access
//! masks, commutativity flags, unit and record counts). The scheduler
//! holds its work units as a type-level `WorkUnitBundle` and its stores
//! as a type-level `AccessSet`, but nothing turned those into the
//! runtime `PlanInputs` the plan stage needs. This module is that
//! projection: it is what lets the scheduler compute a plan from what
//! the consumer registered.
//!
//! The mechanism reuses the engine's frunk-style index witness (the
//! `Here` / `There` types and the disjoint head-vs-tail recursion that
//! `dispatch::engine_ctx`'s `Selector` / `Project` already ship). A
//! store's bit index is its position in the global `Stores` list; the
//! witness for that position infers at the call site, so no
//! const-carrying `IndexOf` (which would hit the marker-trait coherence
//! wall) is needed. `Selector` never needed the numeric index because it
//! pulls pointers by recursion; the mask projection does, hence the
//! `WitnessIndex` const here.

use arvo::strategy::Identity;
use arvo::USize;
use arvo_tensor::{cap_size, Capacity};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::WorkUnit;

use crate::dispatch::engine_ctx::{Here, There};

use super::{AccessMask, PlanInputs};

/// Compile-time ceiling: the skeleton `AccessMask` backs its bits in a
/// single `USize` word, so a store list wider than 64 would silently
/// drop high bits. Mirrors `DirtyMask`'s `_ASSERT_FITS_IN_USIZE`. The
/// associated const evaluates on monomorphisation only when referenced;
/// the projection entry points discharge it with `let _ = ...`. `CS` is
/// the store capacity.
struct StoreCeiling<CS: Capacity>(core::marker::PhantomData<CS>);

impl<CS: Capacity> StoreCeiling<CS> {
    const ASSERT_FITS: () = assert!( // lint:allow(no-bare-numeric) reason: const-context size assertion; tracked: #429
        cap_size(CS::CAP) <= 64,
        "projection: store capacity > 64 exceeds the skeleton AccessMask single-word backing (mirrors DirtyMask); widen when arvo-bitmask multi-container ships.",
    );
}

/// Runtime store-bit index carried by a peano position witness.
///
/// `Here` is index zero; `There<I>` is one past `I`. The existing
/// `Selector` / `Project` recursion never needed the numeric value
/// (it pulls pointers by structural recursion); the mask projection
/// needs it to pick which bit to set.
pub trait WitnessIndex {
    /// The store-bit index this witness names.
    const INDEX: USize;
}

impl WitnessIndex for Here {
    const INDEX: USize = USize::ZERO;
}

impl<I: WitnessIndex> WitnessIndex for There<I> {
    // lint:allow(no-bare-numeric) reason: peano successor on the inner index; tracked: #121
    const INDEX: USize = USize(I::INDEX.0 + 1);
}

/// Pure type-level "the cons-list contains `Target` at position
/// `Index`" witness over an `AccessSet` cons-list.
///
/// The disjoint head (`Here`) and tail (`There<I>`) impls never overlap
/// (the `Index` parameter discriminates them), so the solver infers
/// `Index` for a `(StoresList, Target)` pair. Unlike `Selector`, this is
/// driven by the list type alone, with no runtime value.
pub trait Locate<Target, Index> {}

impl<Target, Tail> Locate<Target, Here> for Cons<Target, Tail> {}

impl<Target, Head, Tail, I> Locate<Target, There<I>> for Cons<Head, Tail> where
    Tail: Locate<Target, I>
{
}

/// Fold an access set into an `AccessMask` by setting, for each member,
/// the bit at that member's located index within the global `Stores`
/// list.
///
/// `Indices` is the parallel witness cons-list (one position per
/// member); it infers at the call site exactly as `Project<R, Indices>`
/// infers its selector indices. `CS` is the store capacity.
pub trait MaskProject<Stores, Indices, CS: Capacity> {
    /// Set this access set's bits into `mask`, returning the result.
    fn project_mask(mask: AccessMask<CS>) -> AccessMask<CS>;
}

impl<Stores, CS: Capacity> MaskProject<Stores, Empty, CS> for Empty {
    #[inline]
    fn project_mask(mask: AccessMask<CS>) -> AccessMask<CS> {
        mask
    }
}

impl<Stores, M, Tail, I, ITail, CS: Capacity> MaskProject<Stores, Cons<I, ITail>, CS>
    for Cons<M, Tail>
where
    Stores: Locate<M, I>,
    I: WitnessIndex,
    Tail: MaskProject<Stores, ITail, CS>,
{
    #[inline]
    fn project_mask(mask: AccessMask<CS>) -> AccessMask<CS> {
        let mask = mask.set(I::INDEX);
        <Tail as MaskProject<Stores, ITail, CS>>::project_mask(mask)
    }
}

/// Walk a `WorkUnitBundle` cons-list, projecting each unit's `Read` and
/// `Write` access sets into `PlanInputs` at its position.
///
/// `Witnesses` is the parallel per-unit `(ReadIdx, WriteIdx)` witness
/// list (a cons-list of pairs, one pair per unit). Carried as a trait
/// parameter so each index stays constrained (the same shape
/// `Project<R, Indices>` uses to dodge an unconstrained-parameter
/// error); the whole nested list infers at the call site. `CU` is the
/// unit capacity, `CS` the store capacity.
pub trait BundleProject<Stores, Witnesses, CU: Capacity, CS: Capacity> {
    /// Fill `inputs` starting at unit index `idx`.
    fn project_bundle(inputs: &mut PlanInputs<CU, CS>, idx: USize);
}

impl<Stores, CU: Capacity, CS: Capacity> BundleProject<Stores, Empty, CU, CS> for Empty {
    #[inline]
    fn project_bundle(_inputs: &mut PlanInputs<CU, CS>, _idx: USize) {}
}

impl<Stores, W, T, RI, WI, WT, CU: Capacity, CS: Capacity>
    BundleProject<Stores, Cons<(RI, WI), WT>, CU, CS> for Cons<W, T>
where
    W: WorkUnit,
    W::Read: MaskProject<Stores, RI, CS>,
    W::Write: MaskProject<Stores, WI, CS>,
    T: BundleProject<Stores, WT, CU, CS>,
{
    fn project_bundle(inputs: &mut PlanInputs<CU, CS>, idx: USize) {
        let i = idx.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal array index; tracked: #121
        let reads = <W::Read as MaskProject<Stores, RI, CS>>::project_mask(AccessMask::empty());
        let writes = <W::Write as MaskProject<Stores, WI, CS>>::project_mask(AccessMask::empty());
        let mut access = reads;
        access.union_with(&writes);
        inputs.reads.as_mut()[i] = reads;
        inputs.writes.as_mut()[i] = writes;
        inputs.access.as_mut()[i] = access;
        inputs.commutative.as_mut()[i] = W::COMMUTATIVE;
        // lint:allow(no-bare-numeric) reason: unit-count successor; tracked: #121
        let next = USize(i + 1);
        inputs.unit_count = next;
        <T as BundleProject<Stores, WT, CU, CS>>::project_bundle(inputs, next);
    }
}

/// Project a single access set into an `AccessMask` over the given
/// global `Stores` list. `Indices` infers at the call site. Used by the
/// bundle projection per unit and directly testable without work units.
pub fn project_access_set<Set, Stores, Indices, CS: Capacity>() -> AccessMask<CS>
where
    Set: MaskProject<Stores, Indices, CS>,
{
    let _ = StoreCeiling::<CS>::ASSERT_FITS;
    <Set as MaskProject<Stores, Indices, CS>>::project_mask(AccessMask::empty())
}

/// Project a registered work-unit bundle into runtime `PlanInputs`.
///
/// `Wus` is the type-level `WorkUnitBundle`; `Stores` is the global
/// store `AccessSet` whose member positions are the mask bit indices;
/// `Witnesses` (inferred) is the parallel per-unit index list.
/// `record_count` is a runtime input, not a property of the bundle.
/// `CU` is the unit capacity, `CS` the store capacity.
///
/// The plan stage's `compute_execution_plan` consumes the result.
pub fn plan_inputs_from_bundle<Wus, Stores, Witnesses, CU: Capacity, CS: Capacity>(
    record_count: USize,
) -> PlanInputs<CU, CS>
where
    Wus: BundleProject<Stores, Witnesses, CU, CS>,
{
    let _ = StoreCeiling::<CS>::ASSERT_FITS;
    let mut inputs = PlanInputs::new();
    inputs.record_count = record_count;
    <Wus as BundleProject<Stores, Witnesses, CU, CS>>::project_bundle(&mut inputs, USize::ZERO);
    inputs
}
