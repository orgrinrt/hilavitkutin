//! A persistent worker pool on the engine's own frame protocol.
//!
//! The head-and-tail arms cannot run through the scheduler, so they run on
//! this crew instead. It is built from the same parts the scheduler's pool
//! is: a `PoolFrame` of the same type, `frame_publish` and
//! `frame_await_done` on the caller, `frame_await`, `frame_done_arrive` and
//! `frame_exit_arrive` on each worker, parked on the platform wait between
//! frames. Like the scheduler, the caller publishes and waits while the
//! workers do the frame's work. What one frame costs here is therefore what
//! it costs the engine's pool, and an arm on the crew differs from an engine
//! arm in the record walk, not in how its threads are woken.

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::thread::JoinHandle;

use arvo::USize;
use hilavitkutin::thread::{
    MAX_CORES,
    await_exit,
    frame_await,
    frame_await_done,
    frame_done_arrive,
    frame_exit_arrive,
    frame_publish,
    request_shutdown,
};
use hilavitkutin_api::platform::PoolFrame;

/// The frame's work as a type-erased call: a trampoline and the closure it
/// reads. Written by the caller only while every worker is parked.
#[derive(Copy, Clone)]
struct Job {
    run: unsafe fn(*const (), usize),
    ctx: *const (),
}

unsafe fn idle(_ctx: *const (), _core: usize) {}

/// # Safety
///
/// `ctx` points at a live `F` for the whole frame.
unsafe fn trampoline<F: Fn(usize) + Sync>(ctx: *const (), core: usize) {
    // SAFETY: the caller of `Crew::frame` keeps `F` alive until every worker
    // has arrived at the frame's done barrier.
    let f = unsafe { &*ctx.cast::<F>() };
    f(core);
}

struct Shared {
    frame:   PoolFrame<'static, MAX_CORES, 1>,
    job:     UnsafeCell<Job>,
    workers: usize,
}

// SAFETY: the frame words are atomics; `progress_slots` is dangling and never
// read; `job` is written only between frames (every worker parked) and read
// only inside one, ordered by the publish's Release and the await's Acquire.
unsafe impl Send for Shared {}
unsafe impl Sync for Shared {}

fn pool_frame() -> PoolFrame<'static, MAX_CORES, 1> {
    PoolFrame {
        shutdown:            AtomicBool::new(false),
        phase_arrived:       AtomicU32::new(0),
        barrier_sense:       AtomicU32::new(0),
        seq:                 AtomicU32::new(0),
        done:                AtomicU32::new(0),
        exited:              AtomicU32::new(0),
        predicted_wait_ns:   [AtomicU32::new(0)],
        idle_accumulator:    [const { AtomicU64::new(0) }; MAX_CORES],
        park_count:          [const { AtomicU64::new(0) }; MAX_CORES],
        progress_slots:      NonNull::dangling(),
        progress_slot_count: USize(0),
        _arena:              PhantomData,
    }
}

/// A pool of `workers` threads that run one closure per frame, each with its
/// own core index.
pub struct Crew {
    shared:  Arc<Shared>,
    handles: Vec<JoinHandle<()>>,
}

impl Crew {
    /// Spawn `workers` threads and park them. Clamped to `1..=MAX_CORES`,
    /// as the scheduler clamps its executor's count.
    pub fn new(workers: usize) -> Self {
        let workers = workers.clamp(1, MAX_CORES);
        let shared = Arc::new(Shared {
            frame: pool_frame(),
            job: UnsafeCell::new(Job {
                run: idle,
                ctx: std::ptr::null(),
            }),
            workers,
        });
        let handles = (0 .. workers)
            .map(|core| {
                let s = Arc::clone(&shared);
                std::thread::spawn(move || mainloop(&s, core))
            })
            .collect();
        Self {
            shared,
            handles,
        }
    }

    /// How many workers the crew runs.
    pub fn workers(&self) -> usize {
        self.shared.workers
    }

    /// Run one frame: every worker calls `f(core)` once, and this returns
    /// after the last of them has finished.
    pub fn frame<F: Fn(usize) + Sync>(&mut self, f: &F) {
        // SAFETY: `&mut self` means no frame is in flight, so every worker is
        // parked in `frame_await` and none reads `job`. `f` outlives the frame:
        // `frame_await_done` returns only after every worker's done arrival.
        unsafe {
            *self.shared.job.get() = Job {
                run: trampoline::<F>,
                ctx: (f as *const F).cast(),
            };
        }
        frame_publish(&self.shared.frame);
        frame_await_done(&self.shared.frame, USize(self.shared.workers));
    }
}

fn mainloop(s: &Shared, core: usize) {
    let expected = USize(s.workers);
    let mut last = USize(0);
    loop {
        last = frame_await(&s.frame, last);
        if s.frame.shutdown.load(Ordering::Relaxed) {
            frame_exit_arrive(&s.frame, expected);
            return;
        }
        // SAFETY: the job was written before the publish this await observed
        // (Release then Acquire) and is not written again until every worker
        // has arrived at the done barrier below.
        let job = unsafe { *s.job.get() };
        // SAFETY: `Crew::frame` keeps the closure alive through the frame.
        unsafe { (job.run)(job.ctx, core) };
        frame_done_arrive(&s.frame, expected);
    }
}

impl Drop for Crew {
    fn drop(&mut self) {
        request_shutdown(&self.shared.frame);
        await_exit(&self.shared.frame, USize(self.shared.workers));
        for h in self.handles.drain(..) {
            let _ = h.join();
        }
    }
}
