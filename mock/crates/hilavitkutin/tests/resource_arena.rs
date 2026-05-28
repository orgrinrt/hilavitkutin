//! Resource-arena tests for the B2a data plane.
//!
//! A stack-backed counting `MemoryProvider` (fixed `[MaybeUninit<u8>;
//! N]` bump allocator with allocate/deallocate counters) drives the
//! arena round-trip, destructor, and deallocation-pairing checks. No
//! `std::alloc`; stays `#![no_std]`-compatible (the test harness itself
//! is std, but the provider allocates from a fixed stack buffer).
//!
//! Arena internals are reached through the engine's hidden `__`
//! accessors (`Scheduler::__arena` / `__memory_provider`,
//! `ArenaResourceNode::__ptr` / `__tail`).

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::scheduler::{BuildError, NullMemoryProvider, Scheduler};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::Resource;

// ---------------------------------------------------------------------
// Stack-backed counting test memory provider.
// ---------------------------------------------------------------------

/// Fixed-buffer bump allocator counting allocate / deallocate pairs.
///
/// `N` is the byte capacity. Interior mutability via `Cell` behind
/// `&self` (tests are single-threaded). The `Send + Sync` impls are
/// sound only under that single-threaded use.
struct BumpProvider<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
    allocs: Cell<usize>,
    deallocs: Cell<usize>,
}

impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self {
            buf: UnsafeCell::new([const { MaybeUninit::uninit() }; N]),
            used: Cell::new(0),
            allocs: Cell::new(0),
            deallocs: Cell::new(0),
        }
    }

    fn alloc_count(&self) -> usize {
        self.allocs.get()
    }

    fn dealloc_count(&self) -> usize {
        self.deallocs.get()
    }
}

// SAFETY: the provider is never shared across threads in these tests;
// the MemoryProviderApi bound requires Send + Sync.
unsafe impl<const N: usize> Send for BumpProvider<N> {}
unsafe impl<const N: usize> Sync for BumpProvider<N> {}

impl<const N: usize> MemoryProviderApi for BumpProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = self.buf.get() as *mut u8;
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > N {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        self.allocs.set(self.allocs.get() + 1);
        // SAFETY: `aligned + len <= N`, so the offset is in bounds of
        // the owned buffer.
        unsafe { base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {
        self.deallocs.set(self.deallocs.get() + 1);
    }

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// A provider that delegates to a bump allocator but counts through
// process-static atomics, so the counts survive the provider being
// moved into (and dropped with) the scheduler.
struct CountingProvider<const N: usize> {
    inner: BumpProvider<N>,
}
unsafe impl<const N: usize> Send for CountingProvider<N> {}
unsafe impl<const N: usize> Sync for CountingProvider<N> {}

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static DEALLOCS: AtomicUsize = AtomicUsize::new(0);

impl<const N: usize> MemoryProviderApi for CountingProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::SeqCst);
        // SAFETY: delegates to the inner bump allocator.
        unsafe { self.inner.allocate(len, align) }
    }
    unsafe fn deallocate(&self, ptr: *mut u8, len: USize) {
        DEALLOCS.fetch_add(1, Ordering::SeqCst);
        // SAFETY: delegates to the inner bump allocator.
        unsafe { self.inner.deallocate(ptr, len) }
    }
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// Distinct resource value types so `multiple_resources` exercises a
// three-node arena chain with three different `T` identities.
#[derive(PartialEq, Eq, Debug)]
struct Ra(u32);
#[derive(PartialEq, Eq, Debug)]
struct Rb(u16);
#[derive(PartialEq, Eq, Debug)]
struct Rc(u8);

#[test]
fn resource_arena_round_trip() {
    let provider = BumpProvider::<256>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(Ra(99)))
        .build(provider)
        .unwrap_or_else(|_| panic!("build should succeed"));
    // The arena holds one ArenaResourceNode<Ra, ArenaTail>; deref its
    // recorded pointer and confirm the moved-in value.
    // SAFETY: the pointer was written with Ra(99) at build time and the
    // scheduler (hence arena) is still alive.
    let value = unsafe { &*scheduler.__arena().__ptr().as_ptr() };
    assert_eq!(*value, Ra(99));
}

#[test]
fn resource_arena_multiple_resources() {
    // Three distinct types, all reachable through the arena chain.
    let provider = BumpProvider::<256>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(Ra(1)))
        .with(Resource::new(Rb(2)))
        .with(Resource::new(Rc(3)))
        .build(provider)
        .unwrap_or_else(|_| panic!("build should succeed"));
    // `.with` prepends, so registration order (Ra, Rb, Rc) reverses on
    // the cons-list: head is the last registered (Rc), then Rb, then Ra.
    let arena = scheduler.__arena();
    // SAFETY: all three pointers were written at build time; alive.
    let head = unsafe { &*arena.__ptr().as_ptr() };
    assert_eq!(*head, Rc(3));
    let mid = unsafe { &*arena.__tail().__ptr().as_ptr() };
    assert_eq!(*mid, Rb(2));
    let last = unsafe { &*arena.__tail().__tail().__ptr().as_ptr() };
    assert_eq!(*last, Ra(1));
}

#[test]
fn resource_arena_drop_deallocates() {
    ALLOCS.store(0, Ordering::SeqCst);
    DEALLOCS.store(0, Ordering::SeqCst);
    {
        let provider = CountingProvider::<256> {
            inner: BumpProvider::<256>::new(),
        };
        let scheduler = Scheduler::builder()
            .with(Resource::new(Ra(1)))
            .with(Resource::new(2u64))
            .build(provider)
            .unwrap_or_else(|_| panic!("build should succeed"));
        // two resources allocated, none freed yet.
        assert_eq!(ALLOCS.load(Ordering::SeqCst), 2);
        assert_eq!(DEALLOCS.load(Ordering::SeqCst), 0);
        drop(scheduler);
    }
    // every allocate paired with a deallocate after the scheduler drops.
    assert_eq!(ALLOCS.load(Ordering::SeqCst), DEALLOCS.load(Ordering::SeqCst));
    assert_eq!(DEALLOCS.load(Ordering::SeqCst), 2);
}

#[test]
fn resource_arena_drop_runs_destructor() {
    static DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DropCounter;
    impl Drop for DropCounter {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::SeqCst);
        }
    }

    DROPS.store(0, Ordering::SeqCst);
    {
        let provider = BumpProvider::<256>::new();
        let _scheduler = Scheduler::builder()
            .with(Resource::new(DropCounter))
            .build(provider)
            .unwrap_or_else(|_| panic!("build should succeed"));
        // not yet dropped.
        assert_eq!(DROPS.load(Ordering::SeqCst), 0);
    }
    // scheduler dropped at end of block; DropCounter::drop fired via the
    // arena DropArena walk.
    assert_eq!(DROPS.load(Ordering::SeqCst), 1);
}

#[test]
fn build_records_one_allocation_per_resource() {
    let provider = BumpProvider::<256>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(Ra(5)))
        .build(provider)
        .unwrap_or_else(|_| panic!("build should succeed"));
    assert_eq!(scheduler.__memory_provider().alloc_count(), 1);
    assert_eq!(scheduler.__memory_provider().dealloc_count(), 0);
}

#[test]
fn allocation_failure_returns_err() {
    // NullMemoryProvider always returns null; building a scheduler that
    // needs a resource block fails with AllocationFailed.
    let result = Scheduler::builder()
        .with(Resource::new(Ra(5)))
        .build(NullMemoryProvider);
    assert_eq!(result.err(), notko::Maybe::Is(BuildError::AllocationFailed));
}
