//! Execution plan meta-resource.
//!
//! Per-lane WU assignment; cached and reused across frames.

use arvo::USize;

#[derive(Copy, Clone, Debug)]
pub struct LaneAssignment {
    pub lane_id: USize,
    pub first_unit: USize,
    pub unit_count: USize,
}

#[derive(Copy, Clone)]
pub struct ExecutionPlan<const MAX_LANES: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pub lanes: [LaneAssignment; MAX_LANES],
    pub count: USize,
}

impl<const MAX_LANES: usize> ExecutionPlan<MAX_LANES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pub const fn new() -> Self {
        Self {
            lanes: [LaneAssignment {
                lane_id: USize(0),
                first_unit: USize(0),
                unit_count: USize(0),
            }; MAX_LANES],
            count: USize(0),
        }
    }
}

impl<const MAX_LANES: usize> Default for ExecutionPlan<MAX_LANES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    fn default() -> Self {
        Self::new()
    }
}
