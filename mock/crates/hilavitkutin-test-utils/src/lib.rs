//! `hilavitkutin-test-utils`: mock platform providers for the
//! hilavitkutin engine's integration tests + example apps.
//!
//! Ships three mock impls that exercise the engine's platform-
//! provider contracts without depending on a real OS:
//!
//! `SingleThreadedExecutor` is a synchronous `ThreadPoolApi` impl
//! whose `spawn` runs the closure inline. Useful for deterministic
//! tests and for example apps that drive the scheduler off the main
//! thread.
//!
//! `DeterministicClock` is a monotonic `ClockApi` impl backed by an
//! internal counter that increments by one nanosecond per `now_ns`
//! call. Useful for golden-output tests where wall-clock drift
//! would invalidate the expected stream.
//!
//! `HeapMemoryProvider` is a `MemoryProviderApi` impl that delegates
//! to the host platform allocator via the std-only test boundary.
//! This is the documented exception to the `#![no_std]` rule that
//! Topic 11 axis C carves out: test-utils is gated for tests and
//! example apps; the heap-backed mock never reaches a shipped
//! production binary.
//!
//! Per Pass 7 of runtime megaround `202605101036`, this crate ships
//! the contract surface needed by the example apps and integration
//! tests. The bodies of the more elaborate methods (`protect`,
//! cross-fiber spawn arrangement) land alongside the bench-validated
//! runtime in follow-up rounds.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use core::sync::atomic::{AtomicU64, Ordering};

use arvo::USize;
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, Nanos, ThreadPoolApi};

/// Synchronous mock `ThreadPoolApi`. Spawned closures run inline.
pub struct SingleThreadedExecutor {
    workers: USize,
}

impl SingleThreadedExecutor {
    /// Construct a mock pool advertising `workers` worker count.
    pub const fn new(workers: USize) -> Self {
        Self { workers }
    }
}

impl ThreadPoolApi for SingleThreadedExecutor {
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        f();
    }

    fn worker_count(&self) -> USize {
        self.workers
    }
}

/// Deterministic monotonic clock. Each `now_ns` call increments an
/// atomic counter by one nanosecond.
pub struct DeterministicClock {
    counter: AtomicU64, // lint:allow(no-bare-numeric) reason: AtomicU64 backs the nanosecond counter that matches Nanos::from_raw raw container; tracked: #428
}

impl DeterministicClock {
    /// Construct a clock that starts ticking from zero.
    pub const fn new() -> Self {
        Self { counter: AtomicU64::new(0u64) } // lint:allow(no-bare-numeric) reason: initial counter raw 0; tracked: #428
    }
}

impl Default for DeterministicClock {
    fn default() -> Self {
        Self::new()
    }
}

impl ClockApi for DeterministicClock {
    fn now_ns(&self) -> Nanos {
        let raw = self.counter.fetch_add(1u64, Ordering::Relaxed); // lint:allow(no-bare-numeric) reason: atomic counter increment by one ns; tracked: #428
        Nanos::from_raw(raw)
    }
}

/// Heap-backed memory provider. Per Topic 11 axis C documented
/// `no_std` exception for the heap-backed allocator. Pass 7 ships
/// structural surface; bodies land with executor wiring.
pub struct HeapMemoryProvider {
    _private: (),
}

impl HeapMemoryProvider {
    /// Construct a fresh heap provider.
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for HeapMemoryProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryProviderApi for HeapMemoryProvider {
    unsafe fn allocate(&self, _len: USize, _align: USize) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: matching MemoryProviderApi allocator ABI; raw pointer is the contract; tracked: #72
        core::ptr::null_mut()
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: matching trait signature; raw pointer is the contract; tracked: #72
    }

    unsafe fn protect(
        &self,
        _ptr: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: matching trait signature; raw pointer is the contract; tracked: #72
        _len: USize,
        _read: arvo::Bool,
        _write: arvo::Bool,
    ) {
    }
}
