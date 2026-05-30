//! Phases: waist-delimited segments of the plan.
//!
//! A phase is a contiguous segment of the execution plan delimited by
//! waists (narrow cut points in the dependency graph). All work in
//! one phase finishes before the next phase starts. Phases own
//! trunks; trunks own components.

use arvo::strategy::Identity;
use arvo::USize;
use arvo_tensor::Capacity;

use hilavitkutin_api::PhaseId;

use crate::plan::dims::PlanDims;
use crate::plan::trunk::Trunk;
use crate::strategy::PhaseStrategy;

/// Phase split points: `boundaries[i]` is the first unit index of
/// phase `i`. Phase 0 always starts at unit 0.
///
/// Analysis intermediate produced by step 3 (waist detection).
/// Sized by the phase capacity `D::Phases`.
pub struct PhaseBoundaries<D: PlanDims> {
    pub boundaries: <D::Phases as Capacity>::Array<USize>,
    pub phase_count: USize,
}

impl<D: PlanDims> PhaseBoundaries<D> {
    pub fn new() -> Self {
        Self {
            boundaries: <D::Phases as Capacity>::filled(USize::ZERO),
            phase_count: USize::ZERO,
        }
    }
}

impl<D: PlanDims> Copy for PhaseBoundaries<D> where <D::Phases as Capacity>::Array<USize>: Copy {}

impl<D: PlanDims> Clone for PhaseBoundaries<D>
where
    <D::Phases as Capacity>::Array<USize>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: PlanDims> Default for PhaseBoundaries<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: PlanDims> core::fmt::Debug for PhaseBoundaries<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PhaseBoundaries")
            .field("phase_count", &self.phase_count.0)
            .finish_non_exhaustive()
    }
}

/// Per-phase codegen configuration.
///
/// Picked at plan time and frozen for the duration of the plan. The
/// adapt subsystem refreshes the runtime `PhaseStrategy` between
/// frames; `PhaseConfig` is the static plan-stage choice that shaped
/// the codegen output.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PhaseConfig {
    /// Maximise fusion: pack as many WUs as possible per dispatch.
    MaxFuse,
    /// Balanced split between fusion and parallelism.
    Balanced,
    /// Maximise split: every WU dispatches independently.
    MaxSplit,
}

impl Default for PhaseConfig {
    fn default() -> Self {
        Self::Balanced
    }
}

/// One phase: trunks running together within a waist-delimited
/// segment. Sized by the trunk-per-phase, component-per-trunk,
/// unit-per-fiber, and column-per-fiber capacities, all projected
/// from one `D: PlanDims`.
pub struct Phase<D: PlanDims> {
    pub id: PhaseId,
    pub trunks: <D::TrunksPerPhase as Capacity>::Array<Trunk<D>>,
    pub trunk_count: USize,
    /// Plan-time strategy classification.
    pub strategy: PhaseStrategy,
    /// Codegen-time configuration.
    pub config: PhaseConfig,
}

impl<D: PlanDims> Phase<D>
where
    crate::plan::fiber::Fiber<D>: Copy,
    <D::ComponentsPerTrunk as Capacity>::Array<crate::plan::trunk::TrunkComponent<D>>: Copy,
{
    pub fn new() -> Self {
        Self {
            id: PhaseId::ZERO,
            trunks: <D::TrunksPerPhase as Capacity>::filled(Trunk::new()),
            trunk_count: USize::ZERO,
            strategy: PhaseStrategy::Balanced,
            config: PhaseConfig::Balanced,
        }
    }
}

impl<D: PlanDims> Copy for Phase<D> where <D::TrunksPerPhase as Capacity>::Array<Trunk<D>>: Copy {}

impl<D: PlanDims> Clone for Phase<D>
where
    <D::TrunksPerPhase as Capacity>::Array<Trunk<D>>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<D: PlanDims> Default for Phase<D>
where
    crate::plan::fiber::Fiber<D>: Copy,
    <D::ComponentsPerTrunk as Capacity>::Array<crate::plan::trunk::TrunkComponent<D>>: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<D: PlanDims> core::fmt::Debug for Phase<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Phase")
            .field("id", &self.id)
            .field("trunk_count", &self.trunk_count.0)
            .field("strategy", &self.strategy)
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}
