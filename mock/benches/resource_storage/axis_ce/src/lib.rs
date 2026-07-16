//! Axis C (column-count capacity) and axis E (cross-core false-sharing) for the
//! resource-storage-model round 202606210600.
//!
//! These are not latency benches (those run through the mockspace bench harness
//! in `../`). C is a capacity ASSERTION against the real
//! `hilavitkutin_providers::ArenaColumnStorage<_, Dim<256>>`: the per-member-
//! column layouts (V2 decomposed, V5 handle-table) reserve one column per scalar
//! member, so a resource set's distinct-column count is resources x members and
//! crosses the engine's 256-store cap, whereas the per-resource layouts (V0/V1
//! blob, V3 shape-bound, V4 erased) reserve one column per resource and stay
//! well under it. E is a contention measurement with real threads: V3's shared
//! shape-bound column, written by resources owned by different cores, suffers
//! false sharing that the per-resource-line layouts do not.

use std::sync::atomic::{AtomicU64, Ordering};

use arvo::USize;
use arvo_tensor::Dim;
use hilavitkutin_api::{ColumnStorage, MemoryProviderApi, StoreId};
use hilavitkutin_providers::{ArenaColumnStorage, StorageError};

/// A bump `MemoryProvider` over a fixed heap block, for the capacity test.
pub struct TestProvider {
    buf: std::cell::UnsafeCell<Vec<u8>>,
    used: std::cell::Cell<usize>,
}
// SAFETY: the capacity test is single-threaded; the cells are only touched from
// one thread. The trait requires Send + Sync, satisfied trivially here.
unsafe impl Send for TestProvider {}
unsafe impl Sync for TestProvider {}
impl TestProvider {
    pub fn new(bytes: usize) -> Self {
        TestProvider {
            buf: std::cell::UnsafeCell::new(vec![0u8; bytes]),
            used: std::cell::Cell::new(0),
        }
    }
}
impl MemoryProviderApi for TestProvider {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let a = align.0.max(1);
        let used = (self.used.get() + a - 1) / a * a;
        // SAFETY: single-threaded test; buf outlives all returned pointers.
        let buf = unsafe { &mut *self.buf.get() };
        if used + len.0 > buf.len() {
            return core::ptr::null_mut();
        }
        self.used.set(used + len.0);
        buf.as_mut_ptr().add(used)
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _p: *mut u8, _l: USize, _r: arvo::Bool, _w: arvo::Bool) {}
}

