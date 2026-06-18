//! GATE-2 R4c contract-mapping de-risk: the persistent-pool mechanism (proven in
//! sketch 202606071600) mapped onto the ACTUAL `hilavitkutin_api::ThreadPoolApi`
//! trait, resolving its two contract unknowns. No alloc, no Box, no dyn.
//!
//! Sketch ...1600 used raw `pthread_create` + `pthread_join` directly, so it
//! proved the lifecycle + parking + carrier handoff but NOT that the mechanism
//! fits the contract the engine actually programs against:
//!
//!   `trait ThreadPoolApi { fn spawn<F>(&self, f: F) where F: FnOnce()+Send+'static;
//!                          fn worker_count(&self) -> USize; }`
//!
//! Two unknowns this resolves:
//!
//!   1. NO-ALLOC GENERIC-CLOSURE SPAWN. `spawn<F>` cannot Box F (no alloc). The
//!      engine's worker closure captures exactly one pointer (a `*const
//!      WorkerCtx`, with core_id + shared state behind it, Scheduler-owned), so F
//!      is pointer-sized and is smuggled through the pthread `*mut c_void` arg via
//!      `transmute_copy`, guarded by `const { assert!(size/align fit) }`. A fatter
//!      F is a compile error, not UB. The trampoline is the monomorphic
//!      `tramp::<F>`.
//!
//!   2. JOIN WITH NO JOIN METHOD. The contract is fire-and-forget spawn; there is
//!      no join. Workers are spawned DETACHED; soundness (the carrier outlives
//!      every reader) comes from a worker-exit counter the driver waits on at
//!      shutdown via the shipped `atomic_wait` (the same primitive sketch ...1600
//!      proved), NOT `pthread_join`. The driver blocks until `exited == ncores`
//!      before its state (the carrier) can drop.
//!
//! Plus `worker_count` via `sysconf(_SC_NPROCESSORS_ONLN)`. Real `ThreadPoolApi`
//! impl, real parking, detached threads, multi-frame, ncores in {1,2,3},
//! deadlock-clean (a lost wakeup or a missed exit hangs; run under a timeout).
//!
//! Outcome at bottom.

use core::ffi::c_void;
use core::mem::{align_of, size_of, transmute_copy, ManuallyDrop};
use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, Ordering};

use arvo::USize;
use hilavitkutin::thread::{atomic_wait, atomic_wake_all};
use hilavitkutin_api::platform::ThreadPoolApi;

const MAX_CORES: usize = 8;
const N_REC: usize = 4096;

/// A real `ThreadPoolApi` impl on the os target: detached pthreads, no alloc.
/// Stateless (ZST); the persistent worker contexts are owned by the driver
/// (the Scheduler), not the pool, so `spawn` retains nothing.
struct OsPool;

/// Monomorphic per-F trampoline. `arg` carries the pointer-sized F by value
/// (its bytes were copied into the pointer slot by `spawn`).
extern "C" fn tramp<F: FnOnce()>(arg: *mut c_void) -> *mut c_void {
    // SAFETY: `arg`'s bit pattern is exactly the bytes of F (size_of::<F>() <=
    // size_of::<*mut c_void>(), checked in spawn). Reconstruct F by value and
    // call it once. `transmute_copy` reads size_of::<F>() bytes from &arg.
    let f: F = unsafe { transmute_copy::<*mut c_void, F>(&arg) };
    f();
    ptr::null_mut()
}

impl ThreadPoolApi for OsPool {
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // No-alloc closure handoff: F must fit in the pthread arg pointer. A
        // single-pointer-capturing worker closure (the engine's shape) fits;
        // anything fatter is a compile error here, not heap-allocated.
        const {
            assert!(
                size_of::<F>() <= size_of::<*mut c_void>(),
                "OsPool::spawn: closure must be pointer-sized (no alloc to box a fatter F)"
            );
            assert!(align_of::<F>() <= align_of::<*mut c_void>(), "OsPool::spawn: closure over-aligned");
        }
        // Copy F's bytes into a pointer-sized slot, then forget the original so
        // its destructor does not run here (the trampoline owns it now).
        let held = ManuallyDrop::new(f);
        // SAFETY: F is pointer-sized (checked above); read its bytes into a
        // *mut c_void value. `held` is forgotten via ManuallyDrop.
        let arg: *mut c_void = unsafe { transmute_copy::<F, *mut c_void>(&held) };

        // SAFETY: pthread attr lifecycle is local; detached so no join is needed
        // (the driver's exit-counter barrier provides the ordering instead).
        unsafe {
            let mut attr: libc::pthread_attr_t = core::mem::zeroed();
            libc::pthread_attr_init(&mut attr);
            libc::pthread_attr_setdetachstate(&mut attr, libc::PTHREAD_CREATE_DETACHED);
            let mut tid: libc::pthread_t = core::mem::zeroed();
            let rc = libc::pthread_create(&mut tid, &attr, tramp::<F>, arg);
            libc::pthread_attr_destroy(&mut attr);
            assert_eq!(rc, 0, "pthread_create");
        }
    }

    fn worker_count(&self) -> USize {
        // SAFETY: sysconf is pure; _SC_NPROCESSORS_ONLN is a stable selector.
        let n = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
        USize(if n < 1 { 1 } else { n as usize })
    }
}

