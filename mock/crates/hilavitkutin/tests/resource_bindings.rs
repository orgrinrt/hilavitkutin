//! Resource-bindings tests for the data plane over `ColumnStorage`.
//!
//! A stack-backed counting `MemoryProvider` (fixed `[MaybeUninit<u8>;
//! N]` bump allocator with allocate/deallocate counters) backs an
//! `ArenaColumnStorage`, which drives the bindings round-trip and the
//! allocation-pairing checks. No `std::alloc`; stays
//! `#![no_std]`-compatible (the test harness itself is std, but the
//! provider allocates from a fixed stack buffer).
//!
//! Resources are `ColumnValue` (`Copy + 'static`): the store reserves a
//! one-record column per resource, the bindings records the column base
//! pointer, and the store frees the bytes on its own `Drop`. Arena
//! internals are reached through the engine's hidden `__` accessors
//! (`Scheduler::__bindings`, `ResourceBinding::__ptr` / `__tail`).

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::scheduler::{BuildError, NullMemoryProvider, Scheduler};
use hilavitkutin_api::Resource;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_providers::ArenaColumnStorage;

/// Wrap a provider in the default-capacity bindings store. The return type
/// omits `D`, which applies the `Dim<256>` default and anchors inference
/// (a bare `ArenaColumnStorage::new(p)` call site leaves `D` ambiguous).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

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

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {
        self.deallocs.set(self.deallocs.get() + 1);
    }

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// A provider that delegates to a bump allocator but counts through
// process-static atomics, so the counts survive the provider being
// moved into (and dropped with) the store / scheduler.
struct CountingProvider<const N: usize> {
    inner: BumpProvider<N>,
}
unsafe impl<const N: usize> Send for CountingProvider<N> {}
unsafe impl<const N: usize> Sync for CountingProvider<N> {}

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static DEALLOCS: AtomicUsize = AtomicUsize::new(0);

// The two `CountingProvider` tests share the global `ALLOCS` / `DEALLOCS`
// counters, each resetting them at its start and asserting an exact total.
// Cargo runs tests in parallel, so without serialisation one test's
// reset-then-assert window can observe another's increments. This lock
// serialises the counter-sharing tests against each other (other tests still
// run in parallel). Poison is recovered: a panic in one test must not cascade
// into a spurious poison-panic in the next.
static COUNTING_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Alignment observed at the most recent `deallocate`, so a test can
/// check the value survived the round trip rather than trusting it did.
static LAST_DEALLOC_ALIGN: AtomicUsize = AtomicUsize::new(0);

impl<const N: usize> MemoryProviderApi for CountingProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        // SAFETY: delegates to the inner bump allocator.
        let ptr = unsafe { self.inner.allocate(len, align) };
        // Count successful allocations only. A null return reserves no
        // column, so it must not count toward the live-column total that
        // DEALLOCS is balanced against (matches BumpProvider's own
        // success-only count).
        if !ptr.is_null() {
            ALLOCS.fetch_add(1, Ordering::SeqCst);
        }
        ptr
    }
    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize) {
        DEALLOCS.fetch_add(1, Ordering::SeqCst);
        // The contract says `align` comes back as it went out. Assert
        // it rather than forward it blindly: a caller that guesses here
        // frees with a layout that does not match the one it allocated,
        // which is undefined behaviour under a layout-taking allocator
        // and silent under a bump one. This provider is a bump arena,
        // so without the assertion the mismatch would never surface.
        assert!(
            *align == 0 || align.0.is_power_of_two(),
            "deallocate got a non-power-of-two alignment: {}",
            *align
        );
        LAST_DEALLOC_ALIGN.store(*align, Ordering::SeqCst);
        // SAFETY: delegates to the inner bump allocator.
        unsafe { self.inner.deallocate(ptr, len, align) }
    }
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

// Distinct resource value types so `multiple_resources` exercises a
// three-node bindings chain with three different `T` identities. Resources
// are `ColumnValue` now, so each derives `Copy`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Ra(u32);
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Rb(u16);
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Rc(u8);

#[test]
fn resource_bindings_round_trip() {
    let provider = BumpProvider::<256>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(Ra(99)))
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));
    // The bindings holds one ResourceBinding<Ra, BindingNil>; deref its
    // recorded pointer, which now points into the store-reserved column,
    // and confirm the moved-in value.
    // SAFETY: the pointer was written with Ra(99) at build time and the
    // scheduler (hence the store backing the column) is still alive.
    let value = unsafe { &*scheduler.__bindings().__ptr().as_ptr() };
    assert_eq!(*value, Ra(99));
}

