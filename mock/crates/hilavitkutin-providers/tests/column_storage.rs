//! `ArenaColumnStorage` behavioral conformance.
//!
//! Reserve a column, write through the raw mutable base, read the same
//! records back through the const base, and confirm the count. A second
//! column of a different scalar width proves slot isolation.
//!
//! The test provider is a bump arena with a no-op `deallocate`: it
//! honors 64-byte alignment on `allocate` and never frees per block, so
//! `ArenaColumnStorage::Drop` (which calls `deallocate`) has nothing to
//! release. The two tests on what a free is given wrap it in a provider that
//! records every block handed out and handed back.

use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin_api::{ColumnStorage, MemoryProviderApi, StoreId};
use hilavitkutin_providers::{ArenaColumnStorage, StorageError};
use notko::Outcome;

/// Bump-arena `MemoryProvider` for tests: a leaked heap buffer, an
/// atomic cursor, alignment-respecting `allocate`, no-op `deallocate`.
struct BumpProvider {
    base:   *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test allocator ABI raw pointer; tracked: #72
    size:   USize,
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
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
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

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: bump arena does not free per block; tracked: #72

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
        for i in 0 .. 5usize {
            base.add(i).write((i as u32) * 7);
        }
        let read = store.column_ptr::<u32>(id);
        for i in 0 .. 5usize {
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
        for i in 0 .. 4usize {
            pa.add(i).write(1000 + i as u32);
            pb.add(i).write(i as u16);
        }
        let ra = store.column_ptr::<u32>(a);
        let rb = store.column_ptr::<u16>(b);
        for i in 0 .. 4usize {
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
    assert!(matches!(
        outcome,
        Outcome::Err(StorageError::LengthOverflow)
    ));
    // Nothing was reserved.
    assert_eq!(store.count(id), USize(0));
}

/// One block as the provider saw it, at `allocate` or at `deallocate`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Block {
    addr:  usize,
    len:   usize,
    align: usize,
}

/// What a [`RecordingProvider`] was asked for, in order.
#[derive(Default)]
struct Ledger {
    reserved: std::sync::Mutex<Vec<Block>>,
    freed:    std::sync::Mutex<Vec<Block>>,
}

impl Ledger {
    fn reserved(&self) -> Vec<Block> {
        self.reserved
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    fn freed(&self) -> Vec<Block> {
        self.freed.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }
}

/// A bump arena that writes every block it hands out and every block it is
/// handed back into a ledger the test keeps. The ledger is per test, so the
/// tests using it share nothing and need no serialising. It is leaked, as the
/// arena's buffer is, because a provider outlives any borrow.
struct RecordingProvider {
    inner:  BumpProvider,
    ledger: &'static Ledger,
}

impl MemoryProviderApi for RecordingProvider {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        // SAFETY: forwarded as received.
        let ptr = unsafe { self.inner.allocate(len, align) };
        if !ptr.is_null() {
            self.ledger
                .reserved
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(Block {
                    addr:  ptr as usize,
                    len:   *len,
                    align: *align,
                });
        }
        ptr
    }

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize) {
        self.ledger
            .freed
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(Block {
                addr:  ptr as usize,
                len:   *len,
                align: *align,
            });
    }

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

/// A provider that frees by layout needs the block back exactly as it gave
/// it out: same address, same length, same alignment. A column is reserved
/// at cache-line alignment, so a free that guesses a word is a layout
/// mismatch, and a bump arena never notices. Reserving one id a second time
/// frees the first block, and this reads what that free was given.
#[test]
fn a_second_reserve_frees_the_first_block_as_it_was_reserved() {
    let ledger: &'static Ledger = Box::leak(Box::default());
    let provider = RecordingProvider {
        inner: BumpProvider::with_bytes(USize(1 << 16)),
        ledger,
    };
    let mut store: ArenaColumnStorage<RecordingProvider> = ArenaColumnStorage::new(provider);

    let id = StoreId(USize(0));
    assert!(matches!(
        store.reserve::<u64>(id, USize(8)),
        Outcome::Ok(())
    ));
    let first = ledger.reserved();
    assert_eq!(first.len(), 1, "the first reserve takes one block");
    assert_eq!(
        first[0].align, 64,
        "a column is reserved at cache-line alignment"
    );
    assert!(ledger.freed().is_empty(), "nothing is freed yet");

    assert!(matches!(
        store.reserve::<u64>(id, USize(16)),
        Outcome::Ok(())
    ));
    assert_eq!(
        ledger.freed(),
        first,
        "the second reserve hands the first block back as it was given out"
    );

    // Keep the store alive to here so its drop is not what freed the block.
    assert_eq!(store.count(id), USize(16));
}

/// The same property at the other site that frees: dropping the store
/// hands back every block it still holds, each as it was reserved.
#[test]
fn drop_frees_every_live_block_as_it_was_reserved() {
    let ledger: &'static Ledger = Box::leak(Box::default());
    {
        let provider = RecordingProvider {
            inner: BumpProvider::with_bytes(USize(1 << 16)),
            ledger,
        };
        let mut store: ArenaColumnStorage<RecordingProvider> = ArenaColumnStorage::new(provider);
        assert!(matches!(
            store.reserve::<u32>(StoreId(USize(0)), USize(4)),
            Outcome::Ok(())
        ));
        assert!(matches!(
            store.reserve::<u64>(StoreId(USize(1)), USize(32)),
            Outcome::Ok(())
        ));
        assert!(
            ledger.freed().is_empty(),
            "nothing is freed before the drop"
        );
    }
    let reserved = ledger.reserved();
    assert_eq!(reserved.len(), 2, "two columns, two blocks");
    assert!(
        reserved.iter().all(|b| b.align == 64),
        "every column is reserved at cache-line alignment: {reserved:?}"
    );
    assert_eq!(
        ledger.freed(),
        reserved,
        "the drop hands back every block as it was given out, in slot order"
    );
}
