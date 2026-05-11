//! Phase boundaries: waist-derived phase splits (domain 11).
//!
//! A phase is a segment of the execution plan delimited by waists
//! (narrow cut points in the DAG). Produced by step 4.
//!
//! `PhaseId` re-exported via `crate::plan` from `hilavitkutin_api`
//! (USize-shaped, canonical engine id type).

use arvo::strategy::Identity;
use arvo::USize;

/// Phase split points: `boundaries[i]` is the first node index of
/// phase `i`. Phase 0 always starts at node 0.
#[derive(Copy, Clone, Debug)]
pub struct PhaseBoundaries<const MAX_PHASES: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pub boundaries: [USize; MAX_PHASES],
    /// Number of phases actually populated (1..=MAX_PHASES).
    pub phase_count: USize,
}

impl<const MAX_PHASES: usize> PhaseBoundaries<MAX_PHASES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    /// Single phase starting at node 0.
    pub const fn new() -> Self {
        Self {
            boundaries: [USize::ZERO; MAX_PHASES],
            phase_count: USize::ZERO,
        }
    }
}

impl<const MAX_PHASES: usize> Default for PhaseBoundaries<MAX_PHASES> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    fn default() -> Self {
        Self::new()
    }
}
