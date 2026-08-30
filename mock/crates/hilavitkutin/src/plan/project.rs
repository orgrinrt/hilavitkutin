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
use arvo::{Bool, USize};
use arvo_tensor::{Capacity, cap_size};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::footprint::ResourceFootprint;
use hilavitkutin_api::store::{Accum, Column, Resource, StagedResource, Virtual};
use hilavitkutin_api::{HasSchedule, WorkUnit};

use crate::dispatch::engine_ctx::{Here, There};

use super::inputs::MorselBudget;
use super::{AccessMask, PlanInputs};

/// Compile-time ceiling: the skeleton `AccessMask` backs its bits in a
/// single `USize` word, so a store list wider than 64 would silently
/// drop high bits. Mirrors `DirtyMask`'s `_ASSERT_FITS_IN_USIZE`. The
/// associated const evaluates on monomorphisation only when referenced;
/// the projection entry points discharge it with `let _ = ...`. `CS` is
/// the store capacity.
struct StoreCeiling<CS: Capacity>(core::marker::PhantomData<CS>);

impl<CS: Capacity> StoreCeiling<CS> {
    const ASSERT_FITS: () = assert!(
        // lint:allow(no-bare-numeric) reason: const-context size assertion; tracked: #429
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
pub const trait MaskProject<Stores, Indices, CS: Capacity> {
    /// Set this access set's bits into `mask`, returning the result.
    fn project_mask(mask: AccessMask<CS>) -> AccessMask<CS>;
}

const impl<Stores, CS: Capacity> MaskProject<Stores, Empty, CS> for Empty {
    #[inline]
    fn project_mask(mask: AccessMask<CS>) -> AccessMask<CS> {
        mask
    }
}

const impl<Stores, M, Tail, I, ITail, CS: Capacity> MaskProject<Stores, Cons<I, ITail>, CS>
    for Cons<M, Tail>
where
    Stores: Locate<M, I>,
    I: WitnessIndex,
    Tail: [const] MaskProject<Stores, ITail, CS>,
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

// E4 slice 1: schedule-recovered so an `On<V>` unit projects its masks too.
impl<Stores, W, T, RI, WI, WT, CU: Capacity, CS: Capacity>
    BundleProject<Stores, Cons<(RI, WI), WT>, CU, CS> for Cons<W, T>
where
    W: HasSchedule + WorkUnit<<W as HasSchedule>::Sched>,
    <W as WorkUnit<<W as HasSchedule>::Sched>>::Read: MaskProject<Stores, RI, CS>,
    <W as WorkUnit<<W as HasSchedule>::Sched>>::Write: MaskProject<Stores, WI, CS>,
    T: BundleProject<Stores, WT, CU, CS>,
{
    fn project_bundle(inputs: &mut PlanInputs<CU, CS>, idx: USize) {
        let i = idx.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal array index; tracked: #121
        let reads = <<W as WorkUnit<<W as HasSchedule>::Sched>>::Read as MaskProject<
            Stores,
            RI,
            CS,
        >>::project_mask(AccessMask::empty());
        let writes = <<W as WorkUnit<<W as HasSchedule>::Sched>>::Write as MaskProject<
            Stores,
            WI,
            CS,
        >>::project_mask(AccessMask::empty());
        let mut access = reads;
        access.union_with(&writes);
        inputs.reads.as_mut()[i] = reads;
        inputs.writes.as_mut()[i] = writes;
        inputs.access.as_mut()[i] = access;
        inputs.commutative.as_mut()[i] = <W as WorkUnit<<W as HasSchedule>::Sched>>::COMMUTATIVE;
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

/// Per-store-marker classification: is this store an accumulator?
///
/// Disjoint concrete impls (no blanket) keep the fold clear of the
/// marker-trait coherence wall the module already documents: a blanket
/// `false` plus a specific `Accum` `true` would overlap and demand
/// specialization. Each store marker states its own kind.
pub trait StoreAccumKind {
    /// `Bool::TRUE` only for `Accum<T>`; every other store marker is `FALSE`.
    const IS_ACCUM: Bool;
}

impl<T> StoreAccumKind for Accum<T> {
    const IS_ACCUM: Bool = Bool::TRUE;
}

impl<T> StoreAccumKind for Column<T> {
    const IS_ACCUM: Bool = Bool::FALSE;
}

impl<T> StoreAccumKind for Resource<T> {
    const IS_ACCUM: Bool = Bool::FALSE;
}

impl<T> StoreAccumKind for StagedResource<T> {
    const IS_ACCUM: Bool = Bool::FALSE;
}

impl<T> StoreAccumKind for Virtual<T> {
    const IS_ACCUM: Bool = Bool::FALSE;
}

/// Fold the global `Stores` cons-list into an `AccessMask` marking the
/// position of every accumulator store.
///
/// The bit positions are the store's index in the global `Stores` list,
/// the same Stores-list-position space `MaskProject` uses for the
/// per-unit access masks, so `writes[u].overlaps(&accum_mask)` is a
/// sound test for "unit `u` writes an accumulator." This is why the mask
/// is folded over `Stores` here and not recorded at the store drain,
/// whose `StoreId` space skips zero-sized resources and so does not match
/// the access-mask space.
pub trait AccumStoresMask<CS: Capacity> {
    /// Set the accumulator-position bits into `mask`, walking from store
    /// position `idx`.
    fn accum_mask(mask: AccessMask<CS>, idx: USize) -> AccessMask<CS>;
}

impl<CS: Capacity> AccumStoresMask<CS> for Empty {
    #[inline]
    fn accum_mask(mask: AccessMask<CS>, _idx: USize) -> AccessMask<CS> {
        mask
    }
}

impl<H: StoreAccumKind, T: AccumStoresMask<CS>, CS: Capacity> AccumStoresMask<CS> for Cons<H, T> {
    #[inline]
    fn accum_mask(mask: AccessMask<CS>, idx: USize) -> AccessMask<CS> {
        let mask = if H::IS_ACCUM.0 { mask.set(idx) } else { mask };
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: store-position successor on the fold index; tracked: #121
        <T as AccumStoresMask<CS>>::accum_mask(mask, USize(idx.0 + 1))
    }
}

/// Build the accumulator-store mask for a global `Stores` list.
///
/// `Stores` is the same global store `AccessSet` whose positions the
/// per-unit access masks index. `CS` is the store capacity.
pub fn accum_stores_mask<Stores, CS: Capacity>() -> AccessMask<CS>
where
    Stores: AccumStoresMask<CS>,
{
    let _ = StoreCeiling::<CS>::ASSERT_FITS;
    <Stores as AccumStoresMask<CS>>::accum_mask(AccessMask::empty(), USize::ZERO)
}

/// Element byte size of a store marker's value type, `ceil(BIT_WIDTH / 8)`.
///
/// Disjoint concrete impls per store-marker shape (no blanket), mirroring
/// `StoreAccumKind`, so the `StoreSizes` fold stays clear of the marker-trait
/// coherence wall a blanket-plus-specific pair would hit. Each data-bearing
/// marker pulls its inner `T` and reads `ColumnValue::BIT_WIDTH` (the spec hook,
/// not raw `size_of`, so this stays correct once sub-byte bitpacking makes them
/// diverge). `Virtual<T>` is a fired marker carrying no record bytes, so zero.
pub trait StoreElemBytes {
    /// `ceil(<T as ColumnValue>::BIT_WIDTH / 8)` for this store's element type.
    const BYTES: USize;
}

/// Round a bit count up to whole bytes.
const fn bytes_of_bits(bits: USize) -> USize {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: byte-ceil arithmetic on the const bit width; tracked: #121
    USize((bits.0 + 7) / 8)
}

impl<T: ColumnValue> StoreElemBytes for Column<T> {
    const BYTES: USize = bytes_of_bits(<T as ColumnValue>::BIT_WIDTH);
}
// A resource value's L1-morsel contribution is its Seq/Map collection
// footprint (canonical R5 via `ResourceFootprint`); Field scalars ride
// the register budget, so a bare-scalar resource reports 0.
impl<T: ResourceFootprint> StoreElemBytes for Resource<T> {
    const BYTES: USize = <T as ResourceFootprint>::L1_BYTES;
}
// Accumulator-bearing fibers dispatch unit-outer, off the morsel-window
// L1 budget entirely.
impl<T> StoreElemBytes for Accum<T> {
    const BYTES: USize = USize::ZERO;
}
impl<T: ResourceFootprint> StoreElemBytes for StagedResource<T> {
    const BYTES: USize = <T as ResourceFootprint>::L1_BYTES;
}
impl<T> StoreElemBytes for Virtual<T> {
    const BYTES: USize = USize::ZERO;
}

/// Fold the global `Stores` cons-list into a `CS`-capacity byte-size array:
/// `out[i]` = element byte size of store `i`. Mirrors `AccumStoresMask`'s
/// per-store walk, writing a size into a slot instead of setting a mask bit.
/// The bit/slot positions are the store's index in the global `Stores` list,
/// the same position space the per-unit write masks index, so a fiber's write
/// mask selects the right size slots.
pub trait StoreSizes<CS: Capacity> {
    /// Write each store's byte size into `out`, walking from store position `idx`.
    fn fill_sizes(out: &mut [USize], idx: USize);
}

impl<CS: Capacity> StoreSizes<CS> for Empty {
    #[inline]
    fn fill_sizes(_out: &mut [USize], _idx: USize) {}
}

impl<H: StoreElemBytes, T: StoreSizes<CS>, CS: Capacity> StoreSizes<CS> for Cons<H, T> {
    #[inline]
    fn fill_sizes(out: &mut [USize], idx: USize) {
        if idx.0 < out.len() {
            out[idx.0] = <H as StoreElemBytes>::BYTES;
        }
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: store-position successor on the fold index; tracked: #121
        <T as StoreSizes<CS>>::fill_sizes(out, USize(idx.0 + 1));
    }
}

/// Build the per-store element-byte-size array for a global `Stores` list.
///
/// `Stores` is the same global store `AccessSet` whose positions the per-unit
/// access masks index; `CS` is the store capacity. The per-fiber write-byte sum
/// (A3b) walks a fiber's write `AccessMask<CS>` against this array.
pub fn store_sizes<Stores, CS: Capacity>() -> <CS as Capacity>::Array<USize>
where
    Stores: StoreSizes<CS>,
    <CS as Capacity>::Array<USize>: Copy,
{
    let _ = StoreCeiling::<CS>::ASSERT_FITS;
    let mut arr = <CS as Capacity>::filled(USize::ZERO);
    <Stores as StoreSizes<CS>>::fill_sizes(arr.as_mut(), USize::ZERO);
    arr
}

/// Project a registered work-unit bundle into runtime `PlanInputs`.
///
/// `Wus` is the type-level `WorkUnitBundle`; `Stores` is the global
/// store `AccessSet` whose member positions are the mask bit indices;
/// `Witnesses` (inferred) is the parallel per-unit index list.
/// `record_count` and the morsel `budget` are runtime inputs, not
/// properties of the bundle. `CU` is the unit capacity, `CS` the store
/// capacity.
///
/// The plan stage's `compute_execution_plan` consumes the result.
pub fn plan_inputs_from_bundle<Wus, Stores, Witnesses, CU: Capacity, CS: Capacity>(
    record_count: USize,
    budget: MorselBudget,
) -> PlanInputs<CU, CS>
where
    Wus: BundleProject<Stores, Witnesses, CU, CS>,
    Stores: AccumStoresMask<CS>,
    Stores: StoreSizes<CS>,
{
    let _ = StoreCeiling::<CS>::ASSERT_FITS;
    let mut inputs = PlanInputs::new();
    inputs.record_count = record_count;
    inputs.accum_stores = accum_stores_mask::<Stores, CS>();
    inputs.morsel_budget = budget;
    <Stores as StoreSizes<CS>>::fill_sizes(inputs.store_sizes.as_mut(), USize::ZERO);
    <Wus as BundleProject<Stores, Witnesses, CU, CS>>::project_bundle(&mut inputs, USize::ZERO);
    inputs
}
