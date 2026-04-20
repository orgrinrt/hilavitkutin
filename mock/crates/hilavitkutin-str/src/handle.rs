//! `Str` — 4-byte interned string handle.
//!
//! Bit layout (nibble-aligned):
//! - bit 31: `0` = const (compile-time), `1` = runtime (arena)
//! - bits 30-28: reserved flags
//! - bits 27-0: 28-bit ID (268M unique entries)

/// Interned string handle. 4 bytes everywhere. Comparison is integer equality.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Str(pub u32);

impl Str {
    /// The const-origin polarity at bit 31: bit *cleared* means the
    /// handle was produced by `str_const!` (compile-time). This is a
    /// *flag pattern*, not a non-zero mask — its value is `0` because
    /// the runtime bit is the one set by `__runtime`. To test origin,
    /// call `is_const()` / `is_runtime()` — do not AND against this
    /// constant expecting a non-zero result on a const handle.
    pub const CONST_ORIGIN_FLAG: u32 = 0;
    /// Mask for the runtime-origin bit (bit 31 = 1).
    pub const RUNTIME_MASK: u32 = 1 << 31;
    /// Mask for the 28-bit ID (bits 27-0).
    pub const ID_MASK: u32 = 0x0FFF_FFFF;

    /// Construct a const-origin `Str` from a pre-masked ID. Not for direct
    /// use — `str_const!()` is the only intended caller.
    #[doc(hidden)]
    pub const fn __make(id: u32) -> Self {
        Self(id & Self::ID_MASK)
    }

    /// Construct a runtime-origin `Str` from a pre-masked ID. Not for direct
    /// use — `StringInterner` is the only intended caller.
    #[doc(hidden)]
    pub const fn __runtime(id: u32) -> Self {
        Self((id & Self::ID_MASK) | Self::RUNTIME_MASK)
    }

    /// `true` if this handle was produced by `str_const!()`.
    pub const fn is_const(self) -> bool {
        (self.0 & Self::RUNTIME_MASK) == 0
    }

    /// `true` if this handle was produced by the runtime interner.
    pub const fn is_runtime(self) -> bool {
        !self.is_const()
    }

    /// The 28-bit ID portion of this handle.
    pub const fn id(self) -> u32 {
        self.0 & Self::ID_MASK
    }
}
