//! Runtime metrics feeding adapt decisions (domain 22).
//!
//! Populated by PMC / perf_event sampling (follow-up round — see
//! BACKLOG). Consumed by `select_adapt_config` + `update_adapt`
//! stubs in `adapt::mod`.

use arvo::ufixed::UFixed;
use arvo::USize;
use hilavitkutin_api::Nanos;

/// Runtime counters feeding adapt decisions. Default-zero.
///
/// - `cache_miss_rate` — per-morsel L1/L2 miss rate in
///   basis points (0..=10000).
/// - `branch_miss_rate` — per-morsel branch-miss rate in
///   basis points.
/// - `phase_completion_time_ns` — wall time for the most recent
///   phase pass.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdaptMetrics {
    pub cache_miss_rate: USize,
    pub branch_miss_rate: USize,
    pub phase_completion_time_ns: Nanos,
}

impl AdaptMetrics {
    /// Construct a zero-initialised metrics record.
    pub const fn new() -> Self {
        Self {
            cache_miss_rate: USize(0),
            branch_miss_rate: USize(0),
            phase_completion_time_ns: UFixed::from_raw(0),
        }
    }
}
