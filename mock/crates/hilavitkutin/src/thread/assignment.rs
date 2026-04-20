//! Per-core role assignment (domain 20).
//!
//! Parallel-array layout mirrors the pattern used in
//! `plan::FiberGrouping` and `plan::PhaseBoundaries` (parallel
//! arrays + count). Keeps the struct `Copy`-friendly for
//! const-construction.

use crate::plan::FiberId;

/// Per-core role record.
///
/// - `trunk_index[i]` — which trunk core `i` owns. `u16::MAX`
///   means the core has no trunk assigned (available for
///   branches / convergence / leftover work).
/// - `fiber_assignments[i]` — primary fiber pinned to core `i`.
/// - `morsel_size_multiplier[i]` — size multiplier in
///   basis-points-style units (100 = 1.0x, 200 = 2.0x). Integer
///   avoids float in no-std + no-alloc context.
/// - `assigned_count` — count of populated slots
///   (0..=MAX_CORES).
#[derive(Copy, Clone, Debug)]
pub struct CoreAssignment<const MAX_CORES: usize> {
    pub trunk_index: [u16; MAX_CORES],
    pub fiber_assignments: [FiberId; MAX_CORES],
    pub morsel_size_multiplier: [u16; MAX_CORES],
    pub assigned_count: u16,
}

impl<const MAX_CORES: usize> CoreAssignment<MAX_CORES> {
    /// Empty assignment: every core has no trunk (sentinel
    /// `u16::MAX`), fiber 0, default multiplier 100 (1.0x).
    pub const fn new() -> Self {
        Self {
            trunk_index: [u16::MAX; MAX_CORES],
            fiber_assignments: [FiberId(0); MAX_CORES],
            morsel_size_multiplier: [100; MAX_CORES],
            assigned_count: 0,
        }
    }
}

impl<const MAX_CORES: usize> Default for CoreAssignment<MAX_CORES> {
    fn default() -> Self {
        Self::new()
    }
}
