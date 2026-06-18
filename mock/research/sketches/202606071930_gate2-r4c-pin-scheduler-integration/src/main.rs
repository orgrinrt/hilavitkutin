//! GATE-2 R4c round B type-composition de-risk: the persistent pool as INLINE
//! fields of a generic, `Pin`ned, self-referential mini-scheduler.
//!
//! Proves the exact shape round B grafts onto the real `Scheduler`, which the
//! earlier sketches did not (they kept the shared state as stack locals):
//!   - `PoolFrame<'static, C, P>` as an inline field (dangling progress_slots;
//!     the frame protocol never reads them),
//!   - a self-referential `[WorkerCtx<C, P>; C]` field, each ctx holding a
//!     `*const MiniSched<C, P>`, populated at first `run_parallel` under `Pin`,
//!   - `run_parallel(self: Pin<&mut Self>, pool, ncores)` that spawns workers
//!     ONCE through the real `OsThreadPool::spawn` (slice 1, pointer-sized-closure
//!     smuggle) and reuses them across frames via the shipped frame helpers
//!     (round A),
//!   - `Drop` shutdown-join via `request_shutdown` + `await_exit` (no thread
//!     join; the contract is fire-and-forget, workers are detached).
//!
//! `Pin` supplies the stable address the workers' raw pointers need: the runtime
//! `Scheduler` keeps no provider post-build and `PoolFrame` is not a
//! `ColumnValue`, so the pool state cannot live in the arena; inline-plus-Pin is
//! the placement. A trivial disjoint-slice write stands in for the real
//! `run_gated` per-core walk (devirt proven by sketch 202606070400). `seq`
//! doubles as the frame number, so no separate frame-value channel is needed.
//!
//! Outcome at bottom.

use core::marker::{PhantomData, PhantomPinned};
use core::pin::Pin;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

use arvo::USize;
use hilavitkutin::thread::{
    await_exit, frame_await, frame_await_done, frame_done_arrive, frame_exit_arrive, frame_publish,
    request_shutdown,
};
use hilavitkutin::OsThreadPool;
use hilavitkutin_api::platform::{PoolFrame, ThreadPoolApi};

const NREC: usize = 4096;
const MAXC: usize = 8;
const MAXP: usize = 2;

fn new_pool<const C: usize, const P: usize>() -> PoolFrame<'static, C, P> {
    PoolFrame {
        shutdown: AtomicBool::new(false),
        phase_arrived: AtomicU32::new(0),
        seq: AtomicU32::new(0),
        done: AtomicU32::new(0),
        exited: AtomicU32::new(0),
        predicted_wait_ns: core::array::from_fn(|_| AtomicU32::new(0)),
        idle_accumulator: core::array::from_fn(|_| AtomicU64::new(0)),
        park_count: core::array::from_fn(|_| AtomicU64::new(0)),
        progress_slots: NonNull::dangling(),
        progress_slot_count: USize(0),
        _arena: PhantomData,
    }
}

/// Per-worker context: a back-pointer to the (pinned) scheduler plus the core
/// id. Pointer-sized capture is one `*const WorkerCtx`, so the spawn smuggle
/// fits. Stored inline in the scheduler at a stable (pinned) address.
struct WorkerCtx<const C: usize, const P: usize> {
    sched: *const MiniSched<C, P>,
    core_id: usize,
}

/// Stand-in for the real `Scheduler`: the pool state lives inline, the value is
/// `!Unpin`, and the worker ctxs point back at it.
struct MiniSched<const C: usize, const P: usize> {
    pool: PoolFrame<'static, C, P>,
    ctxs: [WorkerCtx<C, P>; C],
    out: [u64; NREC],
    ncores: usize,
    spawned: bool,
    _pin: PhantomPinned,
}

impl<const C: usize, const P: usize> MiniSched<C, P> {
    fn new(ncores: usize) -> Self {
        Self {
            pool: new_pool(),
            ctxs: core::array::from_fn(|_| WorkerCtx { sched: core::ptr::null(), core_id: 0 }),
            out: [0u64; NREC],
            ncores,
            spawned: false,
            _pin: PhantomPinned,
        }
    }

