//! Growth that keeps records and addresses, and the constraint each shape
//! puts on the morsel window.

use b3_paged_column::{Paged, Reserved, straddling_morsels};

#[test]
fn a_paged_column_grows_without_moving_a_record() {
    let mut c: Paged<u64, 4096> = Paged::new();
    for i in 0 .. 10 {
        c.push(1000 + i);
    }
    let held = c.ptr(5);
    for i in 10 .. 1_000_000 {
        c.push(1000 + i);
    }
    assert_eq!(c.len(), 1_000_000);
    assert_eq!(c.ptr(5), held, "record 5 did not move");
    assert_eq!(unsafe { held.read() }, 1005);
    for i in (0 .. 1_000_000).step_by(9973) {
        assert_eq!(c.get(i), 1000 + i as u64);
    }
}

#[test]
fn a_reserved_column_grows_without_moving_its_base() {
    let mut c: Reserved<u64> = Reserved::reserve(64 << 20); // room for 64 Mi records, 512 MiB
    c.grow_to(1000);
    for i in 0 .. 1000 {
        c.write(i, 7 * i as u64);
    }
    let base = c.base();
    let committed_small = c.committed_bytes();
    c.grow_to(10_000_000);
    for i in 1000 .. 10_000_000 {
        c.write(i, 7 * i as u64);
    }
    assert_eq!(c.base(), base, "the base did not move");
    for i in (0 .. 10_000_000).step_by(99_991) {
        assert_eq!(c.read(i), 7 * i as u64);
    }
    println!(
        "reserved 512 MiB; committed {} bytes at 1000 records, {} bytes at 10 M records",
        committed_small,
        c.committed_bytes()
    );
}

/// A paged column only keeps the hot loop contiguous if no morsel crosses a
/// page. The trunk's window for a fiber writing one 4-byte column is
/// 24576 / 4 = 6144 records (hilavitkutin/tests/a1_fiber_morsel_size.rs:123-127),
/// not a power of two, so over power-of-two pages some morsels straddle.
/// Expected red: the case that says the window rule and a paged layout
/// do not compose as they stand.
#[test]
fn the_trunk_window_over_power_of_two_pages_never_straddles() {
    let n = 1 << 20;
    for p in [4096usize, 8192, 16384] {
        let s = straddling_morsels(n, 6144, p);
        println!(
            "window 6144 over {p}-record pages: {s} of {} morsels straddle",
            n.div_ceil(6144)
        );
    }
    assert_eq!(
        straddling_morsels(n, 6144, 8192),
        0,
        "morsels of 6144 records straddle 8192-record pages"
    );
}

/// Control: a window that divides the page never straddles.
#[test]
fn a_window_dividing_the_page_never_straddles() {
    assert_eq!(straddling_morsels(1 << 20, 4096, 8192), 0);
    assert_eq!(straddling_morsels(1 << 20, 2048, 8192), 0);
}
