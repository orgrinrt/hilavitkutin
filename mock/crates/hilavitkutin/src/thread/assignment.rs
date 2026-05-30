//! Per-core role assignment (domain 20).
//!
//! Parallel-array layout mirrors the pattern used in
//! `plan::FiberGrouping` and `plan::PhaseBoundaries` (parallel
//! arrays + count). Keeps the struct `Copy`-friendly for
//! const-construction.

use arvo::USize;
use arvo::strategy::Identity;
use arvo_tensor::Capacity;

use crate::plan::FiberId;

/// Sentinel meaning "core has no trunk assigned" in `trunk_index`.
///
/// Kept distinct from any valid trunk index by lying above any
/// realistic trunk count.
pub const NO_TRUNK: USize = USize(u16::MAX as usize); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: preserves original 16-bit sentinel value for trunk_index; tracked: #72

/// Per-core role record. Sized by the core capacity `C`.
///
/// - `trunk_index[i]`: which trunk core `i` owns. `NO_TRUNK`
///   means the core has no trunk assigned (available for
///   branches / convergence / leftover work).
/// - `fiber_assignments[i]`: primary fiber pinned to core `i`.
/// - `morsel_size_multiplier[i]`: size multiplier in
///   basis-points-style units (100 = 1.0x, 200 = 2.0x). Integer
///   avoids float in no-std + no-alloc context.
/// - `assigned_count`: count of populated slots
///   (0..=core capacity).
pub struct CoreAssignment<C: Capacity> {
    pub trunk_index: <C as Capacity>::Array<USize>,
    pub fiber_assignments: <C as Capacity>::Array<FiberId>,
    pub morsel_size_multiplier: <C as Capacity>::Array<USize>,
    pub assigned_count: USize,
}

impl<C: Capacity> CoreAssignment<C> {
    /// Empty assignment: every core has no trunk (`NO_TRUNK`
    /// sentinel), fiber 0, default multiplier 100 (1.0x).
    pub fn new() -> Self {
        Self {
            trunk_index: <C as Capacity>::filled(NO_TRUNK),
            fiber_assignments: <C as Capacity>::filled(FiberId::ZERO),
            morsel_size_multiplier: <C as Capacity>::filled(USize(100)), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: default 1.0x multiplier in basis-points; tracked: #72
            assigned_count: USize::ZERO,
        }
    }
}

impl<C: Capacity> Copy for CoreAssignment<C>
where
    <C as Capacity>::Array<USize>: Copy,
    <C as Capacity>::Array<FiberId>: Copy,
{
}

impl<C: Capacity> Clone for CoreAssignment<C>
where
    <C as Capacity>::Array<USize>: Copy,
    <C as Capacity>::Array<FiberId>: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: Capacity> Default for CoreAssignment<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Capacity> core::fmt::Debug for CoreAssignment<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CoreAssignment")
            .field("assigned_count", &self.assigned_count.0)
            .finish_non_exhaustive()
    }
}
