//! `AsStr` + `IntoStr`: ergonomic conversions at API boundaries.

use notko::Maybe;

use crate::handle::Str;
use crate::interner::{ArenaInterner, StringInterner}; // lint:allow(no-alloc) reason: interner wrapper import, not std `String`; tracked: #72

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
    /// the conversion requires interning, and `Maybe::Isnt` is its
    /// refusal: no room for the string.
    fn into_str(self, interner: &StringInterner<impl ArenaInterner>) -> Maybe<Str>; // lint:allow(no-alloc) reason: interner wrapper, not std `String`; tracked: #72
}

impl IntoStr for Str {
    #[rustfmt::skip]
    fn into_str(self, _: &StringInterner<impl ArenaInterner>) -> Maybe<Str> { // lint:allow(no-alloc) reason: interner wrapper, not std `String`; tracked: #72
        Maybe::Is(self)
    }
}

impl IntoStr for &'static str {
    #[rustfmt::skip]
    fn into_str(self, interner: &StringInterner<impl ArenaInterner>) -> Maybe<Str> { // lint:allow(no-alloc) reason: interner wrapper, not std `String`; tracked: #72
        interner.intern_static(self)
    }
}
