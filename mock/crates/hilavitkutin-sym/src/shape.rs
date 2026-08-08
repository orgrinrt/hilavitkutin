//! `SymShape`: how a [`Sym`](crate::Sym)'s 32 bits divide.
//!
//! Three bits of domain tag and twenty-eight of id suits one consumer. A
//! consumer with four domains pays for four it cannot use; one with sixteen
//! cannot have them. That is a consumer's choice wearing a substrate constant,
//! so it is lifted into a trait and the current split becomes one impl.
//!
//! The widths total 32, so they are related by arithmetic. **That arithmetic
//! lives in each impl**, where expressions are legal, rather than in a bound
//! where it would need an inline const expression and therefore a forbidden
//! feature. Every bound names the trait instead.
//!
//! A shape is a wire format. Two `Sym`s built under different shapes divide the
//! same 32 bits differently, so they are not comparable and must not meet in one
//! store.

use arvo::USize;
use arvo_bits::{Bit, Bits, Hot};

/// How a `Sym`'s 32 bits divide, and how many independent minters it admits.
///
/// The four widths partition the handle: one flag bit, `KIND_BITS` of domain
/// tag, and the rest split between an origin naming the minter and a counter
/// running within it. `ORIGIN_BITS` of zero means one minter, which is the
/// single-producer case and what [`Standard`] gives.
pub trait SymShape: Copy + 'static {
    /// The 32-bit layout carrier for this shape.
    ///
    /// Each shape declares its own via `arvo::bitfield!`, so the field
    /// positions are literals written where an impl may write them. That is
    /// what keeps the derivation out of type position, where it would need an
    /// inline const expression and therefore a forbidden feature.
    type Layout: SymLayoutOps;

    /// Bits naming the domain. `1 << KIND_BITS` domains are available.
    const KIND_BITS: USize;

    /// Bits naming which minter produced a handle. Zero means one minter.
    const ORIGIN_BITS: USize;

    /// Bits counting within one minter. Derived here rather than in a bound:
    /// `32 - 1 - KIND_BITS - ORIGIN_BITS`.
    const COUNTER_BITS: USize;

    /// The id field's width, origin and counter together.
    const ID_BITS: USize;

    /// How many minters this shape admits.
    const ORIGINS: USize;
}

/// What a shape's layout carrier must offer.
///
/// `arvo::bitfield!` generates these as inherent methods rather than a trait
/// impl, so each shape writes a short forwarding impl. The forwarding is
/// mechanical; the positions it forwards to are the shape's own.
pub trait SymLayoutOps: Copy + Eq + core::fmt::Debug {
    /// The domain tag at this layout's kind width.
    type Kind: Copy + Eq + core::fmt::Debug;
    /// The id at this layout's id width.
    type Id: Copy + Eq + core::fmt::Debug;

    /// A layout with every field zero.
    fn zeroed() -> Self;
    /// Replace the domain tag.
    fn set_kind(self, kind: Self::Kind) -> Self;
    /// Replace the id.
    fn set_id(self, id: Self::Id) -> Self;
    /// Replace the domain-private flag.
    fn set_flag(self, flag: Bit<Hot>) -> Self;
    /// The domain tag.
    fn get_kind(self) -> Self::Kind;
    /// The id.
    fn get_id(self) -> Self::Id;
    /// The domain-private flag.
    fn get_flag(self) -> Bit<Hot>;
    /// The whole 32 bits, which is what comparison and persistence see.
    fn raw_bits(self) -> Bits<32, Hot>;
}

/// The default shape, and byte-identical to the layout every current consumer
/// already uses: three bits of kind, one minter, twenty-eight of counter.
///
/// The string domain's `0b000` tag and every baked string handle survive it
/// unchanged, because nothing about this split moves.
#[derive(Copy, Clone, Default, Eq, Hash, PartialEq, Debug)]
pub struct Standard;

impl SymShape for Standard {
    type Layout = crate::handle::SymLayout;

    const COUNTER_BITS: USize = USize(28);
    const ID_BITS: USize = USize(28);
    const KIND_BITS: USize = USize(3);
    const ORIGINS: USize = USize(1);
    const ORIGIN_BITS: USize = USize(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The widths must total the handle, or the layout has a hole or an
    /// overlap and every accessor is wrong.
    #[test]
    fn standard_widths_account_for_every_bit() {
        let total = Standard::KIND_BITS.0 + Standard::ORIGIN_BITS.0 + Standard::COUNTER_BITS.0 + 1; // the flag
        assert_eq!(total, 32, "shape must partition the 32-bit handle");
    }

    /// `ID_BITS` is the origin and counter together, and a shape that says
    /// otherwise would mint into bits the id accessor cannot read.
    #[test]
    fn standard_id_is_origin_plus_counter() {
        assert_eq!(
            Standard::ID_BITS.0,
            Standard::ORIGIN_BITS.0 + Standard::COUNTER_BITS.0
        );
    }

    /// Zero origin bits is one minter, not zero minters.
    #[test]
    fn standard_admits_exactly_one_minter() {
        assert_eq!(Standard::ORIGINS.0, 1);
        assert_eq!(Standard::ORIGIN_BITS.0, 0);
    }
}
