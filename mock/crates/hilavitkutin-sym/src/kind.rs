//! `SymKind`: the domain tag on a [`Sym`](crate::Sym), at its shape's width.
//!
//! Kind-assignment table (documented convention, owned here so two domains
//! cannot claim one tag):
//!
//! Under [`Standard`](crate::Standard), which gives the tag three bits:
//!
//! - `0b000`: the string domain (`hilavitkutin-str`). Its const and runtime
//!   origins are the handle's flag bit, not the tag, so both are one domain.
//!   Fixed at `0b000` for byte-compatibility with baked string handles.
//! - `0b001`: a minted-binder domain used by a downstream compiler consumer,
//!   defined in that consumer's crate.
//! - `0b010` through `0b111`: free for future domains.

use arvo_bits::{Bits, Hot};

use crate::shape::{Standard, SymLayoutOps, SymShape};

/// The domain tag naming which domain a [`Sym`](crate::Sym) belongs to.
///
/// The tag is part of a `Sym`'s compared bits, so
/// two `Sym`s with different tags are never equal.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SymKind<S: SymShape = Standard>(<S::Layout as SymLayoutOps>::Kind);

impl<S: SymShape> SymKind<S> {
    /// Build a tag from a value at this shape's kind width.
    pub const fn new(raw: <S::Layout as SymLayoutOps>::Kind) -> Self
    where
        S: [const] SymShape,
    {
        Self(raw)
    }

    /// The tag value, at this shape's kind width.
    pub const fn to_bits(self) -> <S::Layout as SymLayoutOps>::Kind
    where
        S: [const] SymShape,
    {
        self.0
    }
}

impl SymKind<Standard> {
    /// Build a tag from a small literal. The convenience constructor domains
    /// use to declare their `KIND`.
    ///
    /// Only the default shape gets it, because the literal is three bits wide
    /// and a shape that chose a different kind width would silently truncate.
    /// A domain under another shape builds its tag with [`SymKind::new`].
    #[rustfmt::skip]
    pub const fn from_raw(raw: u8) -> Self { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ergonomic kind-literal constructor (definition-site helper-fn exception); the tag is a 3-bit value named by a literal; tracked: #34
        Self(Bits::<3, Hot>::from_raw(raw))
    }
}
