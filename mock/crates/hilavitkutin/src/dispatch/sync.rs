//! Phase-join sync points (domain 17).
//!
//! A SyncPoint is the gate phase N+1 reads to decide whether
//! phase N has produced enough records to start the next morsel.

use crate::plan::FiberId;

/// Phase-join gate. `fiber_id` is the producing fiber, `min_records`
/// is the record count the consumer waits for before running.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct SyncPoint {
    pub fiber_id: FiberId,
    pub min_records: u64,
}

impl SyncPoint {
    /// Construct a new sync point with the given producer and
    /// minimum-record threshold.
    pub const fn new(fiber_id: FiberId, min_records: u64) -> Self {
        Self {
            fiber_id,
            min_records,
        }
    }
}
