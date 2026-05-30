//! Per-core compiled pipeline (domain 17).
//!
//! Encodes the phases this core walks through, plus the fiber
//! assignments inside each phase, plus the morsel boundaries +
//! sync points the core respects while doing so. No dynamic
//! dispatch at runtime: every slot is monomorphised at plan
//! time.

use arvo::USize;
use arvo::strategy::Identity;
use arvo_tensor::Capacity;

use super::{FiberDispatch, MorselRange, SyncPoint};
use crate::plan::{FiberId, PhaseId};

/// Per-core compiled pipeline. `C` is the fiber capacity, bounding the
/// per-core fiber / phase / boundary / sync arrays.
pub struct CoreDispatch<Ctx: 'static, C: Capacity> {
    /// Fiber dispatch records this core owns.
    pub fibers: <C as Capacity>::Array<FiberDispatch<Ctx, C>>,
    pub fiber_count: USize,
    /// Phase ids in execution order.
    pub phases: <C as Capacity>::Array<PhaseId>,
    pub phase_count: USize,
    /// Morsel boundaries (one per scheduled morsel).
    pub morsel_boundaries: <C as Capacity>::Array<MorselRange>,
    pub boundary_count: USize,
    /// Sync points this core respects.
    pub sync_points: <C as Capacity>::Array<SyncPoint>,
    pub sync_point_count: USize,
}

impl<Ctx: 'static, C: Capacity> CoreDispatch<Ctx, C>
where
    <C as Capacity>::Array<FiberDispatch<Ctx, C>>: Sized,
{
    /// Empty skeleton with no fibers, phases, boundaries, or sync
    /// points populated.
    pub fn new() -> Self {
        Self {
            fibers: <C as Capacity>::from_fn(|_| FiberDispatch::new()),
            fiber_count: USize::ZERO,
            phases: <C as Capacity>::filled(PhaseId::ZERO),
            phase_count: USize::ZERO,
            morsel_boundaries: <C as Capacity>::filled(MorselRange {
                start: USize::ZERO,
                len: USize::ZERO,
            }),
            boundary_count: USize::ZERO,
            sync_points: <C as Capacity>::filled(SyncPoint {
                fiber_id: FiberId::ZERO,
                min_records: USize::ZERO,
            }),
            sync_point_count: USize::ZERO,
        }
    }
}

impl<Ctx: 'static, C: Capacity> Default for CoreDispatch<Ctx, C> {
    fn default() -> Self {
        Self::new()
    }
}
