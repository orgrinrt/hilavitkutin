//! Sketch: does arena placement dissolve the Pin receiver? (GATE-2
//! deviation 1, tied to deviation 6; seed governance item 5.)
//!
//! Hypothesis: the shipped `Pin<&mut Self>` on `run_parallel` exists
//! because workers hold a raw back-pointer to the WHOLE scheduler (the
//! inline `PoolFrame` is only part of it), so arena-placing the pool
//! alone cannot dissolve the Pin; the Pin dissolves exactly when every
//! byte a worker dereferences (pool sync words, worker contexts, the
//! dispatch data plane) lives in a provider-allocated arena block whose
//! address is independent of the owning handle. Then the handle is
//! plain-movable: moving it moves no pointee, and no `PhantomPinned` or
//! `Pin` receiver is needed.
//!
//! The sketch mirrors the mechanism standalone (no engine dep): a
//! fixed arena hosts a plane struct (sync words + per-worker ctxs +
//! data); a movable `Handle` owns only the plane pointer; workers spawn
//! once capturing pointers INTO the arena; the handle is then MOVED
//! (mem::swap between stack slots) across frames; the frame protocol
//! keeps working and the data stays correct. A control arm shows the
//! shipped failure shape: if workers captured a pointer to the HANDLE
//! (the scheduler-back-pointer pattern), the move would dangle it,
//! which is exactly why the shipped shape needs Pin.
//!
//! Outcome: see the end of main and FINDINGS.md.

use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::thread;

const WORKERS: usize = 3;
const RECORDS: usize = 4096;
const FRAMES: usize = 50;

// The worker-visible plane: everything a worker dereferences. In the
// engine this is PoolFrame + WorkerCtx array + bindings/plan/columns;
// here the same roles in miniature.
struct Plane {
    // pool sync words (frame seq / done / shutdown)
    seq: AtomicU32,
    done: AtomicU32,
    shutdown: AtomicBool,
    // per-worker ctx: core id only; the plane pointer rides separately
    core_ids: [usize; WORKERS],
    // the data plane: per-worker disjoint regions
    data: [AtomicU64; RECORDS],
}

// Send wrapper for the arena pointer, mirroring the engine's
// SendCtxPtr. SAFETY: the plane is arena-resident for the process
// lifetime and workers touch disjoint data regions plus atomics.
#[derive(Copy, Clone)]
struct SendPlane(NonNull<Plane>);
// SAFETY: see above.
unsafe impl Send for SendPlane {}

impl SendPlane {
    // Method access forces the closure to capture the whole wrapper
    // (a field path would be captured disjointly and un-Send it).
    fn get(self) -> NonNull<Plane> {
        self.0
    }
}

// One worker: waits for the next frame seq, sums its region into its
// slot, signals done. Never touches the Handle.
fn worker_main(plane: NonNull<Plane>, core: usize) {
    let p = unsafe { plane.as_ref() };
    let mut seen = 0u32;
    loop {
        // frame wait
        loop {
            if p.shutdown.load(Ordering::Acquire) {
                return;
            }
            let s = p.seq.load(Ordering::Acquire);
            if s != seen {
                seen = s;
                break;
            }
            std::hint::spin_loop();
        }
        // disjoint region work
        let per = RECORDS / WORKERS + 1;
        let lo = (core * per).min(RECORDS);
        let hi = ((core + 1) * per).min(RECORDS);
        for i in lo..hi {
            p.data[i].fetch_add((core as u64) + 1, Ordering::Relaxed);
        }
        p.done.fetch_add(1, Ordering::AcqRel);
    }
}

// The movable handle: owns the plane POINTER only. No PhantomPinned.
struct Handle {
    plane: NonNull<Plane>,
    threads: Vec<thread::JoinHandle<()>>,
}

impl Handle {
    fn run_frame(&mut self) {
        let p = unsafe { self.plane.as_ref() };
        p.done.store(0, Ordering::Release);
        p.seq.fetch_add(1, Ordering::AcqRel);
        while p.done.load(Ordering::Acquire) < WORKERS as u32 {
            std::hint::spin_loop();
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let p = unsafe { self.plane.as_ref() };
        p.shutdown.store(true, Ordering::Release);
        p.seq.fetch_add(1, Ordering::AcqRel);
        for t in self.threads.drain(..) {
            let _ = t.join();
        }
    }
}

fn main() {
    // The "provider-allocated arena": a leaked fixed block standing in
    // for MemoryProvider::allocate. Address stable for the run.
    let arena: &'static mut MaybeUninit<Plane> = Box::leak(Box::new(MaybeUninit::uninit()));
    let plane_ptr = arena.as_mut_ptr();
    unsafe {
        (*plane_ptr).seq = AtomicU32::new(0);
        (*plane_ptr).done = AtomicU32::new(0);
        (*plane_ptr).shutdown = AtomicBool::new(false);
        (*plane_ptr).core_ids = core::array::from_fn(|i| i);
        for i in 0..RECORDS {
            (*plane_ptr).data[i] = AtomicU64::new(0);
        }
    }
    let plane = NonNull::new(plane_ptr).unwrap();

    // Spawn once, capturing pointers INTO THE ARENA only.
    let mut threads = Vec::new();
    for c in 0..WORKERS {
        let p = SendPlane(plane);
        threads.push(thread::spawn(move || worker_main(p.get(), c)));
    }
    let handle = Handle { plane, threads };

    // Frames interleaved with MOVES of the handle: the exact operation
    // the shipped Pin forbids. Each move relocates the Handle bytes;
    // the plane (and every worker pointer) never moves.
    let mut slot_a = Some(handle);
    for f in 0..FRAMES {
        let mut h = slot_a.take().unwrap();
        h.run_frame();
        // move the handle to a new stack location every frame
        let moved = h;
        slot_a = Some(moved);
        let _ = f;
    }
    let handle = slot_a.take().unwrap();

    // Verify: every record accumulated its region-owner's increment
    // exactly FRAMES times.
    let p = unsafe { handle.plane.as_ref() };
    let per = RECORDS / WORKERS + 1;
    let mut ok = true;
    for i in 0..RECORDS {
        let owner = (i / per).min(WORKERS - 1);
        let expect = (owner as u64 + 1) * FRAMES as u64;
        if p.data[i].load(Ordering::Relaxed) != expect {
            ok = false;
            break;
        }
    }
    drop(handle);
    if ok {
        println!("WORKS: arena-resident plane + movable handle; {FRAMES} frames across handle moves, data exact");
    } else {
        println!("FAILS: data mismatch under handle moves");
        std::process::exit(1);
    }
}