/// Pool-lifetime shared state (the Scheduler-owned slot the workers read).
struct Shared {
    seq: AtomicU32,
    done: AtomicU32,
    /// Worker-exit counter: each worker fetch_add(1) on observing shutdown, then
    /// wakes the driver. The driver waits on `exited == ncores` before its state
    /// (the carrier) drops. This is the join-without-a-join-method.
    exited: AtomicU32,
    shutdown: AtomicBool,
    ncores: AtomicU32,
    carrier: AtomicPtr<u64>,
    frame_val: AtomicU32,
}

/// Per-worker context, owned by the driver. The worker closure captures exactly
/// one `*const WorkerCtx` (pointer-sized, so `spawn`'s smuggle fits).
struct WorkerCtx {
    shared: *const Shared,
    core_id: usize,
}

/// Send wrapper so the one-pointer closure satisfies `F: Send`. SAFETY: the
/// pointee (WorkerCtx -> Shared) is driver-owned and outlives every worker (the
/// exit-counter barrier guarantees it); the workers touch disjoint output ranges.
#[derive(Copy, Clone)]
struct SendPtr(*const WorkerCtx);
unsafe impl Send for SendPtr {}

#[inline(never)]
fn dispatch_core(carrier: *mut u64, core_id: usize, ncores: usize, frame_val: u64) {
    let chunk = N_REC / ncores;
    let start = core_id * chunk;
    let end = if core_id + 1 == ncores { N_REC } else { start + chunk };
    let mut i = start;
    while i < end {
        // SAFETY: disjoint range, carrier alive for the frame (driver blocks on
        // done before reuse and on exited before drop).
        unsafe { *carrier.add(i) = frame_val.wrapping_mul(1000).wrapping_add(i as u64) };
        i += 1;
    }
}

fn worker_main(ctx: *const WorkerCtx) {
    // SAFETY: ctx is driver-owned, valid until the driver observes exited.
    let (s, core_id) = unsafe { (&*(*ctx).shared, (*ctx).core_id) };
    let mut last_seen: u32 = 0;
    loop {
        loop {
            let cur = s.seq.load(Ordering::Acquire);
            if cur != last_seen {
                last_seen = cur;
                break;
            }
            atomic_wait(&s.seq, cur);
        }
        if s.shutdown.load(Ordering::Relaxed) {
            // Exit barrier: record departure and wake the driver.
            s.exited.fetch_add(1, Ordering::Release);
            atomic_wake_all(&s.exited);
            return;
        }
        let ncores = s.ncores.load(Ordering::Relaxed) as usize;
        if core_id < ncores {
            let carrier = s.carrier.load(Ordering::Relaxed);
            let frame_val = s.frame_val.load(Ordering::Relaxed) as u64;
            dispatch_core(carrier, core_id, ncores, frame_val);
        }
        let prev = s.done.fetch_add(1, Ordering::Release);
        if prev + 1 == s.ncores.load(Ordering::Relaxed) {
            atomic_wake_all(&s.done);
        }
    }
}

fn run_frame(s: &Shared, spawned: u32, carrier: *mut u64, frame_val: u32) {
    s.done.store(0, Ordering::Relaxed);
    s.carrier.store(carrier, Ordering::Relaxed);
    s.frame_val.store(frame_val, Ordering::Relaxed);
    s.seq.fetch_add(1, Ordering::Release);
    atomic_wake_all(&s.seq);
    loop {
        let d = s.done.load(Ordering::Acquire);
        if d == spawned {
            break;
        }
        atomic_wait(&s.done, d);
    }
}

fn expected(frame_val: u32, i: usize) -> u64 {
    (frame_val as u64).wrapping_mul(1000).wrapping_add(i as u64)
}

