//! `Sym`: the 4-byte generic interned-identity handle.
//!
//! Bit layout (declared on `SymLayout` via `arvo::bitfield!`):
//! - bits 27 to 0: 28-bit `id` (268M identities per domain)
//! - bits 30 to 28: 3-bit `kind` (the domain tag, eight domains)
//! - bit 31: 1-bit `flag` (a domain-private flag; the string domain uses it
//!   for the const-versus-runtime origin, a domain that does not need it
//!   leaves it zero)

use arvo::bitfield;
use arvo_bits::{Bit, Bits, Hot};

use crate::kind::SymKind;

bitfield! {
    /// Internal layout carrier for `Sym`. Not part of the public API.
    pub struct SymLayout: 32 {
        /// Domain-private flag (string origin for the string domain).
        flag: 1 at 31,
        /// 3-bit domain tag.
        kind: 3 at 28,
        /// 28-bit interned identity.
        id: 28 at 0,
    }
}

/// Generic interned-identity handle. 4 bytes everywhere. Comparison is integer
/// equality across the whole layout, so two `Sym`s of different `kind` are
/// never equal whatever their ids or flags.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Sym(SymLayout);

impl Sym {
    /// Build a handle for `kind` with `id`, flag zero.
    pub const fn new(kind: SymKind, id: Bits<28, Hot>) -> Self {
        Self(SymLayout::new().with_kind(kind.to_bits()).with_id(id))
    }

    /// Return a copy of this handle with its domain-private flag set to `bit`.
    pub const fn with_flag(self, bit: Bit<Hot>) -> Self {
        Self(self.0.with_flag(bit))
    }

    /// The domain tag.
    pub const fn kind(self) -> SymKind {
        SymKind::new(self.0.kind())
    }

    /// The 28-bit id portion.
    pub const fn id(self) -> Bits<28, Hot> {
        self.0.id()
    }

    /// The domain-private flag bit.
    pub const fn flag(self) -> Bit<Hot> {
        self.0.flag()
    }

    /// The raw 32-bit handle. Substrate-typed view for tests, structural
    /// assertions, and persistence.
    pub const fn to_bits(self) -> Bits<32, Hot> {
        self.0.to_bits()
    }
}
