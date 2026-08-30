//! Per-unit metadata + mutable cost table.
//!
//! `UnitMeta` is the per-WU plan-stage record: access-set bits, scheduling
//! hint, commutative-flag, dependency offsets. Frozen for the lifetime of
//! the plan.
//!
//! `CostTable` is its mutable sibling: per-unit cost estimates that the
//! adapt subsystem refreshes between frames. Kept separate from `UnitMeta`
//! so the immutable plan structure can stay `&` while costs mutate.

use arvo::USize;
use arvo::strategy::Identity;
use arvo_tensor::Capacity;

use hilavitkutin_api::UnitId;

/// Plan-stage metadata for a single work unit.
///
/// Populated by `build_dag` + `topo_sort` (steps 1 to 2) and stable
/// thereafter. Carries enough state for codegen to emit the right
/// dispatch shape without re-walking the access set.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UnitMeta {
    /// Stable id within the plan.
    pub id: UnitId,
    /// Topological depth from the DAG root (step 2 output).
    pub topo_depth: USize,
    /// Upward rank: longest path to any sink (fused step 8 output).
    pub upward_rank: USize,
    /// True iff the WU declared `COMMUTATIVE`.
    pub commutative: arvo::Bool,
}

impl UnitMeta {
    /// Zero-valued meta. Used as the default array fill before the
    /// plan-stage chain populates real values.
    pub const fn new() -> Self {
        Self {
            id: UnitId::ZERO,
            topo_depth: USize::ZERO,
            upward_rank: USize::ZERO,
            commutative: arvo::Bool::FALSE,
        }
    }
}

impl Default for UnitMeta {
    fn default() -> Self {
        Self::new()
    }
}

/// Per-unit mutable cost estimates.
///
/// Refreshed between frames by the adapt subsystem. Sized to the unit
/// capacity `C` so it travels alongside the immutable plan without
/// dictating array-size relationships at the call sites.
pub struct CostTable<C: Capacity> {
    /// Estimated single-record cost in nanoseconds, per unit.
    pub estimated_cost_ns: <C as Capacity>::Array<USize>,
}

impl<C: Capacity> CostTable<C> {
    /// All-zero cost table.
    pub fn new() -> Self {
        Self {
            estimated_cost_ns: <C as Capacity>::filled(USize::ZERO),
        }
    }
}

impl<C: Capacity> Copy for CostTable<C> where <C as Capacity>::Array<USize>: Copy {}

impl<C: Capacity> Clone for CostTable<C>
where
    <C as Capacity>::Array<USize>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Capacity> Default for CostTable<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Capacity> core::fmt::Debug for CostTable<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CostTable").finish_non_exhaustive()
    }
}
