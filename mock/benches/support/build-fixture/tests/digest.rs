//! The fixture's digest is the committed one at every point, which is what
//! holds it deterministic, and a fixture that differs is refused before
//! anything builds it.

use build_fixture::{COMMITTED, Fixture, POINTS, SEED, committed_digest};

#[test]
fn every_point_writes_the_committed_digest() {
    let found: Vec<(usize, u64)> = POINTS
        .iter()
        .map(|&n| (n, Fixture::at(n).expect("a point").digest()))
        .collect();
    assert_eq!(found, COMMITTED.to_vec(), "found {found:x?}");
}

#[test]
fn another_seed_gives_another_digest_and_is_refused() {
    let at = Fixture::at(25).expect("the small point");
    let other = Fixture {
        seed: SEED ^ 1,
        ..at
    };
    assert_ne!(other.digest(), at.digest());
    let root = std::env::temp_dir().join("build-fixture-test-refused");
    let refused = other.write(&root, None);
    assert!(
        refused.as_ref().is_err_and(|e| e.contains("committed")),
        "a fixture whose digest is not committed must be refused: {refused:?}"
    );
    assert!(
        !root.join("Cargo.toml").exists(),
        "nothing is written when refused"
    );
}

#[test]
fn a_point_that_is_not_twelve_c_plus_one_has_no_fixture() {
    assert!(Fixture::at(24).is_none());
    assert!(
        Fixture::at(13).is_none(),
        "one crate per domain has no implementation"
    );
    assert!(committed_digest(37).is_none());
}

#[test]
fn the_crate_count_is_the_point() {
    for n in POINTS {
        let f = Fixture::at(n).expect("a point");
        assert_eq!(f.graph.crate_count(), n);
        assert_eq!(f.graph.roles().len(), n);
        let crates = f
            .files()
            .iter()
            .filter(|(p, _)| p.ends_with("/Cargo.toml"))
            .count();
        assert_eq!(crates, n, "one manifest per crate at n={n}");
    }
}
