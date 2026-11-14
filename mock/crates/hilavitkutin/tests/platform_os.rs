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
    for i in 0 .. *len {
        // SAFETY: ptr covers `len` bytes per the trait contract.
        unsafe { ptr.add(i).write(0xAB) };
    }

    // SAFETY: ptr came from allocate with the same len.
    unsafe { provider.deallocate(ptr, len, align) };
}

/// A mapping is page-aligned and no more, so an alignment wider than a page
/// cannot be met and one that is not a power of two means nothing; both are
/// refused with null rather than handed a block that only looks aligned. The
/// page-wide alignment, the widest a mapping meets, is handed a block at it.
#[test]
fn os_memory_refuses_an_alignment_a_mapping_cannot_meet() {
    let provider = OsMemoryProvider::new();
    // SAFETY: sysconf takes no pointer and reads no caller memory.
    let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
    let len = USize(4096);
    for refused in [page * 2, 3, 0] {
        // SAFETY: a refusal hands out nothing, so there is nothing to free.
        let ptr = unsafe { provider.allocate(len, USize(refused)) };
        assert!(
            ptr.is_null(),
            "align {refused} with {page}-byte pages is refused"
        );
    }
    // SAFETY: a power of two no wider than a page, len is positive.
    let ptr = unsafe { provider.allocate(len, USize(page)) };
    assert!(!ptr.is_null() && ptr.addr() % page == 0);
    // SAFETY: ptr came from allocate with the same len and align.
    unsafe { provider.deallocate(ptr, len, USize(page)) };
}

#[test]
fn os_memory_protect_is_ok_stub() {
    let provider = OsMemoryProvider::new();
    let len = USize(4096);

    // SAFETY: see roundtrip test.
    let align = USize(16);
    let ptr = unsafe { provider.allocate(len, align) };
    assert!(!ptr.is_null());

    // Stub: protect is a no-op this round. Verify it doesn't
    // panic or corrupt the pointer.
    // SAFETY: ptr is owned by this provider and covers `len`.
    unsafe { provider.protect(ptr, len, Bool::TRUE, Bool::TRUE) };

    // SAFETY: ptr is still valid after the stubbed protect.
    unsafe { provider.deallocate(ptr, len, align) };
}

#[test]
fn os_clock_is_monotonic() {
    let clock = OsClock::new();
    let a = clock.now_ns();
    // Spin briefly to make a forward delta measurable.
    for _ in 0 .. 1_000 {
        core::hint::spin_loop();
    }
    let b = clock.now_ns();
    let a_raw = a.to_raw();
    let b_raw = b.to_raw();
    assert!(
        b_raw >= a_raw,
        "clock went backwards: {} -> {}",
        a_raw,
        b_raw
    );
}
