//! A memory provider over one buffer that lives for the whole program.
//!
//! `StaticBumpMemory` hands out blocks of a `&'static mut [MaybeUninit<u8>]`
//! the consumer gives it, front to back, and never takes one back. It is the
//! provider for a consumer without an operating system, which builds it over a
//! `static` buffer, and for the engine's tests, which build it over a leaked
//! one.

use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin_api::MemoryProviderApi;

/// Bump memory provider over a buffer borrowed for the rest of the program.
///
/// The buffer is borrowed exclusively and for `'static`, so no block moves or
/// is freed while the provider lives or after, whatever happens to the provider
/// value. The cursor is the offset of the first free byte and only ever grows.
pub struct StaticBumpMemory {
    base: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw byte pointer; tracked: #72
    cap:  USize,
    used: AtomicUsize,
}

impl StaticBumpMemory {
    /// A provider handing out the bytes of `buffer`.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub const fn new(buffer: &'static mut [MaybeUninit<u8>]) -> Self { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a byte buffer is bytes; tracked: #72
        Self {
            cap:  USize(buffer.len()),
            base: buffer.as_mut_ptr().cast(),
            used: AtomicUsize::new(0), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the cursor starts at the first byte; tracked: #72
        }
    }

    /// How many bytes of the buffer are handed out, alignment padding included.
    pub fn used(&self) -> USize {
        USize(self.used.load(Ordering::Relaxed))
    }

    /// The length of the buffer.
    pub const fn capacity(&self) -> USize {
        self.cap
    }
}

// SAFETY: the buffer behind `base` is borrowed exclusively for `'static` by
// this provider, so no other owner reaches it from any thread, and every
// access the provider makes to its own state goes through the atomic cursor.
unsafe impl Send for StaticBumpMemory {}
// SAFETY: as above; `allocate` hands each caller a range no other caller gets.
unsafe impl Sync for StaticBumpMemory {}

// SAFETY: the four promises of `MemoryProviderApi`.
// 1. `allocate` rounds the address of the first free byte up to `align` and
//    checks that `len` bytes from there fit in the buffer, so the block is
//    aligned and inside memory valid for reads and writes.
// 2. The buffer is borrowed for `'static` and never released, and `base` is
//    the buffer's address, not the provider's, so a block keeps its address
//    across every move of the provider and outlives it.
// 3. The cursor advances by compare-exchange from the value the block was
//    placed after, so two blocks never share a byte.
// 4. The cursor is the only state that changes, and it is atomic.
unsafe impl MemoryProviderApi for StaticBumpMemory {
    #[rustfmt::skip] // keeps the allow on the signature it governs
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI returns raw pointer by contract; tracked: #72
        if !align.0.is_power_of_two() {
            return core::ptr::null_mut();
        }
        let mask = align.0 - 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: power-of-two alignment mask; tracked: #72
        let mut used = self.used.load(Ordering::Relaxed);
        loop {
            let pad = self.base.addr().wrapping_add(used).wrapping_neg() & mask;
            let Some(end) = used.checked_add(pad).and_then(|start| start.checked_add(len.0)) else {
                return core::ptr::null_mut();
            };
            if end > self.cap.0 {
                return core::ptr::null_mut();
            }
            match self.used.compare_exchange_weak(used, end, Ordering::Relaxed, Ordering::Relaxed) {
                // SAFETY: `used + pad + len <= cap`, so the offset is inside
                // the buffer `base` points at.
                Ok(_) => return unsafe { self.base.add(used + pad) },
                Err(now) => used = now,
            }
        }
    }

    unsafe fn deallocate(
        &self,
        _ptr: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        _len: USize,
        _align: USize,
    ) {
    }

    unsafe fn protect(
        &self,
        _ptr: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        _len: USize,
        _read: Bool,
        _write: Bool,
    ) {
    }
}
