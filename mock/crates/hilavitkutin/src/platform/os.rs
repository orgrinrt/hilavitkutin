//! OS platform tier: raw syscalls via libc.
//!
//! Backs `MemoryProviderApi` with mmap/munmap and `ClockApi` with
//! `clock_gettime(CLOCK_MONOTONIC)`. There is no thread pool here:
//! the engine ships the `ThreadPoolApi` contract and the consumer
//! supplies the executor that spawns OS threads, from its own code or
//! from a crate outside the `hilavitkutin*` family, gated per target
//! and feature.

use core::ffi::c_void;
use core::ptr;

use arvo::{Bool, USize};
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, Nanos};

/// mmap/munmap-backed memory provider.
///
/// Pages come from anonymous private mappings; alignment is page-
/// aligned by construction, so the requested `align` is honoured
/// for any power-of-two value up to the page size. A larger one, or
/// one that is not a power of two, is refused with null.
#[derive(Copy, Clone, Debug)]
pub struct OsMemoryProvider;

impl OsMemoryProvider {
    /// Construct a fresh provider.
    ///
    /// Stateless: every instance maps through the kernel directly.
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for OsMemoryProvider {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: the four promises of `MemoryProviderApi`. Each block is a fresh
// anonymous private mapping, page-aligned, of `len` bytes readable and
// writable (1), freed only by the `munmap` in `deallocate` and placed by the
// kernel, not inside the provider, so moving or dropping the provider leaves
// it where it is (2). The kernel never maps two live mappings over one page
// (3), and the provider holds no state, so concurrent calls are mmap and munmap
// on separate ranges (4). An `align` the mapping cannot meet is refused.
unsafe impl MemoryProviderApi for OsMemoryProvider {
    #[rustfmt::skip] // keeps the allow on the signature it governs
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        // SAFETY: sysconf takes no pointer and reads no caller memory.
        let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
        let page = USize(page.max(0) as usize); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sysconf returns a C long at the libc boundary; tracked: #72
        if !align.0.is_power_of_two() || page.0 == 0 || align.0 > page.0 {
            return ptr::null_mut();
        }
        // MAP_ANON | MAP_PRIVATE, PROT_READ | PROT_WRITE.
        // Caller responsibility (per trait contract): null on OOM.
        let addr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                *len as libc::size_t,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANON,
                -1,
                0,
            )
        };

        if addr == libc::MAP_FAILED {
            ptr::null_mut()
        } else {
            addr as *mut u8 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: libc mmap returns *mut c_void; cast to allocator ABI ptr; tracked: #72
        }
    }

    unsafe fn deallocate(
        &self,
        ptr: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        len: USize,
        _align: USize,
    ) {
        // `munmap` frees by address and length alone, so the
        // alignment the block was allocated with is not needed here.
        // Ignore the return value; a failed munmap on a pointer
        // produced by our allocate would be a consumer bug. The
        // trait contract says the pointer becomes invalid after
        // this call regardless.
        let _ = unsafe { libc::munmap(ptr as *mut c_void, *len as libc::size_t) };
    }

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        // FIXME(hilavitkutin_7_unmarked_placeholders): no mprotect call yet; the wiring lands with the persistence
        // mmap-file round, BACKLOG "Memory protection (mprotect)".
    }
}

/// `clock_gettime(CLOCK_MONOTONIC)`-backed clock.
#[derive(Copy, Clone, Debug)]
pub struct OsClock;

impl OsClock {
    /// Construct a fresh clock handle.
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for OsClock {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ClockApi for OsClock {
    fn now_ns(&self) -> Nanos {
        let mut ts = libc::timespec {
            tv_sec:  0,
            tv_nsec: 0,
        };
        // SAFETY: `ts` is a stack-owned timespec; libc writes
        // through the pointer once and never retains it. Return
        // value is ignored; CLOCK_MONOTONIC is available on every
        // tier-1 unix target.
        let _ = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
        let sec = ts.tv_sec as u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: CLOCK_MONOTONIC timespec -> ns bit pattern for Nanos; tracked: #72
        let nsec = ts.tv_nsec as u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: CLOCK_MONOTONIC timespec -> ns bit pattern for Nanos; tracked: #72
        let raw = sec.wrapping_mul(1_000_000_000).wrapping_add(nsec);
        Nanos::from_raw(raw)
    }
}
