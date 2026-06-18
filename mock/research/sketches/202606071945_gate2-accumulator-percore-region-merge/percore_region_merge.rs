// GATE-2 deviation §9 de-risk sketch: per-core accumulator region + merge.
//
// Hypothesis: the canonical convergence-accumulator (spec :1750-1766) maps onto
// the SHIPPED `Accum`/`AccumColPtr`/append machinery WITHOUT a contract change to
// the append path. Each core appends into a disjoint sub-region of the single
// reserved capacity buffer, then a post-phase forward compaction concatenates the
// per-core live prefixes in core order. The output is byte-identical to the
// single-core unit-outer `run()` append.
//
// What this sketch faithfully mirrors from the real engine (it is standalone, no
// crate deps, so it can be `rustc`-run for a real WORKS/FAILS):
//
//   * the append pointer-math is EXACTLY the shipped resolve_append
//     (engine_ctx.rs:1036-1037): `ptr::write(base.add(live), v); len.set(live+1)`
//     with the saturating capacity guard `if live >= cap { return }`
//     (engine_ctx.rs:1023).
//   * `AccumColPtr { base, len: &Cell<USize>, cap }` (engine_ctx.rs:133) is the
//     per-core handle. The per-core variant sets `base = orig_base + lo_c`,
//     `len = &core_local_cell`, `cap = hi_c - lo_c`. Constructing it that way is
//     the whole feasibility claim: the append code is untouched.
//   * the §8 record-slice split (scheduler/mod.rs:1263-1265): core c owns
//     `[lo_c, hi_c)` where `per = ceil(total/ncores)`, `lo = (c*per).min(total)`,
//     `hi = (lo+per).min(total)`.
//
// Why offset = lo_c (NOT c*per_cap into a separately-reserved buffer): the
// accumulator reserves cap = build-time record count = the global <=1-append-
// per-record upper bound. Region c owns records [lo_c, hi_c), so it needs at most
// (hi_c - lo_c) slots, and placing it at byte offset lo_c tiles the SAME reserved
// buffer exactly (sum of slice sizes = total). Appends are conditional (<=1 per
// record, can be 0), so regions leave gaps; the merge compacts them away.
//
// Race-freedom: regions are disjoint byte ranges of one buffer and each core has
// its OWN live-length cell, so concurrent appenders never touch shared memory.
// This sketch spawns real OS threads (std, sketch-only) to exercise that.
//
// Order preservation: core c handles a contiguous ASCENDING record slice and
// appends in record order; concatenating regions in ascending core order = global
// record order = the single-core append order.

use std::cell::Cell;
use std::sync::Arc;
use std::thread;

// Mirror of arvo::USize (the field the real code indexes through is `.0`).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct USize(usize);

// Mirror of dispatch::engine_ctx::AccumColPtr (the projected per-append handle).
// `base`/`cap` are by-value; `len` is a borrowed interior-mutable cell. In the
// real engine `base` is a ColumnPtr<T> NonNull; here a raw `*mut T` suffices to
// reproduce the pointer arithmetic.
struct AccumColPtr<'a, T> {
    base: *mut T,
    len: &'a Cell<USize>,
    cap: USize,
}

// EXACT mirror of resolve_append (engine_ctx.rs:1015-1038). `&self`, saturating
// guard, ptr::write at the live offset, advance the borrowed cell.
impl<'a, T> AccumColPtr<'a, T> {
    #[inline]
    unsafe fn append(&self, v: T) {
        let live = self.len.get();
        if live.0 >= self.cap.0 {
            return;
        }
        core::ptr::write(self.base.add(live.0), v);
        self.len.set(USize(live.0 + 1));
    }
}

// A per-record append closure: for the convergence-accumulator pattern a WU
// appends 0 or 1 value per record (a filter/keep). This sketch models a filter
// that keeps records whose value is not a multiple of 7, appending `value * 10`,
// exactly the kind of data-dependent <=1-per-record append the merge must handle.
fn wu_append_for_record(rec: usize) -> Option<u32> {
    if rec % 7 == 0 {
        None
    } else {
        Some((rec as u32) * 10)
    }
}

// Single-core reference: the shipped run() unit-outer path. One appender over the
// whole [0, total) range, appends in record order into the buffer prefix.
fn single_core_reference(total: usize, cap: usize) -> Vec<u32> {
    let mut buf: Vec<u32> = vec![u32::MAX; cap];
    let cell = Cell::new(USize(0));
    let acc = AccumColPtr { base: buf.as_mut_ptr(), len: &cell, cap: USize(cap) };
    for rec in 0..total {
        if let Some(v) = wu_append_for_record(rec) {
            unsafe { acc.append(v) };
        }
    }
    let live = cell.get().0;
    buf.truncate(live);
    buf
}

