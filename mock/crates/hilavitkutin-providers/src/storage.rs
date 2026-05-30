//! Naive arena-backed `ColumnStorage`.
//!
//! `ArenaColumnStorage` is the default backing for the unified columnar
//! store. The engine stays generic over `CS: ColumnStorage`; consumers
//! (and the engine's own tests) wire this in. It is a placeholder: a
//! flat slot table over a consumer-supplied `MemoryProvider`, with each
//! column a separate provider allocation. The columnar engine replaces
//! it later without touching any `CS: ColumnStorage` consumer.

use core::mem::size_of;

use arvo::USize;
use arvo::strategy::Identity;
use arvo_tensor::{Capacity, Dim};
use notko::{Maybe, Outcome};

use hilavitkutin_api::{ColumnStorage, ColumnValue, MemoryProviderApi, StoreId};

/// 64-byte cache-line alignment for every column base (R6).
// lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: cache-line width is a fixed layout constant; tracked: #72
const CACHE_LINE_ALIGN: USize = USize(64);

/// One reserved column: its base pointer, allocated byte length (for
/// `deallocate`), and record count.
#[derive(Copy, Clone)]
struct ColumnSlot {
    ptr: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: allocator ABI raw byte pointer; tracked: #72
    len_bytes: USize,
    count: USize,
}

/// Naive arena-backed `ColumnStorage`.
///
/// `M` is the allocator. `D` caps the distinct-column count; the slot
/// table is `D::Array<Maybe<ColumnSlot>>` indexed by `StoreId`. The
/// default `Dim<256>` matches the engine's `Mask256` store ceiling.
pub struct ArenaColumnStorage<M: MemoryProviderApi, D: Capacity = Dim<256>> {
    provider: M,
    slots: D::Array<Maybe<ColumnSlot>>,
}

impl<M: MemoryProviderApi, D: Capacity> ArenaColumnStorage<M, D> {
    /// Build an empty store over `provider`. No columns reserved.
    pub fn new(provider: M) -> Self {
        Self {
            provider,
            slots: D::filled(Maybe::Isnt),
        }
    }
}

/// Failure returned by [`ArenaColumnStorage::reserve`].
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StorageError {
    /// The `StoreId` index is at or beyond this store's column cap.
    IdOutOfRange,
    /// The `MemoryProvider` returned null (out of memory).
    Exhausted,
    /// `count * size_of::<T>()` overflows `usize`; no block fits.
    LengthOverflow,
}

impl<M: MemoryProviderApi, D: Capacity> ColumnStorage for ArenaColumnStorage<M, D> {
    type Error = StorageError;

    fn reserve<T: ColumnValue>(&mut self, id: StoreId, count: USize) -> Outcome<(), StorageError> {
        if *id.0 >= self.slots.as_ref().len() {
            return Outcome::Err(StorageError::IdOutOfRange);
        }
        // Free a prior allocation if this id is being re-reserved. Read
        // the slot by value (ColumnSlot is Copy) so no borrow is held
        // across the provider call.
        if let Maybe::Is(old) = self.slots.as_ref()[*id.0] {
            if !old.ptr.is_null() {
                // SAFETY: old.ptr came from a prior allocate on this
                // provider with old.len_bytes.
                unsafe {
                    self.provider.deallocate(old.ptr, old.len_bytes);
                }
            }
        }
        // Byte length = record count times element size, checked so a
        // large count cannot wrap to a small length and hand back an
        // undersized block (a write of `count` records would overrun).
        // lint:allow(no-bare-numeric) reason: byte length math; size_of returns usize by contract; tracked: #345
        let bytes = match (*count).checked_mul(size_of::<T>()) {
            Some(n) => USize(n),
            None => return Outcome::Err(StorageError::LengthOverflow),
        };
        let slot = if bytes == USize::ZERO {
            ColumnSlot {
                ptr: core::ptr::null_mut(),
                len_bytes: USize::ZERO,
                count,
            }
        } else {
            // SAFETY: CACHE_LINE_ALIGN (64) is a power of two and bytes
            // is non-zero; the returned pointer is null-checked below.
            let ptr = unsafe { self.provider.allocate(bytes, CACHE_LINE_ALIGN) };
            if ptr.is_null() {
                self.slots.as_mut()[*id.0] = Maybe::Isnt;
                return Outcome::Err(StorageError::Exhausted);
            }
            ColumnSlot {
                ptr,
                len_bytes: bytes,
                count,
            }
        };
        self.slots.as_mut()[*id.0] = Maybe::Is(slot);
        Outcome::Ok(())
    }

    unsafe fn column_ptr<T: ColumnValue>(&self, id: StoreId) -> *const T {
        if *id.0 >= self.slots.as_ref().len() {
            return core::ptr::null();
        }
        match self.slots.as_ref()[*id.0] {
            Maybe::Is(slot) => slot.ptr as *const T,
            Maybe::Isnt => core::ptr::null(),
        }
    }

    unsafe fn column_ptr_mut<T: ColumnValue>(&self, id: StoreId) -> *mut T {
        if *id.0 >= self.slots.as_ref().len() {
            return core::ptr::null_mut();
        }
        match self.slots.as_ref()[*id.0] {
            Maybe::Is(slot) => slot.ptr as *mut T,
            Maybe::Isnt => core::ptr::null_mut(),
        }
    }

    fn count(&self, id: StoreId) -> USize {
        if *id.0 >= self.slots.as_ref().len() {
            return USize::ZERO;
        }
        match self.slots.as_ref()[*id.0] {
            Maybe::Is(slot) => slot.count,
            Maybe::Isnt => USize::ZERO,
        }
    }

    fn release(&mut self, _id: StoreId) {
        // Naive: no-op. Reader-count reclamation is a columnar-engine
        // concern; this placeholder frees only on Drop.
    }
}

impl<M: MemoryProviderApi, D: Capacity> Drop for ArenaColumnStorage<M, D> {
    fn drop(&mut self) {
        for slot in self.slots.as_ref() {
            if let Maybe::Is(s) = slot {
                if !s.ptr.is_null() {
                    // SAFETY: s.ptr came from a prior allocate on this
                    // provider with s.len_bytes; the arena is dropped
                    // once, so no double free.
                    unsafe {
                        self.provider.deallocate(s.ptr, s.len_bytes);
                    }
                }
            }
        }
    }
}
