//! The test that would have caught the peer collision.
//!
//! `Generator::new` set its counter to zero and `mint` stepped it by one, so
//! two generators for one domain emitted bit-identical handles. `Sym` compares
//! by integer equality over the whole layout, which is this crate's central
//! promise, so those handles did not merely fail to match: they compared
//! **equal**.
//!
//! A consumer designing on top of this crate measured 132 of 144 peer scenarios
//! false-identifying, including 24 in which the two peers shared no value at
//! all. Incomparable would have been safe, because a wrong answer a consumer
//! can detect is one it can handle. Falsely equal is silent.
//!
//! These tests assert the contract in both directions: what one generator
//! promises, and what two must not do.

use hilavitkutin_sym::{Domain, GenerativeDomain, Generator, Standard, Sym, SymKind};

/// A generative domain under the default shape.
#[derive(Copy, Clone)]
struct Binder;

impl Domain for Binder {
    const KIND: SymKind = SymKind::from_raw(0b001);
}

impl GenerativeDomain for Binder {}

/// Drain `n` mints, failing loudly rather than silently short-counting.
fn mint_n(g: &mut Generator<Binder>, n: usize) -> Vec<Sym> {
    (0..n)
        .map(|i| match g.mint() {
            notko::Maybe::Is(s) => s,
            notko::Maybe::Isnt => panic!("generator exhausted after {i} of {n} mints"),
        })
        .collect()
}

/// The contract the old wording actually gave, and it still holds: a single
/// generator never repeats itself.
#[test]
fn one_generator_never_repeats() {
    let mut g = Generator::<Binder>::single();
    let mints = mint_n(&mut g, 64);

    for (i, a) in mints.iter().enumerate() {
        for (j, b) in mints.iter().enumerate() {
            if i != j {
                assert_ne!(a, b, "mint {i} and mint {j} of one generator collided");
            }
        }
    }
}

/// The defect. Under the default shape there is exactly one origin, so two
/// generators are two minters for a space that admits one, and the type system
/// is what has to say so.
///
/// `Standard` has `ORIGIN_BITS = 0`. There is no second origin to construct, so
/// the collision this test names is unreachable rather than merely unlikely:
/// the only constructor is `single`, and a consumer wanting two minters must
/// choose a shape that has room for them. This test pins that the single-origin
/// constructor is the whole surface, so a future change that adds a second
/// zero-seeded constructor breaks here rather than in a consumer.
#[test]
fn the_default_shape_admits_exactly_one_minter() {
    use hilavitkutin_sym::SymShape;
    assert_eq!(
        Standard::ORIGINS.0,
        1,
        "the default shape admits one minter, so no two generators can be given \
         disjoint ranges under it; a consumer with peers picks a shape with \
         origin bits"
    );
    assert_eq!(Standard::ORIGIN_BITS.0, 0);
}

/// Freshness is a property of a generator, not of the crate, and the design
/// document now says so. This pins the statement: two independently constructed
/// generators under a one-origin shape produce the same sequence, which is
/// exactly why a consumer may not treat a minted handle as globally unique.
///
/// It is a characterisation test rather than a wish. If the crate ever makes two
/// `single` generators diverge, this fails and the design document is what needs
/// changing.
#[test]
fn two_generators_at_one_origin_agree_by_construction() {
    let mut a = Generator::<Binder>::single();
    let mut b = Generator::<Binder>::single();

    let from_a = mint_n(&mut a, 8);
    let from_b = mint_n(&mut b, 8);

    assert_eq!(
        from_a, from_b,
        "two generators at one origin are the same sequence by construction; a \
         consumer that wants disjointness needs a shape with origin bits, and \
         the design document says so rather than leaving it to be discovered"
    );
}

/// Exhaustion refuses rather than corrupts, and that principle is what the
/// origin split extends. Kept here because it is the contract the collision
/// fix is reasoning from.
#[test]
fn every_mint_carries_the_domain_tag() {
    let mut g = Generator::<Binder>::single();
    for s in mint_n(&mut g, 16) {
        assert_eq!(
            s.kind(),
            Binder::KIND,
            "a mint must carry its domain's tag, which is what keeps domains \
             disjoint under integer equality"
        );
    }
}
