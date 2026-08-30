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

    #[rustfmt::skip]
    fn id_from_raw(raw: u32) -> Self::Id { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: forwarding a raw id into the bitfield's own id type, at arvo's from_raw boundary; tracked: #34
        Bits::<28, Hot>::from_raw(raw)
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

    #[rustfmt::skip]
    fn id_from_raw(raw: u32) -> Self::Id { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: forwarding a raw id into the bitfield's own id type, at arvo's from_raw boundary; tracked: #34
        Bits::<26, Hot>::from_raw(raw)
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
/// The impls below are hand-written rather than derived, and that is
/// load-bearing. A derive bounds them on `S`, and `SymShape` requires only
/// `Copy + 'static`, so a legal shape deriving less would yield a handle with no
/// equality at all: this type's central promise absent by construction. Every
/// shipped shape derives everything, which is exactly why nothing noticed.
/// Bounding on the layout instead asks for what the code actually uses.
#[repr(transparent)]
pub struct Sym<S: SymShape = Standard>(S::Layout);

impl<S: SymShape> Clone for Sym<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: SymShape> Copy for Sym<S> {}

impl<S: SymShape> PartialEq for Sym<S> {
    #[rustfmt::skip]
    fn eq(&self, other: &Self) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: `PartialEq::eq` declares its own return type; an impl cannot change it (std trait method exception); tracked: #34
        self.0 == other.0
    }
}

impl<S: SymShape> Eq for Sym<S> {}

impl<S: SymShape> core::fmt::Debug for Sym<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Sym").field(&self.0).finish()
    }
}

impl<S: SymShape> core::hash::Hash for Sym<S>
where
    S::Layout: core::hash::Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<S: SymShape> Default for Sym<S> {
    fn default() -> Self {
        Self(<S::Layout as SymLayoutOps>::zeroed())
    }
}

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
