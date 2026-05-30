//! Per-fiber dispatch record (domain 17).
//!
//! Pairs a monomorphised function pointer with the metadata the
//! engine needs to drive one fiber through its morsel range under
//! the right sync conditions.

use arvo::USize;
use arvo::strategy::Identity;
use arvo_tensor::Capacity;
use hilavitkutin_api::FiberShape;
use notko::Maybe;

use super::{MorselRange, SyncPoint, WuFn};
use crate::plan::{FiberId, PhaseId};

/// Per-fiber dispatch record.
///
/// `C` is the core capacity, bounding the sync-point array length:
/// each fiber has at most one SyncPoint per core that could run the
/// producer phase before it.
pub struct FiberDispatch<Ctx: 'static, C: Capacity> {
    /// Monomorphised body. `Maybe::None` in skeleton state.
    pub body: Maybe<WuFn<Ctx>>,
    pub fiber_id: FiberId,
    pub phase: PhaseId,
    pub morsel_range: MorselRange,
    pub sync_points: <C as Capacity>::Array<SyncPoint>,
    pub sync_point_count: USize,
}

impl<Ctx: 'static, C: Capacity> FiberDispatch<Ctx, C> {
    /// Empty skeleton record with no body and zero metadata.
    pub fn new() -> Self {
        Self {
            body: Maybe::Isnt,
            fiber_id: FiberId::ZERO,
            phase: PhaseId::ZERO,
            morsel_range: MorselRange {
                start: USize::ZERO,
                len: USize::ZERO,
            },
            sync_points: <C as Capacity>::filled(SyncPoint {
                fiber_id: FiberId::ZERO,
                min_records: USize::ZERO,
            }),
            sync_point_count: USize::ZERO,
        }
    }
}

impl<Ctx: 'static, C: Capacity> Default for FiberDispatch<Ctx, C> {
    fn default() -> Self {
        Self::new()
    }
}

/// Per-fiber dispatch entrypoint, monomorphised per shape.
///
/// Topic 4 axis D + sketch
/// `mock/research/sketches/202605101036-fibershape-typing/` (WORKS).
/// Each unique fiber-shape `S` in the plan produces its own
/// monomorphised instance of `run_fiber`. The codegen layer assembles
/// a LOCAL `&[WuFn]` slice from `S::WuTuple` and walks it under the
/// morsel-loop body that lands in subsequent Pass 3 CHANGE blocks
/// (morsel iteration, arena progress, S3 fence, micro-morsel sync).
///
/// `#[inline(never)]` matches Domain 17 inline-discipline for the
/// per-fiber program: the body is the LLVM optimisation unit; the
/// per-WU shims (`super::wu_fn::invoke_wu_in_fiber`) are
/// `#[inline(always)]` and dissolve into the body.
#[inline(never)]
pub fn run_fiber<S: FiberShape>() {
    // The walk over `S::WuTuple`'s typed sequence + morsel loop body
    // wire in across remaining Pass 3 CHANGE blocks. SHAPE_ID is
    // referenced here so dropping a shape from the plan surfaces as a
    // compile error at this site (the lock-bearing identity check).
    let _shape_id = S::SHAPE_ID;
}
