//! `Generator`: the mint-only producer for a generative domain.

use core::marker::PhantomData;

use arvo::strategy::Identity;
use arvo::traits::FromConstant;
use arvo::{Bool, USize, Uint};
use arvo_bits::{Bits, Hot};
use notko::Maybe;

use crate::domain::GenerativeDomain;
use crate::handle::Sym;

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
/// use hilavitkutin_sym::{Domain, Generator, InterningDomain, SymKind};
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
    exhausted: Bool,
    _domain: PhantomData<D>,
}

impl<D: GenerativeDomain> Generator<D> {
    /// A fresh generator for a domain minted in exactly one place.
    ///
    /// **Freshness is a property of this generator, not of the crate.** Two
    /// generators built here start at the same id and emit bit-identical
    /// handles, and because `Sym` compares by integer equality those handles
    /// compare *equal* rather than merely failing to match. That is silent, and
    /// silence is why this constructor names the case it serves.
    ///
    /// A consumer with more than one independent minter for one domain cannot
    /// get disjointness from this constructor and must not try: it chooses a
    /// shape with origin bits, under which a generator can only be built by
    /// naming which origin it is, so a collision requires two peers to claim
    /// the same origin explicitly rather than to share a default.
    pub const fn single() -> Self {
        Self {
            next: <Uint<28, Hot> as Identity>::ZERO,
            exhausted: Bool(false),
            _domain: PhantomData,
        }
    }

    /// Mint a fresh handle. Returns `Maybe::Isnt` once the 28-bit id space is
    /// exhausted, rather than wrapping the counter and reissuing a live id.
    pub fn mint(&mut self) -> Maybe<Sym> {
        if self.exhausted.0 {
            return Maybe::Isnt;
        }
        // id-allocator boundary: project the typed counter to the handle's
        // 28-bit id field. The one low-level numeric edge of this producer,
        // mirroring the arena id boundary in hilavitkutin-str.
        let id = Bits::<28, Hot>::from_raw(self.next.to_raw()); // lint:allow(no-bare-numeric) reason: Uint-to-Bits id projection at the id-allocator boundary; to_raw/from_raw are arvo's container projections; tracked: #34
        let sym = Sym::new(D::KIND, id);
        if self.next == MAX_ID {
            self.exhausted = Bool(true);
        } else {
            self.next = self.next + ONE;
        }
        Maybe::Is(sym)
    }
}

impl<D: GenerativeDomain> Default for Generator<D> {
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
            exhausted: Bool(false),
            _domain: PhantomData,
        };
        assert!(matches!(g.mint(), Maybe::Is(_)));
        assert!(matches!(g.mint(), Maybe::Isnt));
        assert!(matches!(g.mint(), Maybe::Isnt));
    }
}
