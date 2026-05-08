//! Per-phase adaptive tuning parameters (domain 22).
//!
//! Carries the three knobs runtime adaptation turns between
//! frames: how aggressively to fuse, how large to size morsels,
//! and where to split further. Consumed by `select_adapt_config`
//! + `update_adapt` stubs in `adapt::mod`.

use arvo::USize;

use crate::plan::PhaseId;

/// Per-phase adaptive tuning parameters.
///
/// - `max_fuse_threshold`: max records fuseable into a single
///   WU body.
/// - `morsel_size_multiplier`: basis-points-style multiplier
///   (100 = 1.0x, 200 = 2.0x). Integer avoids float in
///   no-std + no-alloc context.
/// - `split_threshold`: record count above which a phase
///   splits into additional morsels.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AdaptConfig {
    pub phase_id: PhaseId,
    pub max_fuse_threshold: USize,
    pub morsel_size_multiplier: USize,
    pub split_threshold: USize,
}

impl AdaptConfig {
    /// Construct an AdaptConfig with explicit parameter values.
    pub const fn new(
        phase_id: PhaseId,
        max_fuse_threshold: USize,
        morsel_size_multiplier: USize,
        split_threshold: USize,
    ) -> Self {
        Self {
            phase_id,
            max_fuse_threshold,
            morsel_size_multiplier,
            split_threshold,
        }
    }
}
