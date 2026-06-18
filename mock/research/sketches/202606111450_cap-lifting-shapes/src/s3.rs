//! s3: macro-generated per-cap instantiations at blessed sizes.
//!
//! No generics, no GCE, no const traits: a macro stamps a concrete
//! scheduler-shaped struct per blessed cap bundle. The consumer picks a
//! name, not a number; arbitrary values need a new macro invocation
//! (which any consumer crate can write itself, so the set is open, but
//! each instantiation is a full copy of the struct + impl).

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

struct WorkerCtx {
    sched: *const (),
    core_id: usize,
}

macro_rules! sched_at {
    ($name:ident, dirty = $dirty:literal, cores = $cores:literal, accums = $accums:literal) => {
        pub struct $name {
            plan_dirty: [AtomicBool; $dirty],
            worker_ctxs: [WorkerCtx; $cores],
            accum_live: [AtomicUsize; $cores * $accums],
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    plan_dirty: [const { AtomicBool::new(false) }; $dirty],
                    worker_ctxs: [const {
                        WorkerCtx {
                            sched: core::ptr::null(),
                            core_id: 0,
                        }
                    }; $cores],
                    accum_live: [const { AtomicUsize::new(0) }; $cores * $accums],
                }
            }

            pub fn mark_dirty(&self, i: usize) {
                self.plan_dirty[i].store(true, Ordering::Relaxed);
            }

            pub fn dirty_count(&self) -> usize {
                self.plan_dirty
                    .iter()
                    .filter(|b| b.load(Ordering::Relaxed))
                    .count()
            }

            pub fn widths(&self) -> (usize, usize, usize) {
                (
                    self.plan_dirty.len(),
                    self.worker_ctxs.len(),
                    self.accum_live.len(),
                )
            }
        }
    };
}

sched_at!(SchedDefault, dirty = 256, cores = 256, accums = 16);
sched_at!(SchedTiny, dirty = 8, cores = 4, accums = 2);
sched_at!(SchedWide, dirty = 1024, cores = 512, accums = 32);

pub fn run() {
    let d = SchedDefault::new();
    let t = SchedTiny::new();
    let w = SchedWide::new();
    d.mark_dirty(200);
    t.mark_dirty(3);
    w.mark_dirty(1000);
    println!(
        "s3: default {:?} dirty={} | tiny {:?} dirty={} | wide {:?} dirty={}",
        d.widths(),
        d.dirty_count(),
        t.widths(),
        t.dirty_count(),
        w.widths(),
        w.dirty_count()
    );
    println!("s3: WORKS");
}