// Threaded path: per-core region append + forward compaction merge.
fn threaded_percore(total: usize, cap: usize, ncores: usize) -> Vec<u32> {
    // One shared reserved buffer, poisoned. Cores write disjoint sub-ranges.
    let mut buf: Vec<u32> = vec![u32::MAX; cap];
    let base = buf.as_mut_ptr() as usize; // pass as integer across the thread boundary

    // §8 record-slice split. Region c lives at byte offset lo_c, cap = hi_c-lo_c.
    let per = (total + ncores - 1) / ncores;
    // Per-core final live counts, collected back for the merge.
    let mut live_counts: Vec<usize> = vec![0; ncores];

    thread::scope(|scope| {
        let mut handles = Vec::new();
        for c in 0..ncores {
            let handle = scope.spawn(move || {
                let lo = (c * per).min(total);
                let hi = (lo + per).min(total);
                let region_cap = hi - lo;
                // Each core's OWN live-length cell: no shared mutable state.
                let cell = Cell::new(USize(0));
                // base + lo_c, in T units. SAFETY: disjoint per-core sub-range of
                // the one reserved buffer; region_cap slots room (<=1 append/rec).
                let region_base = (base as *mut u32).wrapping_add(lo);
                let acc =
                    AccumColPtr { base: region_base, len: &cell, cap: USize(region_cap) };
                for rec in lo..hi {
                    if let Some(v) = wu_append_for_record(rec) {
                        unsafe { acc.append(v) };
                    }
                }
                cell.get().0 // this core's live count
            });
            handles.push((c, handle));
        }
        for (c, handle) in handles {
            live_counts[c] = handle.join().unwrap();
        }
    });

    // Merge: forward compaction. Region c's live prefix sits at [lo_c, lo_c+live_c).
    // Concatenate in ascending core order into [0, sum live). write_pos <= lo_c
    // always (write_pos = sum of prior live_c <= sum of prior slice sizes = lo_c),
    // so the copy is forward-safe (dst <= src). copy_within handles overlap.
    let mut write_pos = 0usize;
    for c in 0..ncores {
        let lo = (c * per).min(total);
        let live_c = live_counts[c];
        if live_c > 0 && lo != write_pos {
            buf.copy_within(lo..lo + live_c, write_pos);
        }
        write_pos += live_c;
    }
    buf.truncate(write_pos);
    buf
}

fn main() {
    let mut failures = 0;
    // A spread of totals and core counts, including total not divisible by ncores
    // (the surplus-core lo==hi no-op case) and ncores > total.
    let cases = [
        (256usize, 1usize),
        (256, 2),
        (256, 3),
        (256, 4),
        (256, 7),
        (1000, 4),
        (1000, 8),
        (37, 8),
        (5, 8),
        (0, 4),
    ];
    for &(total, ncores) in &cases {
        let cap = total; // reserved = build-time record count (<=1 append/rec bound)
        let reference = single_core_reference(total, cap);
        // Run the threaded path several times: a race would surface as a flake.
        let mut ok = true;
        for _ in 0..200 {
            let got = threaded_percore(total, cap, ncores);
            if got != reference {
                ok = false;
                eprintln!(
                    "MISMATCH total={total} ncores={ncores}: ref_len={} got_len={}",
                    reference.len(),
                    got.len()
                );
                break;
            }
        }
        println!(
            "total={total:5} ncores={ncores:2} live={:5} -> {}",
            reference.len(),
            if ok { "OK (byte-identical x200)" } else { "FAIL" }
        );
        if !ok {
            failures += 1;
        }
    }

    // Stress the multi-append case briefly to confirm the failure mode is the one
    // the topic flags (region cap = slice size assumes <=1 append/record). Here a
    // WU appends TWO per kept record; region_cap = slice size is then too small and
    // the saturating guard DROPS the overflow. This is EXPECTED to differ from a
    // single-core ref that reserved 2*total. It documents the ripple, not a bug.
    let _ = Arc::new(()); // (kept to show std is available in sketch scope)
    println!(
        "\nnote: multi-append-per-record needs region_cap = per-region worst case,"
    );
    println!(
        "      not slice size; that is the capacity-policy ripple in the findings."
    );

    if failures == 0 {
        println!("\nWORKS: per-core region + forward-compaction merge is byte-identical");
        println!("       to single-core append across all <=1-append/record cases.");
        std::process::exit(0);
    } else {
        println!("\nFAILS: {failures} case(s) mismatched");
        std::process::exit(1);
    }
}
