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
    Domain, GenerativeDomain, Generator, MinterId, SixteenMinters, Standard, Sym, SymKind, WideKind,
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
    let mut a = match Generator::<PeerBinder>::at(origin(0)) {
        notko::Maybe::Is(g) => g,
        notko::Maybe::Isnt => panic!("a shipped shape refused an in-range origin"),
    };
    let mut b = match Generator::<PeerBinder>::at(origin(1)) {
        notko::Maybe::Is(g) => g,
        notko::Maybe::Isnt => panic!("a shipped shape refused an in-range origin"),
    };

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
            let mut g = match Generator::<PeerBinder>::at(origin(o)) {
                notko::Maybe::Is(g) => g,
                notko::Maybe::Isnt => panic!("a shipped shape refused an in-range origin"),
            };
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
    let mut g = match Generator::<PeerBinder>::at(origin(0)) {
        notko::Maybe::Is(g) => g,
        notko::Maybe::Isnt => panic!("a shipped shape refused an in-range origin"),
    };
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

/// Every mint carries its domain's tag, which is what keeps domains disjoint
/// under integer equality regardless of shape or origin.
#[test]
fn every_mint_carries_the_domain_tag() {
    let mut g = Generator::<Binder>::single();
    for s in mint_n(&mut g, 16) {
        assert_eq!(s.kind(), Binder::KIND);
    }
}

/// **The kind width varies, demonstrated rather than declared.**
///
/// A domain on the wide-tag shape carries `0b10000`, a value no three-bit tag
/// can hold. If every shape shared one layout this could not be written, which
/// is why it is the test that proves the trait varies anything at all.
#[test]
fn a_wide_shape_carries_a_tag_no_narrow_shape_could_hold() {
    #[derive(Copy, Clone)]
    struct WideDomain;

    impl Domain for WideDomain {
        type Shape = WideKind;

        const KIND: SymKind<WideKind> = SymKind::new(arvo_bits::Bits::from_raw(0b10000));
    }

    impl GenerativeDomain for WideDomain {}

    let mut g = match Generator::<WideDomain>::at(hilavitkutin_sym::OneOrigin) {
        notko::Maybe::Is(g) => g,
        notko::Maybe::Isnt => panic!("a shipped shape refused an in-range origin"),
    };
    let s = match g.mint() {
        notko::Maybe::Is(s) => s,
        notko::Maybe::Isnt => panic!("fresh generator refused its first mint"),
    };

    assert_eq!(
        s.kind().to_bits().to_raw(),
        0b10000,
        "the tag must survive a round trip at five bits wide"
    );
}

/// **A generator that refuses once must keep refusing.**
///
/// The exhaustion test above stops at the first refusal. A generator that
/// refuses once and then yields again is the same defect wearing one passing
/// assertion, and nothing distinguished the two.
#[test]
fn a_minter_keeps_refusing_once_exhausted() {
    let mut g = match Generator::<PeerBinder>::at(origin(0)) {
        notko::Maybe::Is(g) => g,
        notko::Maybe::Isnt => panic!("a shipped shape refused an in-range origin"),
    };
    let span = 1_usize << 24;
    for _ in 0..span {
        assert!(matches!(g.mint(), notko::Maybe::Is(_)));
    }
    for i in 0..8 {
        assert!(
            matches!(g.mint(), notko::Maybe::Isnt),
            "an exhausted minter yielded again on refusal {i}"
        );
    }
}

/// Each origin owns its span at **both** ends.
///
/// The disjointness tests above sample the first few ids of each origin, which
/// is the interior of every span. A boundary checked from one side is half
/// checked: an off-by-one in `origin_base` shows at the first id and an
/// off-by-one in `origin_ceiling` shows only at the last.
#[test]
fn every_origin_owns_its_first_and_last_id() {
    let span = 1_u32 << 24;
    for o in 0u8..16 {
        let mut g = match Generator::<PeerBinder>::at(origin(o)) {
            notko::Maybe::Is(g) => g,
            notko::Maybe::Isnt => panic!("a shipped shape refused an in-range origin"),
        };

        let first = match g.mint() {
            notko::Maybe::Is(s) => s,
            notko::Maybe::Isnt => panic!("origin {o} refused its first mint"),
        };
        let expected_first = u32::from(o) * span;
        assert_eq!(
            first.id().to_raw(),
            expected_first,
            "origin {o} did not start at its own base"
        );

        for _ in 1..span {
            assert!(
                matches!(g.mint(), notko::Maybe::Is(_)),
                "origin {o} refused inside its own span"
            );
        }
        assert!(
            matches!(g.mint(), notko::Maybe::Isnt),
            "origin {o} minted past its own ceiling"
        );
    }
}

/// **The crate's own promise, minted rather than reasoned about.**
///
/// Two origins of one domain never produce one handle. Nine rounds of width
/// laws related declarations to each other and to layout types, and none of
/// them ever minted anything, so a shape could place its origins outside its own
/// id field and two minters would collide while every law passed.
///
/// Asserted per shape rather than for `SixteenMinters` alone, because the shape
/// that broke it was the one no disjointness test covered.
#[test]
fn two_origins_never_collide_under_any_shape() {
    // Sixteen origins, eight mints each: enough that an overlapping run shows,
    // small enough to stay a unit test.
    let minted: Vec<Vec<Sym<SixteenMinters>>> = (0u8..16)
        .map(|o| {
            let mut g = match Generator::<PeerBinder>::at(origin(o)) {
                notko::Maybe::Is(g) => g,
                notko::Maybe::Isnt => panic!("a shipped shape refused an in-range origin"),
            };
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
                    assert_ne!(
                        x, y,
                        "origins {oa} and {ob} minted the same handle, so two peers \
                         holding them compare equal rather than failing to match"
                    );
                }
            }
        }
    }

    // A single-origin shape has nothing to collide with, and saying so keeps the
    // law honest about what it covers rather than silently skipping the case.
    let mut only = match Generator::<WideDomainSingle>::at(hilavitkutin_sym::OneOrigin) {
        notko::Maybe::Is(g) => g,
        notko::Maybe::Isnt => panic!("a shipped shape refused an in-range origin"),
    };
    let a = mint_n(&mut only, 8);
    for (i, x) in a.iter().enumerate() {
        for (j, y) in a.iter().enumerate() {
            if i != j {
                assert_ne!(x, y, "a single minter repeated itself at {i} and {j}");
            }
        }
    }
}

/// A domain on the wide shape, for the single-origin half of the law above.
#[derive(Copy, Clone)]
struct WideDomainSingle;

impl Domain for WideDomainSingle {
    type Shape = WideKind;

    const KIND: SymKind<WideKind> = SymKind::new(arvo_bits::Bits::from_raw(0b00010));
}

impl GenerativeDomain for WideDomainSingle {}
