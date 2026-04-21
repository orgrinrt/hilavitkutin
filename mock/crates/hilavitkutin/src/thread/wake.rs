//! Wake strategy: how a pool thread waits for work (domain 20).
//!
//! DESIGN prescribes the hybrid spin-park default (128 spin iters
//! then park). PureSpin / PurePark are escape hatches for
//! latency-only / throughput-only workloads.

use arvo::USize;

/// Wake strategy for a pool thread waiting on work.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WakeStrategy {
    /// Spin `spin_iters` rounds (via `core::hint::spin_loop`) then
    /// park. Default strategy per DESIGN (128 spin iters).
    HybridSpinPark { spin_iters: USize },
    /// Busy-wait forever, never park. Lowest wake latency, highest
    /// CPU cost.
    PureSpin,
    /// Park immediately, no spinning. Lowest CPU cost, highest
    /// wake latency.
    PurePark,
}

impl WakeStrategy {
    /// Default hybrid strategy: 128 spin iters then park.
    pub const fn default_hybrid() -> Self {
        Self::HybridSpinPark { spin_iters: USize(128) }
    }
}

impl Default for WakeStrategy {
    fn default() -> Self {
        Self::default_hybrid()
    }
}
