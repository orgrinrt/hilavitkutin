//! Std platform tier — fallback using `std::alloc`, `std::thread`,
//! `std::time::Instant`.
//!
//! Used when `platform-std` is active (typically in CI or on hosts
//! where hand-rolled syscalls are undesirable). The crate itself
//! remains `#![no_std]`; `std` is pulled in only for this module.

extern crate std;

use core::ptr;

use arvo::newtype::{Bool, USize};
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, ThreadPoolApi};
use hilavitkutin_api::Nanos;
use std::alloc::{alloc, dealloc, Layout};
use std::sync::OnceLock;
use std::time::Instant;

/// `std::alloc`-backed memory provider.
///
/// Allocation size and alignment pair is stored by the caller;
/// deallocation requires the same `len` used for allocate (per
/// trait contract). A scratch allocation alignment of one word is
/// used when `align == 0` to satisfy `Layout`.
#[derive(Copy, Clone, Debug)]
pub struct StdMemoryProvider;

impl StdMemoryProvider {
    /// Construct a fresh provider.
    #[inline]
    pub const fn new() -> Self {
        Self
    }

    /// Normalise alignment to a non-zero power of two.
    #[inline]
    fn layout_for(len: USize, align: USize) -> Layout {
        let a = if *align == 0 { core::mem::align_of::<usize>() } else { *align }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Layout ABI default alignment; tracked: #72
        // SAFETY: `len` and `a` satisfy Layout's invariants for
        // any caller honouring the trait contract (power-of-two
        // alignment). Callers that violate this get an Err from
        // from_size_align, which we translate to a zero-sized
        // layout — the subsequent `alloc` will return null.
        Layout::from_size_align(*len, a).unwrap_or_else(|_| {
            Layout::from_size_align(0, core::mem::align_of::<usize>()).unwrap() // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Layout ABI fallback alignment; tracked: #72
        })
    }
}

impl Default for StdMemoryProvider {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryProviderApi for StdMemoryProvider {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        let layout = Self::layout_for(len, align);
        if layout.size() == 0 {
            return ptr::null_mut();
        }
        // SAFETY: `layout` carries a non-zero size and a power-of-
        // two alignment (enforced by Layout::from_size_align).
        unsafe { alloc(layout) }
    }

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        if ptr.is_null() || *len == 0 {
            return;
        }
        // The trait contract requires the caller to pass the same
        // `len` used for allocate; alignment defaults to word size
        // for matching.
        let layout = Self::layout_for(len, USize(core::mem::align_of::<usize>())); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Layout ABI word alignment default; tracked: #72
        // SAFETY: contract delegated to the caller per
        // `MemoryProviderApi::deallocate`.
        unsafe { dealloc(ptr, layout) }
    }

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        // std::alloc has no page-protection primitive. Matching
        // the OS tier's skeleton-Ok stance until the persistence
        // round wires a real backend.
    }
}

/// `std::thread`-backed thread pool.
///
/// Skeleton — `spawn` accepts only a parameterless `fn()` via
/// `std::thread::spawn` without `Box<dyn _>`. Generic-closure
/// dispatch lands in sub-round 5a4.
#[derive(Copy, Clone, Debug)]
pub struct StdThreadPool;

impl StdThreadPool {
    /// Construct a fresh pool handle.
    #[inline]
    pub const fn new() -> Self {
        Self
    }

    /// Spawn a parameterless entry point on a fresh thread.
    pub fn spawn_fn(&self, f: fn()) {
        // `fn()` is Send + 'static; std::thread::spawn consumes
        // it directly without needing a boxed closure.
        let _ = std::thread::spawn(move || f());
    }
}

impl Default for StdThreadPool {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ThreadPoolApi for StdThreadPool {
    fn spawn<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // See os.rs — generic-closure support arrives in 5a4.
        let _ = _f;
    }

    fn worker_count(&self) -> USize {
        USize(1)
    }
}

/// `std::time::Instant`-backed monotonic clock.
///
/// Uses a fixed epoch captured on first access so `now_ns` can
/// return a plain `u64` delta (Instant has no epoch-absolute
/// conversion).
#[derive(Copy, Clone, Debug)]
pub struct StdClock;

static EPOCH: OnceLock<Instant> = OnceLock::new();

impl StdClock {
    /// Construct a fresh clock handle.
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for StdClock {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ClockApi for StdClock {
    fn now_ns(&self) -> Nanos {
        let epoch = EPOCH.get_or_init(Instant::now);
        let raw = Instant::now().duration_since(*epoch).as_nanos() as u64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Duration::as_nanos returns u128; truncate to u64 for Nanos; tracked: #72
        Nanos::from_raw(raw)
    }
}