#[test]
fn resource_bindings_multiple_resources() {
    // Three distinct types, all reachable through the bindings chain, each
    // backed by its own reserved column in the store.
    let provider = BumpProvider::<256>::new();
    let scheduler = Scheduler::builder()
        .with(Resource::new(Ra(1)))
        .with(Resource::new(Rb(2)))
        .with(Resource::new(Rc(3)))
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));
    // `.with` prepends, so registration order (Ra, Rb, Rc) reverses on
    // the cons-list: head is the last registered (Rc), then Rb, then Ra.
    let bindings = scheduler.__bindings();
    // SAFETY: all three pointers were written at build time into their
    // own columns; the store is alive.
    let head = unsafe { &*bindings.__ptr().as_ptr() };
    assert_eq!(*head, Rc(3));
    let mid = unsafe { &*bindings.__tail().__ptr().as_ptr() };
    assert_eq!(*mid, Rb(2));
    let last = unsafe { &*bindings.__tail().__tail().__ptr().as_ptr() };
    assert_eq!(*last, Ra(1));
}

#[test]
fn resource_bindings_drop_deallocates() {
    let _serial = COUNTING_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ALLOCS.store(0, Ordering::SeqCst);
    DEALLOCS.store(0, Ordering::SeqCst);
    {
        let provider = CountingProvider::<256> {
            inner: BumpProvider::<256>::new(),
        };
        let scheduler = Scheduler::builder()
            .with(Resource::new(Ra(1)))
            .with(Resource::new(2u64))
            .build(store(provider), USize(0))
            .unwrap_or_else(|_| panic!("build should succeed"));
        // two resources reserved one column each, none freed yet.
        assert_eq!(ALLOCS.load(Ordering::SeqCst), 2);
        assert_eq!(DEALLOCS.load(Ordering::SeqCst), 0);
        drop(scheduler);
    }
    // every reserve paired with a free after the store (held by the
    // scheduler) drops.
    assert_eq!(
        ALLOCS.load(Ordering::SeqCst),
        DEALLOCS.load(Ordering::SeqCst)
    );
    assert_eq!(DEALLOCS.load(Ordering::SeqCst), 2);
}

#[test]
fn zst_resource_round_trips_without_reserving() {
    // A zero-sized resource records a dangling pointer and reserves no
    // column: the store allocates nothing for it, yet the value still
    // round-trips through the recorded pointer (a ZST read touches no
    // memory). This is the #622 ZST-resource guard.
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    struct Marker;
    impl hilavitkutin_api::footprint::ResourceFootprint for Marker {
        const L1_BYTES: arvo::USize = arvo::USize(0);
    }

    let _serial = COUNTING_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ALLOCS.store(0, Ordering::SeqCst);
    DEALLOCS.store(0, Ordering::SeqCst);
    {
        let provider = CountingProvider::<256> {
            inner: BumpProvider::<256>::new(),
        };
        let scheduler = Scheduler::builder()
            .with(Resource::new(Marker))
            .build(store(provider), USize(0))
            .unwrap_or_else(|_| panic!("build should succeed"));
        // No allocation for a ZST resource: the drain skips the reserve.
        assert_eq!(ALLOCS.load(Ordering::SeqCst), 0);
        // SAFETY: the ZST value was written to a dangling, aligned pointer
        // at build time; reading a ZST back touches no memory.
        let value = unsafe { &*scheduler.__bindings().__ptr().as_ptr() };
        assert_eq!(*value, Marker);
        drop(scheduler);
    }
    // Nothing was reserved, so nothing is freed.
    assert_eq!(DEALLOCS.load(Ordering::SeqCst), 0);
}

#[test]
fn allocation_failure_returns_err() {
    // A store backed by `NullMemoryProvider` (always returns null) fails
    // to reserve a column for a non-ZST resource; the build reports
    // AllocationFailed.
    let result = Scheduler::builder()
        .with(Resource::new(Ra(5)))
        .build(store(NullMemoryProvider), USize(0));
    assert_eq!(result.err(), notko::Maybe::Is(BuildError::AllocationFailed));
}

