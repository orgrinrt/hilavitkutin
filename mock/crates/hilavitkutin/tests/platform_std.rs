//! Std-tier platform smoke tests.
//!
//! Mirrors `platform_os.rs` against `StdMemoryProvider` /
//! `StdClock`.

#![cfg(feature = "platform-std")]

use arvo::{Bool, USize};
use hilavitkutin::{StdClock, StdMemoryProvider, StdThreadPool};
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, ThreadPoolApi};

#[test]
fn std_memory_allocate_deallocate_roundtrip() {
    let provider = StdMemoryProvider::new();
    let len = USize(4096);
    let align = USize(16);

    // SAFETY: alignment is a power of two, len is positive.
    let ptr = unsafe { provider.allocate(len, align) };
    assert!(!ptr.is_null(), "std alloc returned null for a 4KiB request");

    for i in 0..*len {
        // SAFETY: ptr covers `len` bytes per the trait contract.
        unsafe { ptr.add(i).write(0xCD) };
    }

    // SAFETY: ptr came from allocate with the same len.
    unsafe { provider.deallocate(ptr, len, align) };
}

#[test]
fn std_memory_protect_is_ok_stub() {
    let provider = StdMemoryProvider::new();
    let len = USize(4096);

    // SAFETY: see roundtrip test.
    let align = USize(16);
    let ptr = unsafe { provider.allocate(len, align) };
    assert!(!ptr.is_null());

    // SAFETY: ptr is owned by this provider and covers `len`.
    unsafe { provider.protect(ptr, len, Bool::TRUE, Bool::TRUE) };

    // SAFETY: ptr is still valid after the stubbed protect.
    unsafe { provider.deallocate(ptr, len, align) };
}

#[test]
fn std_clock_is_monotonic() {
    let clock = StdClock::new();
    let a = clock.now_ns();
    for _ in 0..1_000 {
        core::hint::spin_loop();
    }
    let b = clock.now_ns();
    let a_raw = a.to_raw();
    let b_raw = b.to_raw();
    assert!(b_raw >= a_raw, "clock went backwards: {} -> {}", a_raw, b_raw);
}

/// The std tier's pool must actually run what it is handed.
///
/// `run_parallel` publishes a frame and then blocks waiting for workers,
/// so a `spawn` that drops its closure does not fail loudly: it hangs.
/// This asserts the closure ran, with a bounded wait so a regression
/// reports as a failure rather than as a test that never returns.
#[test]
fn std_thread_pool_spawn_runs_the_closure() {
    use std::sync::mpsc;
    use std::time::Duration;

    let pool = StdThreadPool::new();
    let (tx, rx) = mpsc::channel();

    pool.spawn(move || {
        let _ = tx.send(());
    });

    assert!(
        rx.recv_timeout(Duration::from_secs(5)).is_ok(),
        "StdThreadPool::spawn did not run the closure within 5s; \
         the pool accepted the work and dropped it"
    );
}
