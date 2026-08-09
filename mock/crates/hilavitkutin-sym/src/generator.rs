//! `Generator`: the mint-only producer for a generative domain.

use core::marker::PhantomData;

use arvo::traits::FromConstant;
use arvo::{Bool, USize, Uint};
use arvo_bits::Hot;
use notko::Maybe;

use crate::domain::GenerativeDomain;
use crate::handle::Sym;
use crate::shape::{OneOrigin, Standard, SymShape};

/// One, the mint step.
const ONE: Uint<28, Hot> = <Uint<28, Hot> as FromConstant>::from_constant::<{ USize(1) }>(); // lint:allow(no-bare-numeric) reason: the increment step as a typed constant (definition-site literal); tracked: #34

/// Mints fresh handles for a generative domain: a monotonic counter over the
/// 28-bit id width, tagged with the domain's `KIND`. No allocation, no arena.
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
///     const KIND: SymKind = SymKind::from_raw(0b100);
/// }
/// impl InterningDomain for InterningOnly {
///     type Value = str;
/// }
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
    /// **Origins divide the id space by construction.** Two generators at
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

    /// Mint a fresh handle. Returns `Maybe::Isnt` once **this origin's** range
    /// is exhausted, rather than wrapping the counter, reissuing a live id, or
    /// walking into the next origin's range.
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

    const CEIL: Uint<28, Hot> =
        <Uint<28, Hot> as FromConstant>::from_constant::<{ USize((1 << 28) - 1) }>(); // lint:allow(no-bare-numeric) reason: the 28-bit ceiling as a typed constant in a test; tracked: #34

    #[test]
    fn exhausts_at_ceiling_without_wrapping() {
        // Seed the counter at the 28-bit ceiling. One more mint issues that
        // last id, after which the generator is exhausted and returns Isnt
        // rather than wrapping the counter back to a live id.
        let mut g: Generator<TestGen> = Generator {
            next: CEIL,
            ceiling: CEIL,
            exhausted: Bool(false),
            _domain: PhantomData,
        };
        assert!(matches!(g.mint(), Maybe::Is(_)));
        assert!(matches!(g.mint(), Maybe::Isnt));
        assert!(matches!(g.mint(), Maybe::Isnt));
    }
}
