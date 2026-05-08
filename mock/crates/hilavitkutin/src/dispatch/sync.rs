//! Phase-join sync points (domain 17).
//!
//! A SyncPoint is the gate phase N+1 reads to decide whether
//! phase N has produced enough records to start the next morsel.

use arvo::USize;

use crate::plan::FiberId;

/// Phase-join gate. `fiber_id` is the producing fiber, `min_records`
/// is the record count the consumer waits for before running.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SyncPoint {
    pub fiber_id: FiberId,
    pub min_records: USize,
}

impl SyncPoint {
    /// Construct a new sync point with the given producer and
    /// minimum-record threshold.
    pub const fn new(fiber_id: FiberId, min_records: USize) -> Self {
        Self {
            fiber_id,
            min_records,
        }
    }
}
