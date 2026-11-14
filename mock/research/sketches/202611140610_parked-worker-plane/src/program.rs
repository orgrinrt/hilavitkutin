//! The canon shape: each core runs its own program, allocated by the provider
//! at the first frame and immutable after it, holding its record range, its
//! bindings and a pointer to the shared sync words. The worker holds a shared
//! reference to its program across the park and reads nothing else of the
//! scheduler's.

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::atomic::Ordering;

use crate::words::{Bindings, SendPtr, Words, provider_block, range_of, unit_body};
use crate::{Arm, CORES, Frame};

/// One core's program.
struct Program {
    range:    (usize, usize),
    bindings: Bindings,
    sync:     *const Words,
}

/// The scheduler: the sync words, the programs and its own state.
pub struct Sched {
    sync:       *mut Words,
    programs:   [*mut Program; CORES],
    bindings:   Bindings,
    plan_dirty: bool,
    dirty:      u32,
    spawned:    bool,
    _pin:       PhantomPinned,
}

fn worker(program: *const Program) {
    // SAFETY: the program is a provider block, written once before the spawn
    // and alive until the scheduler's `Drop` has waited every worker out.
    let p = unsafe { &*program };
    // SAFETY: the sync words are a provider block alive as long.
    let words = unsafe { &*p.sync };
    let mut last = 0;
    loop {
        last = words.await_frame(last);
        if words.shutdown.load(Ordering::Relaxed) {
            words.exited.fetch_add(1, Ordering::Release);
            return;
        }
        let band = words.band.load(Ordering::Relaxed);
        unit_body(p.range, &p.bindings, band);
        words.done.fetch_add(1, Ordering::Release);
    }
}

impl Arm for Sched {
    fn build() -> Self {
        Self {
            sync:       provider_block(Words::default()),
            programs:   [std::ptr::null_mut(); CORES],
            bindings:   Bindings::new(),
            plan_dirty: true,
            dirty:      0,
            spawned:    false,
            _pin:       PhantomPinned,
        }
    }

    fn run_parallel(self: Pin<&mut Self>) -> Frame {
        // SAFETY: the scheduler is not moved out of its pin.
        let me: &mut Self = unsafe { self.get_unchecked_mut() };
        if !me.spawned {
            for c in 0 .. CORES {
                let program = provider_block(Program {
                    range:    range_of(c),
                    bindings: me.bindings,
                    sync:     me.sync,
                });
                me.programs[c] = program;
                let p = SendPtr(program.cast_const());
                std::thread::spawn(move || {
                    let p = p;
                    worker(p.0)
                });
            }
            me.spawned = true;
        }
        me.bindings.reset();
        let band = core::mem::replace(&mut me.plan_dirty, false);
        // SAFETY: the sync words are a live provider block of atomics.
        unsafe { (*me.sync).publish_and_await(band) };
        Frame {
            out:  me.bindings.read_out(),
            band: band as u32,
        }
    }

    fn replace_value(&mut self, v: u32) {
        // SAFETY: the slot is a live provider block and every worker is parked.
        unsafe { *self.bindings.slot = v };
        self.dirty += 1;
    }

    fn replace_resource(&mut self, v: u32) {
        self.replace_value(v);
        self.plan_dirty = true;
    }
}

impl Drop for Sched {
    fn drop(&mut self) {
        if self.spawned {
            // SAFETY: the sync words are a live provider block.
            unsafe { (*self.sync).shut_down() };
        }
    }
}
