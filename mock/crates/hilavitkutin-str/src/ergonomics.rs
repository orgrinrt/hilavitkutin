//! `AsStr` + `IntoStr` — ergonomic conversions at API boundaries.

use crate::handle::Str;
use crate::interner::{ArenaInterner, StringInterner}; // lint:allow(no-alloc) -- interner wrapper import, not std `String`.

/// Cheap access to a carried `Str` handle.
pub trait AsStr {
    /// Return the carried `Str` handle.
    fn as_str(&self) -> Str;
}

impl AsStr for Str {
    fn as_str(&self) -> Str {
        *self
    }
}

/// Convert into a `Str`, interning through an [`StringInterner`] when
/// needed.
pub trait IntoStr {
    /// Convert `self` into a `Str`. `interner` is consulted only when
    /// the conversion requires interning.
    fn into_str(self, interner: &StringInterner<impl ArenaInterner>) -> Str; // lint:allow(no-alloc) -- interner wrapper, not std `String`.
}

impl IntoStr for Str {
    fn into_str(self, _: &StringInterner<impl ArenaInterner>) -> Str { // lint:allow(no-alloc) -- interner wrapper, not std `String`.
        self
    }
}

impl IntoStr for &'static str {
    fn into_str(self, interner: &StringInterner<impl ArenaInterner>) -> Str { // lint:allow(no-alloc) -- interner wrapper, not std `String`.
        interner.intern_static(self)
    }
}
