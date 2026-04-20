//! Per-phase adaptive tuning parameters (domain 22).
//!
//! Carries the three knobs runtime adaptation turns between
//! frames: how aggressively to fuse, how large to size morsels,
//! and where to split further. Consumed by `select_adapt_config`
//! + `update_adapt` stubs in `adapt::mod`.

use crate::plan::PhaseId;

/// Per-phase adaptive tuning parameters.
///
/// - `max_fuse_threshold` — max records fuseable into a single
///   WU body.
/// - `morsel_size_multiplier` — basis-points-style multiplier
///   (100 = 1.0x, 200 = 2.0x). Integer avoids float in
///   no-std + no-alloc context.
/// - `split_threshold` — record count above which a phase
///   splits into additional morsels.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct AdaptConfig {
    pub phase_id: PhaseId,
    pub max_fuse_threshold: u32,
    pub morsel_size_multiplier: u16,
    pub split_threshold: u32,
}

impl AdaptConfig {
    /// Construct an AdaptConfig with explicit parameter values.
    pub const fn new(
        phase_id: PhaseId,
        max_fuse_threshold: u32,
        morsel_size_multiplier: u16,
        split_threshold: u32,
    ) -> Self {
        Self {
            phase_id,
            max_fuse_threshold,
            morsel_size_multiplier,
            split_threshold,
        }
    }
}
