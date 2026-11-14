//! The two halves the shipped provider does not separate. The record's storage atom
//! reserves address space and commits pages as a column grows (loimu storage
//! research `:19-21`, core design topic `:683-685`); the api's provider contract
//! has `protect` (`hilavitkutin-api/src/platform.rs:49-55`), and the os tier's
//! `protect` is an empty body under a FIXME (`hilavitkutin/src/platform/os.rs:80-83`).

use arvo::USize;
use hilavitkutin::OsMemoryProvider;
use hilavitkutin_api::MemoryProviderApi;
use s4_growth::{overcommit_mode, rss_kib};

const GROW: usize = 65_536; // records committed per pass

/// Reserve with no access and no commit charge, then commit each pass's growth
/// with `mprotect`. This is what `protect` would do if the os tier's body did it.
#[test]
fn a_reservation_committed_as_it_grows_keeps_its_base_and_its_records() {
    let capacity_records: usize = 1 << 32;
    let bytes = capacity_records * 8;
    let base = unsafe {
        libc::mmap(
            core::ptr::null_mut(),
            bytes,
            libc::PROT_NONE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
            -1,
            0,
        )
    };
    eprintln!("overcommit_memory = {}; PROT_NONE reserve of {} GiB: {}", overcommit_mode(), bytes >> 30, if base == libc::MAP_FAILED { "refused" } else { "granted" });
    assert_ne!(base, libc::MAP_FAILED);
    let base = base as *mut i64;
    let rss0 = rss_kib();
    let mut live = 0usize;
    for _pass in 0..16 {
        let next = live + GROW;
        let r = unsafe { libc::mprotect(base.add(live) as *mut libc::c_void, GROW * 8, libc::PROT_READ | libc::PROT_WRITE) };
        assert_eq!(r, 0, "commit of the next chunk");
        for i in live..next {
            unsafe { base.add(i).write(i as i64) };
        }
        live = next;
    }
    let rss1 = rss_kib();
    eprintln!("rss before growth {rss0} KiB, after {live} records {rss1} KiB, touched {} KiB", live * 8 / 1024);
    for i in [0usize, 1, GROW, live - 1] {
        assert_eq!(unsafe { base.add(i).read() }, i as i64, "record {i} after growth");
    }
    unsafe { libc::munmap(base as *mut libc::c_void, bytes) };
}

/// The shipped provider with a capacity under physical memory: granted, and only
/// what is touched becomes resident. So the capacity-plus-live-count arm works on
/// this tier today up to roughly physical memory under heuristic overcommit.
#[test]
fn the_shipped_provider_grants_a_capacity_under_physical_memory_and_touch_decides_residency() {
    let mem = OsMemoryProvider::new();
    let bytes: usize = 4 << 30;
    let rss0 = rss_kib();
    let base = unsafe { mem.allocate(USize(bytes), USize(64)) } as *mut i64;
    assert!(!base.is_null(), "4 GiB refused");
    let rss1 = rss_kib();
    let live = 16 * GROW;
    for i in 0..live {
        unsafe { base.add(i).write(i as i64) };
    }
    let rss2 = rss_kib();
    eprintln!("4 GiB capacity: rss {rss0} -> {rss1} after reserve -> {rss2} KiB after {live} records ({} KiB touched)", live * 8 / 1024);
    assert!(rss1 - rss0 < 1024);
    assert!(rss2 - rss1 < (live * 8 / 1024) as u64 + 4096);
    unsafe { mem.deallocate(base as *mut u8, USize(bytes), USize(64)) };
}
