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

mod stable_provider;

use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin::scheduler::{BuildError, NullMemoryProvider, Scheduler};
use hilavitkutin_api::Resource;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_providers::ArenaColumnStorage;
use stable_provider::{StaticBumpMemory, stable_provider};

/// Wrap a provider in the default-capacity bindings store. The return type
/// omits `D`, which applies the `Dim<256>` default and anchors inference
/// (a bare `ArenaColumnStorage::new(p)` call site leaves `D` ambiguous).
fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// ---------------------------------------------------------------------
// Stack-backed counting test memory provider.
// ---------------------------------------------------------------------

// A provider that delegates to a bump allocator but counts through
// process-static atomics, so the counts survive the provider being
// moved into (and dropped with) the store / scheduler.
struct CountingProvider {
    inner: StaticBumpMemory,
}

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

/// Alignment observed at the most recent successful `allocate`.
static LAST_ALLOC_ALIGN: AtomicUsize = AtomicUsize::new(0);

/// Alignment observed at the most recent `deallocate`, so a test can
/// check the value survived the round trip rather than trusting it did.
static LAST_DEALLOC_ALIGN: AtomicUsize = AtomicUsize::new(0);

// SAFETY: every block comes from `inner` and goes back to it unchanged, so
// the promises `StaticBumpMemory` keeps are kept here; the counters are atomics.
unsafe impl MemoryProviderApi for CountingProvider {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        // SAFETY: delegates to the inner bump allocator.
        let ptr = unsafe { self.inner.allocate(len, align) };
        // Count successful allocations only. A null return reserves no
        // column, so it must not count toward the live-column total that
        // DEALLOCS is balanced against (matches the bump provider's own
        // success-only count).
        if !ptr.is_null() {
            ALLOCS.fetch_add(1, Ordering::SeqCst);
            LAST_ALLOC_ALIGN.store(*align, Ordering::SeqCst);
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
    let provider = stable_provider(256, 0);
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
    let provider = stable_provider(256, 0);
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
        let provider = CountingProvider {
            inner: stable_provider(256, 0),
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
        let provider = CountingProvider {
            inner: stable_provider(256, 0),
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
        let provider = CountingProvider {
            inner: stable_provider(64, 0),
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
/// The store reserves every column at cache-line alignment, and a provider
/// that frees by layout needs that alignment back. The bump providers used
/// throughout these tests ignore alignment on free, so a store that guessed
/// would keep the suite green. This drops a built scheduler and compares
/// what the provider was given at the free with what it was given at the
/// reserve. The store's own two freeing sites are read block by block in
/// `hilavitkutin-providers/tests/column_storage.rs`; this is the engine
/// reaching one of them through the scheduler's drop.
#[test]
fn a_dropped_scheduler_frees_its_column_at_the_alignment_it_reserved() {
    let _serial = COUNTING_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    LAST_ALLOC_ALIGN.store(0, Ordering::SeqCst);
    LAST_DEALLOC_ALIGN.store(0, Ordering::SeqCst);
    DEALLOCS.store(0, Ordering::SeqCst);

    let reserved_at = {
        let provider = CountingProvider {
            inner: stable_provider(256, 0),
        };
        let scheduler = Scheduler::builder()
            .with(Resource::new(Ra(7)))
            .build(store(provider), USize(0))
            .unwrap_or_else(|_| panic!("build should succeed"));
        let reserved_at = LAST_ALLOC_ALIGN.load(Ordering::SeqCst);
        assert_eq!(
            reserved_at, 64,
            "the build reserves the resource's column at cache-line alignment"
        );
        assert_eq!(
            DEALLOCS.load(Ordering::SeqCst),
            0,
            "nothing is freed while the scheduler lives"
        );
        drop(scheduler);
        reserved_at
    };

    assert_eq!(
        DEALLOCS.load(Ordering::SeqCst),
        1,
        "the drop frees the one column"
    );
    assert_eq!(
        LAST_DEALLOC_ALIGN.load(Ordering::SeqCst),
        reserved_at,
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
    let provider = stable_provider(256, 0);
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
