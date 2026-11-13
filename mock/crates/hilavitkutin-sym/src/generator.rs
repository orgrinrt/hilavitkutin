//! `Generator`: the mint-only producer for a generative domain.

use core::marker::PhantomData;

use arvo::traits::FromConstant;
use arvo::{Bool, USize, Uint};
use arvo_bits::Hot;
use notko::Maybe;

use crate::domain::GenerativeDomain;
use crate::handle::Sym;
use crate::shape::{OneOrigin, SymLayoutOps, SymShape};

/// One, the mint step.
const ONE: Uint<28, Hot> = <Uint<28, Hot> as FromConstant>::from_constant::<{ USize(1) }>(); // lint:allow(no-bare-numeric) reason: the increment step as a typed constant (definition-site literal); tracked: #34

/// Mints fresh handles for a generative domain: a monotonic counter over the
/// shape's counter width (28 bits under `Standard`, 24 under `SixteenMinters`,
/// 26 under `WideKind`), tagged with the domain's `KIND`. No allocation, no
/// arena.
///
/// A `Generator` can only be built for a [`GenerativeDomain`]. Building one for
/// an interning-only domain is a type error:
///
/// ```compile_fail,E0277
/// use hilavitkutin_sym::{Domain, Generator, InterningDomain, Standard, SymKind};
/// struct InterningOnly;
/// impl Domain for InterningOnly {
///     type Shape = Standard;
///     const KIND: SymKind = SymKind::from_raw(0b100);
/// }
/// impl InterningDomain for InterningOnly {
///     type Value = str;
/// }
/// // InterningOnly is not a GenerativeDomain, so this does not compile.
/// let _g = Generator::<InterningOnly>::single();
/// ```
///
/// The `Domain` impl above is complete and compiles on its own, so the refusal
/// is about the missing `GenerativeDomain` and nothing else. Previously it
/// omitted `type Shape`, which made the block fail for two reasons at once and
/// shipped an uncompilable `Domain` example to anyone reading this page:
///
/// ```
/// use hilavitkutin_sym::{Domain, InterningDomain, Standard, SymKind};
/// struct InterningOnly;
/// impl Domain for InterningOnly {
///     type Shape = Standard;
///
///     const KIND: SymKind = SymKind::from_raw(0b100);
/// }
/// impl InterningDomain for InterningOnly {
///     type Value = str;
/// }
/// ```
pub struct Generator<D: GenerativeDomain> {
    next:      Uint<28, Hot>,
    ceiling:   Uint<28, Hot>,
    exhausted: Bool,
    _domain:   PhantomData<D>,
}

impl<D: GenerativeDomain> Generator<D> {
    /// A fresh generator for `origin`.
    ///
    /// Origins divide the id space by construction. Two generators at
    /// two origins cannot produce the same handle however far either counts,
    /// because each starts at its own base and stops at its own ceiling.
    ///
    /// A shape with one minter has one origin and its constructor is
    /// [`Generator::single`], which needs no argument because there is nothing
    /// to choose. Under such a shape two generators do agree, and they agree
    /// because the shape says there is one minter, not because a default was
    /// applied where a choice belonged.
    pub fn at(origin: <D::Shape as SymShape>::Origin) -> Maybe<Self> {
        if !crate::shape::origin_index_in_range::<D::Shape>(origin) {
            return Maybe::Isnt;
        }
        Maybe::Is(Self {
            next:      Uint::<28, Hot>::from_raw(crate::shape::origin_base::<D::Shape>(origin)),
            ceiling:   Uint::<28, Hot>::from_raw(crate::shape::origin_ceiling::<D::Shape>(origin)),
            exhausted: Bool(false),
            _domain:   PhantomData,
        })
    }

    /// Mint a fresh handle. Returns `Maybe::Isnt` once this origin's range
    /// is exhausted, rather than wrapping the counter, reissuing a live id, or
    /// walking into the next origin's range.
    pub fn mint(&mut self) -> Maybe<Sym<D::Shape>> {
        if self.exhausted.0 {
            return Maybe::Isnt;
        }
        // The shape owns the projection, because it owns how wide the id is.
        let id = <<D::Shape as SymShape>::Layout as SymLayoutOps>::id_from_raw(self.next.to_raw());
        let sym = Sym::<D::Shape>::new(D::KIND, id);
        if self.next == self.ceiling {
            self.exhausted = Bool(true);
        } else {
            self.next = self.next + ONE;
        }
        Maybe::Is(sym)
    }
}

