//! Platform contracts.
//!
//! Three runtime surfaces the engine needs from the host: memory
//! allocation and protection, thread-pool spawn and sizing, and a
//! monotonic clock. Consumed by monomorphisation: no `dyn`.

use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo::{fbits, ibits, Bool, USize};

/// Nanoseconds since a platform-defined epoch.
///
/// Monotonic instant type returned by `ClockApi::now_ns`. Backed by
/// `arvo::UFixed<64, 0, Hot>`; the `Hot` strategy dispatches to the
/// host's native 64-bit unsigned.
pub type Nanos = UFixed<{ ibits(64) }, { fbits(0) }, Hot>;

/// Memory provider.
///
/// Backing allocator for arena slabs. `allocate` / `deallocate`
/// track raw pointers sized by `USize`. `protect` toggles page-level
/// read/write permissions; consumers use it for sealed resource
/// pages.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not implement Memory provider contract",
    note = "Provide a platform-specific impl. The engine builds against `MemoryProviderApi` via const generics; supply your own `MemoryProvider` to the scheduler at construction time."
)]
pub trait MemoryProviderApi: Send + Sync + 'static {
    /// Allocate `len` bytes at `align` alignment.
    ///
    /// # Safety
    ///
    /// Returned pointer is valid until a matching `deallocate`.
    /// Alignment must be a power of two. OOM returns null; caller
    /// checks.
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI returns raw pointer by contract; tracked: #72

    /// Release a previously-allocated block.
    ///
    /// # Safety
    ///
    /// `ptr` must come from a prior `allocate` on the same provider
    /// with the same `len`. Deallocation invalidates the pointer.
    unsafe fn deallocate(&self, ptr: *mut u8, len: USize); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72

    /// Change page permissions over a block.
    ///
    /// # Safety
    ///
    /// `ptr` must cover `len` bytes of pages owned by this provider.
    /// Revoking read or write while a live borrow exists is UB.
    unsafe fn protect(&self, ptr: *mut u8, len: USize, read: Bool, write: Bool); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
}

/// Thread-pool provider.
///
/// Threads are spawned once at pipeline construction. `spawn` is
/// generic over the closure type so the pool can monomorphise per
/// call site. No `dyn`.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not implement ThreadPool provider contract",
    note = "Provide a platform-specific impl. The engine builds against `ThreadPoolApi` via const generics; supply your own `ThreadPool` to the scheduler at construction time."
)]
pub trait ThreadPoolApi: Send + Sync + 'static {
    /// Submit `f` for execution on a pool worker.
    ///
    /// Implementations may block, queue, or steal; the engine makes
    /// no assumption about scheduling fairness.
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static;

    /// Number of worker threads in the pool.
    fn worker_count(&self) -> USize;
}

/// Monotonic clock.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not implement Clock provider contract",
    note = "Provide a platform-specific impl. The engine builds against `ClockApi` via const generics; supply your own `Clock` to the scheduler at construction time."
)]
pub trait ClockApi: Send + Sync + 'static {
    /// Current time in nanoseconds since a platform-defined epoch.
    fn now_ns(&self) -> Nanos;
}

// Accessor traits expressed directly rather than via ctx's
// `provider!` macro: the macro emits a `Context<P>` inherent impl
// that would violate the orphan rule from this downstream crate.
// Each accessor is a plain trait: `HasX { type Provider: XApi; fn
// method(&self) -> &Self::Provider; }`.

/// Provider-tuple entry point for the memory provider.
#[diagnostic::on_unimplemented(
    message = "provider tuple `{Self}` does not expose a Memory provider",
    note = "Compose the provider tuple with the `provider_generic!` accessor. The substrate's `Context<P>` framework wires this from the scheduler builder."
)]
pub trait HasMemoryProvider {
    /// Concrete provider type the tuple exposes.
    type Provider: MemoryProviderApi;
    /// Borrow the memory provider.
    fn memory(&self) -> &Self::Provider;
}

/// Provider-tuple entry point for the thread pool.
#[diagnostic::on_unimplemented(
    message = "provider tuple `{Self}` does not expose a ThreadPool provider",
    note = "Compose the provider tuple with the `provider_generic!` accessor. The substrate's `Context<P>` framework wires this from the scheduler builder."
)]
pub trait HasThreadPool {
    /// Concrete pool type the tuple exposes.
    type Provider: ThreadPoolApi;
    /// Borrow the thread pool.
    fn threads(&self) -> &Self::Provider;
}

/// Provider-tuple entry point for the clock.
#[diagnostic::on_unimplemented(
    message = "provider tuple `{Self}` does not expose a Clock provider",
    note = "Compose the provider tuple with the `provider_generic!` accessor. The substrate's `Context<P>` framework wires this from the scheduler builder."
)]
pub trait HasClock {
    /// Concrete clock type the tuple exposes.
    type Provider: ClockApi;
    /// Borrow the clock.
    fn clock(&self) -> &Self::Provider;
}

// ---------------------------------------------------------------------
// Runtime data plane: PoolFrame + WakeStrategy + Executor.
// Topic 6 axes A / G / I / J / K + Topic 3 amendment M11.
// ---------------------------------------------------------------------

use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize};

