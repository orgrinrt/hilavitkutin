//! A memory provider over a consumer's buffer that takes blocks back.
//!
//! `StaticFreeListMemory` keeps the free space of its buffer as a list of free
//! blocks in address order, each holding its own list node in its first bytes.
//! `allocate` cuts the first block that fits, `deallocate` puts a block back
//! and merges it with the free blocks it touches. It is for a consumer that
//! gives memory back and wants it again: a store reserved again on every
//! rebuild, an arena whose live peak is far below what it allocates.

use core::cell::UnsafeCell;
use core::mem::{MaybeUninit, size_of};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use arvo::{Bool, USize};
use hilavitkutin_api::MemoryProviderApi;

use super::Buffer;

/// A free block's list node, written into the block's first bytes.
#[repr(C)]
struct Node {
    /// The free block's length in bytes, a multiple of [`GRAIN`].
    len:  USize,
    /// The next free block by address, or null.
    next: *mut Node,
}

/// The unit every block's start and length are a multiple of: the size of a
/// node, two machine words, so a piece a split leaves can always hold one.
const GRAIN: usize = size_of::<Node>(); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a layout constant from size_of; tracked: #72

/// The list, changed only under the lock.
struct List {
    /// The first free block by address, or null.
    head:  *mut Node,
    /// Whether the buffer's interior was laid down as the first free block.
    /// A `const` constructor cannot write into the buffer, so the first call
    /// under the lock does it.
    ready: Bool,
}

/// Memory provider over a buffer that is its own for as long as it lives, and
/// that takes blocks back.
///
/// The buffer is borrowed exclusively for `'static` under
/// [`StaticFreeListMemory::new`], or promised so until the provider is dropped
/// under [`StaticFreeListMemory::from_raw_parts`], as for `StaticBumpMemory`.
pub struct StaticFreeListMemory {
    buffer: Buffer,
    list:   UnsafeCell<List>,
    used:   AtomicUsize,
    // FIXME(the_stack_has_no_lock_so_a_provider_keeps_a_spin_flag_of_its_own):
    // a one-flag spin lock of the provider's own, since the stack has none.
    lock:   AtomicBool,
}

/// Holds the lock and gives it back when dropped.
struct Held<'a>(&'a AtomicBool);

impl Drop for Held<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

/// `n` rounded up to the grain, a zero taking one grain; `None` past `usize`.
#[rustfmt::skip] // keeps the allow on the signature it governs
const fn grains(n: usize) -> Option<usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: byte-length arithmetic below the allocator ABI; tracked: #72
    let n = if n == 0 { 1 } else { n };
    match n.checked_add(GRAIN - 1) {
        Some(m) => Some(m & !(GRAIN - 1)),
        None => None,
    }
}

impl StaticFreeListMemory {
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
    /// is dropped. That is what [`StaticFreeListMemory::new`] gets from its
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
            list: UnsafeCell::new(List {
                head:  core::ptr::null_mut(),
                ready: Bool::FALSE,
            }),
            used: AtomicUsize::new(0), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: nothing is handed out at first; tracked: #72
            lock: AtomicBool::new(false),
        }
    }

    /// How many bytes are handed out and not given back, grain padding
    /// included.
    pub fn used(&self) -> USize {
        USize(self.used.load(Ordering::Relaxed))
    }

    /// The length of the buffer.
    pub const fn capacity(&self) -> USize {
        self.buffer.capacity()
    }

    fn hold(&self) -> Held<'_> {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }
        Held(&self.lock)
    }

    /// The list, laid down on first use.
    ///
    /// # Safety
    ///
    /// The caller holds the lock, so no other call reaches the list.
    #[allow(clippy::mut_from_ref)]
    unsafe fn list(&self) -> &mut List {
        // SAFETY: the caller holds the lock.
        let list = unsafe { &mut *self.list.get() };
        if !list.ready.0 {
            list.ready = Bool::TRUE;
            let base = self.buffer.base();
            let lead = base.addr().wrapping_neg() & (GRAIN - 1);
            let cap = self.buffer.capacity().0;
            if lead < cap {
                let len = (cap - lead) & !(GRAIN - 1);
                if len > 0 {
                    // SAFETY: `lead + len <= cap`, so the node lies inside the
                    // buffer, at an address that is a multiple of the grain and
                    // so of the node's alignment.
                    let node = unsafe { base.add(lead) }.cast::<Node>();
                    // SAFETY: `node` is in bounds and aligned, as above, and
                    // `len` is a nonzero multiple of `GRAIN`, a node's size, so the
                    // write stays inside the buffer; the lock is held, so no
                    // other reference to these bytes exists.
                    unsafe {
                        node.write(Node {
                            len:  USize(len),
                            next: core::ptr::null_mut(),
                        })
                    };
                    list.head = node;
                }
            }
        }
        list
    }
}