impl<D: GenerativeDomain> Generator<D> {
    /// A fresh generator for a domain minted in exactly one place.
    ///
    /// Offered on every shape whose origin is [`OneOrigin`], `Standard` and
    /// `WideKind` among them, and on no other. A shape with several origins
    /// has no `single`, because there is a choice to make, and asking for it
    /// is a compile error. The bound sits on the function and not on the
    /// impl, because an inherent impl cannot carry a `[const]` bound.
    ///
    /// ```compile_fail,E0271
    /// use hilavitkutin_sym::{Domain, GenerativeDomain, Generator, SixteenMinters, SymKind};
    /// struct Peers;
    /// impl Domain for Peers {
    ///     type Shape = SixteenMinters;
    ///
    ///     const KIND: SymKind = SymKind::from_raw(0b010);
    /// }
    /// impl GenerativeDomain for Peers {}
    /// // Sixteen minters means sixteen origins, so `at` is the way in.
    /// let _g = Generator::<Peers>::single();
    /// ```
    ///
    /// Total rather than fallible: the one origin's index must be inside the
    /// range the shape reserved, and the function asserts it. In a constant
    /// that is a build error for the shape that says otherwise, and anywhere
    /// else a panic at the first call, since a shape with one origin and no
    /// room for it is a defect in the shape. Going through [`Generator::at`]
    /// would force every caller to handle a refusal that cannot happen.
    /// Pinned by `single_is_total_on_every_one_origin_shape`, by the two
    /// panic tests, and by the doctests below: a one-origin shape that
    /// reports index 1 against zero reserved bits is a build error in a
    /// constant,
    ///
    /// ```compile_fail,E0080
    /// #![feature(const_trait_impl)]
    /// use arvo::USize;
    /// use hilavitkutin_sym::shape::OneOrigin;
    /// use hilavitkutin_sym::{Domain, GenerativeDomain, Generator, Standard, SymKind, SymShape};
    /// #[derive(Copy, Clone)]
    /// struct NoRoom;
    /// const impl SymShape for NoRoom {
    ///     type Layout = <Standard as SymShape>::Layout;
    ///     type Origin = OneOrigin;
    ///     const COUNTER_BITS: USize = <Standard as SymShape>::COUNTER_BITS;
    ///     const ID_BITS: USize = <Standard as SymShape>::ID_BITS;
    ///     const KIND_BITS: USize = <Standard as SymShape>::KIND_BITS;
    ///     const ORIGIN_BITS: USize = <Standard as SymShape>::ORIGIN_BITS;
    ///     fn origin_index(_: Self::Origin) -> USize { USize(1) }
    /// }
    /// struct Peers;
    /// impl Domain for Peers {
    ///     type Shape = NoRoom;
    ///     const KIND: SymKind<NoRoom> = SymKind::new(arvo_bits::Bits::from_raw(0b010));
    /// }
    /// impl GenerativeDomain for Peers {}
    /// const G: Generator<Peers> = Generator::single();
    /// fn main() { let _g = G; }
    /// ```
    ///
    /// and the same shape with index 0, the one in range, builds, so the
    /// refusal above is the range check's and nothing else's:
    ///
    /// ```
    /// #![feature(const_trait_impl)]
    /// use arvo::USize;
    /// use hilavitkutin_sym::shape::OneOrigin;
    /// use hilavitkutin_sym::{
    ///     Domain,
    ///     GenerativeDomain,
    ///     Generator,
    ///     Standard,
    ///     SymKind,
    ///     SymShape,
    /// };
    /// #[derive(Copy, Clone)]
    /// struct Room;
    /// const impl SymShape for Room {
    ///     type Layout = <Standard as SymShape>::Layout;
    ///     type Origin = OneOrigin;
    ///
    ///     const COUNTER_BITS: USize = <Standard as SymShape>::COUNTER_BITS;
    ///     const ID_BITS: USize = <Standard as SymShape>::ID_BITS;
    ///     const KIND_BITS: USize = <Standard as SymShape>::KIND_BITS;
    ///     const ORIGIN_BITS: USize = <Standard as SymShape>::ORIGIN_BITS;
    ///
    ///     fn origin_index(_: Self::Origin) -> USize {
    ///         USize(0)
    ///     }
    /// }
    /// struct Peers;
    /// impl Domain for Peers {
    ///     type Shape = Room;
    ///
    ///     const KIND: SymKind<Room> =
    ///         SymKind::new(arvo_bits::Bits::from_raw(0b010));
    /// }
    /// impl GenerativeDomain for Peers {}
    /// const G: Generator<Peers> = Generator::single();
    /// fn main() {
    ///     let _g = G;
    /// }
    /// ```
    pub const fn single() -> Self
    where
        D::Shape: [const] SymShape<Origin = OneOrigin>,
    {
        assert!(
            crate::shape::origin_index_in_range::<D::Shape>(OneOrigin),
            "a one-origin shape must keep its only origin inside its reserved range"
        );
        Self {
            next:      Uint::<28, Hot>::from_raw(crate::shape::origin_base::<D::Shape>(OneOrigin)),
            ceiling:   Uint::<28, Hot>::from_raw(crate::shape::origin_ceiling::<D::Shape>(
                OneOrigin,
            )),
            exhausted: Bool(false),
            _domain:   PhantomData,
        }
    }
}

impl<D: GenerativeDomain> Default for Generator<D>
where
    D::Shape: SymShape<Origin = OneOrigin>,
{
    fn default() -> Self {
        Self::single()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Domain;
    use crate::kind::SymKind;
    use crate::shape::Standard;

    struct TestGen;
    impl Domain for TestGen {
        type Shape = Standard;

        const KIND: SymKind = SymKind::from_raw(0b010);
    }
    impl GenerativeDomain for TestGen {}

    const CEIL: Uint<28, Hot> =
        <Uint<28, Hot> as FromConstant>::from_constant::<{ USize((1 << 28) - 1) }>(); // lint:allow(no-bare-numeric) reason: the 28-bit ceiling as a typed constant in a test; tracked: #34

    #[test]
    fn exhausts_at_ceiling_without_wrapping() {
        // Seed the counter at the 28-bit ceiling. One more mint issues that
        // last id, after which the generator is exhausted and returns Isnt
        // rather than wrapping the counter back to a live id.
        let mut g: Generator<TestGen> = Generator {
            next:      CEIL,
            ceiling:   CEIL,
            exhausted: Bool(false),
            _domain:   PhantomData,
        };
        match g.mint() {
            Maybe::Is(s) => assert_eq!(s.id().to_raw(), CEIL.to_raw()),
            Maybe::Isnt => panic!("the ceiling id is still issued"),
        }
        assert!(matches!(g.mint(), Maybe::Isnt));
        assert!(matches!(g.mint(), Maybe::Isnt));
    }
}
