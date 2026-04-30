//! OS-tier platform smoke tests.
//!
//! Exercises allocate/deallocate and monotonic clock progression
//! through `OsMemoryProvider` / `OsClock`. Thread-pool tests defer
//! to sub-round 5a4 (see BACKLOG).

#![cfg(feature = "platform-os")]

use arvo::{Bool, USize};
use hilavitkutin::{OsClock, OsMemoryProvider};
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi};

#[test]
fn os_memory_allocate_deallocate_roundtrip() {
    let provider = OsMemoryProvider::new();
    let len = USize(4096);
    let align = USize(16);

    // SAFETY: alignment is a power of two, len is positive.
    let ptr = unsafe { provider.allocate(len, align) };
    assert!(!ptr.is_null(), "mmap returned null for a 4KiB request");

    // Touch every byte so the kernel actually faults in pages.
    for i in 0..*len {
        // SAFETY: ptr covers `len` bytes per the trait contract.
        unsafe { ptr.add(i).write(0xAB) };
    }

    // SAFETY: ptr came from allocate with the same len.
    unsafe { provider.deallocate(ptr, len) };
}

#[test]
fn os_memory_protect_is_ok_stub() {
    let provider = OsMemoryProvider::new();
    let len = USize(4096);

    // SAFETY: see roundtrip test.
    let ptr = unsafe { provider.allocate(len, USize(16)) };
    assert!(!ptr.is_null());

    // Stub: protect is a no-op this round. Verify it doesn't
    // panic or corrupt the pointer.
    // SAFETY: ptr is owned by this provider and covers `len`.
    unsafe { provider.protect(ptr, len, Bool::TRUE, Bool::TRUE) };

    // SAFETY: ptr is still valid after the stubbed protect.
    unsafe { provider.deallocate(ptr, len) };
}

#[test]
fn os_clock_is_monotonic() {
    let clock = OsClock::new();
    let a = clock.now_ns();
    // Spin briefly to make a forward delta measurable.
    for _ in 0..1_000 {
        core::hint::spin_loop();
    }
    let b = clock.now_ns();
    assert!(b >= a, "clock went backwards: {} -> {}", a, b);
}
