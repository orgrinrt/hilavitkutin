//! The bench's own executor behind `ThreadPoolApi`.
//!
//! The engine ships the contract and spawns no threads itself, so this
//! bench, being a consumer, supplies one: a detached `std::thread` per
//! `spawn`. `new` sizes it by `std::thread::available_parallelism`, which is
//! the same count `std_threads` gives the parallel std baselines;
//! `with_workers` names the count, for a bench sweeping it. The scheduler's
//! pool spawns its workers once and keeps them, so the spawn path is outside
//! every timed region.

use arvo::USize;
use hilavitkutin_api::platform::ThreadPoolApi;

/// A `ThreadPoolApi` executor backed by `std::thread`.
#[derive(Copy, Clone, Debug)]
pub struct BenchExecutor {
    workers: usize,
}

impl BenchExecutor {
    /// The executor sized to the machine's available parallelism.
    pub fn new() -> Self {
        Self::with_workers(crate::std_threads())
    }

    /// The executor reporting `workers` as its worker count. The engine
    /// clamps the count into `1..=MAX_CORES` itself.
    pub const fn with_workers(workers: usize) -> Self {
        Self { workers }
    }
}

impl Default for BenchExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreadPoolApi for BenchExecutor {
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = std::thread::spawn(f);
    }

    fn worker_count(&self) -> USize {
        USize(self.workers)
    }
}
