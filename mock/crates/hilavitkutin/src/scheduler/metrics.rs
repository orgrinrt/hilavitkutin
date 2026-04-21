//! Scheduler metrics resource (domain 22 read surface).
//!
//! Frame timing + morsel dispatch counters consumers read for
//! adaptive cadence decisions.

use arvo::ufixed::UFixed;
use arvo::USize;
use hilavitkutin_api::Nanos;

#[derive(Copy, Clone)]
pub struct SchedulerMetrics {
    pub frame_time_ns: Nanos,
    pub morsels_dispatched: USize,
    pub changes_detected: USize,
}

impl SchedulerMetrics {
    pub const fn new() -> Self {
        Self {
            frame_time_ns: UFixed::from_raw(0),
            morsels_dispatched: USize(0),
            changes_detected: USize(0),
        }
    }
}

impl Default for SchedulerMetrics {
    fn default() -> Self {
        Self::new()
    }
}
