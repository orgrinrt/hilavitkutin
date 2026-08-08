//! `Generator`: the mint-only producer for a generative domain.

use core::marker::PhantomData;

use arvo::strategy::Identity;
use arvo::traits::FromConstant;
use arvo::{Bool, USize, Uint};
use arvo_bits::Hot;
use notko::Maybe;

use crate::domain::GenerativeDomain;
use crate::handle::Sym;
use crate::shape::{OneOrigin, Standard, SymShape};

/// The largest 28-bit id. After minting this id the generator is exhausted.
const MAX_ID: Uint<28, Hot> =
    <Uint<28, Hot> as FromConstant>::from_constant::<{ USize((1 << 28) - 1) }>(); // lint:allow(no-bare-numeric) reason: the 28-bit id-space ceiling as a typed constant (definition-site literal); tracked: #34

/// One, the mint step.
const ONE: Uint<28, Hot> = <Uint<28, Hot> as FromConstant>::from_constant::<{ USize(1) }>(); // lint:allow(no-bare-numeric) reason: the increment step as a typed constant (definition-site literal); tracked: #34

/// Mints fresh handles for a generative domain: a monotonic counter over the
/// 28-bit id width, tagged with the domain's `KIND`. No allocation, no arena.
///
/// A `Generator` can only be built for a [`GenerativeDomain`]. Building one for
/// an interning-only domain is a type error:
///
/// ```compile_fail
/// use hilavitkutin_sym::{Domain, Generator, InterningDomain, Standard, SymKind};
/// struct InterningOnly;
/// impl Domain for InterningOnly {
///     const KIND: SymKind = SymKind::from_raw(0b100);
/// }
/// impl InterningDomain for InterningOnly {
///     type Value = str;
/// }
/// // InterningOnly is not a GenerativeDomain, so this does not compile.
/// let _g = Generator::<InterningOnly>::single();
/// ```
pub struct Generator<D: GenerativeDomain> {
    next: Uint<28, Hot>,
    ceiling: Uint<28, Hot>,
    exhausted: Bool,
    _domain: PhantomData<D>,
}

impl<D: GenerativeDomain> Generator<D> {
    /// A fresh generator for `origin`.
    ///
    /// **Origins partition the id space by construction.** Two generators at
    /// two origins cannot produce the same handle however far either counts,
    /// because each starts at its own base and stops at its own ceiling.
    ///
    /// A shape with one minter has one origin and its constructor is
    /// [`Generator::single`], which needs no argument because there is nothing
    /// to choose. Under such a shape two generators do agree, and they agree
    /// because the shape says there is one minter, not because a default was
    /// applied where a choice belonged.
    pub fn at(origin: <D::Shape as SymShape>::Origin) -> Self {
        Self {
            next: <D::Shape as SymShape>::origin_base(origin),
            ceiling: <D::Shape as SymShape>::origin_ceiling(origin),
            exhausted: Bool(false),
            _domain: PhantomData,
        }
    }

    /// Mint a fresh handle. Returns `Maybe::Isnt` once the 28-bit id space is
    /// exhausted, rather than wrapping the counter and reissuing a live id.
    pub fn mint(&mut self) -> Maybe<Sym<D::Shape>> {
        if self.exhausted.0 {
            return Maybe::Isnt;
        }
        // The shape owns the projection, because it owns how wide the id is.
        let id = <D::Shape as SymShape>::id_from_counter(self.next);
        let sym = Sym::<D::Shape>::new(D::KIND, id);
        if self.next == self.ceiling {
            self.exhausted = Bool(true);
        } else {
            self.next = self.next + ONE;
        }
        Maybe::Is(sym)
    }
}

impl<D: GenerativeDomain<Shape = Standard>> Generator<D> {
    /// A fresh generator for a domain minted in exactly one place.
    ///
    /// The default shape has one origin, so there is nothing to name. A
    /// consumer with several independent minters picks a shape with origin
    /// bits, and then [`Generator::at`] is the only way in.
    pub fn single() -> Self {
        Self::at(OneOrigin)
    }
}

impl<D: GenerativeDomain<Shape = Standard>> Default for Generator<D> {
    fn default() -> Self {
        Self::single()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Domain;
    use crate::kind::SymKind;

    struct TestGen;
    impl Domain for TestGen {
        type Shape = Standard;

        const KIND: SymKind = SymKind::from_raw(0b010);
    }
    impl GenerativeDomain for TestGen {}

    #[test]
    fn exhausts_at_ceiling_without_wrapping() {
        // Seed the counter at the 28-bit ceiling. One more mint issues that
        // last id, after which the generator is exhausted and returns Isnt
        // rather than wrapping the counter back to a live id.
        let mut g: Generator<TestGen> = Generator {
            next: MAX_ID,
            ceiling: MAX_ID,
            exhausted: Bool(false),
            _domain: PhantomData,
        };
        assert!(matches!(g.mint(), Maybe::Is(_)));
        assert!(matches!(g.mint(), Maybe::Isnt));
        assert!(matches!(g.mint(), Maybe::Isnt));
    }
}
