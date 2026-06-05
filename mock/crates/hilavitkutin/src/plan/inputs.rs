//! Plan inputs: the descriptor bundle `build_plan` consumes.
//!
//! Skeleton: const arrays of AccessMask per unit + record count
//! estimate + commutativity flags. Populated by the scheduler
//! builder during WU registration (domain 11).
//!
//! `UnitId` re-exported via `crate::plan` from `hilavitkutin_api`
//! (USize-shaped, canonical engine id type).

use arvo::strategy::Identity;
use arvo::{Bool, USize};
use arvo_tensor::Capacity;

use super::access::AccessMask;

/// Descriptor bundle for `build_plan`. `CU` is the unit capacity
/// (number of WUs); `CS` is the store capacity (number of distinct
/// stores accessible to any unit).
pub struct PlanInputs<CU: Capacity, CS: Capacity> {
    /// Union of read + write stores per unit.
    pub access: <CU as Capacity>::Array<AccessMask<CS>>,
    /// Write-only mask per unit.
    pub writes: <CU as Capacity>::Array<AccessMask<CS>>,
    /// Read-only mask per unit.
    pub reads: <CU as Capacity>::Array<AccessMask<CS>>,
    /// Commutativity flag per unit (COMMUTATIVE scheduling hint).
    pub commutative: <CU as Capacity>::Array<Bool>,
    /// Number of units actually populated (0..=unit capacity).
    pub unit_count: USize,
    /// Estimated record count per frame. Drives strategy
    /// selection (domain 21) and morsel sizing (domain 12).
    pub record_count: USize,
    /// Accumulator-store positions in the global `Stores` list, in the
    /// same bit space as `writes`. `writes[u].overlaps(&accum_stores)`
    /// is true iff unit `u` writes an accumulator, the per-fiber
    /// morsel-locality signal.
    pub accum_stores: AccessMask<CS>,
}

impl<CU: Capacity, CS: Capacity> PlanInputs<CU, CS> {
    /// Zero-filled default: no units registered, no records.
    pub fn new() -> Self {
        Self {
            access: <CU as Capacity>::filled(AccessMask::empty()),
            writes: <CU as Capacity>::filled(AccessMask::empty()),
            reads: <CU as Capacity>::filled(AccessMask::empty()),
            commutative: <CU as Capacity>::filled(Bool::FALSE),
            unit_count: USize::ZERO,
            record_count: USize::ZERO,
            accum_stores: AccessMask::empty(),
        }
    }
}

impl<CU: Capacity, CS: Capacity> Copy for PlanInputs<CU, CS>
where
    <CU as Capacity>::Array<AccessMask<CS>>: Copy,
    <CU as Capacity>::Array<Bool>: Copy,
{
}

impl<CU: Capacity, CS: Capacity> Clone for PlanInputs<CU, CS>
where
    <CU as Capacity>::Array<AccessMask<CS>>: Copy,
    <CU as Capacity>::Array<Bool>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<CU: Capacity, CS: Capacity> Default for PlanInputs<CU, CS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<CU: Capacity, CS: Capacity> core::fmt::Debug for PlanInputs<CU, CS> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PlanInputs")
            .field("unit_count", &self.unit_count.0)
            .field("record_count", &self.record_count.0)
            .finish_non_exhaustive()
    }
}
