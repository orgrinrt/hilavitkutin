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
use arvo::USize;

use hilavitkutin_api::{FiberId, StoreId, UnitId};
use notko::Maybe;

use crate::dispatch::approach::DispatchApproach;

/// Per-unit fiber assignment (intermediate; analysis output of steps
/// 5 to 8).
#[derive(Copy, Clone, Debug)]
pub struct FiberGrouping<const MAX_UNITS: usize, const MAX_FIBERS: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// `assignment[i]` is the FiberId that unit `i` belongs to.
    pub assignment: [FiberId; MAX_UNITS],
    /// Number of fibers actually used (0..=MAX_FIBERS).
    pub fiber_count: USize,
}

impl<const MAX_UNITS: usize, const MAX_FIBERS: usize> FiberGrouping<MAX_UNITS, MAX_FIBERS> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pub const fn new() -> Self {
        Self {
            assignment: [FiberId::ZERO; MAX_UNITS],
            fiber_count: USize::ZERO,
        }
    }
}

impl<const MAX_UNITS: usize, const MAX_FIBERS: usize> Default // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    for FiberGrouping<MAX_UNITS, MAX_FIBERS>
{
    fn default() -> Self {
        Self::new()
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
/// Each fiber owns up to `MAX_UNITS_PER_FIBER` units and references
/// up to `MAX_COLUMNS_PER_FIBER` stores. Sizing is per-fiber rather
/// than `MAX_UNITS` / `MAX_COLUMNS` to keep the per-fiber footprint
/// independent of pipeline-wide caps (Topic 3 audit-2 m3).
#[derive(Copy, Clone, Debug)]
pub struct Fiber<const MAX_UNITS_PER_FIBER: usize, const MAX_COLUMNS_PER_FIBER: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// Stable id within the enclosing plan.
    pub id: FiberId,
    /// Units in the fiber (in dispatch order). `unit_count` records
    /// how many of the `MAX_UNITS_PER_FIBER` slots are populated.
    pub units: [UnitId; MAX_UNITS_PER_FIBER],
    pub unit_count: USize,
    /// Stores the fiber touches (read or write). `column_count`
    /// records the populated count.
    pub columns: [StoreId; MAX_COLUMNS_PER_FIBER],
    pub column_count: USize,
    /// Head+tail convergence if the fiber qualifies; absent otherwise.
    pub head_tail: Maybe<HeadTailConvergence>,
    /// Codegen shape chosen for the fiber.
    pub dispatch_approach: DispatchApproach,
}

impl<const MAX_UNITS_PER_FIBER: usize, const MAX_COLUMNS_PER_FIBER: usize> // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    Fiber<MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>
{
    pub const fn new() -> Self {
        Self {
            id: FiberId::ZERO,
            units: [UnitId::ZERO; MAX_UNITS_PER_FIBER],
            unit_count: USize::ZERO,
            columns: [StoreId(USize::ZERO); MAX_COLUMNS_PER_FIBER],
            column_count: USize::ZERO,
            head_tail: Maybe::Isnt,
            dispatch_approach: DispatchApproach::IndirectPerFiber,
        }
    }
}

impl<const MAX_UNITS_PER_FIBER: usize, const MAX_COLUMNS_PER_FIBER: usize> Default // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    for Fiber<MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>
{
    fn default() -> Self {
        Self::new()
    }
}
