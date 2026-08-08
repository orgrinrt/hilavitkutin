//! `Str`: 4-byte interned string handle, a view over a sym-core `Sym`.
//!
//! `Str` is one domain of `hilavitkutin-sym`'s generic identity core. The
//! handle is a `Sym` whose domain `kind` is `STR_DOMAIN` (`0b000`); the
//! const-versus-runtime origin is the handle's flag bit. Both are byte-identical
//! to the previous standalone layout: a const handle is nibble `0b0000`, a
//! runtime handle `0b1000`.

use arvo::Bool;
use arvo_bits::{Bit, Bits, Hot};
use hilavitkutin_sym::{Sym, SymKind, SymLayout};

/// Interned string handle. 4 bytes everywhere. Comparison is integer equality.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Str(Sym);

impl Str {
    /// The string domain's tag (`0b000`). Const and runtime handles differ by
    /// the flag bit, not the tag.
    pub const STR_DOMAIN: SymKind = SymKind::from_raw(0b000);

    /// Mask for the runtime-origin bit (bit 31 = 1). The runtime origin is the
    /// `Sym` flag bit; `Str` re-exposes its mask under the historical name.
    pub const RUNTIME_MASK: Bits<32, Hot> = SymLayout::flag_MASK;
    /// Mask for the 28-bit ID (bits 27-0). Forwards to `SymLayout::id_MASK`.
    pub const ID_MASK: Bits<32, Hot> = SymLayout::id_MASK;

    /// Construct a const-origin `Str` from a 28-bit ID. Not for direct
    /// use: `str_const!()` is the only intended caller.
    #[doc(hidden)]
    pub const fn __make(id: Bits<28, Hot>) -> Self {
        Self(Sym::new(Self::STR_DOMAIN, id))
    }

    /// Construct a runtime-origin `Str` from a 28-bit ID. Not for direct
    /// use: `StringInterner` is the only intended caller.
    #[doc(hidden)]
    pub const fn __runtime(id: Bits<28, Hot>) -> Self {
        Self(Sym::new(Self::STR_DOMAIN, id).with_flag(Bit::<Hot>::from_raw(1)))
    }

    /// `true` if this handle was produced by `str_const!()`.
    pub const fn is_const(self) -> Bool {
        Bool(self.0.kind().to_bits().to_raw() == 0 && self.0.flag().to_raw() == 0)
    }

    /// `true` if this handle was produced by the runtime interner.
    pub const fn is_runtime(self) -> Bool {
        Bool(!self.is_const().0)
    }

    /// The 28-bit ID portion of this handle.
    pub const fn id(self) -> Bits<28, Hot> {
        self.0.id()
    }

    /// The underlying `Sym`. The sym-core view of this string handle.
    pub const fn as_sym(self) -> Sym {
        self.0
    }

    /// Wrap a `Sym` as a `Str` without checking its domain. Crate-internal:
    /// the string `Interner` impl checks `kind == STR_DOMAIN` before calling
    /// this, so every constructed `Str` still carries the string kind.
    pub(crate) const fn from_sym(sym: Sym) -> Self {
        Self(sym)
    }

    /// The domain tag of the underlying `Sym`.
    pub const fn kind(self) -> SymKind {
        self.0.kind()
    }

    /// The raw 32-bit handle as a `Bits<32, Hot>`. Substrate-typed
    /// view for tests, structural assertions, and persistence.
    pub const fn to_bits(self) -> Bits<32, Hot> {
        self.0.to_bits()
    }
}
