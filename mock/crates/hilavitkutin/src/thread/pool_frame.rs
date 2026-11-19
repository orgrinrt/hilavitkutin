//! The runtime data plane: the pool frame workers share and the wake
//! strategy that picks their parking tier.
//!
//! Only the engine and its workers read either, so both are declared
//! in the engine.

use core::marker::PhantomData;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize};

use arvo::USize;

/// Per-pool runtime data plane carrier. Hot fields live on per-pool
/// cache lines, NOT per-fiber lines
/// (per-fiber lines hold progress counters via `progress_slots` arena
/// indirection).
///
/// Const generics `MAX_CORES` and `MAX_PHASES` bound the slot arrays.
/// `#[repr(C, align(64))]` so the carrier sits on its own cache line
/// boundary; the inner field layout is hand-tuned for false-sharing
/// avoidance (predicted_wait_ns and idle_accumulator are per-core, so
/// they land on separate lines per index).
///
/// `shutdown`: workers Relaxed-load after each morsel; on
/// `true`, exit mainloop cleanly. Scheduler::Drop sets it then wakes
/// all parked workers via the platform atomic-wait primitive.
///
/// `phase_arrived`: centralised atomic counter. Workers Release
/// fetch_add(1) on phase exit; last arriver wakes all parked workers and
/// resets the counter. Tree-barrier deferred until 32+ core benches
/// show measurable cacheline ping-pong (BACKLOG).
///
/// `predicted_wait_ns`: per-phase atomic slot. AdaptWu writes
/// at `ScheduleEnd`; workers Relaxed-load on phase entry to pick the
/// parking tier (spin / futex / park) per the WakeStrategy thresholds.
///
/// `idle_accumulator` + `park_count`: per-core
/// wait-time accumulators. Park entry/exit increments the accumulator
/// (Release on increment paired with phase-boundary Release); AdaptWu
/// reads with Acquire at `ScheduleEnd`.
///
/// `progress_slots`: non-owning pointer into the
/// plan-stage scratch arena's `[AtomicUsize; MAX_FIBERS]` region.
/// Codegen emits Release stores against `progress_slots.add(slot_idx)`.
/// The `'arena` lifetime parameter ties the borrow to the scratch
/// arena that owns the `[AtomicUsize; MAX_FIBERS]` storage. Any
/// `&'_ PoolFrame<'arena, ...>` borrow is constrained by `'arena`;
/// dropping the arena before workers stop is a borrow-checker error
/// rather than a use-after-free at runtime. The `PhantomData` field
/// expresses the relationship without changing the runtime layout.
#[repr(C, align(64))]
pub struct PoolFrame<'arena, const MAX_CORES: usize, const MAX_PHASES: usize> {
    /// Shutdown signal. Set by `Scheduler::Drop`. Workers Relaxed-load.
    pub shutdown: AtomicBool,

    /// Phase-barrier counter. Workers fetch_add(1) on phase exit
    /// (Release); last arriver futex-wakes all and resets.
    pub phase_arrived: AtomicU32,

    /// Waist-barrier sense / generation word. Paired with
    /// `phase_arrived` for the worker-side sense-reversing barrier:
    /// the last arriver of an episode resets `phase_arrived` and bumps
    /// this (Release), then futex-wakes all; followers park on it and
    /// wake when it changes from the value they snapshotted on entry.
    /// The flip makes the barrier reusable across the many waists of a
    /// frame with no reset race (a fast worker that loops to the next
    /// episode waits for the next flip, never a stale count).
    pub barrier_sense: AtomicU32,

    /// Frame sequence / worker wake word. The scheduler bumps it
    /// (Release) and futex-wakes to publish a new frame; parked
    /// workers wait on it. Distinct from `phase_arrived` (the
    /// within-frame waist barrier): `seq` marks frame START. Drives
    /// the per-frame protocol in `thread::frame`.
    pub seq: AtomicU32,

    /// Frame-completion counter / scheduler wake word. Each worker
    /// fetch_add(1)s (Release) on finishing a frame; the last arriver
    /// futex-wakes the scheduler, which is parked on it. Reset to zero
    /// by the scheduler before each `frame_publish`.
    pub done: AtomicU32,

    /// Worker-exit counter. Each worker fetch_add(1)s (Release) on
    /// observing `shutdown`, then futex-wakes the scheduler. The
    /// scheduler waits for this to reach the worker count before the
    /// `PoolFrame` (and the carrier its workers read) can drop. This
    /// is the join with no join method: the `ThreadPoolApi` contract
    /// is fire-and-forget and workers are detached, so the exit
    /// counter, not a thread join, provides the shutdown ordering.
    pub exited: AtomicU32,

    /// Per-phase predicted wait time in nanoseconds, written by
    /// AdaptWu at `ScheduleEnd`, read by workers at phase entry.
    /// Drives WakeStrategy tier selection.
    pub predicted_wait_ns: [AtomicU32; MAX_PHASES],

    /// Per-core park-time accumulator in nanoseconds. Owning core
    /// fetch_add(1)s on park entry/exit. AdaptWu reads with Acquire
    /// fence at `ScheduleEnd`.
    pub idle_accumulator: [AtomicU64; MAX_CORES],

    /// Per-core park count. Pairs with `idle_accumulator` for the
    /// core-idle adapt axis.
    pub park_count: [AtomicU64; MAX_CORES],

    /// Base pointer to `[AtomicUsize; MAX_FIBERS]` in plan-stage
    /// scratch arena. Codegen emits Release stores against
    /// `progress_slots.add(slot_idx)`. Non-owning; the `'arena`
    /// lifetime on the enclosing struct ties this borrow to the
    /// arena's allocation.
    pub progress_slots: NonNull<AtomicUsize>,

    /// Number of valid progress slots starting at `progress_slots`.
    pub progress_slot_count: USize,

    /// Borrow tracker tying the `progress_slots` pointer to the arena
    /// it indexes into. Zero-sized at runtime; load-bearing at the
    /// type level so dropping the arena while a worker holds a
    /// `&PoolFrame<'arena>` is a compile-time error.
    pub _arena: PhantomData<&'arena [AtomicUsize]>,
}