/// Reserve `columns` distinct one-record `u32` columns into a fresh
/// `ArenaColumnStorage<_, Dim<256>>`. Returns `Ok(())` if every reserve
/// succeeded, or the `StorageError` of the first that failed. This is the
/// capacity probe: it models a layout that needs `columns` distinct store ids.
pub fn reserve_n_columns(columns: usize) -> Result<(), StorageError> {
    let provider = TestProvider::new(columns * 64 + (1 << 16));
    let mut store: ArenaColumnStorage<TestProvider, Dim<256>> = ArenaColumnStorage::new(provider);
    for id in 0..columns {
        match store.reserve::<u32>(StoreId(USize(id)), USize(1)) {
            notko::Outcome::Ok(()) => {}
            notko::Outcome::Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Distinct-column count for a layout over `resources` resources each with `m`
/// scalar members + a Seq + a Map (3 collection columns folded to 1 here for the
/// count). Per-member layouts (V2, V5) need one column per member per resource;
/// per-resource layouts (V0/V1/V3/V4) need one (blob / shared / erased) column
/// per resource. V3 shares one column across all resources (1 total).
pub fn column_count(layout: &str, resources: usize, m: usize) -> usize {
    match layout {
        // one column per scalar member per resource, plus per-resource seq+map
        "V2_decomposed" | "V5_handletable" => resources * (m + 2),
        // one blob / erased store per resource
        "V0_blob" | "V1_snapshot" | "V4_erased" => resources,
        // all resources share one shape-bound column (+ shared seq/map)
        "V3_shapebound" => 3,
        other => panic!("unknown layout {other}"),
    }
}

/// Axis E: cross-core false sharing. `threads` threads, each owning a distinct
/// resource slot. Packed: every slot is a `u32` inside ONE 64-byte cache line
/// (the V3 shape-bound hazard, several resources' members sharing one column on
/// one line). Padded: each slot on its own 64-byte line (the per-resource-line
/// layouts). Each thread writes only its own slot in a tight loop; the packed
/// layout makes the cores fight over one line. Returns (packed_ns, padded_ns).
pub fn false_sharing(threads: usize, iters_per: u64) -> (u128, u128) {
    let slots = threads.max(2).min(16); // up to 16 u32 fit on one 64-byte line
    let packed = contend(true, slots, iters_per);
    let padded = contend(false, slots, iters_per);
    (packed, padded)
}

fn contend(packed: bool, slots: usize, iters_per: u64) -> u128 {
    let line = 64usize;
    // Packed: one line holds all slots. Padded: one line per slot.
    let region = if packed { line } else { line * slots };
    // 64-byte-aligned backing block.
    let mut backing = vec![0u64; (region + line) / 8 + 8];
    let base = {
        let p = backing.as_mut_ptr() as usize;
        (p + line - 1) / line * line
    };
    let addrs: Vec<usize> = (0..slots)
        .map(|s| if packed { base + s * 4 } else { base + s * line })
        .collect();
    // keep backing alive for the scope
    let _guard = &backing;

    let warmup = 3u64;
    let timed = 15u64;
    let mut samples: Vec<u128> = Vec::new();
    for rep in 0..(warmup + timed) {
        let t = std::time::Instant::now();
        std::thread::scope(|sc| {
            for &addr in &addrs {
                sc.spawn(move || {
                    let p = addr as *mut u32;
                    let mut v = 1u32;
                    let mut it = 0u64;
                    while it < iters_per {
                        // SAFETY: addr is a reserved u32 this thread exclusively
                        // writes; the backing block outlives the scope.
                        unsafe { std::ptr::write_volatile(p, v) };
                        v = v.wrapping_add(1);
                        it += 1;
                    }
                });
            }
        });
        if rep >= warmup {
            samples.push(t.elapsed().as_nanos());
        }
    }
    let _ = AtomicU64::new(0).load(Ordering::Relaxed); // touch the import
    samples.sort_unstable();
    samples[samples.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Axis C: the per-resource layouts stay under the Dim<256> cap; the
    /// per-member layouts cross it on a realistic resource set.
    #[test]
    fn axis_c_column_count_cap() {
        // A modest resource set: 40 resources, 4 scalar members each.
        let resources = 40;
        let m = 4;

        // Per-resource layouts: column count == resources (40), well under 256.
        for layout in ["V0_blob", "V1_snapshot", "V4_erased"] {
            let cols = column_count(layout, resources, m);
            assert!(cols <= 256, "{layout} expected <= 256 columns, got {cols}");
            assert!(reserve_n_columns(cols).is_ok(), "{layout} reserve of {cols} should succeed");
        }
        // V3 shares one column: 3 total.
        assert!(reserve_n_columns(column_count("V3_shapebound", resources, m)).is_ok());

        // Per-member layouts: 40 * (4 + 2) = 240 here, still under 256; bump the
        // resource set to cross the cap and observe IdOutOfRange.
        for layout in ["V2_decomposed", "V5_handletable"] {
            let cols = column_count(layout, resources, m);
            assert_eq!(cols, 240, "{layout} column count at 40x(4+2)");
        }
        // 50 resources x (4+2) = 300 columns: crosses 256.
        let crossing = column_count("V2_decomposed", 50, m);
        assert_eq!(crossing, 300);
        assert_eq!(
            reserve_n_columns(crossing),
            Err(StorageError::IdOutOfRange),
            "per-member layout at 300 columns must hit the Dim<256> cap"
        );
    }

    /// Axis E: the packed (shared-line) layout suffers a measurable false-
    /// sharing penalty vs the padded (per-line) layout. The penalty is hardware-
    /// dependent; the test asserts a clear separation (> 1.5x) rather than an
    /// exact figure, and prints the numbers for the findings doc.
    #[test]
    fn axis_e_false_sharing_penalty() {
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
        let (packed, padded) = false_sharing(threads, 80_000_000);
        let ratio = packed as f64 / padded.max(1) as f64;
        println!(
            "axis E: threads={threads} packed={packed}ns padded={padded}ns penalty={ratio:.3}"
        );
        assert!(
            ratio > 1.5,
            "expected a clear false-sharing penalty (>1.5x), got {ratio:.3} \
             (packed {packed}ns vs padded {padded}ns)"
        );
    }
}
