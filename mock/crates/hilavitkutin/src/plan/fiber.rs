//! Fibers: contiguous WU runs that fit within fiber-budget constraints.
//!
//! A fiber is the smallest unit of dispatch the engine schedules. It
//! shares a morsel arena across its WUs and projects into one
//! cache-friendly codegen body.
//!
//! `FiberGrouping` is the analysis intermediate (steps 5 to 8 output):
//! per-unit fiber assignment. `Fiber` is the shipped plan-stage record
//! that the dispatch stage walks.

use arvo::strategy::Identity;
use arvo::{Bool, USize};
use arvo_tensor::Capacity;

use hilavitkutin_api::{FiberId, StoreId, UnitId};
use notko::Maybe;

use crate::dispatch::approach::DispatchApproach;
use crate::plan::dims::PlanDims;

/// Per-unit fiber assignment (intermediate; analysis output of steps
/// 5 to 8). Sized by the unit capacity `D::Units`.
pub struct FiberGrouping<D: PlanDims> {
    /// `assignment[i]` is the FiberId that unit `i` belongs to.
    pub assignment: <D::Units as Capacity>::Array<FiberId>,
    /// Number of fibers actually used.
    pub fiber_count: USize,
}

impl<D: PlanDims> FiberGrouping<D> {
    pub fn new() -> Self {
        Self {
            assignment: <D::Units as Capacity>::filled(FiberId::ZERO),
            fiber_count: USize::ZERO,
        }
    }
}

impl<D: PlanDims> Copy for FiberGrouping<D> where <D::Units as Capacity>::Array<FiberId>: Copy {}

impl<D: PlanDims> Clone for FiberGrouping<D>
where
    <D::Units as Capacity>::Array<FiberId>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: PlanDims> Default for FiberGrouping<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: PlanDims> core::fmt::Debug for FiberGrouping<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FiberGrouping")
            .field("fiber_count", &self.fiber_count.0)
            .finish_non_exhaustive()
    }
}

/// Accumulation type for head+tail convergence.
///
/// Marks how head and tail accumulators combine. Pure-additive
/// arithmetic is the common case; min/max give reductive aggregation
/// paths; custom punts to a consumer-provided merge fn.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AccumType {
    /// `+` (also `-` after negation).
    Sum,
    /// `min(...)`.
    Min,
    /// `max(...)`.
    Max,
    /// XOR / unique-symmetric-difference accumulation.
    Xor,
    /// Logical AND.
    All,
    /// Logical OR.
    Any,
    /// Custom merge fn supplied by the consumer.
    Custom,
}

impl Default for AccumType {
    fn default() -> Self {
        Self::Sum
    }
}

/// Merge operation between head and tail accumulators.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MergeOp {
    /// `head + tail`.
    Add,
    /// `min(head, tail)`.
    Min,
    /// `max(head, tail)`.
    Max,
    /// `head ^ tail`.
    Xor,
    /// `head & tail`.
    And,
    /// `head | tail`.
    Or,
    /// Custom merge supplied by the consumer.
    Custom,
}

impl Default for MergeOp {
    fn default() -> Self {
        Self::Add
    }
}

/// One accumulator slot in a head+tail-eligible fiber.
///
/// The slot references the storage the accumulator lives in (via
/// `store_id`) and the accumulator type. The dispatch stage emits
/// codegen that initialises the accumulator, runs the WU body, and
/// merges the result through `MergeOp` at the convergence point.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AccumSlot {
    /// Store holding the accumulator's working data.
    pub store_id: StoreId,
    /// How values combine into this slot.
    pub accum_type: AccumType,
}

impl AccumSlot {
    pub const fn new() -> Self {
        Self {
            store_id: StoreId(USize::ZERO),
            accum_type: AccumType::Sum,
        }
    }
}

impl Default for AccumSlot {
    fn default() -> Self {
        Self::new()
    }
}

