//! GATE-2 R4c de-risk (THE threaded-executor keystone): a persistent pthread
//! pool spawned ONCE, PARKED off-CPU between frames via the real shipped
//! atomic-wait primitive, fed each frame's carrier through a shared slot, reused
//! across many frames. No alloc, no Box, no dyn.
//!
//! Why this and not Sketch B (202606070400): Sketch B proved column-disjoint
//! trunks run concurrently with zero sync, joined by the shipped waist barrier,
//! output == single-core. But it used `std::thread::scope` (spawn per frame,
//! auto-join) and ran ONE frame. The real pool's hard part is the LIFECYCLE the
//! `ThreadPoolApi` contract forces:
//!
//!   `spawn<F>(f) where F: FnOnce() + Send + 'static`
//!
//! The worker is `'static`, so a per-frame (non-`'static`) carrier CANNOT be
//! captured into it. The carrier crosses via a Scheduler-owned shared slot the
//! worker reads each frame through a raw pointer (raw pointers are `'static`;
//! join-at-shutdown keeps the deref sound: the frame call blocks until all
//! workers finish, so the carrier outlives every reader). The worker stays
//! monomorphic in the carrier type (the Scheduler is generic over it), so the
//! inner dispatch keeps full devirtualization with no per-phase indirect call.
//!
//! The OTHER hard part, and the reason this sketch exists separately from a spin
//! version: PARKING. A spin loop trivially avoids the park/unpark handoff, which
//! is exactly where lost-wakeup races live, so a spin-based sketch does not
//! de-risk the real mechanism. The spec covers parking (PoolFrame WakeStrategy
//! futex / park tiers); this sketch uses the SHIPPED
//! `hilavitkutin::thread::atomic_wait` / `atomic_wake_all` (the real
//! futex / __ulock / WaitOnAddress primitive) in BOTH directions:
//!   - main -> workers: publish the frame, bump the seq word, wake all parked
//!     workers.
//!   - workers -> main: each worker increments the done word; the last one wakes
//!     the main thread, which is parked on it.
//! Both use the lost-wakeup-safe pattern: load the word, check the condition,
//! and only `atomic_wait(word, observed)` (the futex sleeps iff the word still
//! equals `observed`, so a bump that races between the load and the wait returns
//! immediately instead of sleeping through it). A lost wakeup would HANG, so the
//! multi-frame stress loop is a deadlock probe (run under a timeout).
//!
//! The workload is a trivial disjoint-slice write; the real `RunFiber` walk on a
//! worker thread is already proven by Sketch B. What is NEW here is the pool
//! lifecycle + real parking handoff + per-frame carrier handoff + multi-frame
//! reuse soundness.
//!
//! Outcome at bottom.

use core::ffi::c_void;
use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, Ordering};

use hilavitkutin::thread::{atomic_wait, atomic_wake_all};

const MAX_CORES: usize = 8;
const N_REC: usize = 4096;

/// Pool-lifetime shared state. Lives in `main`'s frame for the whole run, so
/// every worker's raw `*const Shared` stays valid until join. Stands in for the
/// Scheduler-owned `PoolFrame` + per-core program slots.
struct Shared {
    /// Frame sequence / worker wake word. Main bumps (Release) + wakes to
    /// publish a frame; workers park on it via `atomic_wait`.
    seq: AtomicU32,
    /// Done counter / main wake word. Each worker fetch_add(1, Release) on
    /// finishing a frame; the last one wakes the main thread parked on it.
    done: AtomicU32,
    /// Shutdown signal. Set before the final seq bump; workers break on wake.
    shutdown: AtomicBool,
    /// Active core count for the current frame (every spawned worker still ticks
    /// `done`, so one Shared serves any ncores in 1..=MAX_CORES).
    ncores: AtomicU32,
    /// The per-frame carrier, published as a raw pointer (the lifetime-erasure
    /// crux). Here the carrier is the output buffer base; in the engine it is the
    /// bindings / wu_values pointer pair.
    carrier: AtomicPtr<u64>,
    /// Per-frame value the kernel folds in (stands in for frame-varying inputs).
    frame_val: AtomicU32,
}

/// pthread arg: a `*const Shared` plus the worker's core id. Stack-owned by
/// `main` in a fixed array (no heap), passed by raw pointer to pthread_create.
#[repr(C)]
#[derive(Copy, Clone)]
struct WorkerArg {
    shared: *const Shared,
    core_id: usize,
}

/// The monomorphic per-core kernel. In the engine this is
/// `run_gated::<Carrier, Bindings, Mask>` over a morsel; here it writes this
/// core's disjoint output range. Reads the carrier pointer fresh each frame.
#[inline(never)]
fn dispatch_core(carrier: *mut u64, core_id: usize, ncores: usize, frame_val: u64) {
    let chunk = N_REC / ncores;
    let start = core_id * chunk;
    let end = if core_id + 1 == ncores { N_REC } else { start + chunk };
    let mut i = start;
    while i < end {
        // SAFETY: `carrier` points to a [u64; N_REC] alive for the frame (main
        // blocks until all workers finish); `i` is in this core's disjoint
        // range, so no other worker writes this cell.
        unsafe { *carrier.add(i) = frame_val.wrapping_mul(1000).wrapping_add(i as u64) };
        i += 1;
    }
}

