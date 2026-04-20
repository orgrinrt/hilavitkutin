//! Fiber grouping: per-node fiber assignment (domain 14).
//!
//! A fiber is a holistically-feasible contiguous run of WUs
//! sharing a morsel arena. Fiber formation is the output of
//! steps 5-8 in the plan-stage algorithm.

/// Newtype wrapping a fiber index. `#[repr(transparent)]` for
/// stable FFI. u16 is plenty — even a 64-core plan rarely exceeds
/// 100 fibers.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FiberId(pub u16);

/// Per-node fiber assignment.
#[derive(Copy, Clone, Debug)]
pub struct FiberGrouping<const MAX_UNITS: usize, const MAX_FIBERS: usize> {
    /// `assignment[i]` is the FiberId that node `i` belongs to.
    pub assignment: [FiberId; MAX_UNITS],
    /// Number of fibers actually used (0..=MAX_FIBERS).
    pub fiber_count: u16,
}

impl<const MAX_UNITS: usize, const MAX_FIBERS: usize> FiberGrouping<MAX_UNITS, MAX_FIBERS> {
    /// All nodes assigned to fiber 0, fiber_count = 0.
    pub const fn new() -> Self {
        Self {
            assignment: [FiberId(0); MAX_UNITS],
            fiber_count: 0,
        }
    }
}

impl<const MAX_UNITS: usize, const MAX_FIBERS: usize> Default
    for FiberGrouping<MAX_UNITS, MAX_FIBERS>
{
    fn default() -> Self {
        Self::new()
    }
}
