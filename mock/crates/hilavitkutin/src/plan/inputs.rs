//! Plan inputs: the descriptor bundle `build_plan` consumes.
//!
//! Skeleton: const arrays of AccessMask per unit + record count
//! estimate + commutativity flags. Populated by the scheduler
//! builder during WU registration (domain 11).

use super::access::AccessMask;

/// Newtype wrapping a unit (WorkUnit) index. `#[repr(transparent)]`
/// so it round-trips through FFI cleanly.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct NodeId(pub u32);

/// Descriptor bundle for `build_plan`. `MAX_UNITS` bounds the
/// number of WUs; `MAX_STORES` bounds the number of distinct
/// stores accessible to any unit.
#[derive(Copy, Clone, Debug)]
pub struct PlanInputs<const MAX_UNITS: usize, const MAX_STORES: usize> {
    /// Union of read + write stores per unit.
    pub access: [AccessMask<MAX_STORES>; MAX_UNITS],
    /// Write-only mask per unit.
    pub writes: [AccessMask<MAX_STORES>; MAX_UNITS],
    /// Read-only mask per unit.
    pub reads: [AccessMask<MAX_STORES>; MAX_UNITS],
    /// Commutativity flag per unit (COMMUTATIVE scheduling hint).
    pub commutative: [bool; MAX_UNITS],
    /// Number of units actually populated (0..=MAX_UNITS).
    pub unit_count: usize,
    /// Estimated record count per frame. Drives strategy
    /// selection (domain 21) and morsel sizing (domain 12).
    pub record_count: u64,
}

impl<const MAX_UNITS: usize, const MAX_STORES: usize> PlanInputs<MAX_UNITS, MAX_STORES> {
    /// Zero-filled default: no units registered, no records.
    pub const fn new() -> Self {
        Self {
            access: [AccessMask::empty(); MAX_UNITS],
            writes: [AccessMask::empty(); MAX_UNITS],
            reads: [AccessMask::empty(); MAX_UNITS],
            commutative: [false; MAX_UNITS],
            unit_count: 0,
            record_count: 0,
        }
    }
}

impl<const MAX_UNITS: usize, const MAX_STORES: usize> Default
    for PlanInputs<MAX_UNITS, MAX_STORES>
{
    fn default() -> Self {
        Self::new()
    }
}
