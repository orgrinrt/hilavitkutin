//! Trunks: groups of fibers running together within a phase.
//!
//! A trunk is the unit of cross-fiber composition. It carries one or
//! more `TrunkComponent`s; each component is either a `Fiber`, a
//! `Branch` (lateral fan-out into parallel fibers), or a `Bridge`
//! (lateral fan-in from parallel fibers).

use arvo::strategy::Identity;
use arvo::USize;
use arvo_tensor::Capacity;

use hilavitkutin_api::TrunkId;

use crate::plan::dims::PlanDims;
use crate::plan::fiber::Fiber;

/// Connected-component (block) partition of the dependency graph.
///
/// Produced by step 5 (`block_diagonalise`) via arvo-sparse
/// `block_diagonal_via`. `block_count` is the number of distinct
/// blocks; `block_of_unit[i]` is the block id of the unit at index
/// `i`. Each block is an independent sub-graph sharing no edges with
/// the others, hence column-disjoint: blocks map to the trunks that
/// run with zero sync within a phase. Sized by the unit capacity `C`.
pub struct BlockPartition<C: Capacity> {
    /// Number of distinct blocks (connected components).
    pub block_count: USize,
    /// Block id per unit, indexed by unit index.
    pub block_of_unit: <C as Capacity>::Array<USize>,
}

impl<C: Capacity> BlockPartition<C> {
    /// Empty partition (zero blocks). The default before step 5 runs.
    pub fn new() -> Self {
        Self {
            block_count: USize::ZERO,
            block_of_unit: <C as Capacity>::filled(USize::ZERO),
        }
    }
}

impl<C: Capacity> Copy for BlockPartition<C> where <C as Capacity>::Array<USize>: Copy {}

impl<C: Capacity> Clone for BlockPartition<C>
where
    <C as Capacity>::Array<USize>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Capacity> Default for BlockPartition<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Capacity> core::fmt::Debug for BlockPartition<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BlockPartition")
            .field("block_count", &self.block_count.0)
            .finish_non_exhaustive()
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
/// needed for codegen without further analysis. Sized via the fiber's
/// own `D: PlanDims` projections.
pub enum TrunkComponent<D: PlanDims> {
    Fiber(Fiber<D>),
    Branch(Branch),
    Bridge(Bridge),
}

impl<D: PlanDims> TrunkComponent<D> {
    /// Default value for array initialisation: a zero-shaped fiber.
    /// Real values land via the plan-stage block-diagonalisation pass.
    pub fn empty_fiber() -> Self {
        Self::Fiber(Fiber::new())
    }
}

impl<D: PlanDims> Copy for TrunkComponent<D> where Fiber<D>: Copy {}

impl<D: PlanDims> Clone for TrunkComponent<D>
where
    Fiber<D>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: PlanDims> Default for TrunkComponent<D> {
    fn default() -> Self {
        Self::empty_fiber()
    }
}

impl<D: PlanDims> core::fmt::Debug for TrunkComponent<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Fiber(fib) => f.debug_tuple("Fiber").field(fib).finish(),
            Self::Branch(b) => f.debug_tuple("Branch").field(b).finish(),
            Self::Bridge(b) => f.debug_tuple("Bridge").field(b).finish(),
        }
    }
}

/// A trunk: components running together within a phase. Sized by the
/// component-per-trunk capacity projected from `D: PlanDims`.
pub struct Trunk<D: PlanDims> {
    pub id: TrunkId,
    pub components: <D::ComponentsPerTrunk as Capacity>::Array<TrunkComponent<D>>,
    pub component_count: USize,
}

impl<D: PlanDims> Trunk<D>
where
    Fiber<D>: Copy,
{
    pub fn new() -> Self {
        Self {
            id: TrunkId::ZERO,
            components: <D::ComponentsPerTrunk as Capacity>::filled(TrunkComponent::empty_fiber()),
            component_count: USize::ZERO,
        }
    }
}

impl<D: PlanDims> Copy for Trunk<D> where
    <D::ComponentsPerTrunk as Capacity>::Array<TrunkComponent<D>>: Copy
{
}

impl<D: PlanDims> Clone for Trunk<D>
where
    <D::ComponentsPerTrunk as Capacity>::Array<TrunkComponent<D>>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: PlanDims> Default for Trunk<D>
where
    Fiber<D>: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<D: PlanDims> core::fmt::Debug for Trunk<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Trunk")
            .field("id", &self.id)
            .field("component_count", &self.component_count.0)
            .finish_non_exhaustive()
    }
}