/// Per-pool runtime data plane carrier. Topic 6 axes G / I / J + Topic
/// 3 M11. Hot fields live on per-pool cache lines, NOT per-fiber lines
/// (per-fiber lines hold progress counters via `progress_slots` arena
/// indirection).
///
/// Const generics `MAX_CORES` and `MAX_PHASES` bound the slot arrays.
/// `#[repr(C, align(64))]` so the carrier sits on its own cache line
/// boundary; the inner field layout is hand-tuned for false-sharing
/// avoidance (predicted_wait_ns and idle_accumulator are per-core, so
/// they land on separate lines per index).
///
/// `shutdown` (axis G): workers Relaxed-load after each morsel; on
/// `true`, exit mainloop cleanly. Scheduler::Drop sets it then wakes
/// all parked workers via the platform atomic-wait primitive.
///
/// `phase_arrived` (axis I): centralised atomic counter. Workers Release
/// fetch_add(1) on phase exit; last arriver wakes all parked workers and
/// resets the counter. Tree-barrier deferred until 32+ core benches
/// show measurable cacheline ping-pong (BACKLOG).
///
/// `predicted_wait_ns` (axis J): per-phase atomic slot. AdaptWu writes
/// at `ScheduleEnd`; workers Relaxed-load on phase entry to pick the
/// parking tier (spin / futex / park) per the WakeStrategy thresholds.
///
/// `idle_accumulator` + `park_count` (Topic 5 core-idle axis): per-core
/// wait-time accumulators. Park entry/exit increments the accumulator
/// (Release on increment paired with phase-boundary Release); AdaptWu
/// reads with Acquire at `ScheduleEnd`.
///
/// `progress_slots` (Topic 4 axis E): non-owning pointer into the
/// plan-stage scratch arena's `[AtomicUsize; MAX_FIBERS]` region.
/// Codegen emits Release stores against `progress_slots.add(slot_idx)`.
/// Lifetime tied to the `Scheduler::run()` frame; the pointer is valid
/// as long as the arena exists.
#[repr(C, align(64))]
pub struct PoolFrame<const MAX_CORES: usize, const MAX_PHASES: usize> {
    /// Shutdown signal. Set by `Scheduler::Drop`. Workers Relaxed-load.
    pub shutdown: AtomicBool,

    /// Phase-barrier counter. Workers fetch_add(1) on phase exit
    /// (Release); last arriver futex-wakes all and resets.
    pub phase_arrived: AtomicU32,

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
    /// `progress_slots.add(slot_idx)`. Non-owning; arena lifetime is
    /// tied to `Scheduler::run()` frame.
    pub progress_slots: NonNull<AtomicUsize>,

    /// Number of valid progress slots starting at `progress_slots`.
    pub progress_slot_count: USize,
}

// SAFETY: `NonNull<AtomicUsize>` is Send/Sync-safe under the lifetime
// contract: workers receive `Pin<&'frame PoolFrame>` where the 'frame
// outlives every worker's mainloop call.
unsafe impl<const C: usize, const P: usize> Send for PoolFrame<C, P> {}
unsafe impl<const C: usize, const P: usize> Sync for PoolFrame<C, P> {}

/// Per-CoreClass wake strategy. Topic 6 axis K.
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
/// `p_spin = e_spin = USize::ZERO` is pure-park).
pub struct WakeStrategy {
    /// Spin iterations on P-cores before falling back to the atomic-
    /// wait tier. Default 128.
    pub p_spin_iters: USize,
    /// Spin iterations on E-cores before falling back. Default 32.
    pub e_spin_iters: USize,
    /// Nanosecond threshold below which spin + atomic-wait (futex /
    /// ulock / WaitOnAddress) is cheaper than full park. Default 2µs.
    pub futex_threshold_ns: USize,
    /// Nanosecond threshold above which spin is skipped entirely and
    /// the worker parks immediately via the platform atomic-wait
    /// primitive. Default 50µs.
    pub park_threshold_ns: USize,
}

impl WakeStrategy {
    /// Substrate-default hybrid wake strategy. Topic 6 axis K
    /// resolution: P-cores spin longer (128 iters; they cost more to
    /// re-park); E-cores yield quickly (32 iters; cheap to restart).
    /// Thresholds: <2µs spin-only, 2-50µs spin+futex, >50µs park
    /// immediately.
    pub const fn default_hybrid() -> Self {
        Self {
            p_spin_iters: USize(128),
            e_spin_iters: USize(32),
            futex_threshold_ns: USize(2_000),
            park_threshold_ns: USize(50_000),
        }
    }
}

impl Default for WakeStrategy {
    fn default() -> Self {
        Self::default_hybrid()
    }
}

/// Sealed executor trait. Topic 6 axis A + audit-2 m4 + Topic 4 axis G.
///
/// The engine ships exactly one default impl `HybridExecutor` in
/// `hilavitkutin::thread`. Sealed via the api crate's private
/// `Sealed` supertrait so consumer impls of `Executor` cannot bypass
/// Topic 3 S7's cross-thread atomic-ordering protocol.
///
/// Per-worker mainloop entry: `run(pool, core_id, wake_strategy)`
/// walks the pre-computed `CoreProgram`, dispatching morsels, hitting
/// phase barriers, and observing the shutdown signal between morsels.
/// Returns `Outcome<(), ExecutorError>`; cleanly-shut workers return
/// `Outcome::Ok(())`.
pub trait Executor: crate::sealed::Sealed {
    /// Per-worker mainloop. Spawned once per core at
    /// `ThreadPool::build()` time; runs until `pool.shutdown` is set.
    fn run<'frame, const C: usize, const P: usize>(
        &self,
        pool: core::pin::Pin<&'frame PoolFrame<C, P>>,
        core_id: USize,
        wake_strategy: &WakeStrategy,
    ) -> notko::Outcome<(), ExecutorError>;
}

/// Executor failure modes. `#[non_exhaustive]` so future variants
/// don't break consumers.
#[non_exhaustive]
#[derive(Debug)]
pub enum ExecutorError {
    /// Worker exited because `pool.shutdown` was set. Not a true
    /// failure; the cleanup path.
    Shutdown,
}
