//! `SymKind`: the 3-bit domain tag on a [`Sym`](crate::Sym).
//!
//! Kind-assignment table (documented convention, owned here so two domains
//! cannot claim one tag):
//!
//! - `0b000`: the string domain (`hilavitkutin-str`). Its const and runtime
//!   origins are the handle's flag bit, not the tag, so both are one domain.
//!   Fixed at `0b000` for byte-compatibility with baked string handles.
//! - `0b001`: a minted-binder domain used by a downstream compiler consumer,
//!   defined in that consumer's crate.
//! - `0b010` through `0b111`: free for future domains.

use arvo_bits::{Bits, Hot};

/// The 3-bit domain tag naming which domain a [`Sym`](crate::Sym) belongs to.
///
/// Eight domains are available. The tag is part of a `Sym`'s compared bits, so
/// two `Sym`s with different tags are never equal.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SymKind(Bits<3, Hot>);

impl SymKind {
    /// Build a tag from its 3-bit value.
    pub const fn new(raw: Bits<3, Hot>) -> Self {
        Self(raw)
    }

    /// Build a tag from a small literal. The convenience constructor domains
    /// use to declare their `KIND`.
    pub const fn from_raw(raw: u8) -> Self { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: ergonomic kind-literal constructor (definition-site helper-fn exception); the tag is a 3-bit value named by a literal; tracked: #34
        Self(Bits::<3, Hot>::from_raw(raw))
    }

    /// The 3-bit tag value.
    pub const fn to_bits(self) -> Bits<3, Hot> {
        self.0
    }
}
