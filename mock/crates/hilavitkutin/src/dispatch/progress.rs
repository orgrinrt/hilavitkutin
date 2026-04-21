//! Progress counter — per-fiber monotonic record index (domain 17).
//!
//! Release store / Acquire load. Lock-free by construction. Ships
//! `#[repr(transparent)]` over `AtomicUsize` so downstream can lay
//! out a parallel array of counters without padding.

use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::USize;

/// Per-fiber monotonic record index.
#[repr(transparent)]
#[derive(Debug, Default)]
pub struct ProgressCounter(AtomicUsize);

impl ProgressCounter {
    /// Construct a counter initialised to `start`.
    pub const fn new(start: USize) -> Self {
        Self(AtomicUsize::new(start.0))
    }

    /// Release store. Publishes `value` to any thread doing a
    /// later Acquire load on this counter.
    pub fn store(&self, value: USize) {
        self.0.store(*value, Ordering::Release);
    }

    /// Acquire load. Pairs with a Release store from the writer.
    pub fn load(&self) -> USize {
        USize(self.0.load(Ordering::Acquire))
    }
}
