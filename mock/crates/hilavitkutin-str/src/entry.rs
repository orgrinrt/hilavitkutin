//! `StaticStrEntry` — linker-section registration record.

use crate::handle::Str;

/// A single compile-time-registered string entry. Emitted by `str_const!()`
/// into the `.hilavitkutin_strings` linker section and read back by the
/// section walker at startup.
#[repr(C)]
pub struct StaticStrEntry {
    /// The const-origin `Str` handle (content hash truncated to 28 bits).
    pub hash: Str,
    /// The original string literal.
    pub value: &'static str,
}
