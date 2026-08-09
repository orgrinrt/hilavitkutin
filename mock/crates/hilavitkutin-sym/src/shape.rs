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

use arvo::strategy::{BitsContainerFor, Identity, Unsigned};
use arvo::traits::FromConstant;
use arvo::{USize, Uint};
use arvo_bits::{Bit, Bits, Hot};

/// How a `Sym`'s 32 bits divide, and how many independent minters it admits.
///
/// The four widths divide the handle: one flag bit, `KIND_BITS` of domain
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

    /// **Which minter this origin is**, not where its run begins.
    ///
    /// The run is derived from it, so a shape has no pair of numbers to get
    /// wrong. Two earlier designs let a shape supply a base and a ceiling
    /// directly: the first placed origins past the id field, the second placed
    /// them inside it but spaced by less than a counter span, and two minters
    /// collided while every law written for the first case passed.
    ///
    /// A shape with one minter returns zero for its single origin.
    fn origin_index(origin: Self::Origin) -> USize;

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
    /// The domain tag, in a type whose width is the field's real width.
    ///
    /// **The type carries the width**, and that is load-bearing rather than
    /// incidental. `get_kind` returns this and its body returns what the
    /// bitfield hands back, so naming a width the field does not have is a type
    /// error. A mask constant forwarded by hand had no such guarantee, because
    /// nothing read it.
    type Kind: Copy + Eq + core::fmt::Debug + [const] FieldWidth;
    /// The id, in a type whose width is the field's real width.
    type Id: Copy + Eq + core::fmt::Debug + [const] FieldWidth;

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

/// Where in the id space a minter's counter starts.
///
/// **A free function, so no shape can supply a different answer.** Index `i`
/// owns `[i << COUNTER_BITS, ((i + 1) << COUNTER_BITS) - 1]`, and distinct
/// indices own disjoint runs by arithmetic.
///
/// This was a provided trait method for exactly one revision, which was wrong
/// for a reason worth recording: **a provided method is overridable**, so a
/// shape could still supply a colliding base and the illegal state stayed
/// representable. Verified by writing that shape and watching it compile. Free
/// functions are the same instrument `width_of` used, and for the same reason.
#[rustfmt::skip]
pub const fn origin_base<S: [const] SymShape>(origin: S::Origin) -> u32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the raw id-space coordinate at the one id-allocator boundary; tracked: #34
    (S::origin_index(origin).0 as u32) << S::COUNTER_BITS.0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the index projected to the raw id-space coordinate, at arvo's own to_raw boundary; tracked: #34
}

/// The largest id a minter may reach before it is exhausted.
///
/// Free for the same reason as [`origin_base`], and derived from it.
#[rustfmt::skip]
pub const fn origin_ceiling<S: [const] SymShape>(origin: S::Origin) -> u32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the raw id-space coordinate, derived from origin_base at the same boundary; tracked: #34
    origin_base::<S>(origin) + ((1u32 << S::COUNTER_BITS.0) - 1)
}

/// How wide the type carrying a field happens to be.
///
/// Blanket over `Bits`, so no layout can supply a different answer, and
/// `specialization` is forbidden here, which makes that a guarantee rather than
/// a convention.
///
/// **This is one link of the guarantee, not the whole of it**, and saying
/// otherwise was the last mistake this class produced. It pins the declared
/// width to the type the accessors are compiled against. It does not pin the
/// field behind them, because an accessor may convert: a layout declaring a
/// nine-bit `Kind` over a genuine five-bit field, widening on the way out and
/// truncating on the way in, satisfies every signature and passes every law
/// that reads a declared width against a declared type.
///
/// What pins the field is the round trip in `tests/layout.rs`. A field maximum
/// written and read back unchanged fails the moment the field is narrower than
/// the type in front of it, and the whole-handle law fails with it, because the
/// vacated bits then belong to nothing.
///
/// So the route runs declaration, type, round trip, bits that land. Four rounds
/// each named one link and called the question closed; the previous one counted
/// widths from mask constants no accessor read, so a layout could forward any
/// mask at all and everything passed.
pub const trait FieldWidth {
    /// The field's width, in bits.
    const WIDTH: USize;
}