    fn run_parallel(self: Pin<&mut Self>, pool: &OsThreadPool) {
        // SAFETY: we never move out of the pinned value; we take a raw pointer so
        // no `&mut` is held live while the spawned workers read through `*const`.
        let me: *mut Self = unsafe { self.get_unchecked_mut() };
        let ncores = unsafe { (*me).ncores };

        if unsafe { !(*me).spawned } {
            let mut c = 0;
            while c < ncores {
                unsafe {
                    (*me).ctxs[c] = WorkerCtx { sched: me as *const Self, core_id: c };
                }
                let cp = SendCtx(unsafe { &(*me).ctxs[c] as *const WorkerCtx<C, P> });
                pool.spawn(move || {
                    let cp = cp; // capture the Send wrapper whole
                    worker_main::<C, P>(cp.0);
                });
                c += 1;
            }
            unsafe {
                (*me).spawned = true;
            }
        }

        let pool_ref = unsafe { &(*me).pool };
        frame_publish(pool_ref);
        frame_await_done(pool_ref, USize(ncores));
    }
}

impl<const C: usize, const P: usize> Drop for MiniSched<C, P> {
    fn drop(&mut self) {
        if self.spawned {
            // Signal shutdown and wait every worker to leave its mainloop before
            // the inline fields (the pool the workers read) are torn down.
            request_shutdown(&self.pool);
            await_exit(&self.pool, USize(self.ncores));
        }
    }
}

#[derive(Copy, Clone)]
struct SendCtx<const C: usize, const P: usize>(*const WorkerCtx<C, P>);
// SAFETY: the pointee is pinned scheduler-owned storage that outlives every
// worker (Drop joins via await_exit before teardown); workers touch disjoint out
// ranges.
unsafe impl<const C: usize, const P: usize> Send for SendCtx<C, P> {}

fn worker_main<const C: usize, const P: usize>(ctx: *const WorkerCtx<C, P>) {
    // SAFETY: ctx is pinned scheduler-owned storage, valid until await_exit.
    let (sched, core_id) = unsafe { ((*ctx).sched, (*ctx).core_id) };
    let pool = unsafe { &(*sched).pool };
    let ncores = unsafe { (*sched).ncores };
    let out_base = unsafe { (*sched).out.as_ptr() as *mut u64 };

    let mut last = USize(0);
    loop {
        last = frame_await(pool, last);
        if pool.shutdown.load(Ordering::Relaxed) {
            frame_exit_arrive(pool, USize(ncores));
            return;
        }
        // Disjoint slice for this core; `last` (the seq) is the frame number.
        let chunk = NREC / ncores;
        let start = core_id * chunk;
        let end = if core_id + 1 == ncores { NREC } else { start + chunk };
        let mut i = start;
        while i < end {
            // SAFETY: disjoint range; the scheduler outlives the frame (main is
            // parked in frame_await_done until every worker arrives).
            unsafe { *out_base.add(i) = (last.0 as u64) * 1000 + i as u64 };
            i += 1;
        }
        frame_done_arrive(pool, USize(ncores));
    }
}

fn expected(frame: u64, i: usize) -> u64 {
    frame.wrapping_mul(1000).wrapping_add(i as u64)
}

fn run(ncores: usize, frames: u64) {
    let pool = OsThreadPool::new();
    let mut sched = core::pin::pin!(MiniSched::<MAXC, MAXP>::new(ncores));
    let mut f = 1u64;
    while f <= frames {
        sched.as_mut().run_parallel(&pool);
        let out = &sched.as_ref().get_ref().out;
        let mut i = 0;
        while i < NREC {
            assert_eq!(
                out[i],
                expected(f, i),
                "ncores={ncores} frame={f} rec={i}: pinned inline pool must publish \
                 + run every worker + barrier on done"
            );
            i += 1;
        }
        f += 1;
    }
    // sched drops here (pinned in place): Drop joins all workers via await_exit.
}

