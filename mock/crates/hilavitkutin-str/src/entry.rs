//! `StaticStrEntry`: linker-section registration record.

use arvo_bits::{Bits, BitsRefitCtor as _};

use crate::handle::Str;
use crate::hash::const_fnv1a;

/// A single compile-time-registered string entry. Emitted by `str_const!()`
/// into the `.hilavitkutin_strings` linker section and read back by the
/// section walker at startup.
#[repr(C)]
pub struct StaticStrEntry {
    /// The const-origin `Str` handle (content hash truncated to 28 bits).
    pub hash:  Str,
    /// The original string literal.
    pub value: &'static str,
}

impl StaticStrEntry {
    /// Build the entry for a string literal.
    ///
    /// This is the one place an entry's hash is derived from its string:
    /// `str_const!` and the section anchor both come through it.
    pub const fn new(value: &'static str) -> Self {
        Self {
            hash: Str::__make(Bits::<28>::from_narrowed(const_fnv1a(value))),
            value,
        }
    }
}
