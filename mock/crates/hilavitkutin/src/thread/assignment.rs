//! Per-core role assignment (domain 20).
//!
//! Parallel-array layout mirrors the pattern used in
//! `plan::FiberGrouping` and `plan::PhaseBoundaries` (parallel
//! arrays + count). Keeps the struct `Copy`-friendly for
//! const-construction.

use arvo::USize;
use arvo::strategy::Identity;

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
pub struct CoreAssignment<const MAX_CORES: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pub trunk_index: [USize; MAX_CORES],
    pub fiber_assignments: [FiberId; MAX_CORES],
    pub morsel_size_multiplier: [USize; MAX_CORES],
    pub assigned_count: USize,
}

impl<const MAX_CORES: usize> CoreAssignment<MAX_CORES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// Empty assignment: every core has no trunk (`NO_TRUNK`
    /// sentinel), fiber 0, default multiplier 100 (1.0x).
    pub const fn new() -> Self {
        Self {
            trunk_index: [NO_TRUNK; MAX_CORES],
            fiber_assignments: [FiberId(0); MAX_CORES],
            morsel_size_multiplier: [USize(100); MAX_CORES],
            assigned_count: USize::ZERO,
        }
    }
}

impl<const MAX_CORES: usize> Default for CoreAssignment<MAX_CORES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    fn default() -> Self {
        Self::new()
    }
}
