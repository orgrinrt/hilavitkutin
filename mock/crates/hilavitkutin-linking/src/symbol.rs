//! Typed symbol handles and the sealed `LibrarySymbol` trait.
//!
//! Two typed wrappers co-exist:
//!
//! - `Symbol<'lib, T>` resolves a function-pointer symbol. `T` is
//!   sealed-restricted to `extern "C" fn(...)` shapes at arities 0-8.
//!   Access pattern: `transmute_copy` the raw pointer to `T`.
//! - `StaticRef<'lib, T>` resolves a static-data symbol. `T: 'static`
//!   is any FFI-safe data type (typically a `#[repr(C)]` struct). Access
//!   pattern: dereference `*const T`.
//!
//! The two types are separate because the pointer-reinterpretation rules
//! differ; mixing them would hide the soundness contract behind `unsafe`.

use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem;

/// Typed handle to a resolved *function-pointer* symbol.
///
/// The lifetime parameter ties this handle to the `Library` it was
/// resolved from, preventing use-after-close at compile time. `T` is
/// restricted by the sealed `LibrarySymbol` marker to pointer-sized
/// `extern "C"` function pointer shapes.
pub struct Symbol<'lib, T: LibrarySymbol> {
    ptr: *const c_void,
    _marker: PhantomData<(&'lib (), T)>,
}

impl<'lib, T: LibrarySymbol> Symbol<'lib, T> {
    /// Obtain the underlying function pointer.
    ///
    /// Returned by value because `T` is a function pointer type whose
    /// bit pattern equals the raw pointer returned by the platform
    /// loader. The lifetime of any dereference through the returned
    /// pointer is still bound to the `Library` whose lifetime this
    /// `Symbol` borrows.
    pub fn get(&self) -> T {
        debug_assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<*const c_void>(),
        );
        // SAFETY: the sealed LibrarySymbol impls restrict T to
        // pointer-sized extern "C" function pointer types.
        // transmute_copy at matching size reinterprets the raw pointer
        // bit pattern as T. This is the canonical dlsym -> fn-pointer
        // cast.
        unsafe { mem::transmute_copy::<*const c_void, T>(&self.ptr) }
    }

    /// Raw pointer escape hatch.
    pub fn as_raw(&self) -> *const c_void {
        self.ptr
    }

    pub(crate) fn from_raw(ptr: *const c_void) -> Self {
        Self { ptr, _marker: PhantomData }
    }
}

/// Typed handle to a resolved *static-data* symbol.
///
/// Wraps the pointer-to-static pattern that every extension ABI uses
/// for its descriptor export (`#[no_mangle] pub static EXT_DESCRIPTOR:
/// ExtDescriptor = ...;`). The lifetime parameter ties this handle to
/// the `Library` it was resolved from.
///
/// `T: 'static` permits arbitrary FFI-safe data types, typically
/// `#[repr(C)]` structs that an extension exports as a manifest.
pub struct StaticRef<'lib, T: 'static> {
    ptr: *const c_void,
    _marker: PhantomData<&'lib T>,
}

impl<'lib, T: 'static> StaticRef<'lib, T> {
    /// Borrow the underlying static data.
    ///
    /// The returned reference is valid for as long as the `StaticRef`
    /// is held; the `StaticRef` itself cannot outlive the source
    /// `Library`.
    pub fn get(&self) -> &T {
        // SAFETY: the pointer was resolved by the platform loader as
        // the address of a static symbol. `T: 'static` ensures the
        // target data has no non-static lifetimes. The borrow lifetime
        // ties to 'lib which ties to the Library whose handle
        // produced the pointer.
        unsafe { &*(self.ptr as *const T) }
    }

    /// Raw pointer escape hatch.
    pub fn as_raw(&self) -> *const c_void {
        self.ptr
    }

    pub(crate) fn from_raw(ptr: *const c_void) -> Self {
        Self { ptr, _marker: PhantomData }
    }
}

/// Sealed marker trait for types that can be resolved via
/// `Library::resolve`.
///
/// v1 implementations cover `extern "C"` function pointer shapes with
/// zero through eight argument arities. Downstream crates cannot add
/// new impls. This arity range covers realistic plugin-ABI shapes;
/// extending it further in a follow-up round is trivial if a concrete
/// need surfaces.
pub trait LibrarySymbol: sealed::Sealed + Copy {}

macro_rules! impl_library_symbol_for_fn {
    ($($args:ident),*) => {
        impl<R, $($args),*> sealed::Sealed for extern "C" fn($($args),*) -> R {}
        impl<R, $($args),*> LibrarySymbol for extern "C" fn($($args),*) -> R {}
    };
}

// 0-arity
impl_library_symbol_for_fn!();
// 1-8 arities
impl_library_symbol_for_fn!(A);
impl_library_symbol_for_fn!(A, B);
impl_library_symbol_for_fn!(A, B, C);
impl_library_symbol_for_fn!(A, B, C, D);
impl_library_symbol_for_fn!(A, B, C, D, E);
impl_library_symbol_for_fn!(A, B, C, D, E, F);
impl_library_symbol_for_fn!(A, B, C, D, E, F, G);
impl_library_symbol_for_fn!(A, B, C, D, E, F, G, H);

mod sealed {
    pub trait Sealed {}
}