// SAFETY: the buffer is the provider's alone while it lives, by the `'static`
// borrow or the caller's promise, so no other owner reaches it from any
// thread.
unsafe impl Send for StaticFreeListMemory {}
// SAFETY: the list and the free blocks' nodes are read and written only with
// the lock held, and `used` is atomic.
unsafe impl Sync for StaticFreeListMemory {}

// SAFETY: the five promises of `MemoryProviderApi`.
// 1. A block is cut from a free block inside the buffer, at an address rounded
//    up to `align`, and only once the free block holds the padding and the
//    rounded length, so the block is aligned and valid for reads and writes.
// 2. The buffer stays valid until the provider is dropped, and its address is
//    the buffer's, not the provider's, so a block keeps its address across
//    every move of the provider.
// 3. A byte is on the list or handed out and never both: `allocate` takes a
//    block's bytes off the list, `deallocate` puts back only the bytes the
//    caller was handed, and a merge joins only free blocks that touch.
// 4. Every read and write of the list happens with the lock held.
// 5. `allocate_zeroed` is the trait's default, which writes the zeros: a block
//    given back and handed out again carries what its last owner wrote.
unsafe impl MemoryProviderApi for StaticFreeListMemory {
    #[rustfmt::skip] // keeps the allow on the signature it governs
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI returns raw pointer by contract; tracked: #72
        if !align.0.is_power_of_two() {
            return core::ptr::null_mut();
        }
        let Some(want) = grains(len.0) else {
            return core::ptr::null_mut();
        };
        let mask = align.0.max(GRAIN) - 1;
        let _held = self.hold();
        // SAFETY: the lock is held.
        let list = unsafe { self.list() };
        let mut link: *mut *mut Node = &mut list.head;
        // SAFETY: every node on the list was written by this provider inside
        // the buffer and is read only with the lock held.
        unsafe {
            while !(*link).is_null() {
                let node = *link;
                let have = (*node).len.0;
                let pad = node.addr().wrapping_neg() & mask;
                if let Some(need) = pad.checked_add(want)
                    && need <= have
                {
                    let start = node.byte_add(pad);
                    let tail = have - need;
                    let mut rest = (*node).next;
                    if tail > 0 {
                        let after = start.byte_add(want);
                        after.write(Node {
                            len:  USize(tail),
                            next: rest,
                        });
                        rest = after;
                    }
                    if pad > 0 {
                        (*node).len = USize(pad);
                        (*node).next = rest;
                    } else {
                        *link = rest;
                    }
                    self.used.fetch_add(want, Ordering::Relaxed);
                    return start.cast();
                }
                link = &mut (*node).next;
            }
        }
        core::ptr::null_mut()
    }

    unsafe fn deallocate(
        &self,
        ptr: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw pointer; tracked: #72
        len: USize,
        _align: USize,
    ) {
        // The caller passes the `len` it was handed the block for, so the
        // rounding `allocate` did cannot overflow here.
        let Some(back) = grains(len.0) else {
            return;
        };
        let _held = self.hold();
        // SAFETY: the lock is held.
        let list = unsafe { self.list() };
        let at = ptr.addr();
        let mut before: *mut Node = core::ptr::null_mut();
        let mut link: *mut *mut Node = &mut list.head;
        // SAFETY: every node on the list was written by this provider inside
        // the buffer and is read only with the lock held, and `ptr` names
        // `back` bytes the caller was handed and gives back now, at a multiple
        // of the grain, so a node fits there.
        unsafe {
            while !(*link).is_null() && (*link).addr() < at {
                before = *link;
                link = &mut (*before).next;
            }
            let node = ptr.cast::<Node>();
            node.write(Node {
                len:  USize(back),
                next: *link,
            });
            let after = (*node).next;
            if !after.is_null() && at + back == after.addr() {
                (*node).len = USize(back + (*after).len.0);
                (*node).next = (*after).next;
            }
            *link = node;
            if !before.is_null() && before.addr() + (*before).len.0 == at {
                (*before).len = USize((*before).len.0 + (*node).len.0);
                (*before).next = (*node).next;
            }
        }
        self.used.fetch_sub(back, Ordering::Relaxed);
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
