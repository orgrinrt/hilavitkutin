//! `Generator` contract: mints carry the domain kind, are distinct, and stay
//! disjoint from other domains.

use arvo_bits::{Bits, Hot};
use hilavitkutin_sym::{Domain, GenerativeDomain, Generator, Standard, Sym, SymKind};
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
