//! Progress counter: per-fiber monotonic record index (domain 17).
//!
//! Release store / Acquire load. Lock-free by construction. Ships
//! `#[repr(transparent)]` over `AtomicUsize` so downstream can lay
//! out a parallel array of counters without padding.
//!
//! Two access paths:
//!
//! 1. **`ProgressCounter`** the typed wrapper over a stack-resident
//!    or column-resident `AtomicUsize`. Direct Release/Acquire.
//!
//! 2. **Arena indirection** (`store_progress_arena` /
//!    `load_progress_arena`). The plan-stage scratch arena holds
//!    `[AtomicUsize; MAX_FIBERS]`; `PoolFrame.progress_slots` carries
//!    the non-owning `NonNull<AtomicUsize>` base pointer. Codegen
//!    emits an `add x8, x0, x1, lsl #3 ; stlr x2, [x8]` shape on
//!    aarch64 (single Release store, single shift-add address
//!    compute) per Topic 4 axis E + sketch
//!    `mock/research/sketches/202605101036-progress-counter-arena/`
//!    (WORKS). The S3 invariant (Topic 3 corrected): emit an
//!    architectural store-store fence immediately before the
//!    Release store. See `super::sync::emit_progress_release_fence`.

use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::USize;

/// Per-fiber monotonic record index.
#[repr(transparent)]
#[derive(Debug, Default)]
pub struct ProgressCounter(AtomicUsize);

impl ProgressCounter {
    /// Construct a counter initialised to `start`.
    pub const fn new(start: USize) -> Self {
        Self(AtomicUsize::new(start.0))
    }

    /// Release store. Publishes `value` to any thread doing a
    /// later Acquire load on this counter.
    pub fn store(&self, value: USize) {
        self.0.store(*value, Ordering::Release);
    }

    /// Acquire load. Pairs with a Release store from the writer.
    pub fn load(&self) -> USize {
        USize(self.0.load(Ordering::Acquire))
    }
}

/// Release-store to the arena-indirected progress counter at
/// `arena_base.add(slot)`. Pairs with `load_progress_arena` on the
/// consumer side. Topic 4 axis E2.
///
/// # Safety
///
/// `arena_base` must point at the head of an
/// `[AtomicUsize; MAX_FIBERS]` slice live for at least the duration
/// of this call (carried via `'arena` on the enclosing `PoolFrame`).
/// `slot` must be in-bounds relative to that slice. Engine callers
/// satisfy both via plan-stage allocation; consumers do not call
/// this directly.
#[inline(always)]
pub unsafe fn store_progress_arena(arena_base: NonNull<AtomicUsize>, slot: USize, value: USize) {
    // SAFETY: precondition delegated to caller (engine plan-stage).
    let counter = unsafe { &*arena_base.as_ptr().add(slot.0) };
    counter.store(value.0, Ordering::Release);
}

/// Acquire-load from the arena-indirected progress counter at
/// `arena_base.add(slot)`. Pairs with `store_progress_arena` on the
/// producer side. Topic 4 axis E2.
///
/// # Safety
///
/// Same preconditions as `store_progress_arena`.
#[inline(always)]
pub unsafe fn load_progress_arena(arena_base: NonNull<AtomicUsize>, slot: USize) -> USize {
    // SAFETY: precondition delegated to caller (engine plan-stage).
    let counter = unsafe { &*arena_base.as_ptr().add(slot.0) };
    USize(counter.load(Ordering::Acquire))
}
