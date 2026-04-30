//! Plan inputs: the descriptor bundle `build_plan` consumes.
//!
//! Skeleton: const arrays of AccessMask per unit + record count
//! estimate + commutativity flags. Populated by the scheduler
//! builder during WU registration (domain 11).

use arvo::{Bool, USize};

use super::access::AccessMask;

/// Newtype wrapping a unit (WorkUnit) index. `#[repr(transparent)]`
/// so it round-trips through FFI cleanly.
///
/// Named `UnitId` (not `NodeId`) to avoid a name collision with
/// `arvo_bitmask::NodeId` at plan-stage integration sites.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct UnitId(pub u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: domain id newtype; bit-width fixed at 32; exact-width refinement tracked: #72

/// Descriptor bundle for `build_plan`. `MAX_UNITS` bounds the
/// number of WUs; `MAX_STORES` bounds the number of distinct
/// stores accessible to any unit.
#[derive(Copy, Clone, Debug)]
pub struct PlanInputs<const MAX_UNITS: usize, const MAX_STORES: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// Union of read + write stores per unit.
    pub access: [AccessMask<MAX_STORES>; MAX_UNITS],
    /// Write-only mask per unit.
    pub writes: [AccessMask<MAX_STORES>; MAX_UNITS],
    /// Read-only mask per unit.
    pub reads: [AccessMask<MAX_STORES>; MAX_UNITS],
    /// Commutativity flag per unit (COMMUTATIVE scheduling hint).
    pub commutative: [Bool; MAX_UNITS],
    /// Number of units actually populated (0..=MAX_UNITS).
    pub unit_count: USize,
    /// Estimated record count per frame. Drives strategy
    /// selection (domain 21) and morsel sizing (domain 12).
    pub record_count: USize,
}

impl<const MAX_UNITS: usize, const MAX_STORES: usize> PlanInputs<MAX_UNITS, MAX_STORES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// Zero-filled default: no units registered, no records.
    pub const fn new() -> Self {
        Self {
            access: [AccessMask::empty(); MAX_UNITS],
            writes: [AccessMask::empty(); MAX_UNITS],
            reads: [AccessMask::empty(); MAX_UNITS],
            commutative: [Bool::FALSE; MAX_UNITS],
            unit_count: USize(0),
            record_count: USize(0),
        }
    }
}

impl<const MAX_UNITS: usize, const MAX_STORES: usize> Default // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    for PlanInputs<MAX_UNITS, MAX_STORES>
{
    fn default() -> Self {
        Self::new()
    }
}
