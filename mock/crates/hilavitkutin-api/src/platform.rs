//! Platform contracts.
//!
//! Three runtime surfaces the engine needs from the host: memory
//! allocation and protection, thread-pool spawn and sizing, and a
//! monotonic clock. Consumed by monomorphisation — no `dyn`.

use arvo::newtype::{Bool, USize};

/// Memory provider.
///
/// Backing allocator for arena slabs. `allocate` / `deallocate`
/// track raw pointers sized by `USize`. `protect` toggles page-level
/// read/write permissions; consumers use it for sealed resource
/// pages.
pub trait MemoryProviderApi: Send + Sync + 'static {
    /// Allocate `len` bytes at `align` alignment.
    ///
    /// # Safety
    ///
    /// Returned pointer is valid until a matching `deallocate`.
    /// Alignment must be a power of two. OOM returns null; caller
    /// checks.
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8;

    /// Release a previously-allocated block.
    ///
    /// # Safety
    ///
    /// `ptr` must come from a prior `allocate` on the same provider
    /// with the same `len`. Deallocation invalidates the pointer.
    unsafe fn deallocate(&self, ptr: *mut u8, len: USize);

    /// Change page permissions over a block.
    ///
    /// # Safety
    ///
    /// `ptr` must cover `len` bytes of pages owned by this provider.
    /// Revoking read or write while a live borrow exists is UB.
    unsafe fn protect(&self, ptr: *mut u8, len: USize, read: Bool, write: Bool);
}

/// Thread-pool provider.
///
/// Threads are spawned once at pipeline construction. `spawn` is
/// generic over the closure type so the pool can monomorphise per
/// call site. No `dyn`.
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
pub trait ClockApi: Send + Sync + 'static {
    /// Current time in nanoseconds since a platform-defined epoch.
    fn now_ns(&self) -> u64;
}

// Accessor traits expressed directly rather than via ctx's
// `provider!` macro: the macro emits a `Context<P>` inherent impl
// that would violate the orphan rule from this downstream crate.
// Each accessor is a plain trait: `HasX { type Provider: XApi; fn
// method(&self) -> &Self::Provider; }`.

/// Provider-tuple entry point for the memory provider.
pub trait HasMemoryProvider {
    /// Concrete provider type the tuple exposes.
    type Provider: MemoryProviderApi;
    /// Borrow the memory provider.
    fn memory(&self) -> &Self::Provider;
}

/// Provider-tuple entry point for the thread pool.
pub trait HasThreadPool {
    /// Concrete pool type the tuple exposes.
    type Provider: ThreadPoolApi;
    /// Borrow the thread pool.
    fn threads(&self) -> &Self::Provider;
}

/// Provider-tuple entry point for the clock.
pub trait HasClock {
    /// Concrete clock type the tuple exposes.
    type Provider: ClockApi;
    /// Borrow the clock.
    fn clock(&self) -> &Self::Provider;
}
