//! `Str`: 4-byte interned string handle.
//!
//! Bit layout (nibble-aligned):
//! - bit 31: `0` = const (compile-time), `1` = runtime (arena)
//! - bits 30-28: reserved flags
//! - bits 27-0: 28-bit ID (268M unique entries)
//!
//! The layout is declared via `arvo::bitfield!`, which generates
//! the `#[repr(transparent)]` struct over `Bits<32, Hot>` plus
//! per-field accessors and setters typed as `Bits<W, Hot>`.

use arvo::{bitfield, Bool};
use arvo_bits::{Bit, Bits, Hot};

bitfield! {
    /// Internal layout carrier for `Str`. Not part of the public API.
    pub struct StrLayout: 32 {
        /// 1 = runtime-interned, 0 = compile-time.
        origin: 1 at 31,
        /// Reserved flag bits (unused today).
        reserved: 3 at 28,
        /// 28-bit interned identity.
        id: 28 at 0,
    }
}

/// Interned string handle. 4 bytes everywhere. Comparison is integer equality.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Str(StrLayout);

impl Str {
    /// Mask for the runtime-origin bit (bit 31 = 1). Forwards to
    /// `StrLayout::origin_MASK`: authoritative declaration on the
    /// layout.
    pub const RUNTIME_MASK: Bits<32, Hot> = StrLayout::origin_MASK;
    /// Mask for the 28-bit ID (bits 27-0). Forwards to
    /// `StrLayout::id_MASK`.
    pub const ID_MASK: Bits<32, Hot> = StrLayout::id_MASK;

    /// Construct a const-origin `Str` from a 28-bit ID. Not for direct
    /// use: `str_const!()` is the only intended caller.
    #[doc(hidden)]
    pub const fn __make(id: Bits<28, Hot>) -> Self {
        Self(StrLayout::new().with_id(id))
    }

    /// Construct a runtime-origin `Str` from a 28-bit ID. Not for direct
    /// use: `StringInterner` is the only intended caller.
    #[doc(hidden)]
    pub const fn __runtime(id: Bits<28, Hot>) -> Self {
        Self(StrLayout::new().with_id(id).with_origin(Bit::<Hot>::from_raw(1)))
    }

    /// `true` if this handle was produced by `str_const!()`.
    pub const fn is_const(self) -> Bool {
        Bool(self.0.origin().to_raw() == 0)
    }

    /// `true` if this handle was produced by the runtime interner.
    pub const fn is_runtime(self) -> Bool {
        Bool(!self.is_const().0)
    }

    /// The 28-bit ID portion of this handle.
    pub const fn id(self) -> Bits<28, Hot> {
        self.0.id()
    }

    /// The raw 32-bit handle as a `Bits<32, Hot>`. Substrate-typed
    /// view for tests, structural assertions, and persistence.
    pub const fn to_bits(self) -> Bits<32, Hot> {
        self.0.to_bits()
    }
}
