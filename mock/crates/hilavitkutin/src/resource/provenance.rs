//! Pointer-provenance newtypes.
//!
//! Resource storage and column storage live at separate provenance.
//! Distinct `#[repr(transparent)]` wrappers over `NonNull<T>` help
//! LLVM prove noalias when fused WUs read from both.
//!
//! The noalias invariant (domain 19): the resource handle store never
//! aliases the value columns, and the Context projection snapshots each
//! read-set resource value onto the dispatch stack, so the morsel hot
//! loop reads scalar members at stack provenance while column pointers
//! stay heap provenance. `Seq`/`Map` collection members are the designed
//! exception: the snapshot carries only their pointer-plus-length view
//! and the elements stream live from their own columns.

use core::ptr::NonNull;

/// Erased resource-value base pointer.
///
/// The binding records the drained one-record blob's base in erased form
/// plus the value's static shape (`ValueShape`); the typed view is
/// recovered by [`typed`] at projection time. The erased form is the
/// hybrid addressing the storage bench settled: parity with a
/// monomorphised pointer in-process, and the shape-described base is
/// what a dynamic-library or wasm extension boundary can carry without
/// monomorphising the value type.
///
/// Unconditionally `Send + Sync`: the value-type auto-trait gating lives
/// on the owning `ResourceBinding<T>` through its `PhantomData<T>`.
///
/// [`typed`]: ErasedResourcePtr::typed
#[repr(transparent)]
pub struct ErasedResourcePtr(NonNull<u8>); // lint:allow(no-bare-numeric) reason: erased byte base is the addressing contract; tracked: #654

impl ErasedResourcePtr {
    /// # Safety
    /// The pointer must be non-null and name a resource-value base valid
    /// for the lifetime of the borrows derived from it.
    #[inline(always)]
    pub unsafe fn new_unchecked(ptr: *mut u8) -> Self {
        // lint:allow(no-bare-numeric) reason: erased byte base is the addressing contract; tracked: #654
        Self(unsafe { NonNull::new_unchecked(ptr) })
    }

    #[inline(always)]
    pub const fn as_ptr(self) -> *mut u8 {
        // lint:allow(no-bare-numeric) reason: erased byte base is the addressing contract; tracked: #654
        self.0.as_ptr()
    }

    /// Backcast to the typed resource pointer.
    ///
    /// # Safety
    /// The base must have been recorded for a value of type `T` (the
    /// owning binding's type parameter is the witness), correctly
    /// aligned for `T`.
    #[inline(always)]
    pub unsafe fn typed<T>(self) -> ResourcePtr<T> {
        // SAFETY: non-null by construction; the caller proves the base
        // was recorded for a `T`.
        unsafe { ResourcePtr::new_unchecked(self.0.as_ptr() as *mut T) }
    }
}

impl Copy for ErasedResourcePtr {}
impl Clone for ErasedResourcePtr {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

// SAFETY: the erased base is only ever re-typed through the owning
// binding, whose `PhantomData<T>` carries the value type's own
// Send/Sync gating; the bare erased pointer grants no access on its own.
unsafe impl Send for ErasedResourcePtr {}
unsafe impl Sync for ErasedResourcePtr {}

#[repr(transparent)]
pub struct ResourcePtr<T>(NonNull<T>);

impl<T> ResourcePtr<T> {
    /// # Safety
    /// The pointer must be valid for reads / writes of `T` for the
    /// lifetime of the borrow it represents.
    #[inline(always)]
    pub unsafe fn new_unchecked(ptr: *mut T) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr) })
    }

    #[inline(always)]
    pub const fn as_ptr(self) -> *mut T {
        self.0.as_ptr()
    }
}

impl<T> Copy for ResourcePtr<T> {}
impl<T> Clone for ResourcePtr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

// SAFETY: The pointer is valid for the thread's lifetime when
// T: Send/Sync. The aliasing discipline is enforced at the type
// level by the access set parameter on the owning cache: there
// is no thread-local aliasing concern.
unsafe impl<T: Send> Send for ResourcePtr<T> {}
unsafe impl<T: Sync> Sync for ResourcePtr<T> {}

#[repr(transparent)]
pub struct ColumnPtr<T>(NonNull<T>);

impl<T> ColumnPtr<T> {
    /// # Safety
    /// Same as ResourcePtr.
    #[inline(always)]
    pub unsafe fn new_unchecked(ptr: *mut T) -> Self {
        Self(unsafe { NonNull::new_unchecked(ptr) })
    }

    #[inline(always)]
    pub const fn as_ptr(self) -> *mut T {
        self.0.as_ptr()
    }
}

impl<T> Copy for ColumnPtr<T> {}
impl<T> Clone for ColumnPtr<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

// SAFETY: The pointer is valid for the thread's lifetime when
// T: Send/Sync. The aliasing discipline is enforced at the type
// level by the access set parameter on the owning cache: there
// is no thread-local aliasing concern.
unsafe impl<T: Send> Send for ColumnPtr<T> {}
unsafe impl<T: Sync> Sync for ColumnPtr<T> {}
