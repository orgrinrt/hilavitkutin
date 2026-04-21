//! Fiber grouping: per-node fiber assignment (domain 14).
//!
//! A fiber is a holistically-feasible contiguous run of WUs
//! sharing a morsel arena. Fiber formation is the output of
//! steps 5-8 in the plan-stage algorithm.

use arvo::USize;

/// Newtype wrapping a fiber index. `#[repr(transparent)]` for
/// stable FFI. u16 is plenty — even a 64-core plan rarely exceeds
/// 100 fibers.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FiberId(pub u16); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) lint:allow(no-public-raw-field) reason: domain id newtype; bit-width fixed at 16; exact-width refinement tracked: #72

/// Per-node fiber assignment.
#[derive(Copy, Clone, Debug)]
pub struct FiberGrouping<const MAX_UNITS: usize, const MAX_FIBERS: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// `assignment[i]` is the FiberId that node `i` belongs to.
    pub assignment: [FiberId; MAX_UNITS],
    /// Number of fibers actually used (0..=MAX_FIBERS).
    pub fiber_count: USize,
}

impl<const MAX_UNITS: usize, const MAX_FIBERS: usize> FiberGrouping<MAX_UNITS, MAX_FIBERS> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// All nodes assigned to fiber 0, fiber_count = 0.
    pub const fn new() -> Self {
        Self {
            assignment: [FiberId(0); MAX_UNITS],
            fiber_count: USize(0),
        }
    }
}

impl<const MAX_UNITS: usize, const MAX_FIBERS: usize> Default // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    for FiberGrouping<MAX_UNITS, MAX_FIBERS>
{
    fn default() -> Self {
        Self::new()
    }
}
