//! GATE-2 R4c round A: the per-frame wake-word protocol over a real `PoolFrame`.
//!
//! Drives the proven sketch protocol (202606071600 / 202606071700) through the
//! shipped `thread::frame_*` helpers: persistent std-thread workers park on the
//! frame `seq`, wake per frame, write a disjoint output slice, arrive at the
//! `done` barrier; the main thread publishes each frame and parks on `done`;
//! shutdown wakes the workers, which arrive at the `exited` counter the main
//! thread joins on. Fail-first: the helpers + the `seq`/`done`/`exited` fields do
//! not exist before this round, so the file does not compile.
//!
//! `seq` doubles as the frame number (it increments once per publish), so a
//! worker computes frame f's output as `f*1000 + i` with no separate frame-value
//! channel (round B carries the real carrier in the WorkerCtx). A dropped frame,
//! a broken done-barrier, or a missed exit would corrupt the output or hang
//! (the run is bounded by `std::thread::scope`'s join plus the assertions).

use core::marker::PhantomData;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};

use arvo::USize;
use hilavitkutin::thread::{
    await_exit, frame_await, frame_await_done, frame_done_arrive, frame_exit_arrive,
    frame_publish, request_shutdown,
};
use hilavitkutin_api::platform::PoolFrame;

const NREC: usize = 256;

fn make_pool<const C: usize, const P: usize>() -> PoolFrame<'static, C, P> {
    PoolFrame {
        shutdown: AtomicBool::new(false),
        phase_arrived: AtomicU32::new(0),
        barrier_sense: AtomicU32::new(0),
        seq: AtomicU32::new(0),
        done: AtomicU32::new(0),
        exited: AtomicU32::new(0),
        predicted_wait_ns: core::array::from_fn(|_| AtomicU32::new(0)),
        idle_accumulator: core::array::from_fn(|_| AtomicU64::new(0)),
        park_count: core::array::from_fn(|_| AtomicU64::new(0)),
        // The frame protocol never reads progress_slots; a dangling non-null is
        // never dereferenced here.
        progress_slots: NonNull::dangling(),
        progress_slot_count: USize(0),
        _arena: PhantomData,
    }
}

#[derive(Copy, Clone)]
struct SendMut(*mut u64);
// SAFETY: workers write disjoint output ranges; main reads only after the done
// barrier (happens-after via the Release/Acquire on `done`).
unsafe impl Send for SendMut {}
unsafe impl Sync for SendMut {}

fn run(ncores: usize, frames: u32) {
    let pool = make_pool::<8, 2>();
    let mut out = [0u64; NREC];
    let out_ptr = SendMut(out.as_mut_ptr());

    std::thread::scope(|s| {
        for c in 0..ncores {
            let pool = &pool;
            let op = out_ptr;
            s.spawn(move || {
                let op = op; // capture the Send wrapper whole, not the raw field
                let mut last = USize(0);
                loop {
                    last = frame_await(pool, last);
                    if pool.shutdown.load(Ordering::Relaxed) {
                        frame_exit_arrive(pool, USize(ncores));
                        return;
                    }
                    let chunk = NREC / ncores;
                    let start = c * chunk;
                    let end = if c + 1 == ncores { NREC } else { start + chunk };
                    let mut i = start;
                    while i < end {
                        // SAFETY: disjoint range, no other worker writes it; the
                        // buffer outlives the scope.
                        unsafe { *op.0.add(i) = (last.0 as u64) * 1000 + i as u64 };
                        i += 1;
                    }
                    frame_done_arrive(pool, USize(ncores));
                }
            });
        }

        let mut f = 1u32;
        while f <= frames {
            frame_publish(&pool);
            frame_await_done(&pool, USize(ncores));
            let mut i = 0;
            while i < NREC {
                assert_eq!(
                    out[i],
                    (f as u64) * 1000 + i as u64,
                    "ncores={ncores} frame={f} rec={i}: frame protocol must publish \
                     the frame, run every worker, and barrier on done"
                );
                i += 1;
            }
            f += 1;
        }

        request_shutdown(&pool);
        await_exit(&pool, USize(ncores));
    });
}

#[test]
fn frame_protocol_drives_workers_across_frames_and_joins() {
    // ncores == 1 is the degenerate (one worker owns every record); 2 and 3 are
    // real parallelism. All must agree, all frames, and shutdown must join every
    // worker via the exit counter (await_exit returns).
    for ncores in [1usize, 2, 3] {
        run(ncores, 8);
    }
}
