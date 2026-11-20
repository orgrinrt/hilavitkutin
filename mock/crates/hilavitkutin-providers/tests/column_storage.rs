//! `ArenaColumnStorage` behavioral conformance.
//!
//! Reserve a column, write through the raw mutable base, read the same
//! records back through the const base, and confirm the count. A second
//! column of a different scalar width proves slot isolation.
//!
//! The test provider is the crate's `StaticBumpMemory` over a leaked buffer,
//! whose `deallocate` does nothing, or `StaticFreeListMemory` where a test
//! needs the block back. The tests on what a free is given wrap the bump
//! provider in one that records every block handed out and handed back.

use arvo::{Bool, USize};
use hilavitkutin_api::{ColumnStorage, MemoryProviderApi, StoreId};
use hilavitkutin_providers::{
    ArenaColumnStorage,
    StaticBumpMemory,
    StaticFreeListMemory,
    StorageError,
};
use notko::Outcome;

/// The crate's static bump provider over a fresh leaked buffer of `bytes`.
fn bump(bytes: usize) -> StaticBumpMemory {
    StaticBumpMemory::new(std::vec![core::mem::MaybeUninit::uninit(); bytes].leak())
}

/// The crate's freeing provider over a fresh leaked buffer of `bytes`.
fn free_list(bytes: usize) -> StaticFreeListMemory {
    StaticFreeListMemory::new(std::vec![core::mem::MaybeUninit::uninit(); bytes].leak())
}

#[test]
fn reserve_write_read_roundtrips_and_counts() {
    let provider = bump(1 << 16);
    let mut store: ArenaColumnStorage<StaticBumpMemory> = ArenaColumnStorage::new(provider);

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
    let provider = bump(1 << 16);
    let mut store: ArenaColumnStorage<StaticBumpMemory> = ArenaColumnStorage::new(provider);

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
    let provider = bump(1 << 16);
    let mut store: ArenaColumnStorage<StaticBumpMemory> = ArenaColumnStorage::new(provider);

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
    inner:  StaticBumpMemory,
    ledger: &'static Ledger,
}