#[test]
fn partial_failure_frees_the_reserved_columns() {
    // The conjunction the two tests above only prove in halves: when one
    // resource reserves and the next fails, the build must report
    // AllocationFailed AND the store, dropped on the Err arm, must free the
    // one column it did reserve. A leak on the partial-failure path (store
    // not dropped, or Drop not freeing the reserved-so-far columns) would
    // pass both halves yet leak in production.
    //
    // The store reserves each column with CACHE_LINE_ALIGN (64), so a
    // 64-byte buffer holds exactly one column: the first resource reserves
    // at offset 0, the second rounds up to offset 64 and overflows.
    let _serial = COUNTING_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    ALLOCS.store(0, Ordering::SeqCst);
    DEALLOCS.store(0, Ordering::SeqCst);
    {
        let provider = CountingProvider::<64> {
            inner: BumpProvider::<64>::new(),
        };
        let result = Scheduler::builder()
            .with(Resource::new(Ra(1)))
            .with(Resource::new(Rb(2)))
            .build(store(provider), USize(0));
        assert_eq!(result.err(), notko::Maybe::Is(BuildError::AllocationFailed));
    }
    // One column reserved before the failure, one freed on the Err arm: the
    // live-column count balances, so the partial-failure path leaks nothing.
    assert_eq!(ALLOCS.load(Ordering::SeqCst), 1);
    assert_eq!(DEALLOCS.load(Ordering::SeqCst), 1);
}

// A3b: test-local resource values are bare scalars/markers with no Seq/Map
// collection members, so their L1 morsel footprint is zero.
impl hilavitkutin_api::footprint::ResourceFootprint for Ra {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for Rb {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}
impl hilavitkutin_api::footprint::ResourceFootprint for Rc {
    const L1_BYTES: arvo::USize = arvo::USize(0);
}

/// The alignment a column was reserved with must reach its free.
///
/// `ArenaColumnStorage` reserves every column at 64-byte alignment
/// because the canonical design requires it. Before `deallocate` carried
/// `align`, the std provider rebuilt the freeing layout from a word and
/// every column free was a layout mismatch. Nothing caught it: the bump
/// providers used throughout these tests ignore alignment on free, so
/// the suite stayed green while the contract was unsatisfiable.
///
/// This asserts the value observed at the free is the one the reserve
/// used, which is the property the parameter exists to provide.
#[test]
fn column_free_sees_the_alignment_its_reserve_used() {
    let _guard = COUNTING_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    LAST_DEALLOC_ALIGN.store(0, Ordering::SeqCst);

    let provider = CountingProvider::<65536> {
        inner: BumpProvider::new(),
    };
    // SAFETY: 64 is a power of two and the bump arena covers 4KiB.
    let ptr = unsafe { provider.allocate(USize(4096), USize(64)) };
    assert!(!ptr.is_null(), "bump arena refused a 4KiB reservation");
    // SAFETY: ptr came from the allocate directly above.
    unsafe { provider.deallocate(ptr, USize(4096), USize(64)) };

    assert_eq!(
        LAST_DEALLOC_ALIGN.load(Ordering::SeqCst),
        64,
        "the free saw a different alignment than the reserve used"
    );
}

impl hilavitkutin_api::store::Replaceable for Ra {}

/// `replace_value` writes the new value into the data plane.
///
/// The body took `_new`, marked the store dirty, and dropped it, so a consumer
/// saw the dirty flag and concluded the swap had happened. Worse than an
/// unimplemented function: it reported success for work it did not do.
///
/// The signature could not have done otherwise. `T` unified to the marker
/// `Resource<Ra>`, which is `PhantomData` and carries no value; there was
/// nothing to install. The parameter is now the value type, which is also what
/// the bindings are keyed by, so `Selector<Ra, Index>` resolves the slot.
///
/// Green under the swap spec S1 install (round 202607200500): the swap IS the
/// whole-blob write through the same witness the drain wrote through. The
/// collection halves of a swapped value (per-record `Seq` / `Map` element
/// writes, spec S3) stay gated on the #344 collection wiring; `Ra` is scalar,
/// so this test is complete for the scalar-blob half.
#[test]
fn replace_value_installs_into_the_data_plane() {
    let provider = BumpProvider::<256>::new();
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(Ra(99)))
        .build(store(provider), USize(0))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // SAFETY: written with Ra(99) at build time; the scheduler is alive.
    let before = unsafe { *scheduler.__bindings().__ptr().as_ptr() };
    assert_eq!(before, Ra(99), "precondition: the build value is in place");

    scheduler.replace_value(Ra(7));

    // SAFETY: same pointer, still owned by the live scheduler.
    let after = unsafe { *scheduler.__bindings().__ptr().as_ptr() };
    assert_eq!(
        after,
        Ra(7),
        "replace_value must install the new value; Ra(99) here means the \
         argument was dropped and only the dirty flag was set"
    );
}