/// Persistent worker mainloop. `'static` over the raw `*const Shared`; loops
/// until shutdown, PARKED off-CPU on the seq word between frames.
fn worker_main(shared: *const Shared, core_id: usize) {
    // SAFETY: `shared` is valid until join (main outlives all workers).
    let s = unsafe { &*shared };
    let mut last_seen: u32 = 0;
    loop {
        // Park until a new frame is published. Lost-wakeup-safe: read seq, and
        // only sleep on `atomic_wait(seq, cur)`, which the futex performs iff
        // *seq still == cur. A bump that races in after the load makes the wait
        // return at once instead of sleeping through the wakeup.
        loop {
            let cur = s.seq.load(Ordering::Acquire);
            if cur != last_seen {
                last_seen = cur;
                break;
            }
            atomic_wait(&s.seq, cur);
        }
        if s.shutdown.load(Ordering::Relaxed) {
            return;
        }
        let ncores = s.ncores.load(Ordering::Relaxed) as usize;
        if core_id < ncores {
            let carrier = s.carrier.load(Ordering::Relaxed);
            let frame_val = s.frame_val.load(Ordering::Relaxed) as u64;
            dispatch_core(carrier, core_id, ncores, frame_val);
        }
        // Arrive at the done barrier. The last arriver wakes the parked main
        // thread (Release pairs with main's Acquire load, publishing the output
        // writes). `spawned` arrivers expected; see run_frame.
        let prev = s.done.fetch_add(1, Ordering::Release);
        if prev + 1 == s.ncores_spawned() {
            atomic_wake_all(&s.done);
        }
    }
}

impl Shared {
    /// Number of workers that will tick `done` this frame. Equals the active
    /// core count: every spawned worker for this pool is active (the pool is
    /// sized to ncores; ncores < spawned never happens within one run_pool).
    #[inline]
    fn ncores_spawned(&self) -> u32 {
        self.ncores.load(Ordering::Relaxed)
    }
}

extern "C" fn trampoline(arg: *mut c_void) -> *mut c_void {
    // SAFETY: `arg` is a `*const WorkerArg` from a stack-owned array in `main`,
    // valid until join.
    let a = unsafe { &*(arg as *const WorkerArg) };
    worker_main(a.shared, a.core_id);
    ptr::null_mut()
}

/// Run one frame on the persistent pool: publish carrier + frame value, wake the
/// parked workers, then PARK the main thread on the done word until every worker
/// has arrived. `spawned` == ncores here (the pool is sized to ncores).
fn run_frame(s: &Shared, spawned: u32, carrier: *mut u64, frame_val: u32) {
    s.done.store(0, Ordering::Relaxed);
    s.carrier.store(carrier, Ordering::Relaxed);
    s.frame_val.store(frame_val, Ordering::Relaxed);
    // Release: publish the slot writes to the waking workers, then wake them.
    s.seq.fetch_add(1, Ordering::Release);
    atomic_wake_all(&s.seq);
    // Park on done until all `spawned` workers arrive. Lost-wakeup-safe: read
    // done, and only sleep on `atomic_wait(done, d)` (futex sleeps iff *done
    // still == d). The final worker's wake lands either as a real wakeup or as
    // an immediate return when the value already diverged.
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
        shutdown: AtomicBool::new(false),
        ncores: AtomicU32::new(ncores as u32),
        carrier: AtomicPtr::new(ptr::null_mut()),
        frame_val: AtomicU32::new(0),
    };

    // Stack-owned pthread args (no heap). One per spawned worker.
    let mut args = [WorkerArg { shared: &shared as *const Shared, core_id: 0 }; MAX_CORES];
    let mut tids: [libc::pthread_t; MAX_CORES] = unsafe { core::mem::zeroed() };

    // Spawn ONCE. Every worker parks immediately on seq == 0.
    let mut c = 0;
    while c < ncores {
        args[c].core_id = c;
        let rc = unsafe {
            libc::pthread_create(
                &mut tids[c],
                ptr::null(),
                trampoline,
                &args[c] as *const WorkerArg as *mut c_void,
            )
        };
        assert_eq!(rc, 0, "pthread_create core {c}");
        c += 1;
    }

    // The carrier: a single output buffer reused across frames. Each frame
    // publishes its base pointer (the lifetime-erased handoff) and a new value.
    // Every cell is overwritten and verified against the frame-specific formula,
    // so a stale cell from a prior frame (value for frame f-1) is caught.
    let mut out = [0u64; N_REC];

    let mut f = 1u32;
    while f <= frames as u32 {
        run_frame(&shared, ncores as u32, out.as_mut_ptr(), f);
        let mut i = 0;
        while i < N_REC {
            assert_eq!(
                out[i],
                expected(f, i),
                "ncores={ncores} frame={f} rec={i}: persistent-pool output must \
                 equal the formula (== single-core), proving handoff + parking + \
                 barrier"
            );
            i += 1;
        }
        f += 1;
    }

    // Shutdown: set the flag, bump seq once and wake every parked worker, join.
    shared.shutdown.store(true, Ordering::Relaxed);
    shared.seq.fetch_add(1, Ordering::Release);
    atomic_wake_all(&shared.seq);
    let mut c = 0;
    while c < ncores {
        let rc = unsafe { libc::pthread_join(tids[c], ptr::null_mut()) };
        assert_eq!(rc, 0, "pthread_join core {c}");
        c += 1;
    }
}