fn run_pool(ncores: usize, frames: usize) {
    let shared = Shared {
        seq: AtomicU32::new(0),
        done: AtomicU32::new(0),
        exited: AtomicU32::new(0),
        shutdown: AtomicBool::new(false),
        ncores: AtomicU32::new(ncores as u32),
        carrier: AtomicPtr::new(ptr::null_mut()),
        frame_val: AtomicU32::new(0),
    };
    let mut ctxs = [const {
        WorkerCtx { shared: ptr::null(), core_id: 0 }
    }; MAX_CORES];

    let pool = OsPool;
    assert!(pool.worker_count().0 >= 1, "sysconf worker_count must be >= 1");

    // Spawn ONCE through the real ThreadPoolApi contract. Each worker closure
    // captures exactly one pointer (SendPtr), so spawn's no-alloc smuggle fits.
    let mut c = 0;
    while c < ncores {
        ctxs[c] = WorkerCtx { shared: &shared as *const Shared, core_id: c };
        let p = SendPtr(&ctxs[c] as *const WorkerCtx);
        // Bind `p` whole inside so the closure captures the Send wrapper (edition
        // 2021 disjoint capture would otherwise grab the non-Send `p.0` field).
        pool.spawn(move || {
            let p = p;
            worker_main(p.0)
        });
        c += 1;
    }

    let mut out = [0u64; N_REC];
    let mut f = 1u32;
    while f <= frames as u32 {
        run_frame(&shared, ncores as u32, out.as_mut_ptr(), f);
        let mut i = 0;
        while i < N_REC {
            assert_eq!(
                out[i],
                expected(f, i),
                "ncores={ncores} frame={f} rec={i}: ThreadPoolApi-spawned pool output \
                 must equal the formula (== single-core)"
            );
            i += 1;
        }
        f += 1;
    }

    // Shutdown via the exit-counter barrier (no pthread_join): signal, wake, then
    // wait until every detached worker has left its mainloop. Only after this is
    // `shared` (the carrier owner) safe to drop.
    shared.shutdown.store(true, Ordering::Relaxed);
    shared.seq.fetch_add(1, Ordering::Release);
    atomic_wake_all(&shared.seq);
    loop {
        let e = shared.exited.load(Ordering::Acquire);
        if e == ncores as u32 {
            break;
        }
        atomic_wait(&shared.exited, e);
    }
}

fn main() {
    for ncores in [1usize, 2, 3] {
        for _stress in 0..50 {
            run_pool(ncores, 8);
        }
        println!("ncores={ncores}: 8 frames x 50 reps via ThreadPoolApi, exit-joined, formula-exact");
    }
    println!(
        "WORKS: persistent pool driven through the real hilavitkutin_api::ThreadPoolApi \
         (no-alloc pointer-sized-closure smuggle in spawn<F>, detached pthreads, \
         worker_count via sysconf), shut down via a worker-exit-counter barrier \
         (no pthread_join). ncores in {{1,2,3}}, 8 frames x 50 reps, formula-exact \
         (N-core == 1-core), deadlock-clean. No alloc, no Box, no dyn."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release, fat LTO, cgu=1, macOS arm64).
//
// The persistent-pool mechanism (sketch 202606071600) maps cleanly onto the real
// `hilavitkutin_api::ThreadPoolApi`. ncores in {1,2,3}, 8 frames x 50 reps,
// formula-exact (N-core == 1-core), deadlock-clean under a 30s timeout x 3 runs.
//
// SETTLES both R4c contract unknowns:
//
//   1. NO-ALLOC GENERIC-CLOSURE SPAWN. `OsPool::spawn<F: FnOnce()+Send+'static>`
//      copies F's bytes into the pthread `*mut c_void` arg via `transmute_copy`
//      (F forgotten via ManuallyDrop), guarded by `const { assert!(size_of::<F>()
//      <= size_of::<*mut c_void>() && align fits) }`, with a monomorphic
//      `tramp::<F>` reconstructing and calling it. The engine's worker closure
//      captures exactly one pointer (a Send-wrapped `*const WorkerCtx`), so it is
//      pointer-sized and fits. A fatter closure is a COMPILE error, not heap use.
//      Note: edition-2021 disjoint capture grabs the non-Send field unless the
//      Send wrapper is rebound whole inside the closure (`let p = p;`).
//
//   2. JOIN WITH NO JOIN METHOD. Workers are spawned DETACHED (pthread attr
//      PTHREAD_CREATE_DETACHED), so the contract stays fire-and-forget. Shutdown
//      ordering comes from a worker-exit counter: each worker fetch_add(1, Release)
//      on observing shutdown and wakes the driver, which waits `exited == ncores`
//      via the shipped `atomic_wait` before its state (the carrier owner) drops.
//      No `pthread_join`, no tid retention, no contract extension.
//
//   Plus `worker_count` via `sysconf(_SC_NPROCESSORS_ONLN)` (>= 1).
//
// Engine R4c is now fully de-risked (this + sketch ...1600). The build:
//   - OsThreadPool::spawn<F> (os.rs:146 stub) -> the smuggle above; worker_count
//     -> sysconf. The std tier stays a stub (op: os + no_os only).
//   - Scheduler owns the WorkerCtx array + the Shared/PoolFrame slot, spawns C
//     workers once via the ThreadPoolApi provider at build (each closure captures
//     one Send-wrapped *const ctx), parks them; run_parallel publishes carrier +
//     per-core masks, bumps seq, wakes, waits done; Scheduler::Drop sets shutdown,
//     wakes, waits the exit counter (the no-join shutdown), THEN frees.
//   - The carrier in the engine is the Scheduler's own bindings/wu_values (they
//     persist across frames, so the per-frame handoff is even milder than here:
//     only seq/done/masks move; the columns are read fresh from the bindings).
//   - HybridExecutor::run (platform.rs:332 stub) hosts the per-core parked walk
//     (core_phase_mask + run_gated, phase_barrier_arrive between waist phases).
// ---------------------------------------------------------------------
