//! Pre-allocated thread pool (Topic 6 axis G).
//!
//! The pool record holds the worker count, the per-class spin
//! budget, and the wake strategy. Actual worker creation routes
//! through the consumer's `ThreadPoolApi` provider at pipeline
//! construction; the engine never spawns runtime threads itself per
//! the `#![no_std]` substrate discipline.
//!
//! Builder: `ThreadPool::builder()` returns `ThreadPoolBuilder`;
//! `.with_wake_strategy(WakeStrategy)`, `.with_spin_budget(USize)`,
//! `.build()` finalises the record. `WakeStrategy` is the api crate's
//! struct (per-class spin + futex/park thresholds); the legacy
//! `wake::WakeStrategy` enum retires per the pre-1.0 no-legacy-shims
//! rule.
//!
//! Shutdown protocol: `Drop` sets the local `shutdown_requested`
//! flag with `Release`. The scheduler's `Drop` (Pass 6) is what
//! actually publishes the flag into every worker's `PoolFrame.shutdown`
//! atomic and wakes the parked workers via the parking module's
//! `atomic_wake_all`. The pool's own `Drop` here records intent; the
//! scheduler observes it.

use core::sync::atomic::{AtomicBool, Ordering};

use arvo::USize;
use hilavitkutin_api::platform::WakeStrategy;

/// Pre-allocated pool record. Owned by the scheduler; the pool's
/// fields drive both the construction-time worker creation (via the
/// consumer's `ThreadPoolApi`) and the per-worker mainloop's
/// wake-tier selection.
pub struct ThreadPool {
    /// Number of worker threads the scheduler should bring up against
    /// this pool record.
    pub thread_count: USize,
    /// Per-worker spin budget for the SpinThenWait tier. Sentinel
    /// `USize::MAX` reads as "pure spin"; `USize::ZERO` as
    /// "immediate park".
    pub spin_budget: USize,
    /// Wake strategy: per-CoreClass spin budget + futex/park
    /// thresholds. Workers consult this on phase entry to pick
    /// `ParkTier`.
    pub wake_strategy: WakeStrategy,
    /// Shutdown intent flag. Set by `Drop`; observed by the
    /// scheduler's own drop path which publishes the signal into
    /// every worker's `PoolFrame.shutdown`.
    shutdown_requested: AtomicBool,
}

impl ThreadPool {
    /// Construct a pool with the substrate-default wake strategy.
    pub fn new(core_count: USize) -> Self {
        Self {
            thread_count: core_count,
            spin_budget: USize(128), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: documented spin-budget default; tracked: #121
            wake_strategy: WakeStrategy::default_hybrid(),
            shutdown_requested: AtomicBool::new(false),
        }
    }

    /// Open the builder. Equivalent to `ThreadPoolBuilder::new()`.
    pub fn builder() -> ThreadPoolBuilder {
        ThreadPoolBuilder::new()
    }

    /// Observe whether shutdown has been requested. The scheduler
    /// calls this in its own `Drop` to decide whether to publish the
    /// signal to every worker's `PoolFrame.shutdown` slot.
    pub fn shutdown_requested(&self) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: simple flag observer; tracked: #121
        self.shutdown_requested.load(Ordering::Acquire)
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        Self::new(USize(1)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: default thread count; tracked: #121
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Record the intent; the scheduler observes via
        // `shutdown_requested()` and publishes into every worker's
        // `PoolFrame.shutdown`. The pool itself cannot reach the
        // PoolFrame from here (the frame lives in the scratch arena
        // owned by the scheduler).
        self.shutdown_requested.store(true, Ordering::Release);
    }
}

/// Builder for `ThreadPool`. Fluent surface for picking the wake
/// strategy + spin budget at scheduler construction time.
pub struct ThreadPoolBuilder {
    thread_count: USize,
    spin_budget: USize,
    wake_strategy: WakeStrategy,
}

impl ThreadPoolBuilder {
    /// Open a builder at substrate defaults.
    pub fn new() -> Self {
        Self {
            thread_count: USize(1), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: default thread count; tracked: #121
            spin_budget: USize(128), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: documented spin-budget default; tracked: #121
            wake_strategy: WakeStrategy::default_hybrid(),
        }
    }

    /// Pick the per-class spin + futex/park strategy.
    pub fn with_wake_strategy(mut self, strat: WakeStrategy) -> Self {
        self.wake_strategy = strat;
        self
    }

    /// Pick the spin budget for the SpinThenWait tier.
    pub fn with_spin_budget(mut self, budget: USize) -> Self {
        self.spin_budget = budget;
        self
    }

    /// Pick the worker count.
    pub fn with_thread_count(mut self, count: USize) -> Self {
        self.thread_count = count;
        self
    }

    /// Finalise into a `ThreadPool` record. The scheduler picks the
    /// record up at construction time and routes the actual worker
    /// creation through `ThreadPoolApi`.
    pub fn build(self) -> ThreadPool {
        ThreadPool {
            thread_count: self.thread_count,
            spin_budget: self.spin_budget,
            wake_strategy: self.wake_strategy,
            shutdown_requested: AtomicBool::new(false),
        }
    }
}

impl Default for ThreadPoolBuilder {
    fn default() -> Self {
        Self::new()
    }
}
