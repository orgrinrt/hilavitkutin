//! Phase boundaries: waist-derived phase splits (domain 11).
//!
//! A phase is a segment of the execution plan delimited by waists
//! (narrow cut points in the DAG). Produced by step 4.

/// Newtype wrapping a phase index. `#[repr(transparent)]`. u8 is
/// plenty — phases rarely exceed 20.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PhaseId(pub u8);

/// Phase split points — `boundaries[i]` is the first node index of
/// phase `i`. Phase 0 always starts at node 0.
#[derive(Copy, Clone, Debug)]
pub struct PhaseBoundaries<const MAX_PHASES: usize> {
    pub boundaries: [u32; MAX_PHASES],
    /// Number of phases actually populated (1..=MAX_PHASES).
    pub phase_count: u8,
}

impl<const MAX_PHASES: usize> PhaseBoundaries<MAX_PHASES> {
    /// Single phase starting at node 0.
    pub const fn new() -> Self {
        Self {
            boundaries: [0; MAX_PHASES],
            phase_count: 0,
        }
    }
}

impl<const MAX_PHASES: usize> Default for PhaseBoundaries<MAX_PHASES> {
    fn default() -> Self {
        Self::new()
    }
}
