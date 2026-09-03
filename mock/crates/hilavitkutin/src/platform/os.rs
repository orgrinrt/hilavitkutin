//! OS platform tier: raw syscalls via libc.
//!
//! Backs `MemoryProviderApi` with mmap/munmap, `ClockApi` with
//! `clock_gettime(CLOCK_MONOTONIC)`, and `ThreadPoolApi` with a
//! pthread-based skeleton. Real generic-closure spawn + worker
//! sizing via sysconf land in follow-up sub-round 5a4.

use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::{ManuallyDrop, align_of, size_of, transmute_copy};
use core::ptr;

use arvo::{Bool, USize};
use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, Nanos, ThreadPoolApi};

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
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
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

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, _align: USize) {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        // `munmap` frees by address and length alone, so the
        // alignment the block was allocated with is not needed here.
        // Ignore the return value; a failed munmap on a pointer
        // produced by our allocate would be a consumer bug. The
        // trait contract says the pointer becomes invalid after
        // this call regardless.
        let _ = unsafe { libc::munmap(ptr as *mut c_void, *len as libc::size_t) };
    }

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        // Skeleton: real mprotect wiring lands with the persistence
        // mmap-file round. Tracked in BACKLOG under "Memory
        // protection (mprotect)".
    }
}

/// pthread-backed thread pool.
///
/// Skeleton: `spawn` accepts only a parameterless `fn()` via a
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
}

/// Monomorphic per-F pthread entry-point for the generic `spawn`. The `arg`
/// pointer value carries the pointer-sized `F` by value (its bytes were copied
/// into the slot by `spawn`). Reconstruct `F` and call it once.
/// Compile-time guard that `F` fits the pthread argument slot, so `spawn`'s
/// `transmute_copy` into a `*mut c_void` reads no out-of-bounds bytes. Bad `F` is
/// a monomorphisation-time error via the associated const.
struct PtrSizedClosure<F>(PhantomData<F>);

impl<F> PtrSizedClosure<F> {
    const CHECK: () = {
        assert!(
            size_of::<F>() <= size_of::<*mut c_void>(),
            "OsThreadPool::spawn: closure must be pointer-sized (no alloc to box a fatter closure)"
        );
        assert!(
            align_of::<F>() <= align_of::<*mut c_void>(),
            "OsThreadPool::spawn: closure over-aligned for the pthread argument slot"
        );
    };
}

extern "C" fn tramp<F: FnOnce()>(arg: *mut c_void) -> *mut c_void {
    // SAFETY: `arg`'s bit pattern is exactly the bytes of `F` (spawn checked
    // size_of::<F>() <= size_of::<*mut c_void>()). `transmute_copy` reads
    // size_of::<F>() bytes from `&arg`, reconstructing the closure by value; the
    // original was held in `ManuallyDrop` so this is the sole owning copy.
    let f: F = unsafe { transmute_copy::<*mut c_void, F>(&arg) };
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
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // No-alloc generic-closure handoff: the closure is copied by value into
        // the pthread `*mut c_void` argument, so F must be pointer-sized. The
        // engine's worker closure captures exactly one pointer (a Send-wrapped
        // `*const WorkerCtx`), so it fits. A fatter closure is a compile error
        // here, never a heap allocation (no `Box`, no alloc on the engine path).
        // Force the compile-time size/align check (an inline `const {}` block
        // referencing a generic is rejected under `generic_const_exprs`, so the
        // assert lives in an associated const evaluated here at monomorphisation).
        let () = PtrSizedClosure::<F>::CHECK;
        // Hold F so its destructor does not run at the call site; the trampoline
        // (on a successful spawn) owns the copy and drops it after calling.
        let held = ManuallyDrop::new(f);
        // SAFETY: F is pointer-sized (checked above); copy its bytes into the
        // argument value. `held` keeps a bit-identical copy that is only dropped
        // on the spawn-failure path below (so F is dropped exactly once).
        let arg: *mut c_void = unsafe { transmute_copy::<F, *mut c_void>(&held) };

        // SAFETY: the attr lifecycle is local; the thread is detached, so no join
        // handle is retained. The persistent pool's shutdown ordering comes from a
        // Scheduler-owned worker-exit-counter barrier, not from joining here.
        let rc = unsafe {
            let mut attr: libc::pthread_attr_t = core::mem::zeroed();
            libc::pthread_attr_init(&mut attr);
            libc::pthread_attr_setdetachstate(&mut attr, libc::PTHREAD_CREATE_DETACHED);
            let mut tid: libc::pthread_t = core::mem::zeroed();
            let rc = libc::pthread_create(&mut tid, &attr, tramp::<F>, arg);
            libc::pthread_attr_destroy(&mut attr);
            rc
        };

        if rc != 0 {
            // Spawn failed (the trampoline will never run). Reclaim F so its
            // destructor runs exactly once. The contract is best-effort: a failed
            // spawn is silent, per the `ThreadPoolApi` doc.
            // SAFETY: no thread received the copy, so dropping `held`'s F is the
            // sole owner drop.
            let _ = ManuallyDrop::into_inner(held);
        }
    }

    fn worker_count(&self) -> USize {
        // SAFETY: `sysconf` is a pure query; `_SC_NPROCESSORS_ONLN` is a stable
        // selector returning the count of online processors (or -1 on error).
        let n = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
        USize(if n < 1 { 1 } else { n as usize }) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sysconf returns c_long; floor at one core; tracked: #121
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
            tv_sec: 0,
            tv_nsec: 0,
        };
        // SAFETY: `ts` is a stack-owned timespec; libc writes
        // through the pointer once and never retains it. Return
        // value is ignored; CLOCK_MONOTONIC is available on every
        // tier-1 unix target.
        let _ = unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
        let raw = (ts.tv_sec as u64)
            .wrapping_mul(1_000_000_000)
            .wrapping_add(ts.tv_nsec as u64); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: CLOCK_MONOTONIC timespec -> ns bit pattern for Nanos; tracked: #72
        Nanos::from_raw(raw)
    }
}
