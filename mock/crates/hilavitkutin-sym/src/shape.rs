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

use arvo::strategy::Identity;
use arvo::traits::FromConstant;
use arvo::{USize, Uint};
use arvo_bits::{Bit, Bits, Hot};

/// How a `Sym`'s 32 bits divide, and how many independent minters it admits.
///
/// The four widths partition the handle: one flag bit, `KIND_BITS` of domain
/// tag, and the rest split between an origin naming the minter and a counter
/// running within it. `ORIGIN_BITS` of zero means one minter, which is the
/// single-producer case and what [`Standard`] gives.
pub const trait SymShape: Copy + 'static {
    /// The 32-bit layout carrier for this shape.
    ///
    /// Each shape declares its own via `arvo::bitfield!`, so the field
    /// positions are literals written where an impl may write them. That is
    /// what keeps the derivation out of type position, where it would need an
    /// inline const expression and therefore a forbidden feature.
    type Layout: [const] SymLayoutOps;

    /// Bits naming the domain. `1 << KIND_BITS` domains are available.
    const KIND_BITS: USize;

    /// Bits naming which minter produced a handle. Zero means one minter.
    const ORIGIN_BITS: USize;

    /// Bits counting within one minter. Derived here rather than in a bound:
    /// `32 - 1 - KIND_BITS - ORIGIN_BITS`.
    const COUNTER_BITS: USize;

    /// The id field's width, origin and counter together.
    const ID_BITS: USize;

    /// Names which minter produced a handle.
    ///
    /// A shape with one minter sets this to [`OneOrigin`], whose only value is
    /// the single origin, so a generator for it needs no argument and cannot
    /// name a second. A shape with several sets it to a type with one value per
    /// minter, and then a generator cannot be built without saying which it is.
    type Origin: Copy + Eq + core::fmt::Debug;

    /// Where in the id space a minter's counter starts.
    ///
    /// Origins partition the counter space, so two minters at two origins
    /// cannot produce the same id however far either counts.
    fn origin_base(origin: Self::Origin) -> Uint<28, Hot>;

    /// The largest id a minter at `origin` may reach before it is exhausted.
    fn origin_ceiling(origin: Self::Origin) -> Uint<28, Hot>;

    /// Project a counter value into this shape's id field.
    ///
    /// The one low-level numeric edge of a producer, and it lives here rather
    /// than in the generator because each shape decides how wide its id is.
    fn id_from_counter(counter: Uint<28, Hot>) -> <Self::Layout as SymLayoutOps>::Id;
}

