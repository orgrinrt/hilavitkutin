//! Axis C (column-count capacity) for the resource-storage-model round
//! 202606210600.
//!
//! A capacity assertion against the real
//! `hilavitkutin_providers::ArenaColumnStorage<_, Dim<256>>`, not a latency
//! bench: the per-member-column layouts (V2 decomposed, V5 handle-table) reserve
//! one column per scalar member, so a resource set's distinct-column count is
//! resources x members and crosses the engine's 256-store cap, whereas the
//! per-resource layouts (V0/V1 blob, V3 shape-bound, V4 erased) reserve one
//! column per resource and stay well under it.
//!
//! Axis E, the cross-core false-sharing measurement this crate used to time by
//! hand, is the `false_sharing` sweep of the `resource_storage` bench, arms
//! `fs_packed` and `fs_padded`.

use arvo::USize;
use arvo_tensor::Dim;
use hilavitkutin_api::{ColumnStorage, MemoryProviderApi, StoreId};
use hilavitkutin_providers::{ArenaColumnStorage, StorageError};

/// A heap-backed bump `MemoryProviderApi`: the crate's `StaticBumpMemory` over a
/// heap block this provider owns and frees when it drops. The columns are too
/// big for a fixed arena, and a bench builds schedulers per size, so the block
/// is freed rather than leaked.
pub struct TestProvider {
    inner: core::mem::ManuallyDrop<hilavitkutin_providers::StaticBumpMemory>,
    block: *mut [core::mem::MaybeUninit<u8>],
}

impl TestProvider {
    /// A provider over a fresh block of `bytes`.
    pub fn new(bytes: usize) -> Self {
        let block = Box::into_raw(vec![core::mem::MaybeUninit::uninit(); bytes].into_boxed_slice());
        // SAFETY: `block` is a fresh allocation of `bytes` bytes nothing else
        // reaches, and `drop` frees it only after `inner` is dropped, which is
        // the contract of `from_raw_parts`.
        let inner = unsafe {
            hilavitkutin_providers::StaticBumpMemory::from_raw_parts(
                block.cast(),
                arvo::USize(bytes),
            )
        };
        Self {
            inner: core::mem::ManuallyDrop::new(inner),
            block,
        }
    }
}

impl Drop for TestProvider {
    fn drop(&mut self) {
        // SAFETY: `inner` is dropped here and nowhere else, before the block
        // under it is freed, as `from_raw_parts` asks.
        unsafe { core::mem::ManuallyDrop::drop(&mut self.inner) };
        // SAFETY: `block` came from `Box::into_raw` in `new` and is freed once.
        drop(unsafe { Box::from_raw(self.block) });
    }
}

// SAFETY: `block` is owned by this provider alone and freed only in `drop`;
// every other access goes through `inner`, which is Send and Sync.
unsafe impl Send for TestProvider {}
// SAFETY: as above.
unsafe impl Sync for TestProvider {}

// SAFETY: every call goes to `inner`, which keeps the promises of
// `MemoryProviderApi` over `block`; the block lives, unmoved, until this
// provider drops, which is the end of every block's life the trait allows.
unsafe impl MemoryProviderApi for TestProvider {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        // SAFETY: forwarded as received.
        unsafe { self.inner.allocate(len, align) }
    }

    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize) {
        // SAFETY: forwarded as received.
        unsafe { self.inner.deallocate(ptr, len, align) }
    }

    unsafe fn protect(&self, ptr: *mut u8, len: USize, read: arvo::Bool, write: arvo::Bool) {
        // SAFETY: forwarded as received.
        unsafe { self.inner.protect(ptr, len, read, write) }
    }
}

/// Reserve `columns` distinct one-record `u32` columns into a fresh
/// `ArenaColumnStorage<_, Dim<256>>`. Returns `Ok(())` if every reserve
/// succeeded, or the `StorageError` of the first that failed. This is the
/// capacity probe: it models a layout that needs `columns` distinct store ids.
pub fn reserve_n_columns(columns: usize) -> Result<(), StorageError> {
    let provider = TestProvider::new(columns * 64 + (1 << 16));
    let mut store: ArenaColumnStorage<TestProvider, Dim<256>> = ArenaColumnStorage::new(provider);
    for id in 0 .. columns {
        match store.reserve::<u32>(StoreId(USize(id)), USize(1)) {
            notko::Outcome::Ok(()) => {},
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
            assert!(
                reserve_n_columns(cols).is_ok(),
                "{layout} reserve of {cols} should succeed"
            );
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
}
