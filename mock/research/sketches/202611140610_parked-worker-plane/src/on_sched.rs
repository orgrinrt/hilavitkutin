//! The two arms whose workers read the scheduler itself: `today` holds a
//! shared reference to it across every park, `narrow` holds a raw pointer and
//! forms the reference per frame.

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::words::{Bindings, SendPtr, Words, range_of, unit_body};
use crate::{Arm, CORES, Frame};

/// The per-worker context, a field of the scheduler as `worker_ctxs` is.
#[derive(Copy, Clone)]
pub struct Ctx<const HOLD_REF: bool> {
    sched: *const Sched<HOLD_REF>,
    core:  usize,
}

/// The scheduler, with everything the workers read in its own fields.
pub struct Sched<const HOLD_REF: bool> {
    words:       Words,
    bindings:    Bindings,
    plan_dirty:  AtomicBool,
    store_dirty: AtomicU32,
    ncores:      usize,
    spawned:     bool,
    ctx:         [Ctx<HOLD_REF>; CORES],
    _pin:        PhantomPinned,
}

fn worker<const HOLD_REF: bool>(ctx: *const Ctx<HOLD_REF>) {
    // SAFETY: the context is a field of the pinned scheduler, alive until its
    // `Drop` has waited every worker out.
    let Ctx {
        sched,
        core,
    } = unsafe { *ctx };
    let mut last = 0;
    if HOLD_REF {
        // As `worker.rs:101`: one shared reference, held across every park.
        // SAFETY: as above.
        let s = unsafe { &*sched };
        loop {
            last = s.words.await_frame(last);
            if s.words.shutdown.load(Ordering::Relaxed) {
                s.words.exited.fetch_add(1, Ordering::Release);
                return;
            }
            let band = s.words.band.load(Ordering::Relaxed);
            unit_body(range_of(core), &s.bindings, band);
            s.words.done.fetch_add(1, Ordering::Release);
        }
    } else {
        // The narrower repair: only the raw pointer crosses the park.
        loop {
            // SAFETY: as above; the field projection forms no reference to
            // the whole scheduler.
            let words = unsafe { &(*sched).words };
            last = words.await_frame(last);
            if words.shutdown.load(Ordering::Relaxed) {
                words.exited.fetch_add(1, Ordering::Release);
                return;
            }
            {
                // SAFETY: as above; formed after the wake, dropped before
                // the done arrival.
                let s = unsafe { &*sched };
                let band = s.words.band.load(Ordering::Relaxed);
                unit_body(range_of(core), &s.bindings, band);
            }
            words.done.fetch_add(1, Ordering::Release);
        }
    }
}

impl<const HOLD_REF: bool> Arm for Sched<HOLD_REF> {
    fn build() -> Self {
        Self {
            words:       Words::default(),
            bindings:    Bindings::new(),
            plan_dirty:  AtomicBool::new(true),
            store_dirty: AtomicU32::new(0),
            ncores:      0,
            spawned:     false,
            ctx:         [Ctx {
                sched: std::ptr::null(),
                core:  0,
            }; CORES],
            _pin:        PhantomPinned,
        }
    }

    fn run_parallel(self: Pin<&mut Self>) -> Frame {
        // As `run_parallel.rs:139`: a raw pointer, every write through it.
        // SAFETY: the scheduler is not moved out of its pin.
        let me: *mut Self = unsafe { self.get_unchecked_mut() };
        // SAFETY: every worker is parked or not yet spawned at each write.
        unsafe {
            if !(*me).spawned {
                (*me).ncores = CORES;
                for c in 0 .. CORES {
                    (*me).ctx[c] = Ctx {
                        sched: me,
                        core:  c,
                    };
                    let p = SendPtr(&raw const (*me).ctx[c]);
                    std::thread::spawn(move || {
                        let p = p;
                        worker(p.0)
                    });
                }
                // As `run_parallel.rs:212`: a plain write after the spawn.
                (*me).spawned = true;
            }
            // As `reset_accumulators`, `run_parallel.rs:223-225`.
            (*me).bindings.reset();
            let band = (*me).plan_dirty.swap(false, Ordering::Relaxed);
            (*me).words.publish_and_await(band);
            Frame {
                out:  (*me).bindings.read_out(),
                band: band as u32,
            }
        }
    }

    fn replace_value(&mut self, v: u32) {
        // SAFETY: the slot is a live provider block and every worker is parked.
        unsafe { *self.bindings.slot = v };
        self.store_dirty.fetch_add(1, Ordering::Relaxed);
    }

    fn replace_resource(&mut self, v: u32) {
        self.replace_value(v);
        self.plan_dirty.store(true, Ordering::Relaxed);
    }
}

impl<const HOLD_REF: bool> Drop for Sched<HOLD_REF> {
    fn drop(&mut self) {
        if self.spawned {
            self.words.shut_down();
        }
    }
}
