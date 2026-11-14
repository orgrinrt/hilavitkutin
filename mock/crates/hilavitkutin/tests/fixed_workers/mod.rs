//! A worker-count-pinning executor, shared by the integration tests that run
//! the same frame at a chosen worker count.
//!
//! It sits beside `common` rather than inside it: every test binary that
//! declares `mod common;` compiles the whole module, so an item only some of
//! them use would be dead code in the rest. A binary that pins a worker count
//! declares `mod common;` and `mod fixed_workers;` both, since the executor
//! delegates its spawning to `common::TestExecutor`.

use arvo::USize;
use hilavitkutin_api::platform::ThreadPoolApi;

// `super`, not `crate`: the frozen authors' harness mounts this file and
// `common` side by side one module down.
use super::common::TestExecutor;

/// A `TestExecutor` that reports `W` workers whatever the machine has, so a
/// test can run the same frame at a chosen worker count.
// lint:allow(no-bare-numeric) reason: the worker count is a bare `usize` const-generic parameter; tracked: #121
#[derive(Copy, Clone, Debug, Default)]
pub struct FixedWorkers<const W: usize>;

impl<const W: usize> ThreadPoolApi for FixedWorkers<W> {
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        TestExecutor::new().spawn(f);
    }

    fn worker_count(&self) -> USize {
        USize(W)
    }
}
