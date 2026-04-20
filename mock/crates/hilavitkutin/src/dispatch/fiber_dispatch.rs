//! Per-fiber dispatch record (domain 17).
//!
//! Pairs a monomorphised function pointer with the metadata the
//! engine needs to drive one fiber through its morsel range under
//! the right sync conditions.

use super::{MorselRange, SyncPoint, WuFn};
use crate::plan::{FiberId, PhaseId};

/// Per-fiber dispatch record.
///
/// `MAX_CORES` bounds the sync-point array length: each fiber has
/// at most one SyncPoint per core that could run the producer
/// phase before it.
pub struct FiberDispatch<Ctx: 'static, const MAX_CORES: usize> {
    /// Monomorphised body. `None` in skeleton state.
    pub body: Option<WuFn<Ctx>>,
    pub fiber_id: FiberId,
    pub phase: PhaseId,
    pub morsel_range: MorselRange,
    pub sync_points: [SyncPoint; MAX_CORES],
    pub sync_point_count: u8,
}

impl<Ctx: 'static, const MAX_CORES: usize> FiberDispatch<Ctx, MAX_CORES> {
    /// Empty skeleton record with no body and zero metadata.
    pub const fn new() -> Self {
        Self {
            body: None,
            fiber_id: FiberId(0),
            phase: PhaseId(0),
            morsel_range: MorselRange {
                start: 0,
                len: 0,
            },
            sync_points: [SyncPoint {
                fiber_id: FiberId(0),
                min_records: 0,
            }; MAX_CORES],
            sync_point_count: 0,
        }
    }
}

impl<Ctx: 'static, const MAX_CORES: usize> Default for FiberDispatch<Ctx, MAX_CORES> {
    fn default() -> Self {
        Self::new()
    }
}
