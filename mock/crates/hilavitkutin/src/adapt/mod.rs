//! Runtime adaptation (domain 22).
//!
//! Per-phase tuning parameters + runtime metrics + mode
//! selection. The executor-reorganisation triggers live here;
//! per DESIGN they fire between frames (not during execution,
//! which would invalidate monomorphised dispatch).
//!
//! This module is the *skeleton* for 5a4: public surface is
//! complete; `select_adapt_config` + `update_adapt` stub to
//! `todo!()`. Real EMA heuristics + metric sampling are
//! follow-up rounds (see BACKLOG → Engine 5a4 follow-ups).

pub mod config;
pub mod metrics;

pub use config::AdaptConfig;
pub use metrics::AdaptMetrics;

/// Per-phase adaptation mode. Alias for
/// [`crate::strategy::PhaseStrategy`]: the two concepts are the
/// same enum referenced by different DESIGN sections. Strategy
/// remains the canonical home; adapt re-exports as `AdaptMode`
/// for caller ergonomics (the term "adapt mode" reads naturally
/// inside the adapt module's docs; "phase strategy" reads
/// naturally from the strategy selector's perspective).
pub type AdaptMode = crate::strategy::PhaseStrategy;

use crate::plan::PhaseId;

/// Select an `AdaptConfig` for `phase` given current `metrics`.
///
/// Skeleton: `todo!()`. Real heuristic (EMA latency vs
/// throughput, 1/8 decay per DESIGN) lands as a follow-up
/// round: see BACKLOG.
pub fn select_adapt_config(phase: PhaseId, metrics: &AdaptMetrics) -> AdaptConfig {
    let _ = (phase, metrics);
    todo!("5a4: adaptive mode selection heuristic")
}

/// Update per-phase adaptive configs in place from current
/// `metrics`. Executor-reorganisation trigger between frames
/// only (not during execution, per DESIGN).
///
/// Skeleton: `todo!()`. Real implementation walks phases +
/// updates `max_fuse_threshold`, `morsel_size_multiplier`,
/// `split_threshold` per EMA decay: see BACKLOG.
pub fn update_adapt<const MAX_PHASES: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    configs: &mut [AdaptConfig; MAX_PHASES],
    metrics: &AdaptMetrics,
) {
    let _ = (configs, metrics);
    todo!("5a4: per-phase adaptive config update")
}
