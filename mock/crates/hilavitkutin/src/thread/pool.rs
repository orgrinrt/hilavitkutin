//! Pre-allocated thread pool (domain 20).
//!
//! Skeleton record this round — real OS thread spawning is
//! feature-gated on a future `threading-std` feature (see BACKLOG).
//! The record fields capture the pool's intended shape so
//! downstream code can reference it now.

use arvo::USize;
use arvo::strategy::Identity;

use super::WakeStrategy;

/// Pre-allocated pool.
#[derive(Copy, Clone, Debug)]
pub struct ThreadPool {
    pub thread_count: USize,
    pub spin_budget: USize,
    pub wake_strategy: WakeStrategy,
}

impl ThreadPool {
    /// Construct a stub pool record. No threads are spawned this
    /// round; follow-up round wires real spawning under
    /// `threading-std`.
    pub const fn new(core_count: USize, wake: WakeStrategy) -> Self {
        let spin_budget = match wake {
            WakeStrategy::HybridSpinPark { spin_iters } => spin_iters,
            WakeStrategy::PureSpin => USize(usize::MAX), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: pure-spin sentinel = never park; tracked: #72
            WakeStrategy::PurePark => USize::ZERO,
        };
        Self {
            thread_count: core_count,
            spin_budget,
            wake_strategy: wake,
        }
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        Self::new(USize(1), WakeStrategy::default_hybrid())
    }
}
