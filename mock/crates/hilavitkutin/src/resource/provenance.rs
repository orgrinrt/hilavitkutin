//! Pointer-provenance newtypes.
//!
//! Resource storage and column storage live at separate provenance.
//! Distinct `#[repr(transparent)]` wrappers over `NonNull<T>` help
//! LLVM prove noalias when fused WUs read from both.

use core::ptr::NonNull;

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
// level by the access set parameter on the owning cache — there
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
// level by the access set parameter on the owning cache — there
// is no thread-local aliasing concern.
unsafe impl<T: Send> Send for ColumnPtr<T> {}
unsafe impl<T: Sync> Sync for ColumnPtr<T> {}
