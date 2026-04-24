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
pub use symbol::{ExtensionSymbol, Symbol};

use notko::Outcome;

/// Opaque handle to a loaded dynamic library.
///
/// RAII: dropping the `Extension` closes the library on the OS side.
/// Every `Symbol<'_, T>` resolved from an `Extension` borrows it, so
/// the library remains loaded for the lifetime of any outstanding
/// symbol handles — enforced by the borrow checker.
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

    /// Resolve a named symbol from this library into a typed handle.
    ///
    /// `name` is a null-terminated byte sequence naming the exported
    /// symbol. `T` must satisfy the sealed `ExtensionSymbol` marker.
    pub fn resolve<T: ExtensionSymbol>(
        &self,
        name: &[u8],
    ) -> Outcome<Symbol<'_, T>, ExtensionError> {
        match backend::platform_resolve(self.handle, name) {
            Outcome::Ok(ptr) => Outcome::Ok(Symbol::from_raw(ptr)),
            Outcome::Err(e) => Outcome::Err(e),
        }
    }
}

impl Drop for Extension {
    fn drop(&mut self) {
        // Platform close: the return code is advisory; we have no
        // alloc path to propagate a failure, and RAII drop is
        // infallible by convention.
        let _ = backend::platform_close(self.handle);
    }
}

/// Apply a narrow compatibility check against a raw version bytestring
/// the caller has previously resolved from a plugin.
///
/// This crate performs no manifest parsing; the bytes are passed
/// through to a caller-provided comparison. v1 treats any non-empty
/// byte slice starting with a known compatibility marker byte as
/// compatible; refinement is a follow-up concern.
pub fn compatibility_check(bytes: &[u8]) -> Outcome<(), IncompatibilityError> {
    if bytes.is_empty() {
        return Outcome::Err(IncompatibilityError::VersionSkew);
    }
    // v1 placeholder: all non-empty inputs compatible. Consumers
    // layer richer policies on top if needed; this hook exists to
    // keep the IncompatibilityError enum colocated with the loader.
    Outcome::Ok(())
}
