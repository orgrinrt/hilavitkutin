//! `ArenaColumnStorage` behavioral conformance.
//!
//! Reserve a column, write through the raw mutable base, read the same
//! records back through the const base, and confirm the count. A second
//! column of a different scalar width proves slot isolation.
//!
//! The test provider is a bump arena with a no-op `deallocate`: it
//! honors 64-byte alignment on `allocate` and never frees per block, so
//! `ArenaColumnStorage::Drop` (which calls `deallocate`) has nothing to
//! release.
//!
//! An earlier version of this note explained that `StdMemoryProvider`
//! was avoided here because its `deallocate` rebuilt the `Layout` with
//! word alignment and so mismatched a 64-byte block. That was true, and
//! it meant the suite stayed green by routing around the failing path
//! rather than reporting it. `deallocate` now carries the alignment it
//! was allocated with, so the mismatch is gone and the avoidance no
//! longer has a reason.

use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin_api::{ColumnStorage, MemoryProviderApi, StoreId};
use hilavitkutin_providers::{ArenaColumnStorage, StorageError};
use notko::Outcome;

/// Bump-arena `MemoryProvider` for tests: a leaked heap buffer, an
/// atomic cursor, alignment-respecting `allocate`, no-op `deallocate`.
struct BumpProvider {
    base: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test allocator ABI raw pointer; tracked: #72
    size: USize,
    cursor: AtomicUsize,
}

// SAFETY: the test drives the provider single-threaded; the raw base
// pointer is into a leaked buffer that outlives every use.
unsafe impl Send for BumpProvider {}
unsafe impl Sync for BumpProvider {}

impl BumpProvider {
    fn with_bytes(size: USize) -> Self {
        let buf = vec![0u8; *size].into_boxed_slice();
        let base = Box::leak(buf).as_mut_ptr();
        Self {
            base,
            size,
            cursor: AtomicUsize::new(0),
        }
    }
}

impl MemoryProviderApi for BumpProvider {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        let len = *len;
        let align = (*align).max(1);
        let cur = self.cursor.load(Ordering::Relaxed);
        let aligned = (cur + align - 1) & !(align - 1);
        let end = aligned + len;
        if end > *self.size {
            return core::ptr::null_mut();
        }
        self.cursor.store(end, Ordering::Relaxed);
        // SAFETY: `aligned` stays within the leaked buffer (end <= size).
        unsafe { self.base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {} // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bump arena does not free per block; tracked: #72

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {} // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test allocator ABI raw pointer; tracked: #72
}

#[test]
fn reserve_write_read_roundtrips_and_counts() {
    let provider = BumpProvider::with_bytes(USize(1 << 16));
    let mut store: ArenaColumnStorage<BumpProvider> = ArenaColumnStorage::new(provider);

    let id = StoreId(USize(0));
    let count = USize(5);
    assert!(matches!(store.reserve::<u32>(id, count), Outcome::Ok(())));
    assert_eq!(store.count(id), count);

    // SAFETY: id names a column reserved for u32 with count records.
    unsafe {
        let base = store.column_ptr_mut::<u32>(id);
        for i in 0..5usize {
            base.add(i).write((i as u32) * 7);
        }
        let read = store.column_ptr::<u32>(id);
        for i in 0..5usize {
            assert_eq!(read.add(i).read(), (i as u32) * 7);
        }
    }
}

#[test]
fn two_columns_are_isolated() {
    let provider = BumpProvider::with_bytes(USize(1 << 16));
    let mut store: ArenaColumnStorage<BumpProvider> = ArenaColumnStorage::new(provider);

    let a = StoreId(USize(0));
    let b = StoreId(USize(1));
    assert!(matches!(store.reserve::<u32>(a, USize(4)), Outcome::Ok(())));
    assert!(matches!(store.reserve::<u16>(b, USize(4)), Outcome::Ok(())));

    // SAFETY: a and b name distinct columns reserved for u32 and u16.
    unsafe {
        let pa = store.column_ptr_mut::<u32>(a);
        let pb = store.column_ptr_mut::<u16>(b);
        for i in 0..4usize {
            pa.add(i).write(1000 + i as u32);
            pb.add(i).write(i as u16);
        }
        let ra = store.column_ptr::<u32>(a);
        let rb = store.column_ptr::<u16>(b);
        for i in 0..4usize {
            assert_eq!(ra.add(i).read(), 1000 + i as u32);
            assert_eq!(rb.add(i).read(), i as u16);
        }
    }
    assert_eq!(store.count(a), USize(4));
    assert_eq!(store.count(b), USize(4));
}

#[test]
fn reserve_rejects_byte_length_overflow() {
    let provider = BumpProvider::with_bytes(USize(1 << 16));
    let mut store: ArenaColumnStorage<BumpProvider> = ArenaColumnStorage::new(provider);

    // usize::MAX records of u32 is a byte product of usize::MAX * 4,
    // which overflows usize. The guard must fire before allocation,
    // returning LengthOverflow rather than wrapping to a small length
    // and handing back an undersized block.
    let id = StoreId(USize(0));
    let outcome = store.reserve::<u32>(id, USize(usize::MAX));
    assert!(matches!(outcome, Outcome::Err(StorageError::LengthOverflow)));
    // Nothing was reserved.
    assert_eq!(store.count(id), USize(0));
}
