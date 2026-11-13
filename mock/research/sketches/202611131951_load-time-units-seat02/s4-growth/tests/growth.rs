//! Growth against the trunk at `271909e5`.
//!
//! The api contract says of `ColumnStorage::reserve`: "Re-reserving an existing
//! `id` resizes it" (`hilavitkutin-api/src/storage.rs:46-47`). The founding record
//! names the storage atom "Contiguous, fixed-stride, zero-copy-growth column"
//! (api surface topic `:663`), mmap-backed with reserve and commit (core design
//! topic `:683-685`, loimu storage research `:19-21`).

use arvo::USize;
use hilavitkutin::OsMemoryProvider;
use hilavitkutin_api::{ColumnStorage, MemoryProviderApi, StoreId};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;
use s4_growth::{overcommit_mode, rss_kib};

#[test]
fn re_reserving_a_column_keeps_the_records_it_held() {
    let mut cs: ArenaColumnStorage<OsMemoryProvider> = ArenaColumnStorage::new(OsMemoryProvider::new());
    let id = StoreId(USize(0));
    assert!(matches!(cs.reserve::<i64>(id, USize(4)), Outcome::Ok(())));
    let before = unsafe { cs.column_ptr_mut::<i64>(id) };
    for i in 0..4 {
        unsafe { before.add(i).write(100 + i as i64) };
    }
    assert!(matches!(cs.reserve::<i64>(id, USize(8)), Outcome::Ok(())));
    let after = unsafe { cs.column_ptr::<i64>(id) };
    let held: Vec<i64> = (0..4).map(|i| unsafe { after.add(i).read() }).collect();
    eprintln!("base moved: {}; first four after the resize: {held:?}; count now {}", before as usize != after as usize, cs.count(id).0);
    assert_eq!(held, vec![100, 101, 102, 103], "the resize dropped what the column held");
}

/// The arm where an extent is a capacity fixed at build and a live count moves
/// under it: on the os tier the capacity is address space, and only what is
/// touched is memory. Base pointer stable by construction, nothing copied.
#[test]
fn a_capacity_far_above_the_live_count_costs_address_space_not_memory() {
    let mode = overcommit_mode();
    let mem = OsMemoryProvider::new();
    let capacity_records: usize = 1 << 32; // four billion i64 records, 32 GiB of address space
    let bytes = capacity_records * 8;
    let rss0 = rss_kib();
    let base = unsafe { mem.allocate(USize(bytes), USize(64)) } as *mut i64;
    eprintln!("overcommit_memory = {mode}; reserve of {} GiB: {}", bytes >> 30, if base.is_null() { "refused" } else { "granted" });
    assert!(!base.is_null(), "the provider refused the reservation (overcommit mode {mode})");
    let rss1 = rss_kib();

    // Grow the live count to one million records, pass by pass.
    let mut live = 0usize;
    for _pass in 0..16 {
        let next = live + 65_536;
        for i in live..next {
            unsafe { base.add(i).write(i as i64) };
        }
        live = next;
    }
    let rss2 = rss_kib();
    let touched_kib = (live * 8 / 1024) as u64;
    eprintln!("rss before {rss0} KiB, after reserve {rss1} KiB, after {live} live records {rss2} KiB (touched {touched_kib} KiB)");
    assert!(rss1 - rss0 < 1024, "the reservation itself became resident");
    assert!(rss2 - rss1 < touched_kib + 4096, "more became resident than was touched");
    assert_eq!(unsafe { base.add(live - 1).read() }, (live - 1) as i64);
    unsafe { mem.deallocate(base as *mut u8, USize(bytes), USize(64)) };
}