// SAFETY: `NonNull<AtomicUsize>` carries no Send/Sync auto-impl on its
// own. The `'arena` lifetime on `PoolFrame` ties the borrow to the
// scratch arena's allocation; workers receive `Pin<&'frame PoolFrame<'arena, ...>>`
// borrows that the borrow-checker constrains to `'arena: 'frame`.
// Under those bounds the `AtomicUsize` slice is shared-safe (its own
// methods carry the synchronisation) and the pointer is valid for as
// long as any worker holds the borrow.
unsafe impl<'arena, const C: usize, const P: usize> Send for PoolFrame<'arena, C, P> {}
unsafe impl<'arena, const C: usize, const P: usize> Sync for PoolFrame<'arena, C, P> {}

/// Per-CoreClass wake strategy.
///
/// Controls the three-tier parking selection per worker: spin budget
/// before yielding, futex threshold for sub-50µs waits, park threshold
/// for long-tail waits. Per-CoreClass split because P-cores cost more
/// to leave parked than E-cores; default `default_hybrid()` ships
/// `p_spin_iters = 128`, `e_spin_iters = 32` (4:1 ratio matching Apple
/// Silicon P/E latency asymmetry).
///
/// Per `arvo-toolbox-not-policer.md`: fields are public, the
/// `default_hybrid()` constructor provides sensible defaults, consumers
/// override per-app via struct literal + `..default_hybrid()`. Removes
/// the PureSpin / PurePark enum variants in favour of expressing both
/// via the same struct (`p_spin = e_spin = USize::MAX` is pure-spin;
/// `p_spin = e_spin = <USize as Identity<Additive>>::IDENTITY` is pure-park).
pub struct WakeStrategy {
    /// Spin iterations on P-cores before falling back to the atomic-
    /// wait tier. Default 128.
    pub p_spin_iters:       USize,
    /// Spin iterations on E-cores before falling back. Default 32.
    pub e_spin_iters:       USize,
    /// Nanosecond threshold below which spin + atomic-wait (futex /
    /// ulock / WaitOnAddress) is cheaper than full park. Default 2µs.
    pub futex_threshold_ns: USize,
    /// Nanosecond threshold above which spin is skipped entirely and
    /// the worker parks immediately via the platform atomic-wait
    /// primitive. Default 50µs.
    pub park_threshold_ns:  USize,
}

impl WakeStrategy {
    /// Substrate-default hybrid wake strategy.
    /// P-cores spin longer (128 iters; they cost more to
    /// re-park); E-cores yield quickly (32 iters; cheap to restart).
    /// Thresholds: <2µs spin-only, 2-50µs spin+futex, >50µs park
    /// immediately.
    pub const fn default_hybrid() -> Self {
        Self {
            p_spin_iters:       USize(128),
            e_spin_iters:       USize(32),
            futex_threshold_ns: USize(2_000),
            park_threshold_ns:  USize(50_000),
        }
    }
}

impl Default for WakeStrategy {
    fn default() -> Self {
        Self::default_hybrid()
    }
}
