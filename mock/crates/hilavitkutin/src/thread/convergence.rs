//! Head+tail convergence record (domain 20).
//!
//! Two threads process the same commutative fiber from opposite
//! ends. Whichever thread crosses the midpoint first publishes
//! the meeting record via `meeting_record.store`; the other
//! observes via `meeting_record.load` and stops its half.

use super::ThreadHandle;
use crate::dispatch::ProgressCounter;

/// Head+tail convergence record.
pub struct Convergence {
    pub head_thread: ThreadHandle,
    pub tail_thread: ThreadHandle,
    pub meeting_record: ProgressCounter,
}

impl Convergence {
    /// Construct a fresh convergence record with meeting counter
    /// at zero.
    pub const fn new(head: ThreadHandle, tail: ThreadHandle) -> Self {
        Self {
            head_thread: head,
            tail_thread: tail,
            meeting_record: ProgressCounter::new(0),
        }
    }
}
