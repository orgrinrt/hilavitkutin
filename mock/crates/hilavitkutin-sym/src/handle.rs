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
use crate::shape::{Standard, SymLayoutOps, SymShape};

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

impl crate::shape::SymLayoutOps for SymLayout {
    type Id = Bits<28, Hot>;
    type Kind = Bits<3, Hot>;

    fn get_flag(self) -> Bit<Hot> {
        self.flag()
    }

    fn get_id(self) -> Self::Id {
        self.id()
    }

    fn get_kind(self) -> Self::Kind {
        self.kind()
    }

    fn raw_bits(self) -> Bits<32, Hot> {
        self.to_bits()
    }

    fn set_flag(self, flag: Bit<Hot>) -> Self {
        self.with_flag(flag)
    }

    fn set_id(self, id: Self::Id) -> Self {
        self.with_id(id)
    }

    fn set_kind(self, kind: Self::Kind) -> Self {
        self.with_kind(kind)
    }

    fn zeroed() -> Self {
        Self::new()
    }
}

/// Generic interned-identity handle. 4 bytes everywhere. Comparison is integer
/// equality across the whole layout, so two `Sym`s of different `kind` are
/// never equal whatever their ids or flags.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Sym<S: SymShape = Standard>(S::Layout);

impl<S: SymShape> Sym<S> {
    /// Build a handle for `kind` with `id`, flag zero.
    pub fn new(kind: SymKind<S>, id: <S::Layout as SymLayoutOps>::Id) -> Self {
        Self(S::Layout::zeroed().set_kind(kind.to_bits()).set_id(id))
    }

    /// Return a copy of this handle with its domain-private flag set to `bit`.
    pub fn with_flag(self, bit: Bit<Hot>) -> Self {
        Self(self.0.set_flag(bit))
    }

    /// The domain tag.
    pub fn kind(self) -> SymKind<S> {
        SymKind::new(self.0.get_kind())
    }

    /// The 28-bit id portion.
    pub fn id(self) -> <S::Layout as SymLayoutOps>::Id {
        self.0.get_id()
    }

    /// The domain-private flag bit.
    pub fn flag(self) -> Bit<Hot> {
        self.0.get_flag()
    }

    /// The raw 32-bit handle. Substrate-typed view for tests, structural
    /// assertions, and persistence.
    pub fn to_bits(self) -> Bits<32, Hot> {
        self.0.raw_bits()
    }
}

impl Sym<Standard> {
    /// Build a default-shape handle in a const context.
    ///
    /// The generic methods above go through [`SymLayoutOps`], and a bound of
    /// the form `Layout: [const] SymLayoutOps` is not accepted on the pinned
    /// nightly, so const construction is offered for the default shape rather
    /// than reaching for a feature the workspace forbids. Every consumer that
    /// bakes a handle at compile time uses this shape.
    pub const fn new_const(kind: SymKind<Standard>, id: Bits<28, Hot>) -> Self {
        Self(SymLayout::new().with_kind(kind.to_bits()).with_id(id))
    }

    /// The default-shape handle's raw 32 bits, in a const context.
    pub const fn to_bits_const(self) -> Bits<32, Hot> {
        self.0.to_bits()
    }

    /// Replace the domain-private flag, in a const context.
    pub const fn with_flag_const(self, bit: Bit<Hot>) -> Self {
        Self(self.0.with_flag(bit))
    }

    /// The id portion, in a const context.
    pub const fn id_const(self) -> Bits<28, Hot> {
        self.0.id()
    }

    /// The domain tag, in a const context.
    pub const fn kind_const(self) -> SymKind<Standard> {
        SymKind::new(self.0.kind())
    }

    /// The domain-private flag, in a const context.
    pub const fn flag_const(self) -> Bit<Hot> {
        self.0.flag()
    }
}
