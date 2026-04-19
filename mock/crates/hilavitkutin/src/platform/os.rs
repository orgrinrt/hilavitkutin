//! OS platform tier — raw syscalls via libc.
//!
//! Backs `MemoryProviderApi` with mmap/munmap, `ClockApi` with
//! `clock_gettime(CLOCK_MONOTONIC)`, and `ThreadPoolApi` with a
//! pthread-based skeleton. Real generic-closure spawn + worker
//! sizing via sysconf land in follow-up sub-round 5a4.

use core::ffi::c_void;
use core::ptr;

use arvo::newtype::{Bool, USize};
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, ThreadPoolApi};

/// mmap/munmap-backed memory provider.
///
/// Pages come from anonymous private mappings; alignment is page-
/// aligned by construction, so the requested `align` is honoured
/// for any power-of-two value up to the page size. Larger
/// alignments are left for a follow-up round.
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

impl MemoryProviderApi for OsMemoryProvider {
    unsafe fn allocate(&self, len: USize, _align: USize) -> *mut u8 {
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
            addr as *mut u8
        }
    }

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize) {
        // Ignore the return value; a failed munmap on a pointer
        // produced by our allocate would be a consumer bug. The
        // trait contract says the pointer becomes invalid after
        // this call regardless.
        let _ = unsafe { libc::munmap(ptr as *mut c_void, *len as libc::size_t) };
    }

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {
        // Skeleton: real mprotect wiring lands with the persistence
        // mmap-file round. Tracked in BACKLOG under "Memory
        // protection (mprotect)".
    }
}

/// pthread-backed thread pool.
///
/// Skeleton — `spawn` accepts only a parameterless `fn()` via a
/// trampoline over a thin function pointer. Generic-closure
/// support with queue integration lands in sub-round 5a4.
/// `worker_count` returns `USize(1)` until the same round wires
/// up `sysconf(_SC_NPROCESSORS_ONLN)`.
#[derive(Copy, Clone, Debug)]
pub struct OsThreadPool;

impl OsThreadPool {
    /// Construct a fresh pool handle.
    ///
    /// Stateless skeleton; the real implementation in 5a4 will
    /// carry a pre-allocated worker set.
    #[inline]
    pub const fn new() -> Self {
        Self
    }

    /// Spawn a parameterless entry point on a fresh pthread.
    ///
    /// Skeleton path used by plan-stage wiring until the generic
    /// pool lands in 5a4. The thread is spawned detached-style
    /// (never joined) because the skeleton has no handle type;
    /// callers must not rely on completion.
    ///
    /// # Safety
    ///
    /// `f` must be safe to call on an independent thread. The
    /// caller must tolerate the spawn attempt failing silently
    /// (pthread_create returning non-zero); a real pool in 5a4
    /// will propagate the error.
    pub fn spawn_fn(&self, f: fn()) {
        // Box-free trampoline: smuggle the fn pointer through a
        // usize cast (same size on every target tier-1).
        let raw = f as usize;
        let mut tid: libc::pthread_t = unsafe { core::mem::zeroed() };
        let _ = unsafe {
            libc::pthread_create(
                &mut tid,
                ptr::null(),
                trampoline,
                raw as *mut c_void,
            )
        };
    }
}

/// pthread entry-point trampoline. Monomorphic over `fn()` — the
/// raw pointer argument encodes the consumer-supplied function.
extern "C" fn trampoline(arg: *mut c_void) -> *mut c_void {
    let raw = arg as usize;
    // SAFETY: `raw` was produced from a `fn()` pointer in
    // `OsThreadPool::spawn_fn`. The cast round-trip preserves the
    // ABI-compatible bit pattern on all tier-1 targets.
    let f: fn() = unsafe { core::mem::transmute::<usize, fn()>(raw) };
    f();
    ptr::null_mut()
}

impl Default for OsThreadPool {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl ThreadPoolApi for OsThreadPool {
    fn spawn<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Generic-closure dispatch without `dyn` requires either
        // per-consumer monomorphisation (pool carries a type
        // parameter) or a work-queue whose slots own F. Both land
        // in sub-round 5a4. Until then, the generic entry point
        // is a no-op to keep the trait satisfied.
        //
        // Consumers that need the skeleton right now call
        // `OsThreadPool::spawn_fn` directly with a `fn()`.
        let _ = _f;
    }

    fn worker_count(&self) -> USize {
        USize(1)
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
    fn now_ns(&self) -> u64 {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        // SAFETY: `ts` is a stack-owned timespec; libc writes
        // through the pointer once and never retains it. Return
        // value is ignored; CLOCK_MONOTONIC is available on every
        // tier-1 unix target.
        let _ = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
        (ts.tv_sec as u64).wrapping_mul(1_000_000_000).wrapping_add(ts.tv_nsec as u64)
    }
}
