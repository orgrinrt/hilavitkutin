//! `Sym`: the 4-byte generic interned-identity handle.
//!
//! The layout below is [`Standard`](crate::Standard)'s. Another shape divides
//! the same 32 bits differently, and two shapes are not comparable.
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

const impl crate::shape::SymLayoutOps for SymLayout {
    const ID_MASK: Bits<32, Hot> = Self::id_MASK;
    const KIND_MASK: Bits<32, Hot> = Self::kind_MASK;

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

bitfield! {
    /// Layout carrier for the wide-tag shape: five bits of kind over a
    /// twenty-six bit id. Declared here beside `SymLayout` because a shape
    /// brings its own layout, and the positions are literals written where an
    /// impl may write them.
    pub struct WideKindLayout: 32 {
        /// Domain-private flag.
        flag: 1 at 31,
        /// 5-bit domain tag, thirty-two domains.
        kind: 5 at 26,
        /// 26-bit interned identity.
        id: 26 at 0,
    }
}

const impl crate::shape::SymLayoutOps for WideKindLayout {
    const ID_MASK: Bits<32, Hot> = Self::id_MASK;
    const KIND_MASK: Bits<32, Hot> = Self::kind_MASK;

    type Id = Bits<26, Hot>;
    type Kind = Bits<5, Hot>;

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
    pub const fn new(kind: SymKind<S>, id: <S::Layout as SymLayoutOps>::Id) -> Self
    where
        S: [const] SymShape,
    {
        Self(S::Layout::zeroed().set_kind(kind.to_bits()).set_id(id))
    }

    /// Return a copy of this handle with its domain-private flag set to `bit`.
    pub const fn with_flag(self, bit: Bit<Hot>) -> Self
    where
        S: [const] SymShape,
    {
        Self(self.0.set_flag(bit))
    }

    /// The domain tag.
    pub const fn kind(self) -> SymKind<S>
    where
        S: [const] SymShape,
    {
        SymKind::new(self.0.get_kind())
    }

    /// The 28-bit id portion.
    pub const fn id(self) -> <S::Layout as SymLayoutOps>::Id
    where
        S: [const] SymShape,
    {
        self.0.get_id()
    }

    /// The domain-private flag bit.
    pub const fn flag(self) -> Bit<Hot>
    where
        S: [const] SymShape,
    {
        self.0.get_flag()
    }

    /// The raw 32-bit handle. Substrate-typed view for tests, structural
    /// assertions, and persistence.
    pub const fn to_bits(self) -> Bits<32, Hot>
    where
        S: [const] SymShape,
    {
        self.0.raw_bits()
    }
}
