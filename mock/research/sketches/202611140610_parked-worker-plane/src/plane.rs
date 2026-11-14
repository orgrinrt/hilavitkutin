//! The relocation: the frame words, the per-worker context and what a worker
//! reads live in a plane the provider allocates. The worker holds a shared
//! reference to the plane across the park. The scheduler keeps a pointer to
//! the plane and, beside it, only what the calling thread alone touches; the
//! plane is written after the spawn through its atomics alone.

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::atomic::Ordering;

use crate::words::{Bindings, SendPtr, Words, provider_block, range_of, unit_body};
use crate::{Arm, CORES, Frame};

/// The per-worker context, in the plane.
#[derive(Copy, Clone)]
struct Ctx {
    plane: *const Plane,
    core:  usize,
}

/// What the workers read, in one provider block.
struct Plane {
    words:    Words,
    bindings: Bindings,
    ctx:      [Ctx; CORES],
}

/// The scheduler: the plane's pointer and the calling thread's own state.
pub struct Sched {
    plane:      *mut Plane,
    bindings:   Bindings,
    plan_dirty: bool,
    dirty:      u32,
    spawned:    bool,
    _pin:       PhantomPinned,
}

fn worker(ctx: *const Ctx) {
    // SAFETY: the context is in the plane, a provider block alive until the
    // scheduler's `Drop` has waited every worker out.
    let Ctx {
        plane,
        core,
    } = unsafe { *ctx };
    // SAFETY: as above; nothing forms an exclusive reference into the plane.
    let p = unsafe { &*plane };
    let mut last = 0;
    loop {
        last = p.words.await_frame(last);
        if p.words.shutdown.load(Ordering::Relaxed) {
            p.words.exited.fetch_add(1, Ordering::Release);
            return;
        }
        let band = p.words.band.load(Ordering::Relaxed);
        unit_body(range_of(core), &p.bindings, band);
        p.words.done.fetch_add(1, Ordering::Release);
    }
}

impl Arm for Sched {
    fn build() -> Self {
        let bindings = Bindings::new();
        let plane = provider_block(Plane {
            words: Words::default(),
            bindings,
            ctx: [Ctx {
                plane: std::ptr::null(),
                core:  0,
            }; CORES],
        });
        Self {
            plane,
            bindings,
            plan_dirty: true,
            dirty: 0,
            spawned: false,
            _pin: PhantomPinned,
        }
    }

    fn run_parallel(self: Pin<&mut Self>) -> Frame {
        // SAFETY: the scheduler is not moved out of its pin.
        let me: &mut Self = unsafe { self.get_unchecked_mut() };
        let plane = me.plane;
        if !me.spawned {
            // The plane is written whole before the first worker starts: a
            // worker's reference covers all of it, so a context written after
            // one spawn would race that worker's retag.
            for c in 0 .. CORES {
                // SAFETY: no worker runs yet; the plane is a live provider block.
                unsafe {
                    (*plane).ctx[c] = Ctx {
                        plane,
                        core: c,
                    }
                };
            }
            for c in 0 .. CORES {
                // SAFETY: the plane is a live provider block.
                let p = SendPtr(unsafe { &raw const (*plane).ctx[c] });
                std::thread::spawn(move || {
                    let p = p;
                    worker(p.0)
                });
            }
            me.spawned = true;
        }
        me.bindings.reset();
        let band = core::mem::replace(&mut me.plan_dirty, false);
        // SAFETY: the plane is a live provider block; `words` is atomics.
        let words = unsafe { &(*plane).words };
        words.publish_and_await(band);
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
            // SAFETY: the plane is a live provider block.
            unsafe { (*self.plane).words.shut_down() };
        }
    }
}
