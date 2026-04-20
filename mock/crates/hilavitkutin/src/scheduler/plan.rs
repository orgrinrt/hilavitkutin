//! Execution plan meta-resource.
//!
//! Per-lane WU assignment; cached and reused across frames.

#[derive(Copy, Clone, Debug, Default)]
pub struct LaneAssignment {
    pub lane_id: u32,
    pub first_unit: u32,
    pub unit_count: u32,
}

#[derive(Copy, Clone)]
pub struct ExecutionPlan<const MAX_LANES: usize> {
    pub lanes: [LaneAssignment; MAX_LANES],
    pub count: usize,
}

impl<const MAX_LANES: usize> ExecutionPlan<MAX_LANES> {
    pub const fn new() -> Self {
        Self {
            lanes: [LaneAssignment {
                lane_id: 0,
                first_unit: 0,
                unit_count: 0,
            }; MAX_LANES],
            count: 0,
        }
    }
}

impl<const MAX_LANES: usize> Default for ExecutionPlan<MAX_LANES> {
    fn default() -> Self {
        Self::new()
    }
}
