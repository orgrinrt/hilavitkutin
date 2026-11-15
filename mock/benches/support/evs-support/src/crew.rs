//! A persistent worker pool on the engine's own frame protocol.
//!
//! For an arm that cannot run through the scheduler but has to run threaded:
//! the head-and-tail arms of `single-trunk-convergence`, and the threaded std
//! arms here and in `frame_wide`, which would otherwise spawn their threads
//! inside every timed frame while the engine arm parks its own between frames.
//!
//! It is built from the same parts the scheduler's pool is: a `PoolFrame` of
//! the same type, `frame_publish` and `frame_await_done` on the caller,
//! `frame_await`, `frame_done_arrive` and `frame_exit_arrive` on each worker,
//! parked on the platform wait between frames. Like the scheduler, the caller
//! publishes and waits while the workers do the frame's work. What one frame costs here is therefore what
//! it costs the engine's pool, and an arm on the crew differs from an engine
//! arm in the record walk, not in how its threads are woken.

use std::any::Any;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
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
use hilavitkutin_api::{DefaultRunCfg, RunCfg};

/// The spin budget the engine's frame waits take by default, so the crew waits
/// the way an engine run with no tuning does.
const SPIN_BUDGET: USize = <DefaultRunCfg as RunCfg>::WAKE_SPIN_BUDGET;

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
    /// The first panic a worker caught in the frame, raised again on the
    /// caller once every worker has arrived.
    panic:   Mutex<Option<Box<dyn Any + Send>>>,
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
            panic: Mutex::new(None),
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
    ///
    /// A worker whose `f` panics still arrives at the frame's barrier, so the
    /// frame ends, and the first such panic is raised again here, as a scoped
    /// thread's would be at its join. The crew stays usable after it.
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
        frame_await_done(&self.shared.frame, USize(self.shared.workers), SPIN_BUDGET);
        let caught = self
            .shared
            .panic
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        if let Some(payload) = caught {
            resume_unwind(payload);
        }
    }
}

impl Crew {
    /// Run `f` once per worker over that worker's chunk of `input` and of each
    /// of the `M` slices in `outs`, in one frame.
    ///
    /// Every slice is split alike into `workers()` contiguous chunks of
    /// `len.div_ceil(workers())` items, the last one short, so worker `core`
    /// gets the same range of each, as `chunks_mut` would hand it a scoped
    /// thread. A worker whose range is empty is still called, with empty
    /// slices. The outputs are several slices rather than one so an arm that
    /// writes columns keeps them columns.
    pub fn for_chunks<I, O, const M: usize, F>(&mut self, input: &[I], outs: [&mut [O]; M], f: &F)
    where
        I: Sync,
        O: Send,
        F: Fn(&[I], [&mut [O]; M]) + Sync,
    {
        let n = input.len();
        for out in &outs {
            assert_eq!(
                out.len(),
                n,
                "Crew::for_chunks: an output differs in length from the input"
            );
        }
        let size = n.div_ceil(self.workers()).max(1);
        let bases = outs.map(|out| OutBase(out.as_mut_ptr()));
        self.frame(&|core: usize| {
            let start = (core * size).min(n);
            let end = (start + size).min(n);
            // SAFETY: the ranges of distinct workers are disjoint and inside
            // every output, each borrowed mutably by this call until the
            // frame's done barrier, and `O: Send` lets each worker own its part.
            let parts = core::array::from_fn(|k| unsafe {
                std::slice::from_raw_parts_mut(bases[k].at(start), end - start)
            });
            f(&input[start .. end], parts);
        });
    }
}

/// The base of the slice `Crew::for_chunks` splits, shared with its workers.
struct OutBase<O>(*mut O);

// SAFETY: the workers reach disjoint elements only (see `for_chunks`), which
// is what handing each a `&mut` part of the slice would allow for `O: Send`.
unsafe impl<O: Send> Sync for OutBase<O> {}

impl<O> OutBase<O> {
    /// The element `i` places past the base. A method, so a closure captures
    /// the whole `OutBase` rather than its raw pointer field.
    fn at(&self, i: usize) -> *mut O {
        // SAFETY: callers stay inside the slice (see `for_chunks`).
        unsafe { self.0.add(i) }
    }
}

fn mainloop(s: &Shared, core: usize) {
    let expected = USize(s.workers);
    let mut last = USize(0);
    loop {
        last = frame_await(&s.frame, last, SPIN_BUDGET);
        if s.frame.shutdown.load(Ordering::Relaxed) {
            frame_exit_arrive(&s.frame, expected);
            return;
        }
        // SAFETY: the job was written before the publish this await observed
        // (Release then Acquire) and is not written again until every worker
        // has arrived at the done barrier below.
        let job = unsafe { *s.job.get() };
        // SAFETY: `Crew::frame` keeps the closure alive through the frame.
        // A panic is caught so this worker still arrives below: unwinding past
        // the arrival would leave the caller waiting on the barrier forever.
        // The closure is only borrowed, so nothing it leaves half written is
        // read after the panic is raised on the caller.
        let ran = catch_unwind(AssertUnwindSafe(|| unsafe { (job.run)(job.ctx, core) }));
        if let Err(payload) = ran {
            let mut slot = s
                .panic
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            slot.get_or_insert(payload);
        }
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
