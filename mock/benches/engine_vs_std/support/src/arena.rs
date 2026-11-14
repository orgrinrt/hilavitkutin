//! The engine's memory and the core budget every threaded arm shares.

use arvo::USize;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_providers::ArenaColumnStorage;

/// A heap-backed bump `MemoryProviderApi`: the crate's `StaticBumpMemory` over a
/// heap block this provider owns and frees when it drops. The columns are too
/// big for a fixed arena, and a bench builds schedulers per size, so the block
/// is freed rather than leaked.
pub struct HeapBump {
    inner: core::mem::ManuallyDrop<hilavitkutin_providers::StaticBumpMemory>,
    block: *mut [core::mem::MaybeUninit<u8>],
}

impl HeapBump {
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

impl Drop for HeapBump {
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
unsafe impl Send for HeapBump {}
// SAFETY: as above.
unsafe impl Sync for HeapBump {}

// SAFETY: every call goes to `inner`, which keeps the promises of
// `MemoryProviderApi` over `block`; the block lives, unmoved, until this
// provider drops, which is the end of every block's life the trait allows.
unsafe impl MemoryProviderApi for HeapBump {
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

pub fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

/// Bytes the arena needs for `columns` typed `u32` stores over `n` records,
/// plus 64-byte alignment slack per column and headroom for the plan's own
/// stored columns.
pub fn arena_bytes(columns: usize, n: usize) -> usize {
    columns * n * 4 + columns * 64 + (1 << 16)
}

/// Thread count for the threaded std arms. Matches the worker count
/// `BenchExecutor` gives the engine (the machine's available parallelism), so
/// the parallel arms compare equal core budgets.
pub fn std_threads() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

/// Write `records` into a column through its base pointer, one record each,
/// wrapped in the column's value type.
///
/// White-box: the engine has no supported host-side column write, so the arms
/// reach the column a built scheduler reserved through its `#[doc(hidden)]`
/// bindings, as `frame_wide` does (ticket ef57305fb5f4 tracks the supported
/// write).
///
/// # Safety
///
/// `base` must point at a column reserved for at least `records.len()`
/// records of `T`, alive and not read by a running frame for the call.
pub unsafe fn write_column<T>(base: *mut T, records: &[u32], wrap: impl Fn(u32) -> T) {
    for (i, &v) in records.iter().enumerate() {
        // SAFETY: the caller vouches for the reservation and that no frame runs.
        unsafe { base.add(i).write(wrap(v)) };
    }
}

/// FNV-1a over `n` records read back through a column's base pointer, each
/// by the u32 `value` reads out of it.
///
/// # Safety
///
/// `base` must point at `n` initialised records of `T`, alive for the call.
pub unsafe fn hash_column<T>(base: *const T, n: usize, value: impl Fn(&T) -> u32) -> u64 {
    // SAFETY: the caller vouches for `n` initialised records of `T`.
    bench_arm::fnv1a_by(unsafe { core::slice::from_raw_parts(base, n) }, value)
}
