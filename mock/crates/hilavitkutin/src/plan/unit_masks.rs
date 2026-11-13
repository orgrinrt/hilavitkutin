//! The per-unit masks of a registered bundle, folded into capacity arrays.
//!
//! The grouping and the reach cuts both start from these two folds.

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use arvo_tensor::{Capacity, ConstCapacity};

use super::access::AccessMask;
use super::bundle_masks::BundleMasks;

/// Fold the per-unit read/write masks and lifecycle ranks for a registered
/// bundle into `CU`-capacity locals, returning `(reads, writes, ranks,
/// unit_count)`.
pub(super) const fn masks_of<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
>() -> (
    <CU as ConstCapacity>::Array<AccessMask<CS>>,
    <CU as ConstCapacity>::Array<AccessMask<CS>>,
    <CU as ConstCapacity>::Array<USize>,
    USize,
)
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
{
    // The grouping scratch is a `CU`-capacity GAT array, sliced into the
    // slice-based `BundleMasks::fill` via the `ConstCapacity` slice bridge.
    // `fill` is slice-based, so the per-cons-cell `MaskProject` recursion is
    // capacity-blind (CU appears only here, never in the recursive
    // obligation); the P0.1b probe confirmed no trait-solver overflow. Consumers
    // index the returned arrays through `<CU as ConstCapacity>::get`.
    let mut reads = <CU as ConstCapacity>::filled(AccessMask::<CS>::empty());
    let mut writes = <CU as ConstCapacity>::filled(AccessMask::<CS>::empty());
    let mut ranks = <CU as ConstCapacity>::filled(<USize as Identity<Additive>>::IDENTITY);
    let n = <Wus as BundleMasks<Stores, Witnesses, CS>>::fill(
        <CU as ConstCapacity>::slice_mut(&mut reads),
        <CU as ConstCapacity>::slice_mut(&mut writes),
        <CU as ConstCapacity>::slice_mut(&mut ranks),
        <USize as Identity<Additive>>::IDENTITY,
    );
    (reads, writes, ranks, n)
}

/// Fold, per unit, the plain columns it reads with a reach.
pub(super) const fn reaches_of<
    Wus,
    Stores,
    Witnesses,
    CU: Capacity + [const] ConstCapacity,
    CS: Capacity,
>() -> <CU as ConstCapacity>::Array<AccessMask<CS>>
where
    Wus: [const] BundleMasks<Stores, Witnesses, CS>,
{
    let mut reaches = <CU as ConstCapacity>::filled(AccessMask::<CS>::empty());
    <Wus as BundleMasks<Stores, Witnesses, CS>>::fill_reaches(
        <CU as ConstCapacity>::slice_mut(&mut reaches),
        <USize as Identity<Additive>>::IDENTITY,
    );
    reaches
}