/// What a shape's layout carrier must offer.
///
/// `arvo::bitfield!` generates these as inherent methods rather than a trait
/// impl, so each shape writes a short forwarding impl. The forwarding is
/// mechanical; the positions it forwards to are the shape's own.
pub const trait SymLayoutOps: Copy + Eq + core::fmt::Debug {
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

/// The only origin a single-minter shape has.
///
/// A unit rather than a number, because a shape with one minter has nothing to
/// choose and a constructor taking it would invite a consumer to believe there
/// was a second.
#[derive(Copy, Clone, Default, Eq, Hash, PartialEq, Debug)]
pub struct OneOrigin;

const impl SymShape for Standard {
    type Layout = crate::handle::SymLayout;
    type Origin = OneOrigin;

    const COUNTER_BITS: USize = USize(28);
    const ID_BITS: USize = USize(28);
    const KIND_BITS: USize = USize(3);
    const ORIGIN_BITS: USize = USize(0);

    fn origin_base(_: Self::Origin) -> Uint<28, Hot> {
        <Uint<28, Hot> as Identity>::ZERO
    }

    fn origin_ceiling(_: Self::Origin) -> Uint<28, Hot> {
        // The whole id space, because there is nobody to share it with.
        <Uint<28, Hot> as FromConstant>::from_constant::<{ USize((1 << 28) - 1) }>()
        // lint:allow(no-bare-numeric) reason: the 28-bit id-space ceiling as a typed constant (definition-site literal); tracked: #34
    }

    fn id_from_counter(counter: Uint<28, Hot>) -> <Self::Layout as SymLayoutOps>::Id {
        Bits::<28, Hot>::from_raw(counter.to_raw()) // lint:allow(no-bare-numeric) reason: Uint-to-Bits id projection at the id-allocator boundary; to_raw/from_raw are arvo's container projections; tracked: #34
    }
}

/// A shape for a domain minted in up to sixteen independent places.
///
/// Four bits of the id name the minter and the remaining twenty-four count
/// within it, so two peers at different origins cannot collide however far
/// either counts. The kind width is unchanged, so this shape admits the same
/// eight domains as [`Standard`] and differs from it only in splitting the id.
///
/// It is **not** interoperable with `Standard`: a handle minted here divides
/// the same 32 bits differently, so the two must not meet in one store.
#[derive(Copy, Clone, Default, Eq, Hash, PartialEq, Debug)]
pub struct SixteenMinters;

/// Which of sixteen minters produced a handle.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MinterId(pub Uint<4, Hot>);

const impl SymShape for SixteenMinters {
    type Layout = crate::handle::SymLayout;
    type Origin = MinterId;

    const COUNTER_BITS: USize = USize(24);
    const ID_BITS: USize = USize(28);
    const KIND_BITS: USize = USize(3);
    const ORIGIN_BITS: USize = USize(4);

    fn origin_base(origin: Self::Origin) -> Uint<28, Hot> {
        // The minter's index shifted above its counter, so each origin owns a
        // contiguous run of ids and no two runs overlap. Done at the raw level
        // because `Mul` and `Shl` are not yet const-stable, at the same
        // id-allocator boundary the mint step already crosses.
        Uint::<28, Hot>::from_raw((origin.0.to_raw() as u32) << 24) // lint:allow(no-bare-numeric) reason: placing a 4-bit origin index above the 24-bit counter, at the id-allocator boundary; tracked: #34
    }

    fn origin_ceiling(origin: Self::Origin) -> Uint<28, Hot> {
        // The last id this minter owns: its base plus a full counter span.
        Uint::<28, Hot>::from_raw(((origin.0.to_raw() as u32) << 24) | 0x00FF_FFFF)
        // lint:allow(no-bare-numeric) reason: the per-origin counter ceiling at the id-allocator boundary; tracked: #34
    }

    fn id_from_counter(counter: Uint<28, Hot>) -> <Self::Layout as SymLayoutOps>::Id {
        Bits::<28, Hot>::from_raw(counter.to_raw()) // lint:allow(no-bare-numeric) reason: Uint-to-Bits id projection at the id-allocator boundary; to_raw/from_raw are arvo's container projections; tracked: #34
    }
}

/// A shape whose **tag** is wider, not merely whose id divides differently.
///
/// Five bits of kind, thirty-two domains, twenty-six of id, one minter. It
/// exists because a trait whose every implementation shares one layout has not
/// demonstrated the thing it was lifted for: until one shape's tag is a
/// different width, the kind width is a parameter over a single value.
///
/// Not interoperable with [`Standard`], like any other shape.
#[derive(Copy, Clone, Default, Eq, Hash, PartialEq, Debug)]
pub struct WideKind;

const impl SymShape for WideKind {
    type Layout = crate::handle::WideKindLayout;
    type Origin = OneOrigin;

    const COUNTER_BITS: USize = USize(26);
    const ID_BITS: USize = USize(26);
    const KIND_BITS: USize = USize(5);
    const ORIGIN_BITS: USize = USize(0);

    fn origin_base(_: Self::Origin) -> Uint<28, Hot> {
        <Uint<28, Hot> as Identity>::ZERO
    }

    fn origin_ceiling(_: Self::Origin) -> Uint<28, Hot> {
        <Uint<28, Hot> as FromConstant>::from_constant::<{ USize((1 << 26) - 1) }>()
        // lint:allow(no-bare-numeric) reason: the 26-bit id-space ceiling as a typed constant (definition-site literal); tracked: #34
    }

    fn id_from_counter(counter: Uint<28, Hot>) -> <Self::Layout as SymLayoutOps>::Id {
        Bits::<26, Hot>::from_raw(counter.to_raw()) // lint:allow(no-bare-numeric) reason: Uint-to-Bits id projection at the id-allocator boundary; to_raw/from_raw are arvo's container projections; tracked: #34
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every width a shape declares, summed, must be the whole handle. A shape
    /// with a hole or an overlap has every accessor wrong.
    ///
    /// Asserted for **every** shape rather than for the default alone:
    /// choosing which instantiations to check is choosing what not to find out.
    #[test]
    fn every_shape_accounts_for_all_thirty_two_bits() {
        fn check<S: SymShape>(name: &str) {
            let total = S::KIND_BITS.0 + S::ORIGIN_BITS.0 + S::COUNTER_BITS.0 + 1;
            assert_eq!(total, 32, "{name} does not partition the handle");
        }
        check::<Standard>("Standard");
        check::<SixteenMinters>("SixteenMinters");
        check::<WideKind>("WideKind");
    }

    /// The id is the origin and the counter together, for every shape. A shape
    /// that says otherwise mints into bits its id accessor cannot read.
    #[test]
    fn every_shape_id_is_origin_plus_counter() {
        fn check<S: SymShape>(name: &str) {
            assert_eq!(
                S::ID_BITS.0,
                S::ORIGIN_BITS.0 + S::COUNTER_BITS.0,
                "{name} id width disagrees with its parts"
            );
        }
        check::<Standard>("Standard");
        check::<SixteenMinters>("SixteenMinters");
        check::<WideKind>("WideKind");
    }

    /// The kind width genuinely varies across shipped shapes. Without this the
    /// trait is a parameter over one value and the branch's second purpose is
    /// undelivered.
    #[test]
    fn the_kind_width_varies_across_shapes() {
        assert_eq!(Standard::KIND_BITS.0, 3);
        assert_eq!(WideKind::KIND_BITS.0, 5);
        assert_ne!(
            Standard::KIND_BITS.0,
            WideKind::KIND_BITS.0,
            "two shapes must differ in tag width or the trait varies nothing"
        );
    }
}
