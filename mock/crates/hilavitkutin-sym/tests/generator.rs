#![feature(const_trait_impl)]
//! `Generator` contract: mints carry the domain kind, are distinct, and stay
//! disjoint from other domains.

use arvo_bits::{Bits, Hot};
use hilavitkutin_sym::{Domain, GenerativeDomain, Generator, Standard, Sym, SymKind, WideKind};
use notko::Maybe;

/// A generative test domain at kind `0b010` (outside the reserved string and
/// binder tags).
struct TestGen;
impl Domain for TestGen {
    type Shape = Standard;

    const KIND: SymKind = SymKind::from_raw(0b010);
}
impl GenerativeDomain for TestGen {}

/// A second generative domain at a different kind, for disjointness.
struct OtherGen;
impl Domain for OtherGen {
    type Shape = Standard;

    const KIND: SymKind = SymKind::from_raw(0b011);
}
impl GenerativeDomain for OtherGen {}

fn expect(m: Maybe<Sym>) -> Sym {
    match m {
        Maybe::Is(s) => s,
        Maybe::Isnt => panic!("generator returned Isnt unexpectedly"),
    }
}

#[test]
fn mint_carries_domain_kind() {
    let mut g = Generator::<TestGen>::single();
    let s = expect(g.mint());
    assert_eq!(s.kind(), TestGen::KIND);
}

#[test]
fn mints_are_distinct() {
    let mut g = Generator::<TestGen>::single();
    let a = expect(g.mint());
    let b = expect(g.mint());
    let c = expect(g.mint());
    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_ne!(a, c);
}

#[test]
fn different_domains_never_equal() {
    // Same id, different domain kind: the handles must not be equal, because
    // the kind is part of the compared bits.
    let id = Bits::<28, Hot>::from_raw(7);
    let a = Sym::new(TestGen::KIND, id);
    let b = Sym::new(OtherGen::KIND, id);
    assert_ne!(a, b);
    assert_ne!(a.kind(), b.kind());
}

#[test]
fn minted_handle_is_disjoint_from_string_kind() {
    // A minted binder-style handle (kind 0b010) must never equal a string
    // handle (kind 0b000), whatever the id.
    let string_kind = SymKind::from_raw(0b000);
    let id = Bits::<28, Hot>::from_raw(0);
    let mut g = Generator::<TestGen>::single();
    let minted = expect(g.mint());
    let string_like = Sym::new(string_kind, id);
    assert_ne!(minted, string_like);
    assert_ne!(minted.kind(), string_kind);
}

/// A generative domain on the wide-kind shape: five bits of kind, a 26-bit
/// counter, one origin.
struct WideGen;
impl Domain for WideGen {
    type Shape = WideKind;

    const KIND: SymKind<WideKind> = SymKind::new(arvo_bits::Bits::from_raw(0b01010));
}
impl GenerativeDomain for WideGen {}

/// `Generator::single` takes no origin and returns no refusal. That is sound
/// only while the shape it is offered on has one origin and that origin's
/// index is in range, which is what `Generator::at` would have checked. It is
/// also a `const fn`, so a generator can sit in a constant; the constant
/// below stops compiling if it stops being one.
#[test]
fn single_is_total_on_every_one_origin_shape() {
    use hilavitkutin_sym::shape::{OneOrigin, origin_index_in_range};

    assert!(
        origin_index_in_range::<Standard>(OneOrigin),
        "the one origin of the default shape must be in range, or `single` \
         skips a refusal that can happen"
    );
    assert!(
        origin_index_in_range::<WideKind>(OneOrigin),
        "the one origin of the wide-kind shape must be in range too"
    );

    const AT_REST: Generator<TestGen> = Generator::single();
    let mut g = AT_REST;
    assert_eq!(expect(g.mint()).kind(), TestGen::KIND);
}

#[test]
fn single_mints_on_the_wide_kind_shape() {
    const AT_REST: Generator<WideGen> = Generator::single();
    let mut g = AT_REST;
    let a = g.mint();
    let b = g.mint();
    match (a, b) {
        (Maybe::Is(a), Maybe::Is(b)) => {
            assert_ne!(a, b);
            assert_eq!(a.kind(), WideGen::KIND);
            assert_eq!(a.id().to_raw(), 0);
            assert_eq!(b.id().to_raw(), 1);
        },
        _ => panic!("a fresh wide-kind generator mints"),
    }
    // `Default` is offered on the same shapes as `single`.
    let mut d = Generator::<WideGen>::default();
    assert!(matches!(d.mint(), Maybe::Is(_)));
}
