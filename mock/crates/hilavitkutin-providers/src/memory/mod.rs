//! Memory providers over one buffer that outlives them.
//!
//! Both providers here hand out blocks of a buffer the consumer gives them,
//! which is the provider's alone for as long as the provider lives.
//! `StaticBumpMemory` hands them out front to back and never takes one back;
//! `StaticFreeListMemory` takes them back and hands them out again. The buffer
//! and the promise about it are one private type both hold.

mod bump;
mod free_list;

use core::mem::MaybeUninit;

use arvo::USize;
pub use bump::StaticBumpMemory;
pub use free_list::StaticFreeListMemory;

/// A buffer that is its holder's alone for as long as the holder lives.
///
/// Built from a slice borrowed exclusively for `'static` under
/// [`Buffer::new`], or promised so until the holder is dropped under
/// [`Buffer::from_raw_parts`]. The buffer's address is not the holder's, so
/// moving the holder moves no byte of it.
pub(crate) struct Buffer {
    base: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw byte pointer; tracked: #72
    cap:  USize,
}

impl Buffer {
    /// The bytes of `buffer`.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub(crate) const fn new(buffer: &'static mut [MaybeUninit<u8>]) -> Self { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a byte buffer is bytes; tracked: #72
        let len = USize(buffer.len());
        // SAFETY: the slice is borrowed exclusively for `'static`, so its bytes
        // are valid, nobody else reaches them, and nothing releases them.
        unsafe { Self::from_raw_parts(buffer.as_mut_ptr(), len) }
    }

    /// The `len` bytes at `base`.
    ///
    /// # Safety
    ///
    /// The `len` bytes at `base` are valid for reads and writes, nothing but
    /// the holder reads or writes them, and they stay so until the holder is
    /// dropped. That is what [`Buffer::new`] gets from its `'static` borrow; a
    /// caller whose buffer is not `'static` promises it here, and frees the
    /// buffer only after the holder is gone.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub(crate) const unsafe fn from_raw_parts(base: *mut MaybeUninit<u8>, len: USize) -> Self { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a byte buffer is bytes; tracked: #72
        Self {
            base: base.cast(),
            cap:  len,
        }
    }

    /// The address of the first byte.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub(crate) const fn base(&self) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw byte pointer; tracked: #72
        self.base
    }

    /// The length of the buffer.
    pub(crate) const fn capacity(&self) -> USize {
        self.cap
    }
}
