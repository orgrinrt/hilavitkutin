//! Phases: waist-delimited segments of the plan.
//!
//! A phase is a contiguous segment of the execution plan delimited by
//! waists (narrow cut points in the dependency graph). All work in
//! one phase finishes before the next phase starts. Phases own
//! trunks; trunks own components.

use arvo::strategy::Identity;
use arvo::{Cap, USize};
use arvo_tensor::cap_size;

use hilavitkutin_api::PhaseId;

use crate::plan::trunk::Trunk;
use crate::strategy::PhaseStrategy;

/// Phase split points: `boundaries[i]` is the first unit index of
/// phase `i`. Phase 0 always starts at unit 0.
///
/// Analysis intermediate produced by step 3 (waist detection).
#[derive(Copy, Clone, Debug)]
pub struct PhaseBoundaries<const MAX_PHASES: Cap>
where
    [(); cap_size(MAX_PHASES)]:,
{
    pub boundaries: [USize; cap_size(MAX_PHASES)],
    pub phase_count: USize,
}

impl<const MAX_PHASES: Cap> PhaseBoundaries<MAX_PHASES>
where
    [(); cap_size(MAX_PHASES)]:,
{
    pub const fn new() -> Self {
        Self { boundaries: [USize::ZERO; cap_size(MAX_PHASES)], phase_count: USize::ZERO }
    }
}

impl<const MAX_PHASES: Cap> Default for PhaseBoundaries<MAX_PHASES>
where
    [(); cap_size(MAX_PHASES)]:,
{
    fn default() -> Self {
        Self::new()
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
/// segment.
#[derive(Copy, Clone, Debug)]
pub struct Phase<
    const MAX_TRUNKS_PER_PHASE: Cap,
    const MAX_COMPONENTS_PER_TRUNK: Cap,
    const MAX_UNITS_PER_FIBER: Cap,
    const MAX_COLUMNS_PER_FIBER: Cap,
>
where
    [(); cap_size(MAX_TRUNKS_PER_PHASE)]:,
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    pub id: PhaseId,
    pub trunks: [Trunk<MAX_COMPONENTS_PER_TRUNK, MAX_UNITS_PER_FIBER, MAX_COLUMNS_PER_FIBER>;
        cap_size(MAX_TRUNKS_PER_PHASE)],
    pub trunk_count: USize,
    /// Plan-time strategy classification.
    pub strategy: PhaseStrategy,
    /// Codegen-time configuration.
    pub config: PhaseConfig,
}

impl<
        const MAX_TRUNKS_PER_PHASE: Cap,
        const MAX_COMPONENTS_PER_TRUNK: Cap,
        const MAX_UNITS_PER_FIBER: Cap,
        const MAX_COLUMNS_PER_FIBER: Cap,
    >
    Phase<
        MAX_TRUNKS_PER_PHASE,
        MAX_COMPONENTS_PER_TRUNK,
        MAX_UNITS_PER_FIBER,
        MAX_COLUMNS_PER_FIBER,
    >
where
    [(); cap_size(MAX_TRUNKS_PER_PHASE)]:,
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    pub const fn new() -> Self {
        Self {
            id: PhaseId::ZERO,
            trunks: [Trunk::new(); cap_size(MAX_TRUNKS_PER_PHASE)],
            trunk_count: USize::ZERO,
            strategy: PhaseStrategy::Balanced,
            config: PhaseConfig::Balanced,
        }
    }
}

impl<
        const MAX_TRUNKS_PER_PHASE: Cap,
        const MAX_COMPONENTS_PER_TRUNK: Cap,
        const MAX_UNITS_PER_FIBER: Cap,
        const MAX_COLUMNS_PER_FIBER: Cap,
    > Default
    for Phase<
        MAX_TRUNKS_PER_PHASE,
        MAX_COMPONENTS_PER_TRUNK,
        MAX_UNITS_PER_FIBER,
        MAX_COLUMNS_PER_FIBER,
    >
where
    [(); cap_size(MAX_TRUNKS_PER_PHASE)]:,
    [(); cap_size(MAX_COMPONENTS_PER_TRUNK)]:,
    [(); cap_size(MAX_UNITS_PER_FIBER)]:,
    [(); cap_size(MAX_COLUMNS_PER_FIBER)]:,
{
    fn default() -> Self {
        Self::new()
    }
}
