//! GATE-2 R4c slice 1: `OsThreadPool::spawn` runs a real worker closure.
//!
//! Proves the os tier's `ThreadPoolApi::spawn<F>` actually launches `f` on a
//! worker thread (no-alloc, pointer-sized-closure smuggle, detached pthread) and
//! that `worker_count` reports a real core count. Fail-first: the prior no-op
//! `spawn` never flips the flag, so the bounded wait expires and the assert
//! fails (it does NOT hang). Mechanism proven by sketch
//! `202606071700_gate2-threadpoolapi-contract`.
//!
//! Default `platform-os` feature only (the os tier; the std tier is a stub by
//! design). `Arc<AtomicBool>` is pointer-sized and Send, so the closure capturing
//! one clone fits the spawn smuggle, and the Arc keeps the flag alive until both
//! the test and the worker drop their clones (no use-after-free without a join).

#![cfg(feature = "platform-os")]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use hilavitkutin::OsThreadPool;
use hilavitkutin_api::platform::ThreadPoolApi;

#[test]
fn spawn_runs_a_pointer_sized_closure_on_a_worker() {
    let pool = OsThreadPool::new();

    let flag = Arc::new(AtomicBool::new(false));
    let flag_worker = Arc::clone(&flag);
    // The closure captures one Arc (pointer-sized, Send), flips the flag.
    pool.spawn(move || {
        flag_worker.store(true, Ordering::Release);
    });

    // Bounded wait so a no-op spawn fails the assert instead of hanging.
    let deadline = Instant::now() + Duration::from_secs(2);
    while !flag.load(Ordering::Acquire) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(
        flag.load(Ordering::Acquire),
        "OsThreadPool::spawn must run the closure on a worker thread (flag never \
         flipped within the deadline: spawn did not launch the closure)"
    );
}

#[test]
fn worker_count_reports_at_least_one_core() {
    let pool = OsThreadPool::new();
    assert!(
        pool.worker_count().0 >= 1,
        "worker_count must report at least one core via sysconf"
    );
}
