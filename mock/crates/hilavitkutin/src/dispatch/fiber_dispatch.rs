//! Per-fiber dispatch record (domain 17).
//!
//! Pairs a monomorphised function pointer with the metadata the
//! engine needs to drive one fiber through its morsel range under
//! the right sync conditions.

use arvo::USize;
use notko::Maybe;

use super::{MorselRange, SyncPoint, WuFn};
use crate::plan::{FiberId, PhaseId};

/// Per-fiber dispatch record.
///
/// `MAX_CORES` bounds the sync-point array length: each fiber has
/// at most one SyncPoint per core that could run the producer
/// phase before it.
pub struct FiberDispatch<Ctx: 'static, const MAX_CORES: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// Monomorphised body. `Maybe::None` in skeleton state.
    pub body: Maybe<WuFn<Ctx>>,
    pub fiber_id: FiberId,
    pub phase: PhaseId,
    pub morsel_range: MorselRange,
    pub sync_points: [SyncPoint; MAX_CORES],
    pub sync_point_count: USize,
}

impl<Ctx: 'static, const MAX_CORES: usize> FiberDispatch<Ctx, MAX_CORES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// Empty skeleton record with no body and zero metadata.
    pub const fn new() -> Self {
        Self {
            body: Maybe::Isnt,
            fiber_id: FiberId(0),
            phase: PhaseId(0),
            morsel_range: MorselRange {
                start: USize(0),
                len: USize(0),
            },
            sync_points: [SyncPoint {
                fiber_id: FiberId(0),
                min_records: USize(0),
            }; MAX_CORES],
            sync_point_count: USize(0),
        }
    }
}

impl<Ctx: 'static, const MAX_CORES: usize> Default for FiberDispatch<Ctx, MAX_CORES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    fn default() -> Self {
        Self::new()
    }
}
