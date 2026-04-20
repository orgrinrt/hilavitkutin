//! Runtime metrics feeding adapt decisions (domain 22).
//!
//! Populated by PMC / perf_event sampling (follow-up round — see
//! BACKLOG). Consumed by `select_adapt_config` + `update_adapt`
//! stubs in `adapt::mod`.

/// Runtime counters feeding adapt decisions. Default-zero.
///
/// - `cache_miss_rate` — per-morsel L1/L2 miss rate in
///   basis points (0..=10000).
/// - `branch_miss_rate` — per-morsel branch-miss rate in
///   basis points.
/// - `phase_completion_time_ns` — wall time for the most recent
///   phase pass.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct AdaptMetrics {
    pub cache_miss_rate: u32,
    pub branch_miss_rate: u32,
    pub phase_completion_time_ns: u64,
}

impl AdaptMetrics {
    /// Construct a zero-initialised metrics record.
    pub const fn new() -> Self {
        Self {
            cache_miss_rate: 0,
            branch_miss_rate: 0,
            phase_completion_time_ns: 0,
        }
    }
}
