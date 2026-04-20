//! Scheduler metrics resource (domain 22 read surface).
//!
//! Frame timing + morsel dispatch counters consumers read for
//! adaptive cadence decisions.

#[derive(Copy, Clone, Debug, Default)]
pub struct SchedulerMetrics {
    pub frame_time_ns: u64,
    pub morsels_dispatched: u64,
    pub changes_detected: u64,
}

impl SchedulerMetrics {
    pub const fn new() -> Self {
        Self {
            frame_time_ns: 0,
            morsels_dispatched: 0,
            changes_detected: 0,
        }
    }
}
