//! Per-core compiled pipeline (domain 17).
//!
//! Encodes the phases this core walks through, plus the fiber
//! assignments inside each phase, plus the morsel boundaries +
//! sync points the core respects while doing so. No dynamic
//! dispatch at runtime — every slot is monomorphised at plan
//! time.

use super::{FiberDispatch, MorselRange, SyncPoint};
use crate::plan::{FiberId, PhaseId};

/// Per-core compiled pipeline.
pub struct CoreDispatch<Ctx: 'static, const MAX_FIBERS: usize> {
    /// Fiber dispatch records this core owns.
    pub fibers: [FiberDispatch<Ctx, MAX_FIBERS>; MAX_FIBERS],
    pub fiber_count: u16,
    /// Phase ids in execution order.
    pub phases: [PhaseId; MAX_FIBERS],
    pub phase_count: u8,
    /// Morsel boundaries (one per scheduled morsel).
    pub morsel_boundaries: [MorselRange; MAX_FIBERS],
    pub boundary_count: u16,
    /// Sync points this core respects.
    pub sync_points: [SyncPoint; MAX_FIBERS],
    pub sync_point_count: u16,
}

impl<Ctx: 'static, const MAX_FIBERS: usize> CoreDispatch<Ctx, MAX_FIBERS> {
    /// Empty skeleton with no fibers, phases, boundaries, or sync
    /// points populated.
    pub fn new() -> Self {
        Self {
            fibers: core::array::from_fn(|_| FiberDispatch::new()),
            fiber_count: 0,
            phases: [PhaseId(0); MAX_FIBERS],
            phase_count: 0,
            morsel_boundaries: [MorselRange {
                start: 0,
                len: 0,
            }; MAX_FIBERS],
            boundary_count: 0,
            sync_points: [SyncPoint {
                fiber_id: FiberId(0),
                min_records: 0,
            }; MAX_FIBERS],
            sync_point_count: 0,
        }
    }
}

impl<Ctx: 'static, const MAX_FIBERS: usize> Default for CoreDispatch<Ctx, MAX_FIBERS> {
    fn default() -> Self {
        Self::new()
    }
}
