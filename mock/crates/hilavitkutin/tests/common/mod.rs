//! Test support shared by the integration tests that need real workers.
//!
//! The engine ships `ThreadPoolApi` and nothing that spawns an OS thread;
//! the executor is always the consumer's. These tests are a consumer, so
//! they bring their own: one detached `std::thread` per `spawn`, sized by
//! `std::thread::available_parallelism`.
//!
//! It also holds the engine to the one promise a no-alloc executor relies on:
//! every closure the scheduler hands to `spawn` fits in a single pointer-sized,
//! pointer-aligned slot, so an executor can pass it to an OS thread as the one
//! `void *` argument without boxing it. A fatter closure panics here, at the
//! call site, and fails the test that spawned it.

use core::mem::{align_of, size_of};

use arvo::USize;
use hilavitkutin_api::platform::ThreadPoolApi;

/// Whether a value of `F` fits the one pointer-sized argument an OS thread
/// entry point receives.
pub const fn fits_one_pointer_slot<F>() -> bool {
    size_of::<F>() <= size_of::<*mut ()>() && align_of::<F>() <= align_of::<*mut ()>()
}

/// A `ThreadPoolApi` executor backed by `std::thread`, for tests only.
#[derive(Copy, Clone, Debug, Default)]
pub struct TestExecutor;

impl TestExecutor {
    /// Construct the executor handle. Stateless.
    pub const fn new() -> Self {
        Self
    }
}

impl ThreadPoolApi for TestExecutor {
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        assert!(
            fits_one_pointer_slot::<F>(),
            "the scheduler handed spawn a closure that does not fit one pointer-sized slot"
        );
        // Detached, like the scheduler expects: shutdown ordering comes from
        // the scheduler's worker-exit barrier, not from joining here.
        let _ = std::thread::spawn(f);
    }

    fn worker_count(&self) -> USize {
        USize(std::thread::available_parallelism().map_or(1, |n| n.get()))
    }
}
