#![no_std]

//! Cross-platform dynamic library loading primitive.
//!
//! See `DESIGN.md` for the shipping contract. This crate owns binary
//! loading mechanics only: opening a shared library, resolving explicit
//! named symbols, returning typed handles with lifetime bound to the
//! loader. Plugin descriptor semantics, lifecycle contracts, and role
//! dispatch live one layer up in `hilavitkutin-plugins`.
//!
//! # Arbitrary-time linking invariant
//!
//! Any library may be loaded at any time during the host's execution,
//! invoked immediately, and dropped at any time, independent of other
//! libraries. This crate enforces the invariant by construction: no
//! global registry exists, symbol handles carry loader lifetimes, and
//! lifecycle is strictly per-extension.

mod backend;
mod error;
mod symbol;

pub use error::{ExtensionError, IncompatibilityError};
pub use symbol::{ExtensionSymbol, StaticRef, Symbol};

use core::mem::ManuallyDrop;
use notko::Outcome;

/// Opaque handle to a loaded dynamic library.
///
/// RAII: dropping the `Extension` closes the library on the OS side.
/// `Extension::close` is the explicit form that returns the OS close
/// result, for consumers that want to surface unload errors instead
/// of dropping them silently.
///
/// Every `Symbol<'_, T>` or `StaticRef<'_, T>` resolved from an
/// `Extension` borrows it, so the library remains loaded for the
/// lifetime of any outstanding symbol handles. Enforced by the
/// borrow checker.
pub struct Extension {
    handle: backend::PlatformHandle,
}

impl Extension {
    /// Open a shared library at the given path.
    ///
    /// `path` is a null-terminated byte sequence interpreted per the
    /// host platform's path conventions. Returns `PathNotFound` or
    /// `LoadFailed` on failure.
    pub fn load(path: &[u8]) -> Outcome<Self, ExtensionError> {
        match backend::platform_load(path) {
            Outcome::Ok(handle) => Outcome::Ok(Self { handle }),
            Outcome::Err(e) => Outcome::Err(e),
        }
    }

    /// Resolve a named *function-pointer* symbol from this library.
    ///
    /// `T` must satisfy the sealed `ExtensionSymbol` marker, namely
    /// an `extern "C"` function pointer of arity 0-8. For static-data
    /// symbols use `resolve_static` instead.
    pub fn resolve<T: ExtensionSymbol>(
        &self,
        name: &[u8],
    ) -> Outcome<Symbol<'_, T>, ExtensionError> {
        match backend::platform_resolve(self.handle, name) {
            Outcome::Ok(ptr) => Outcome::Ok(Symbol::from_raw(ptr)),
            Outcome::Err(e) => Outcome::Err(e),
        }
    }

    /// Resolve a named *static-data* symbol from this library.
    ///
    /// Returns a `StaticRef<'_, T>` whose `get()` dereferences to
    /// `&T`. Use for the typical pattern where a plugin exports a
    /// `#[no_mangle] pub static PLUGIN_DESCRIPTOR: PluginDescriptor
    /// = ...;`. The resolved symbol is the address of the static,
    /// not a function pointer to it.
    pub fn resolve_static<T: 'static>(
        &self,
        name: &[u8],
    ) -> Outcome<StaticRef<'_, T>, ExtensionError> {
        match backend::platform_resolve(self.handle, name) {
            Outcome::Ok(ptr) => Outcome::Ok(StaticRef::from_raw(ptr)),
            Outcome::Err(e) => Outcome::Err(e),
        }
    }

    /// Explicitly unload the library and surface the platform close
    /// result.
    ///
    /// Consumes `self` so the compiler prevents double-close. The
    /// `Drop` impl still works for the default RAII path; `close` is
    /// the explicit form for consumers that want to handle unload
    /// errors instead of discarding them.
    pub fn close(self) -> Outcome<(), ExtensionError> {
        // SAFETY: ManuallyDrop::new consumes self; the subsequent
        // manual close call performs the unload exactly once. The
        // inner Drop must not also run, which ManuallyDrop guarantees.
        let this = ManuallyDrop::new(self);
        backend::platform_close(this.handle)
    }
}

impl Drop for Extension {
    fn drop(&mut self) {
        // Platform close: the return code is advisory here; we have
        // no alloc path to propagate a failure, and RAII drop is
        // infallible by convention. Consumers that want to surface
        // unload errors call `close` instead.
        let _ = backend::platform_close(self.handle);
    }
}

/// Apply a narrow compatibility check against a raw version bytestring
/// the caller has previously resolved from a plugin.
///
/// This crate performs no manifest parsing; the bytes are passed
/// through to a caller-provided comparison. v1 treats any non-empty
/// byte slice as compatible; consumers layer richer policies on top.
pub fn compatibility_check(bytes: &[u8]) -> Outcome<(), IncompatibilityError> {
    if bytes.is_empty() {
        return Outcome::Err(IncompatibilityError::VersionSkew);
    }
    Outcome::Ok(())
}
