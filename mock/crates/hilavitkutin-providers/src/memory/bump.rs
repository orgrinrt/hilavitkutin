//! A memory provider that bumps through its buffer and never takes a block back.
//!
//! `StaticBumpMemory` hands out blocks of a buffer the consumer gives it, front
//! to back, and never takes one back. It is the provider for a consumer
//! without an operating system that never gives memory back, which builds it
//! over a `static` buffer, and for the engine's tests, which build it over a
//! leaked one.

use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin_api::MemoryProviderApi;

use super::Buffer;

/// Bump memory provider over a buffer that is its own for as long as it lives.
///
/// The buffer is borrowed exclusively for `'static` under
/// [`StaticBumpMemory::new`], or promised so until the provider is dropped under
/// [`StaticBumpMemory::from_raw_parts`], so no block moves or is freed while the
/// provider lives, whatever happens to the provider value. The cursor is the
/// offset of the first free byte and only ever grows.
pub struct StaticBumpMemory {
    buffer: Buffer,
    used:   AtomicUsize,
}

impl StaticBumpMemory {
    /// A provider handing out the bytes of `buffer`.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub const fn new(buffer: &'static mut [MaybeUninit<u8>]) -> Self { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a byte buffer is bytes; tracked: #72
        Self::over(Buffer::new(buffer))
    }

    /// A provider handing out the `len` bytes at `base`.
    ///
    /// # Safety
    ///
    /// The `len` bytes at `base` are valid for reads and writes, nothing but
    /// the provider reads or writes them, and they stay so until the provider
    /// is dropped. That is what [`StaticBumpMemory::new`] gets from its
    /// `'static` borrow; a caller whose buffer is not `'static` promises it
    /// here, and frees the buffer only after the provider is gone.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub const unsafe fn from_raw_parts(base: *mut MaybeUninit<u8>, len: USize) -> Self { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a byte buffer is bytes; tracked: #72
        // SAFETY: the caller's promise is `Buffer::from_raw_parts`'s.
        Self::over(unsafe { Buffer::from_raw_parts(base, len) })
    }

    const fn over(buffer: Buffer) -> Self {
        Self {
            buffer,
            used: AtomicUsize::new(0), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the cursor starts at the first byte; tracked: #72
        }
    }

    /// How many bytes of the buffer are handed out, alignment padding included.
    pub fn used(&self) -> USize {
        USize(self.used.load(Ordering::Relaxed))
    }

    /// The length of the buffer.
    pub const fn capacity(&self) -> USize {
        self.buffer.capacity()
    }
}

// SAFETY: the buffer is the provider's alone while it lives, by the `'static`
// borrow or the caller's promise, so no other owner reaches it from any
// thread, and every access the provider makes to its own state goes through
// the atomic cursor.
unsafe impl Send for StaticBumpMemory {}
// SAFETY: as above; `allocate` hands each caller a range no other caller gets.
unsafe impl Sync for StaticBumpMemory {}

// SAFETY: the five promises of `MemoryProviderApi`.
// 1. `allocate` rounds the address of the first free byte up to `align` and
//    checks that `len` bytes from there fit in the buffer, so the block is
//    aligned and inside memory valid for reads and writes.
// 2. The buffer stays valid until the provider is dropped, forever under
//    `new` and by the caller's promise under `from_raw_parts`, and its address
//    is the buffer's, not the provider's, so a block keeps its address across
//    every move of the provider until the provider is dropped.
// 3. The cursor advances by compare-exchange from the value the block was
//    placed after, so two blocks never share a byte.
// 4. The cursor is the only state that changes, and it is atomic.
// 5. `allocate_zeroed` is the trait's default, which writes the zeros.
unsafe impl MemoryProviderApi for StaticBumpMemory {
    #[rustfmt::skip] // keeps the allow on the signature it governs
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI returns raw pointer by contract; tracked: #72
        if !align.0.is_power_of_two() {
            return core::ptr::null_mut();
        }
        let base = self.buffer.base();
        let mask = align.0 - 1; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: power-of-two alignment mask; tracked: #72
        let mut used = self.used.load(Ordering::Relaxed);
        loop {
            let pad = base.addr().wrapping_add(used).wrapping_neg() & mask;
            let Some(end) = used.checked_add(pad).and_then(|start| start.checked_add(len.0)) else {
                return core::ptr::null_mut();
            };
            if end > self.buffer.capacity().0 {
                return core::ptr::null_mut();
            }
            match self.used.compare_exchange_weak(used, end, Ordering::Relaxed, Ordering::Relaxed) {
                // SAFETY: `used + pad + len <= cap`, so the offset is inside
                // the buffer `base` points at.
                Ok(_) => return unsafe { base.add(used + pad) },
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
