//! Typed symbol handles and the sealed `ExtensionSymbol` trait.
//!
//! v1 scope: the sealed trait admits pointer-sized function pointer
//! types (arities 0-4). Static-data symbol access (resolving to a
//! `&'static T` where T is a struct) is a follow-up concern; the
//! pointer-reinterpretation rules differ from function pointers and
//! warrant a separate type to stay sound.

use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem;

/// Typed handle to a symbol resolved from an `Extension`.
///
/// The lifetime parameter ties this handle to the `Extension` it was
/// resolved from, preventing use-after-close at compile time. `T` is
/// restricted by the sealed `ExtensionSymbol` marker to pointer-sized
/// function pointer shapes.
pub struct Symbol<'ext, T: ExtensionSymbol> {
    ptr: *const c_void,
    _marker: PhantomData<(&'ext (), T)>,
}

impl<'ext, T: ExtensionSymbol> Symbol<'ext, T> {
    /// Obtain the underlying function pointer.
    ///
    /// Returned by value because `T` is a function pointer type whose
    /// bit pattern equals the raw pointer returned by the platform
    /// loader. The lifetime of any dereference through the returned
    /// pointer is still bound to the `Extension` whose lifetime this
    /// `Symbol` borrows.
    pub fn get(&self) -> T {
        // SAFETY: the sealed ExtensionSymbol impls restrict T to
        // pointer-sized function pointer types. `transmute_copy` at
        // matching size reinterprets the raw pointer bit pattern as
        // T — this is the canonical dlsym -> fn-pointer cast.
        debug_assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<*const c_void>(),
        );
        unsafe { mem::transmute_copy::<*const c_void, T>(&self.ptr) }
    }

    /// Raw pointer escape hatch for advanced consumers.
    ///
    /// Useful for cases this crate's sealed impls do not cover yet
    /// (static-data symbols, exotic ABI shapes). Callers take on the
    /// responsibility of casting the pointer to the correct shape.
    pub fn as_raw(&self) -> *const c_void {
        self.ptr
    }

    pub(crate) fn from_raw(ptr: *const c_void) -> Self {
        Self { ptr, _marker: PhantomData }
    }
}

/// Sealed marker trait for types that can be resolved via
/// `Extension::resolve`.
///
/// v1 implementations cover `extern "C"` function pointer shapes with
/// zero to four argument arities. Downstream crates cannot add new
/// impls. Extending the arity set or adding static-data symbol support
/// happens in a follow-up design round.
pub trait ExtensionSymbol: sealed::Sealed + Copy {}

// 0-arity.
impl<R> sealed::Sealed for extern "C" fn() -> R {}
impl<R> ExtensionSymbol for extern "C" fn() -> R {}

// 1-arity.
impl<A, R> sealed::Sealed for extern "C" fn(A) -> R {}
impl<A, R> ExtensionSymbol for extern "C" fn(A) -> R {}

// 2-arity.
impl<A, B, R> sealed::Sealed for extern "C" fn(A, B) -> R {}
impl<A, B, R> ExtensionSymbol for extern "C" fn(A, B) -> R {}

// 3-arity.
impl<A, B, C, R> sealed::Sealed for extern "C" fn(A, B, C) -> R {}
impl<A, B, C, R> ExtensionSymbol for extern "C" fn(A, B, C) -> R {}

// 4-arity.
impl<A, B, C, D, R> sealed::Sealed for extern "C" fn(A, B, C, D) -> R {}
impl<A, B, C, D, R> ExtensionSymbol for extern "C" fn(A, B, C, D) -> R {}

mod sealed {
    pub trait Sealed {}
}