fn main() {
    for ncores in [1usize, 2, 3] {
        for _stress in 0..20 {
            run(ncores, 8);
        }
        println!("ncores={ncores}: 8 frames x 20 reps, pinned inline pool, formula-exact + joined");
    }
    println!(
        "WORKS: PoolFrame + self-referential worker ctxs as INLINE fields of a \
         Pin'd generic mini-scheduler; run_parallel(self: Pin<&mut Self>) spawns \
         workers ONCE via the real OsThreadPool::spawn smuggle, reuses them across \
         frames via the shipped frame helpers, and Drop joins via await_exit. \
         ncores in {{1,2,3}}, formula-exact (N-core == 1-core), deadlock-clean. \
         No alloc, no Box, no dyn."
    );
}

// ---------------------------------------------------------------------
// OUTCOME: WORKS (nightly-2026-05-28, release, fat LTO, cgu=1, macOS arm64).
//
// The full round-B type composition compiles and runs: an inline
// `PoolFrame<'static, C, P>` field, a self-referential `[WorkerCtx<C,P>; C]`
// (each ctx a `*const MiniSched`) populated at first run_parallel under Pin,
// `run_parallel(self: Pin<&mut Self>, pool)` spawning workers ONCE via the real
// OsThreadPool::spawn smuggle and reusing them across frames via the shipped
// frame helpers, and a `Drop` that joins via request_shutdown + await_exit.
// ncores in {1,2,3}, 8 frames x 20 stress reps each (480 frame dispatches),
// formula-exact (N-core == 1-core), deadlock-clean under a 30s timeout x 3 runs.
//
// SETTLES the round-B placement + composition (the firing's blocker):
//   - PoolFrame inline at 'static with dangling progress_slots compiles and the
//     frame protocol never touches the slots, so the lifetime is a non-issue.
//   - Pin gives the stable address: `core::pin::pin!(MiniSched::new(..))` then
//     `s.as_mut().run_parallel(..)`. No Box, no alloc (stack pin). The worker
//     raw pointers into the pinned value stay valid for its whole life.
//   - The &mut-vs-*const aliasing is avoided by taking `let me: *mut Self =
//     self.get_unchecked_mut()` and working through the raw pointer, so no live
//     &mut is held while workers read through *const (matches the sketch-B
//     SyncBind discipline; the disjoint out-ranges make the writes race-free).
//   - Drop runs in place on the pinned value and joins before the inline fields
//     (the pool the workers read) tear down: order is sound.
//
// Maps to the real Scheduler (round B build):
//   - Add inline `pool: PoolFrame<'static, {MAX_CORES}, {MAX_PHASES}>`,
//     `worker_ctxs: [WorkerCtx<Self-ish>; MAX_CORES]`, `spawned: Bool`, and a
//     stored `ncores`/grouping arrays to the Scheduler struct (zero-init at
//     build). MAX_CORES from thread::class (256) or a Cfg const; MAX_PHASES from
//     a Cfg const. WorkerCtx holds `*const Scheduler<...>` + core_id.
//   - `run_parallel(self: Pin<&mut Self>, pool: &P)` replaces the R4b
//     `&mut self` inline sweep: spawn-once, then frame_publish + frame_await_done.
//   - The worker mainloop is the generic fn carrying run_parallel's where-clauses
//     (RunFiber + BundleMasks + ResetAccumulators); per phase it calls
//     core_phase_mask + self.wu_values.run_gated over morsels, phase_barrier_arrive
//     between waist phases (the trivial slice-write here stands in for that walk).
//   - Scheduler::Drop adds the request_shutdown + await_exit join.
//   - gate2_run_parallel test migrates to `core::pin::pin!` + `as_mut()`.
//   - B1 = struct fields + Pin signature + spawn-once + counting-pool test; B2 =
//     worker mainloop (real run_gated walk + phase barrier); B3 = Drop join.
// ---------------------------------------------------------------------
