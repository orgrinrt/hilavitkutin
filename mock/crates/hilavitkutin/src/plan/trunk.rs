//! Trunks: groups of fibers running together within a phase.
//!
//! A trunk is the unit of cross-fiber composition. It carries one or
//! more `TrunkComponent`s; each component is either a `Fiber`, a
//! `Branch` (lateral fan-out into parallel fibers), or a `Bridge`
//! (lateral fan-in from parallel fibers).

use arvo::strategy::Identity;
use arvo::{Cap, USize};
use arvo_tensor::cap_size;

use hilavitkutin_api::TrunkId;

use crate::plan::fiber::Fiber;

/// Connected-component (block) partition of the dependency graph.
///
/// Produced by step 5 (`block_diagonalise`) via arvo-sparse
/// `block_diagonal_via`. `block_count` is the number of distinct
/// blocks; `block_of_unit[i]` is the block id of the unit at index
/// `i`. Each block is an independent sub-graph sharing no edges with
/// the others, hence column-disjoint: blocks map to the trunks that
/// run with zero sync within a phase.
#[derive(Copy, Clone, Debug)]
pub struct BlockPartition<const MAX_UNITS: Cap>
where
    [(); cap_size(MAX_UNITS)]:,
{
    /// Number of distinct blocks (connected components).
    pub block_count: USize,
    /// Block id per unit, indexed by unit index.
    pub block_of_unit: [USize; cap_size(MAX_UNITS)],
}

impl<const MAX_UNITS: Cap> BlockPartition<MAX_UNITS>
where
    [(); cap_size(MAX_UNITS)]:,
{
    /// Empty partition (zero blocks). The default before step 5 runs.
    pub const fn new() -> Self {
        Self { block_count: USize::ZERO, block_of_unit: [USize::ZERO; cap_size(MAX_UNITS)] }
    }
}

impl<const MAX_UNITS: Cap> Default for BlockPartition<MAX_UNITS>
where
    [(); cap_size(MAX_UNITS)]:,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Lateral fan-out node: splits a single upstream path into multiple
/// parallel branches.
///
/// The plan stage records the branch's degree (`fan_out_count`) and
/// the index range of the resulting fibers within the enclosing
/// trunk's component array. The dispatch stage emits codegen that
/// distributes records across the branches deterministically.
#[derive(Copy, Clone, Debug)]
pub struct Branch {
    /// Number of parallel paths produced by this branch.
    pub fan_out_count: USize,
}

impl Branch {
    pub const fn new() -> Self {
        Self { fan_out_count: USize::ZERO }
    }
}

impl Default for Branch {
    fn default() -> Self {
        Self::new()
    }
}

/// Lateral fan-in node: merges multiple upstream branches into a
/// single downstream path.
#[derive(Copy, Clone, Debug)]
pub struct Bridge {
    /// Number of parallel paths feeding this bridge.
    pub fan_in_count: USize,
}

impl Bridge {
    pub const fn new() -> Self {
        Self { fan_in_count: USize::ZERO }
    }
}

impl Default for Bridge {
    fn default() -> Self {
        Self::new()
    }
}

/// One component of a trunk: a fiber, a branch, or a bridge.
///
/// The plan stage's block-diagonalisation pass (step 6) emits the
/// component sequence. Each component carries the full information
/// needed for codegen without further analysis.
#[derive(Copy, Clone, Debug)]
pub enum TrunkComponent<const MAX_UNITS_PER_FIBER: Cap, const MAX_COLUMNS_PER_FIBER: Cap>
where
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    Fiber(Fiber<MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>),
    Branch(Branch),
    Bridge(Bridge),
}

impl<const MAX_UNITS_PER_FIBER: Cap, const MAX_COLUMNS_PER_FIBER: Cap>
    TrunkComponent<MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>
where
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    /// Default value for array initialisation: a zero-shaped fiber.
    /// Real values land via the plan-stage block-diagonalisation pass.
    pub const fn empty_fiber() -> Self {
        Self::Fiber(Fiber::new())
    }
}

impl<const MAX_UNITS_PER_FIBER: Cap, const MAX_COLUMNS_PER_FIBER: Cap> Default
    for TrunkComponent<MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>
where
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    fn default() -> Self {
        Self::empty_fiber()
    }
}

/// A trunk: components running together within a phase.
#[derive(Copy, Clone, Debug)]
pub struct Trunk<
    const MAX_COMPONENTS_PER_TRUNK: Cap,
    const MAX_UNITS_PER_FIBER: Cap,
    const MAX_COLUMNS_PER_FIBER: Cap,
>
where
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    pub id: TrunkId,
    pub components: [TrunkComponent<MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>;
        cap_size(MAX_COMPONENTS_PER_TRUNK)],
    pub component_count: USize,
}

impl<
        const MAX_COMPONENTS_PER_TRUNK: Cap,
        const MAX_UNITS_PER_FIBER: Cap,
        const MAX_COLUMNS_PER_FIBER: Cap,
    > Trunk<MAX_COMPONENTS_PER_TRUNK, MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>
where
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    pub const fn new() -> Self {
        Self {
            id: TrunkId::ZERO,
            components: [TrunkComponent::empty_fiber(); cap_size(MAX_COMPONENTS_PER_TRUNK)],
            component_count: USize::ZERO,
        }
    }
}

impl<
        const MAX_COMPONENTS_PER_TRUNK: Cap,
        const MAX_UNITS_PER_FIBER: Cap,
        const MAX_COLUMNS_PER_FIBER: Cap,
    > Default for Trunk<MAX_COMPONENTS_PER_TRUNK, MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>
where
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    fn default() -> Self {
        Self::new()
    }
}