#[rustfmt::skip]
const impl<const N: u16, S: BitsContainerFor<N, Unsigned>> FieldWidth for Bits<N, S> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: `Bits`'s own width parameter is declared `u16` by arvo; naming it is the definition-site of this blanket impl; tracked: #34
    const WIDTH: USize = USize(N as usize); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: projecting arvo's own `u16` width parameter into `USize`, the arvo type this returns; tracked: #34
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

    /// One minter, so index zero, and the derived run is the whole id space.
    fn origin_index(_: Self::Origin) -> USize {
        <USize as Identity>::ZERO
    }

    #[rustfmt::skip]
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

    /// The minter's own number. Where its run begins is derived from this and
    /// `COUNTER_BITS`, so the shift that used to be written here, and could
    /// disagree with `COUNTER_BITS`, no longer exists.
    #[rustfmt::skip]
    fn origin_index(origin: Self::Origin) -> USize {
        USize(origin.0.to_raw() as usize) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: reading the origin's own 4-bit value through arvo's to_raw; tracked: #34
    }

    #[rustfmt::skip]
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

    /// One minter, so index zero.
    fn origin_index(_: Self::Origin) -> USize {
        <USize as Identity>::ZERO
    }

    #[rustfmt::skip]
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
        fn check<S: SymShape>(name: &'static str) {
            let total = S::KIND_BITS.0 + S::ORIGIN_BITS.0 + S::COUNTER_BITS.0 + 1;
            assert_eq!(total, 32, "{name} does not account for the whole handle");
        }
        check::<Standard>("Standard");
        check::<SixteenMinters>("SixteenMinters");
        check::<WideKind>("WideKind");
    }

    /// The id is the origin and the counter together, for every shape. A shape
    /// that says otherwise mints into bits its id accessor cannot read.
    #[test]
    fn every_shape_id_is_origin_plus_counter() {
        fn check<S: SymShape>(name: &'static str) {
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
    /// trait is a parameter over one value.
    ///
    /// Stated as a difference between two shapes rather than as either one's
    /// value, because asserting a constant equals the literal its own impl
    /// declares cannot fail.
    #[test]
    fn the_kind_width_varies_across_shapes() {
        assert_ne!(
            Standard::KIND_BITS.0,
            WideKind::KIND_BITS.0,
            "two shapes must differ in tag width or the trait varies nothing"
        );
    }

    /// **The law that makes every other width mean something.**
    ///
    /// A shape declares widths; its layout has them. Nothing tied the two, so a
    /// shape could declare a three-bit field is nine bits wide and pass every
    /// law, which made the declarations descriptions rather than constraints.
    #[test]
    fn every_shape_declares_the_widths_its_layout_actually_has() {
        fn check<S: SymShape>(name: &'static str) {
            assert_eq!(
                S::KIND_BITS.0,
                <<S::Layout as SymLayoutOps>::Kind as FieldWidth>::WIDTH.0,
                "{name} declares a kind width its layout does not carry"
            );
            assert_eq!(
                S::ID_BITS.0,
                <<S::Layout as SymLayoutOps>::Id as FieldWidth>::WIDTH.0,
                "{name} declares an id width its layout does not carry"
            );
        }
        check::<Standard>("Standard");
        check::<SixteenMinters>("SixteenMinters");
        check::<WideKind>("WideKind");
    }

    // The span law that stood here is gone, and its removal is a result rather
    // than a loss. It asserted `origin_ceiling - origin_base == 2^COUNTER_BITS
    // - 1`, which the derivation now computes: `ceiling` is defined as `base +
    // (1 << COUNTER_BITS) - 1`. A test asserting what its subject computes is a
    // tautology and cannot fail, so it is deleted rather than kept for the
    // count. That the law became tautological is the evidence the derivation
    // subsumes it.

    /// Whether `S` maps distinct origins to distinct indices.
    ///
    /// **A verdict rather than an assertion**, so the same law serves the
    /// shipped shapes and a shape built to break it. A law that can only assert
    /// cannot be pointed at a liar.
    ///
    /// This is all that remains testable about origins. Where a run *begins* is
    /// derived from the index, so overlapping runs are unwritable; what a shape
    /// can still get wrong is its own mapping.
    #[rustfmt::skip]
    fn origins_map_to_distinct_indices<S: SymShape>(origins: &[S::Origin]) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a test-local verdict function; `bool` is the predicate's own return and never crosses a public surface; tracked: #34
        origins.iter().enumerate().all(|(i, a)| {
            origins
                .iter()
                .skip(i + 1)
                .all(|b| S::origin_index(*a).0 != S::origin_index(*b).0)
        })
    }

    /// And that every index fits the width the shape reserved for it.
    #[rustfmt::skip]
    fn origin_indices_fit_origin_bits<S: SymShape>(origins: &[S::Origin]) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a test-local verdict function; `bool` is the predicate's own return and never crosses a public surface; tracked: #34
        let count = 1usize << S::ORIGIN_BITS.0;
        origins.iter().all(|o| S::origin_index(*o).0 < count)
    }

    #[rustfmt::skip]
    fn all_minter_ids() -> [MinterId; 16] {
        core::array::from_fn(|i| MinterId(Uint::<4, Hot>::from_raw(i as u8))) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: enumerating the sixteen origins; the index crosses arvo's own from_raw boundary; tracked: #34
    }

    #[test]
    fn every_shipped_shape_maps_origins_injectively() {
        assert!(origins_map_to_distinct_indices::<Standard>(&[OneOrigin]));
        assert!(origins_map_to_distinct_indices::<WideKind>(&[OneOrigin]));
        assert!(origins_map_to_distinct_indices::<SixteenMinters>(
            &all_minter_ids()
        ));
    }

    #[test]
    fn every_shipped_index_fits_its_origin_bits() {
        assert!(origin_indices_fit_origin_bits::<Standard>(&[OneOrigin]));
        assert!(origin_indices_fit_origin_bits::<WideKind>(&[OneOrigin]));
        assert!(origin_indices_fit_origin_bits::<SixteenMinters>(
            &all_minter_ids()
        ));
    }

    /// A shape that maps two origins onto one index.
    ///
    /// **This is the defect that survives the new mechanism**, and it is a
    /// smaller one: it is an error in the shape's own mapping rather than in
    /// the id-space arithmetic, and it cannot make two *distinct* minters
    /// collide, only make two names for one minter.
    ///
    /// Round eleven's liar, which placed origins past the id field, and the
    /// eighth review's, which placed them inside it but overlapping, are both
    /// **unwritable now**: there is no base or ceiling for a shape to supply.
    /// That is the result of this round, and it is recorded here because a
    /// deleted test otherwise reads as coverage lost rather than as a case that
    /// stopped existing.
    #[derive(Copy, Clone, Default, Eq, Hash, PartialEq, Debug)]
    struct TwoOriginsOneIndex;

    const impl SymShape for TwoOriginsOneIndex {
        type Layout = crate::handle::SymLayout;
        type Origin = MinterId;

        const COUNTER_BITS: USize = USize(24);
        const ID_BITS: USize = USize(28);
        const KIND_BITS: USize = USize(3);
        const ORIGIN_BITS: USize = USize(4);

        #[rustfmt::skip]
        fn origin_index(_: Self::Origin) -> USize {
            USize(0) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the deliberate defect this shape carries, mapping every origin onto one index; tracked: #34
        }

        #[rustfmt::skip]
        fn id_from_counter(counter: Uint<28, Hot>) -> <Self::Layout as SymLayoutOps>::Id {
            Bits::<28, Hot>::from_raw(counter.to_raw()) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Uint-to-Bits id projection at the id-allocator boundary; tracked: #34
        }
    }

    #[test]
    fn the_injectivity_law_rejects_a_shape_that_collapses_its_origins() {
        assert!(
            !origins_map_to_distinct_indices::<TwoOriginsOneIndex>(&all_minter_ids()),
            "a shape mapping every origin onto one index must be rejected"
        );
    }

    /// And it passes every width law, which is why a width law was never going
    /// to be the instrument for this.
    #[test]
    fn the_collapsing_shape_still_satisfies_every_width_law() {
        type S = TwoOriginsOneIndex;
        assert_eq!(
            S::KIND_BITS.0 + S::ORIGIN_BITS.0 + S::COUNTER_BITS.0 + 1,
            32
        );
        assert_eq!(S::ID_BITS.0, S::ORIGIN_BITS.0 + S::COUNTER_BITS.0);
    }

    /// **Disjointness is arithmetic now, not an assertion.**
    ///
    /// Derived runs cannot overlap for distinct indices, checked over every
    /// pair a four-bit origin admits. This is the property the two previous
    /// rounds each tried to catch with a law aimed at the construction they had
    /// just found.
    #[test]
    fn derived_runs_never_overlap_for_distinct_indices() {
        let ids = all_minter_ids();
        for (i, a) in ids.iter().enumerate() {
            for b in ids.iter().skip(i + 1) {
                let (ab, ac) = (
                    origin_base::<SixteenMinters>(*a),
                    origin_ceiling::<SixteenMinters>(*a),
                );
                let (bb, bc) = (
                    origin_base::<SixteenMinters>(*b),
                    origin_ceiling::<SixteenMinters>(*b),
                );
                assert!(
                    ac < bb || bc < ab,
                    "runs [{ab}, {ac}] and [{bb}, {bc}] overlap"
                );
            }
        }
    }
}
