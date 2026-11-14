//! The engine's memory and the core budget every threaded arm shares.

use core::cell::Cell;
use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_providers::ArenaColumnStorage;

pub struct HeapBump {
    base: *mut u8,
    cap:  usize,
    used: Cell<usize>,
    // Owns the backing allocation; `base` points into it. The heap block does
    // not move when the Box is moved into the struct, so `base` stays valid.
    _buf: Box<[MaybeUninit<u8>]>,
}
impl HeapBump {
    pub fn new(bytes: usize) -> Self {
        let mut buf: Box<[MaybeUninit<u8>]> = vec![MaybeUninit::uninit(); bytes].into_boxed_slice();
        let base = buf.as_mut_ptr() as *mut u8;
        Self {
            base,
            cap: bytes,
            used: Cell::new(0),
            _buf: buf,
        }
    }
}
unsafe impl Send for HeapBump {}
unsafe impl Sync for HeapBump {}
impl MemoryProviderApi for HeapBump {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = used.div_ceil(align) * align;
        if aligned + len.0 > self.cap {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        unsafe { self.base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: arvo::Bool, _write: arvo::Bool) {}
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
