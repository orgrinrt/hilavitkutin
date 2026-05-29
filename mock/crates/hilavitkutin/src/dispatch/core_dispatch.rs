//! Per-core compiled pipeline (domain 17).
//!
//! Encodes the phases this core walks through, plus the fiber
//! assignments inside each phase, plus the morsel boundaries +
//! sync points the core respects while doing so. No dynamic
//! dispatch at runtime: every slot is monomorphised at plan
//! time.

use arvo::Cap;
use arvo::USize;
use arvo::strategy::Identity;
use arvo_tensor::cap_size;

use super::{FiberDispatch, MorselRange, SyncPoint};
use crate::plan::{FiberId, PhaseId};

/// Per-core compiled pipeline.
pub struct CoreDispatch<Ctx: 'static, const MAX_FIBERS: Cap>
where
    [(); cap_size(MAX_FIBERS)]:,
{
    /// Fiber dispatch records this core owns.
    pub fibers: [FiberDispatch<Ctx, MAX_FIBERS>; cap_size(MAX_FIBERS)],
    pub fiber_count: USize,
    /// Phase ids in execution order.
    pub phases: [PhaseId; cap_size(MAX_FIBERS)],
    pub phase_count: USize,
    /// Morsel boundaries (one per scheduled morsel).
    pub morsel_boundaries: [MorselRange; cap_size(MAX_FIBERS)],
    pub boundary_count: USize,
    /// Sync points this core respects.
    pub sync_points: [SyncPoint; cap_size(MAX_FIBERS)],
    pub sync_point_count: USize,
}

impl<Ctx: 'static, const MAX_FIBERS: Cap> CoreDispatch<Ctx, MAX_FIBERS>
where
    [(); cap_size(MAX_FIBERS)]:,
{
    /// Empty skeleton with no fibers, phases, boundaries, or sync
    /// points populated.
    pub fn new() -> Self {
        Self {
            fibers: core::array::from_fn(|_| FiberDispatch::new()),
            fiber_count: USize::ZERO,
            phases: [PhaseId::ZERO; cap_size(MAX_FIBERS)],
            phase_count: USize::ZERO,
            morsel_boundaries: [MorselRange {
                start: USize::ZERO,
                len: USize::ZERO,
            }; cap_size(MAX_FIBERS)],
            boundary_count: USize::ZERO,
            sync_points: [SyncPoint {
                fiber_id: FiberId::ZERO,
                min_records: USize::ZERO,
            }; cap_size(MAX_FIBERS)],
            sync_point_count: USize::ZERO,
        }
    }
}

impl<Ctx: 'static, const MAX_FIBERS: Cap> Default for CoreDispatch<Ctx, MAX_FIBERS>
where
    [(); cap_size(MAX_FIBERS)]:,
{
    fn default() -> Self {
        Self::new()
    }
}
