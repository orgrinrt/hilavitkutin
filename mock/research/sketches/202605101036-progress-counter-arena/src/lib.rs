//! Sketch — progress-counter-arena lowering.
//!
//! Hypothesis: storing to an `AtomicUsize` reached via raw-pointer
//! arithmetic on a plan-stage scratch arena (`*const AtomicUsize`
//! base + offset) lowers to a single `stlr` on aarch64 with
//! `Ordering::Release`, identical to a direct stack `AtomicUsize`
//! store. If true, Axis E2 (plan-stage scratch arena) is sound.
//!
//! Counter: if the indirection adds `ldr`/`str` pairs around the
//! `stlr`, S3 (Topic 3 single-stlr invariant) is violated and we
//! must drop to E1 (per-core-fn-stack array).

#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

// E2 shape: arena passed as raw pointer, fiber-id as offset.
// Codegen would emit this pattern at the end of every fiber morsel.
#[inline(never)]
pub fn store_progress_arena(
    arena_base: *const AtomicUsize,
    fiber_id: usize,
    record_count: usize,
) {
    let counter = unsafe { &*arena_base.add(fiber_id) };
    counter.store(record_count, Ordering::Release);
}

// Direct stack reference shape (E1 shape, for comparison).
#[inline(never)]
pub fn store_progress_direct(
    counter: &AtomicUsize,
    record_count: usize,
) {
    counter.store(record_count, Ordering::Release);
}

// Acquire-load side, both shapes.
#[inline(never)]
pub fn load_progress_arena(
    arena_base: *const AtomicUsize,
    fiber_id: usize,
) -> usize {
    let counter = unsafe { &*arena_base.add(fiber_id) };
    counter.load(Ordering::Acquire)
}

#[inline(never)]
pub fn load_progress_direct(counter: &AtomicUsize) -> usize {
    counter.load(Ordering::Acquire)
}
