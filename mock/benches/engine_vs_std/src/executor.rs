//! The bench's own executor behind `ThreadPoolApi`.
//!
//! The engine ships the contract and spawns no threads itself, so this
//! bench, being a consumer, supplies one: a detached `std::thread` per
//! `spawn`, sized by `std::thread::available_parallelism`, which is the same
//! count `std_threads` gives the parallel std baselines. The scheduler's
//! pool spawns its workers once and keeps them, so the spawn path is outside
//! every timed region.

use arvo::USize;
use hilavitkutin_api::platform::ThreadPoolApi;

/// A `ThreadPoolApi` executor backed by `std::thread`.
#[derive(Copy, Clone, Debug, Default)]
pub struct BenchExecutor;

impl BenchExecutor {
    /// Construct the executor handle. Stateless.
    pub const fn new() -> Self {
        Self
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
        USize(crate::std_threads())
    }
}
