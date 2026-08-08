#![feature(const_trait_impl)]
//! The test that would have caught the peer collision.
//!
//! `Generator::new` set every counter to zero and `mint` stepped it by one, so
//! two generators for one domain emitted bit-identical handles. `Sym` compares
//! by integer equality over the whole layout, which is this crate's central
//! promise, so those handles did not merely fail to match: they compared
//! **equal**.
//!
//! A consumer designing on top of this crate measured 132 of 144 peer scenarios
//! false-identifying, including 24 in which the two peers shared no value at
//! all. Incomparable would have been safe, because a wrong answer a consumer
//! can detect is one it can handle. Falsely equal is silent.

use hilavitkutin_sym::{
    Domain, GenerativeDomain, Generator, MinterId, SixteenMinters, Standard, Sym, SymKind, SymShape,
};

/// A generative domain under the default shape, minted in one place.
#[derive(Copy, Clone)]
struct Binder;

impl Domain for Binder {
    type Shape = Standard;

    const KIND: SymKind = SymKind::from_raw(0b001);
}

impl GenerativeDomain for Binder {}

/// The same domain concept, on a shape minted in up to sixteen places. A
/// domain belongs to a shape, so this is a distinct domain rather than the
/// same one reconfigured.
#[derive(Copy, Clone)]
struct PeerBinder;

impl Domain for PeerBinder {
    type Shape = SixteenMinters;

    const KIND: SymKind<SixteenMinters> = SymKind::new(arvo_bits::Bits::from_raw(0b001));
}

impl GenerativeDomain for PeerBinder {}

fn origin(i: u8) -> MinterId {
    MinterId(arvo::Uint::<4, arvo::strategy::Hot>::from_raw(i))
}

fn mint_n<D: GenerativeDomain>(g: &mut Generator<D>, n: usize) -> Vec<Sym<D::Shape>> {
    (0..n)
        .map(|i| match g.mint() {
            notko::Maybe::Is(s) => s,
            notko::Maybe::Isnt => panic!("generator exhausted after {i} of {n} mints"),
        })
        .collect()
}

/// A single generator never repeats itself. True before this round and still
/// true; kept because it is the half of the contract that always held.
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

/// **The defect, and the test that fails against the old code.**
///
/// Two minters of one domain, at two origins, minting the same count each.
/// Under the old `Generator::new` both started at zero and every pair collided.
/// Under a shape with origin bits they cannot, because each origin owns a
/// disjoint run of the id space.
#[test]
fn two_origins_never_collide() {
    let mut a = Generator::<PeerBinder>::at(origin(0));
    let mut b = Generator::<PeerBinder>::at(origin(1));

    let from_a = mint_n(&mut a, 64);
    let from_b = mint_n(&mut b, 64);

    for (i, x) in from_a.iter().enumerate() {
        for (j, y) in from_b.iter().enumerate() {
            assert_ne!(
                x, y,
                "mint {i} of origin 0 collided with mint {j} of origin 1; this is \
                 the defect, and two peers would have false-identified here"
            );
        }
    }
}

/// Every origin is disjoint from every other, not merely the first two.
#[test]
fn all_sixteen_origins_are_pairwise_disjoint() {
    let minted: Vec<Vec<Sym<SixteenMinters>>> = (0u8..16)
        .map(|o| {
            let mut g = Generator::<PeerBinder>::at(origin(o));
            mint_n(&mut g, 8)
        })
        .collect();

    for (oa, a) in minted.iter().enumerate() {
        for (ob, b) in minted.iter().enumerate() {
            if oa == ob {
                continue;
            }
            for x in a {
                for y in b {
                    assert_ne!(x, y, "origins {oa} and {ob} produced the same handle");
                }
            }
        }
    }
}

/// A minter stops at its own ceiling rather than walking into the next
/// origin's range. Without this, disjointness holds only until someone counts
/// far enough, which is the silent kind of wrong.
#[test]
fn a_minter_is_exhausted_at_its_own_ceiling_not_the_whole_space() {
    let mut g = Generator::<PeerBinder>::at(origin(0));
    let span = 1_usize << 24;

    // Walking the whole span must succeed, and the next mint must refuse.
    for i in 0..span {
        assert!(
            matches!(g.mint(), notko::Maybe::Is(_)),
            "minter refused at {i}, before its own ceiling"
        );
    }
    assert!(
        matches!(g.mint(), notko::Maybe::Isnt),
        "minter walked past its ceiling into the next origin's range"
    );
}

/// The default shape has one minter and one origin, so `single` needs no
/// argument and there is no second origin to name. This is what makes the
/// single-producer case safe by construction rather than by convention.
#[test]
fn the_default_shape_has_one_origin_and_the_generic_one_has_sixteen() {
    assert_eq!(Standard::ORIGINS.0, 1);
    assert_eq!(Standard::ORIGIN_BITS.0, 0);
    assert_eq!(SixteenMinters::ORIGINS.0, 16);
    assert_eq!(SixteenMinters::ORIGIN_BITS.0, 4);
}

/// Every mint carries its domain's tag, which is what keeps domains disjoint
/// under integer equality regardless of shape or origin.
#[test]
fn every_mint_carries_the_domain_tag() {
    let mut g = Generator::<Binder>::single();
    for s in mint_n(&mut g, 16) {
        assert_eq!(s.kind(), Binder::KIND);
    }
}