/// Head+tail convergence: the plan-stage record describing how a
/// fiber's two ends meet.
///
/// A fiber is head+tail eligible iff all of: COMMUTATIVE, single-
/// trunk-phase, record-count-threshold-met, accumulation-compatible.
/// When eligible, the plan stage records the head/tail accumulator
/// slots and the merge operation; codegen lowers to a two-ended
/// projection with a deterministic merge at the convergence point.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HeadTailConvergence {
    /// Accumulator on the head walker (units flowing forward).
    pub head_accum: AccumSlot,
    /// Accumulator on the tail walker (units flowing backward).
    pub tail_accum: AccumSlot,
    /// Where the merged result lands.
    pub merge_target: AccumSlot,
    /// How head and tail combine.
    pub merge_op: MergeOp,
}

impl HeadTailConvergence {
    pub const fn new() -> Self {
        Self {
            head_accum: AccumSlot::new(),
            tail_accum: AccumSlot::new(),
            merge_target: AccumSlot::new(),
            merge_op: MergeOp::Add,
        }
    }
}

impl Default for HeadTailConvergence {
    fn default() -> Self {
        Self::new()
    }
}

/// Shipped plan-stage fiber record.
///
/// Each fiber owns up to the unit-per-fiber capacity's units and
/// references up to the column-per-fiber capacity's stores. Sizing is
/// per-fiber rather than the pipeline-wide unit / column capacities to
/// keep the per-fiber footprint independent of pipeline-wide caps
/// (Topic 3 audit-2 m3). Both projected from one `D: PlanDims`.
pub struct Fiber<D: PlanDims> {
    /// Stable id within the enclosing plan.
    pub id: FiberId,
    /// Units in the fiber (in dispatch order). `unit_count` records
    /// how many of the unit-per-fiber slots are populated.
    pub units: <D::UnitsPerFiber as Capacity>::Array<UnitId>,
    pub unit_count: USize,
    /// Stores the fiber touches (read or write). `column_count`
    /// records the populated count.
    pub columns: <D::ColumnsPerFiber as Capacity>::Array<StoreId>,
    pub column_count: USize,
    /// Head+tail convergence if the fiber qualifies; absent otherwise.
    pub head_tail: Maybe<HeadTailConvergence>,
    /// Codegen shape chosen for the fiber.
    pub dispatch_approach: DispatchApproach,
    /// True when no unit in the fiber writes an accumulator, so the fiber
    /// can dispatch morsel-outer (every cross-unit dependency is
    /// morsel-local). Computed at fiber formation from the units' write
    /// masks against the accumulator-store set.
    pub morsel_local: Bool,
}

impl<D: PlanDims> Fiber<D> {
    pub fn new() -> Self {
        Self {
            id: FiberId::ZERO,
            units: <D::UnitsPerFiber as Capacity>::filled(UnitId::ZERO),
            unit_count: USize::ZERO,
            columns: <D::ColumnsPerFiber as Capacity>::filled(StoreId(USize::ZERO)),
            column_count: USize::ZERO,
            head_tail: Maybe::Isnt,
            dispatch_approach: DispatchApproach::IndirectPerFiber,
            morsel_local: Bool::TRUE,
        }
    }
}

impl<D: PlanDims> Copy for Fiber<D>
where
    <D::UnitsPerFiber as Capacity>::Array<UnitId>: Copy,
    <D::ColumnsPerFiber as Capacity>::Array<StoreId>: Copy,
{
}

impl<D: PlanDims> Clone for Fiber<D>
where
    <D::UnitsPerFiber as Capacity>::Array<UnitId>: Copy,
    <D::ColumnsPerFiber as Capacity>::Array<StoreId>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: PlanDims> Default for Fiber<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: PlanDims> core::fmt::Debug for Fiber<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Fiber")
            .field("id", &self.id)
            .field("unit_count", &self.unit_count.0)
            .field("column_count", &self.column_count.0)
            .finish_non_exhaustive()
    }
}