fn main() {
    // ncores == 1 is the degenerate of the same path (one persistent worker owns
    // every record); 2 and 3 are real parallelism. All must agree, all frames.
    // A lost wakeup in either park direction would hang here (run under a
    // timeout); wrong output would fail the per-cell assert.
    for ncores in [1usize, 2, 3] {
        for _stress in 0..50 {
            run_pool(ncores, 8);
        }
        println!("ncores={ncores}: 8 frames x 50 stress reps, parked, all formula-exact");
    }
    println!(
        "WORKS: persistent pthread pool spawned ONCE per ncores, PARKED off-CPU \
         between frames via the shipped atomic_wait/atomic_wake_all (real \
         futex/__ulock), fed each frame's carrier through a shared slot \
         (raw-pointer handoff), reused across 8 frames, joined at shutdown. \
         ncores in {{1,2,3}} all produce formula-exact output (N-core == 1-core), \
         race-and-deadlock-clean over 50 stress reps. No alloc, no Box, no dyn."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release, fat LTO, cgu=1, macOS arm64).
//
// A persistent pthread pool spawned ONCE per ncores, PARKED off-CPU between
// frames via the SHIPPED `hilavitkutin::thread::atomic_wait` / `atomic_wake_all`
// (the real __ulock primitive on this host; futex on Linux, WaitOnAddress on
// Windows), fed each frame's carrier through a shared slot via a raw-pointer
// handoff, reused across 8 frames, joined cleanly at shutdown. ncores in
// {1, 2, 3} all produce formula-exact output: ncores==1 is the degenerate of
// the same path (one persistent worker owns every record), 2 and 3 are real
// parallelism, all bit-identical. Deadlock-clean: 3 outer runs x 3 core counts
// x 50 stress reps x 8 frames = 3600 parked frame dispatches under a 30s
// timeout, every run returned 0 (a lost wakeup in either park direction would
// have hung and been killed; wrong output would fail the per-cell assert).
//
// SETTLES (GATE-2 R4c, the threaded-executor keystone), BOTH hard parts:
//
//   1. Lifecycle / lifetime-erasure. spawn<F: FnOnce()+Send+'static> means the
//      worker is 'static and cannot capture the per-frame carrier; publishing
//      the carrier as a raw pointer into a Scheduler-owned shared slot, read
//      fresh each frame by a monomorphic worker, with the frame call blocking on
//      the done barrier until every worker finishes, keeps the deref sound (the
//      carrier outlives every reader). The worker kernel (`dispatch_core`) stays
//      monomorphic, so the engine's inner RunFiber walk (proven on a worker
//      thread by Sketch B 202606070400) keeps full devirtualization.
//
//   2. Real PARKING (the part a spin loop would have faked). Both directions use
//      the shipped atomic-wait primitive with the lost-wakeup-safe pattern: load
//      the word, check the condition, only `atomic_wait(word, observed)` (the
//      futex/__ulock sleeps iff the word still equals `observed`, so a bump
//      racing in between the load and the wait returns immediately rather than
//      sleeping through the wakeup). main -> workers wakes on a seq word;
//      workers -> main wakes on a done word (last arriver wakes). This exercises
//      the park/unpark handoff a spin sketch sidesteps entirely.
//
// Maps to the engine R4c build:
//   - Shared            -> the shipped PoolFrame (phase_arrived barrier +
//                          WakeStrategy already exist; this adds the per-frame
//                          carrier slot + the seq/done wake words, all AtomicU32
//                          so atomic_wait applies directly).
//   - run_frame         -> Scheduler::run_parallel publishes carrier + per-core
//                          masks, bumps seq, wakes, parks on done (replaces
//                          R4b's inline sequential core sweep).
//   - dispatch_core     -> the monomorphic per-(core,phase) run_gated walk from
//                          R4b (core_phase_mask + run_gated), one phase per waist
//                          section with phase_barrier_arrive between phases.
//   - worker_main       -> HybridExecutor::run mainloop (platform.rs:332 stub),
//                          captured into the OsThreadPool-spawned 'static worker,
//                          parking on the seq word via atomic_wait between frames.
//   - spawn-once + join -> OsThreadPool::spawn<F> real impl (os.rs:146 stub):
//                          pthread_create per worker with a Scheduler-owned arg
//                          struct, pthread_join at Scheduler::Drop. sysconf for
//                          worker_count. The std tier stays a stub (op: os +
//                          no_os only). Parking already routes through the shipped
//                          WakeStrategy futex/park tiers (pick_tier), not spin.
// ---------------------------------------------------------------------
