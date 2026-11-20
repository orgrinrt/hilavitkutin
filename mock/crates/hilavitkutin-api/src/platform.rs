//! Platform contracts.
//!
//! Three runtime surfaces the engine needs from the host: memory
//! allocation and protection, thread-pool spawn and sizing, and a
//! monotonic clock. Consumed by monomorphisation: no `dyn`.

use core::marker::PhantomData;

use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo::{Bool, USize, fbits, ibits};

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
///
/// # Safety
///
/// The engine records the blocks this provider returns and reads and
/// writes through them from unsafe code after the provider has been
/// moved, so an implementation promises:
///
/// 1. A non-null block from `allocate(len, align)` is aligned to
///    `align` and valid for reads and writes of `len` bytes.
/// 2. The block stays valid at the same address until the matching
///    `deallocate` on this provider, or until the provider is dropped.
///    Moving the provider moves and frees no block, so a provider that
///    keeps its memory inline in itself cannot implement this trait.
/// 3. Two live blocks do not overlap.
/// 4. Every method may be called from several threads at once.
/// 5. A non-null block from `allocate_zeroed(len, align)` keeps the
///    first promise and reads as `len` zero bytes. The provided method
///    keeps it by writing them; a provider whose fresh blocks are zero
///    overrides it and skips the write.
///
/// Null is always a valid answer from `allocate` and
/// `allocate_zeroed` and means the block was refused. The `# Safety` sections on the methods are the
/// caller's half of the contract.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not implement Memory provider contract",
    note = "Provide a platform-specific impl. The engine builds against `MemoryProviderApi` via const generics; supply your own `MemoryProvider` to the scheduler at construction time."
)]
pub unsafe trait MemoryProviderApi: Send + Sync + 'static {
    /// Allocate `len` bytes at `align` alignment.
    ///
    /// # Safety
    ///
    /// Returned pointer is valid until a matching `deallocate`.
    /// Alignment must be a power of two. OOM returns null; caller
    /// checks.
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI returns raw pointer by contract; tracked: #72

    /// Allocate `len` bytes at `align` alignment, every byte zero.
    ///
    /// The default calls `allocate` and writes the zeros into a
    /// non-null block. A provider whose fresh blocks are already zero
    /// overrides it with `allocate`.
    ///
    /// # Safety
    ///
    /// As `allocate`: alignment must be a power of two, and OOM
    /// returns null, which the caller checks.
    unsafe fn allocate_zeroed(&self, len: USize, align: USize) -> *mut u8 {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI returns raw pointer by contract; tracked: #72
        // SAFETY: the caller keeps `allocate`'s half of the contract.
        let ptr = unsafe { self.allocate(len, align) };
        if !ptr.is_null() {
            // SAFETY: a non-null block from `allocate` is valid for writes
            // of `len` bytes (promise 1).
            unsafe { core::ptr::write_bytes(ptr, 0, len.0) };
        }
        ptr
    }

    /// Release a previously-allocated block.
    ///
    /// # Safety
    ///
    /// `ptr` must come from a prior `allocate` on the same provider
    /// with the same `len` and the same `align`. All three are the
    /// contract: nothing at the free site can recover the alignment,
    /// and an allocator underneath may need the whole layout to free.
    /// Deallocation invalidates the pointer.
    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72

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
    ///
    /// Every closure the engine hands here is no wider and no more
    /// aligned than one pointer, so an implementation can move it into
    /// a thread's single argument slot without allocating. The engine
    /// forces [`OnePointerClosure::FITS`] on it at the spawn site, and
    /// an implementation may force the same gate on its own `F`.
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static;

    /// Number of worker threads in the pool.
    fn worker_count(&self) -> USize;
}

/// Compile-time gate that a closure fits one pointer-sized,
/// pointer-aligned slot.
///
/// Forcing `OnePointerClosure::<F>::FITS` fails the build at
/// monomorphisation when `F` is wider or more aligned than
/// `*const ()`. The engine forces it on the closure it hands
/// [`ThreadPoolApi::spawn`]. An associated const rather than an inline
/// `const {}` block, because the latter is an anonymous generic
/// constant the `generic_const_exprs` grammar rejects.
///
/// The failure is a monomorphisation error, so `cargo check` does not
/// report it and `cargo build` does. A closure capturing two
/// pointer-sized values is refused:
///
/// ```compile_fail,E0080
/// use hilavitkutin_api::platform::OnePointerClosure;
///
/// fn force<F: FnOnce()>(f: F) {
///     let () = OnePointerClosure::<F>::FITS;
///     f();
/// }
///
/// let (a, b) = (1usize, 2usize);
/// force(move || {
///     core::hint::black_box((a, b));
/// });
/// ```
///
/// A zero-sized capture aligned past a pointer is no wider than one
/// pointer and is refused on alignment alone:
///
/// ```compile_fail,E0080
/// use hilavitkutin_api::platform::OnePointerClosure;
///
/// #[derive(Copy, Clone)]
/// #[repr(align(64))]
/// struct OverAligned;
///
/// fn force<F: FnOnce()>(f: F) {
///     let () = OnePointerClosure::<F>::FITS;
///     f();
/// }
///
/// let z = OverAligned;
/// force(move || {
///     core::hint::black_box(z);
/// });
/// ```
///
/// The same shape with one captured value builds:
///
/// ```
/// use hilavitkutin_api::platform::OnePointerClosure;
///
/// fn force<F: FnOnce()>(f: F) {
///     let () = OnePointerClosure::<F>::FITS;
///     f();
/// }
///
/// let a = 1usize;
/// force(move || {
///     core::hint::black_box(a);
/// });
/// ```
pub struct OnePointerClosure<F>(PhantomData<F>);

impl<F> OnePointerClosure<F> {
    /// Evaluates to `()` when `F` fits one pointer slot; fails the
    /// build otherwise.
    pub const FITS: () = {
        assert!(
            core::mem::size_of::<F>() <= core::mem::size_of::<*const ()>(),
            "the closure handed to ThreadPoolApi::spawn is wider than one pointer; an executor cannot carry it without allocating"
        );
        assert!(
            core::mem::align_of::<F>() <= core::mem::align_of::<*const ()>(),
            "the closure handed to ThreadPoolApi::spawn is more aligned than a pointer slot"
        );
    };
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
