//! The const fold of a registered bundle into per-unit masks and ranks.
//!
//! The const grouping reads three things of each unit: the stores its reads
//! wait on, the stores it writes, and its lifecycle rank. One more fold gives
//! the stores a unit reads with a reach, which the phase boundaries and the
//! build-time reach rule read. Both walk the carrier the same way, over the
//! builder's `Cons` bundle and over the scheduler's `WuCons` value list.

use arvo::USize;
use arvo_tensor::Capacity;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::meta::RANK_CONSUMER;
use hilavitkutin_api::work_unit::{HasSchedule, Lifecycle, WorkUnit};
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};

use super::access::AccessMask;
use super::project::MaskProject;

/// What the grouping fold needs from each unit: its read and write access sets.
///
/// Blanket-implemented for every `WorkUnit`, so a registered work-unit bundle
/// satisfies it without any per-unit boilerplate. The indirection lets the
/// grouping be exercised by lightweight fixtures (access sets without the full
/// dispatch machinery) while the engine folds over real units.
pub trait UnitAccess {
    /// The unit's read access set.
    type Read;
    /// The unit's write access set.
    type Write;
    /// The unit's lifecycle rank (E4 slice 2): the outer phase key the grouping
    /// renumbers by, so plan-stage meta units land before consumers and the
    /// schedule-end epilogue after. Defaults to consumer rank, so lightweight
    /// access-set fixtures (which do not carry a schedule) are consumer-ranked.
    const RANK: USize = RANK_CONSUMER;
}

// E4 slice 1: schedule-recovered so an `On<V>` unit (which impls
// `WorkUnit<On<V>>`, not `WorkUnit<Always>`) is also covered by the blanket.
// E4 slice 2: RANK reads the schedule's lifecycle rank (Always / On<V> are
// consumer-rank; OnMeta<V> takes its meta virtual's rank).
impl<W: HasSchedule + WorkUnit<<W as HasSchedule>::Sched>> UnitAccess for W {
    type Read = <W as WorkUnit<<W as HasSchedule>::Sched>>::Read;
    type Write = <W as WorkUnit<<W as HasSchedule>::Sched>>::Write;

    const RANK: USize = <<W as HasSchedule>::Sched as Lifecycle>::RANK;
}

/// Const fold of a registered bundle's per-unit masks.
///
/// The const analog of the runtime `BundleProject`: for each unit it projects
/// its read and write access sets into `AccessMask`s via the const `MaskProject`
/// and writes them at the unit's carrier position. `Witnesses` is the parallel
/// per-unit `(ReadIdx, WriteIdx)` projection-index list (the `BundleProject`
/// shape), threaded so each `MaskProject` index stays constrained. The mask
/// arrays are slices, so the trait carries no array-length parameter.
pub const trait BundleMasks<Stores, Witnesses, CS: Capacity> {
    /// Fill per-unit read and write masks and lifecycle ranks from carrier
    /// position `idx`, returning the next free position (so the top-level
    /// caller learns the unit count).
    ///
    /// `reads` takes the stores a unit's reads wait on: every store of its read
    /// set but the buffered columns, whose reads see the frozen side and so
    /// give neither an edge nor a conflict with a writer. The rank (E4 slice 2)
    /// is `UnitAccess::RANK`, the outer phase key; folding it here reuses the
    /// carrier walk the masks already do.
    fn fill(
        reads: &mut [AccessMask<CS>],
        writes: &mut [AccessMask<CS>],
        ranks: &mut [USize],
        idx: USize,
    ) -> USize;

    /// Fill, per unit, the plain columns it reads with a reach, from carrier
    /// position `idx`, returning the next free position.
    fn fill_reaches(reaches: &mut [AccessMask<CS>], idx: USize) -> USize;
}

// One pair of impls per carrier. The builder holds its units as the `Cons`
// bundle; `Scheduler::run` holds them as `WuVals` (a `WuCons` list of values),
// so the grouping at dispatch folds that shape. Only the cell type differs.
macro_rules! bundle_masks {
    ($nil:ty, $cons:ident) => {
        const impl<Stores, CS: Capacity> BundleMasks<Stores, Empty, CS> for $nil {
            #[inline]
            fn fill(
                _reads: &mut [AccessMask<CS>],
                _writes: &mut [AccessMask<CS>],
                _ranks: &mut [USize],
                idx: USize,
            ) -> USize {
                idx
            }

            #[inline]
            fn fill_reaches(_reaches: &mut [AccessMask<CS>], idx: USize) -> USize {
                idx
            }
        }

        const impl<Stores, Un, T, RI, WI, WT, CS: Capacity>
            BundleMasks<Stores, Cons<(RI, WI), WT>, CS> for $cons<Un, T>
        where
            Un: UnitAccess,
            Un::Read: [const] MaskProject<Stores, RI, CS>,
            Un::Write: [const] MaskProject<Stores, WI, CS>,
            T: [const] BundleMasks<Stores, WT, CS>,
        {
            #[inline]
            fn fill(
                reads: &mut [AccessMask<CS>],
                writes: &mut [AccessMask<CS>],
                ranks: &mut [USize],
                idx: USize,
            ) -> USize {
                let i = idx.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: slice index; tracked: #121
                reads[i] =
                    <Un::Read as MaskProject<Stores, RI, CS>>::project_waits(AccessMask::empty());
                writes[i] =
                    <Un::Write as MaskProject<Stores, WI, CS>>::project_mask(AccessMask::empty());
                ranks[i] = <Un as UnitAccess>::RANK;
                <T as BundleMasks<Stores, WT, CS>>::fill(reads, writes, ranks, USize(i + 1)) // lint:allow(no-bare-numeric) reason: carrier-position successor; tracked: #121
            }

            #[inline]
            fn fill_reaches(reaches: &mut [AccessMask<CS>], idx: USize) -> USize {
                let i = idx.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: slice index; tracked: #121
                reaches[i] =
                    <Un::Read as MaskProject<Stores, RI, CS>>::project_reaches(AccessMask::empty());
                <T as BundleMasks<Stores, WT, CS>>::fill_reaches(reaches, USize(i + 1)) // lint:allow(no-bare-numeric) reason: carrier-position successor; tracked: #121
            }
        }
    };
}

bundle_masks!(Empty, Cons);
bundle_masks!(WuNil, WuCons);