// SAFETY: every block comes from `inner` and goes back to it unchanged, so the
// promises `StaticBumpMemory` keeps are kept here; the ledger is behind a lock.
unsafe impl MemoryProviderApi for RecordingProvider {
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
        inner: bump(1 << 16),
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
            inner: bump(1 << 16),
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

/// A release hands the block back as it was reserved, once: the slot is empty
/// after it, so a second release and the drop free nothing more.
#[test]
fn a_release_frees_the_block_as_it_was_reserved_and_only_once() {
    let ledger: &'static Ledger = Box::leak(Box::default());
    {
        let provider = RecordingProvider {
            inner: bump(1 << 16),
            ledger,
        };
        let mut store: ArenaColumnStorage<RecordingProvider> = ArenaColumnStorage::new(provider);
        let id = StoreId(USize(3));
        assert!(matches!(
            store.reserve::<u64>(id, USize(8)),
            Outcome::Ok(())
        ));
        store.release(id);
        assert_eq!(
            ledger.freed(),
            ledger.reserved(),
            "the release hands the block back as it was given out"
        );
        store.release(id);
        assert_eq!(ledger.freed().len(), 1, "a second release frees nothing");
    }
    assert_eq!(
        ledger.freed().len(),
        1,
        "the drop does not free a released block again"
    );
}

/// A released column reads as unreserved: a count of zero and null bases, and
/// the other columns keep their records.
#[test]
fn a_released_column_reads_unreserved_and_the_others_hold() {
    let mut store = ArenaColumnStorage::<_>::new(free_list(1 << 16));
    let gone = StoreId(USize(0));
    let kept = StoreId(USize(1));
    assert!(matches!(
        store.reserve::<u32>(gone, USize(16)),
        Outcome::Ok(())
    ));
    assert!(matches!(
        store.reserve::<u32>(kept, USize(4)),
        Outcome::Ok(())
    ));
    // SAFETY: `kept` is reserved for 4 records of u32.
    let base = unsafe { store.column_ptr_mut::<u32>(kept) };
    for i in 0 .. 4 {
        // SAFETY: i < 4.
        unsafe { base.add(i).write(i as u32 + 7) };
    }
    store.release(gone);
    assert_eq!(store.count(gone), USize(0));
    // SAFETY: the pointers are read as values and never dereferenced.
    assert!(unsafe { store.column_ptr::<u32>(gone) }.is_null());
    assert!(unsafe { store.column_ptr_mut::<u32>(gone) }.is_null());
    assert_eq!(store.count(kept), USize(4));
    for i in 0 .. 4 {
        // SAFETY: `kept` is still reserved for 4 records of u32.
        assert_eq!(
            unsafe { store.column_ptr::<u32>(kept).add(i).read() },
            i as u32 + 7
        );
    }
}

/// A release of an id with no block, or of one past the cap, changes nothing.
#[test]
fn releasing_an_unreserved_or_out_of_range_id_changes_nothing() {
    let ledger: &'static Ledger = Box::leak(Box::default());
    let provider = RecordingProvider {
        inner: bump(1 << 16),
        ledger,
    };
    let mut store: ArenaColumnStorage<RecordingProvider> = ArenaColumnStorage::new(provider);
    let id = StoreId(USize(0));
    assert!(matches!(
        store.reserve::<u32>(id, USize(4)),
        Outcome::Ok(())
    ));
    store.release(StoreId(USize(5)));
    store.release(StoreId(USize(1 << 20)));
    assert!(ledger.freed().is_empty());
    assert_eq!(store.count(id), USize(4));
}

/// Reserve, then release or reserve again, many cycles over a buffer that
/// holds two columns' worth: the case of ticket
/// `arenacolumnstorage_release_is_a_no_op_so_a_store_under_a_bum`. It holds
/// under a provider that takes blocks back, and a provider that does not runs
/// out, which shows the cycle count reaches past the buffer.
fn cycles_without_running_out<M: MemoryProviderApi>(provider: M, by_release: bool) -> bool {
    let mut store = ArenaColumnStorage::<M>::new(provider);
    let id = StoreId(USize(0));
    for _ in 0 .. 64 {
        if !matches!(store.reserve::<u32>(id, USize(256)), Outcome::Ok(())) {
            return false;
        }
        if by_release {
            store.release(id);
        }
    }
    true
}

#[test]
fn a_store_reserved_again_every_cycle_does_not_run_out() {
    // 1 KiB a reservation, a buffer of a little over two.
    let bytes = 2 * 1024 + 128;
    assert!(
        cycles_without_running_out(free_list(bytes), true),
        "release then reserve"
    );
    assert!(
        cycles_without_running_out(free_list(bytes), false),
        "reserve again over a live block"
    );
    assert!(
        !cycles_without_running_out(bump(bytes), true),
        "a provider that takes nothing back runs out"
    );
}

/// A release gives the bytes back: what the provider has handed out returns to
/// what it was before the reserve.
#[test]
fn a_release_gives_the_bytes_back_to_the_provider() {
    let provider: &'static StaticFreeListMemory = Box::leak(Box::new(free_list(1 << 16)));
    let mut store = ArenaColumnStorage::<Shared>::new(Shared(provider));
    let before = provider.used();
    assert!(matches!(
        store.reserve::<u64>(StoreId(USize(2)), USize(100)),
        Outcome::Ok(())
    ));
    assert!(provider.used().0 >= before.0 + 800);
    store.release(StoreId(USize(2)));
    assert_eq!(provider.used(), before);
}

/// A freeing provider the test keeps a reference to while the store owns the
/// forwarding value.
struct Shared(&'static StaticFreeListMemory);

// SAFETY: every call is forwarded unchanged to a provider that keeps the
// promises, and the provider is leaked, so it outlives every block.
unsafe impl MemoryProviderApi for Shared {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        // SAFETY: forwarded as received.
        unsafe { self.0.allocate(len, align) }
    }

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize) {
        // SAFETY: forwarded as received.
        unsafe { self.0.deallocate(ptr, len, align) }
    }

    unsafe fn protect(&self, ptr: *mut u8, len: USize, read: Bool, write: Bool) {
        // SAFETY: forwarded as received.
        unsafe { self.0.protect(ptr, len, read, write) }
    }
}
