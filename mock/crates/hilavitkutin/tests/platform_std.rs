//! Std-tier platform smoke tests.
//!
//! Mirrors `platform_os.rs` against `StdMemoryProvider` /
//! `StdClock`.

#![cfg(feature = "platform-std")]

use arvo::{Bool, USize};
use hilavitkutin::{StdClock, StdMemoryProvider};
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi};

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
    unsafe { provider.deallocate(ptr, len) };
}

#[test]
fn std_memory_protect_is_ok_stub() {
    let provider = StdMemoryProvider::new();
    let len = USize(4096);

    // SAFETY: see roundtrip test.
    let ptr = unsafe { provider.allocate(len, USize(16)) };
    assert!(!ptr.is_null());

    // SAFETY: ptr is owned by this provider and covers `len`.
    unsafe { provider.protect(ptr, len, Bool::TRUE, Bool::TRUE) };

    // SAFETY: ptr is still valid after the stubbed protect.
    unsafe { provider.deallocate(ptr, len) };
}

#[test]
fn std_clock_is_monotonic() {
    let clock = StdClock::new();
    let a = clock.now_ns();
    for _ in 0..1_000 {
        core::hint::spin_loop();
    }
    let b = clock.now_ns();
    assert!(b >= a, "clock went backwards: {} -> {}", a, b);
}
