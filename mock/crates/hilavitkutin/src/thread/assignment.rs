//! Per-core role assignment (domain 20).
//!
//! Parallel-array layout mirrors the pattern used in
//! `plan::FiberGrouping` and `plan::PhaseBoundaries` (parallel
//! arrays + count). Keeps the struct `Copy`-friendly for
//! const-construction.

use arvo::{Cap, USize};
use arvo::strategy::Identity;
use arvo_tensor::cap_size;

use crate::plan::FiberId;

/// Sentinel meaning "core has no trunk assigned" in `trunk_index`.
///
/// Kept distinct from any valid trunk index by lying above any
/// realistic trunk count.
pub const NO_TRUNK: USize = USize(u16::MAX as usize); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: preserves original 16-bit sentinel value for trunk_index; tracked: #72

/// Per-core role record.
///
/// - `trunk_index[i]`: which trunk core `i` owns. `NO_TRUNK`
///   means the core has no trunk assigned (available for
///   branches / convergence / leftover work).
/// - `fiber_assignments[i]`: primary fiber pinned to core `i`.
/// - `morsel_size_multiplier[i]`: size multiplier in
///   basis-points-style units (100 = 1.0x, 200 = 2.0x). Integer
///   avoids float in no-std + no-alloc context.
/// - `assigned_count`: count of populated slots
///   (0..=MAX_CORES).
#[derive(Copy, Clone, Debug)]
pub struct CoreAssignment<const MAX_CORES: Cap>
where
    [(); cap_size(MAX_CORES)]:,
{
    pub trunk_index: [USize; cap_size(MAX_CORES)],
    pub fiber_assignments: [FiberId; cap_size(MAX_CORES)],
    pub morsel_size_multiplier: [USize; cap_size(MAX_CORES)],
    pub assigned_count: USize,
}

impl<const MAX_CORES: Cap> CoreAssignment<MAX_CORES>
where
    [(); cap_size(MAX_CORES)]:,
{
    /// Empty assignment: every core has no trunk (`NO_TRUNK`
    /// sentinel), fiber 0, default multiplier 100 (1.0x).
    pub const fn new() -> Self {
        Self {
            trunk_index: [NO_TRUNK; cap_size(MAX_CORES)],
            fiber_assignments: [FiberId::ZERO; cap_size(MAX_CORES)],
            morsel_size_multiplier: [USize(100); cap_size(MAX_CORES)],
            assigned_count: USize::ZERO,
        }
    }
}

impl<const MAX_CORES: Cap> Default for CoreAssignment<MAX_CORES>
where
    [(); cap_size(MAX_CORES)]:,
{
    fn default() -> Self {
        Self::new()
    }
}
