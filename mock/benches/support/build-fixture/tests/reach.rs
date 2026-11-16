//! What the generator says an edit at each site reaches, from the graph alone.
//! That the compiler agrees is shown by real builds in `build-call`'s tests.

use build_fixture::graph::{Role, Tier};
use build_fixture::{Fixture, POINTS, Site};

#[test]
fn a_leaf_edit_reaches_its_crate_and_the_facade() {
    for n in POINTS {
        let f = Fixture::at(n).expect("a point");
        let reached = f.graph.reach(f.site_role(Site::Leaf));
        assert_eq!(reached.len(), 2, "n={n}: {reached:?}");
        assert!(reached.contains(&Role::Facade));
    }
}

#[test]
fn a_mid_contract_edit_reaches_its_domain_two_leaf_domains_and_the_facade() {
    for n in POINTS {
        let f = Fixture::at(n).expect("a point");
        let c = f.graph.per_domain;
        assert_eq!(f.reach(Site::Mid), 3 * c + 1, "n={n}");
    }
}

#[test]
fn a_root_contract_edit_reaches_everything_but_the_other_root_domains() {
    for n in POINTS {
        let f = Fixture::at(n).expect("a point");
        let c = f.graph.per_domain;
        assert_eq!(f.reach(Site::Root), 9 * c + 1, "n={n}");
        assert_eq!(n - f.reach(Site::Root), 3 * c, "n={n}");
    }
}

#[test]
fn only_the_facade_takes_implementations() {
    let f = Fixture::at(97).expect("the mid point");
    for role in f.graph.roles() {
        let takes_impl = f
            .graph
            .deps(role)
            .iter()
            .any(|d| matches!(d, Role::Implementation { .. }));
        assert_eq!(takes_impl, role == Role::Facade, "{role:?}");
    }
}

#[test]
fn every_domain_reads_the_root_tier_and_only_the_facade_reads_the_leaf_tier() {
    let f = Fixture::at(25).expect("the small point");
    let g = f.graph;
    for d in 0 .. g.domains {
        let reads = g.reads(d);
        match g.tier(d) {
            Tier::Root => assert!(reads.is_empty()),
            _ => {
                assert!(
                    (0 .. g.domains / 3).all(|r| reads.contains(&r)),
                    "domain {d}"
                )
            },
        }
        assert!(
            reads.iter().all(|r| g.tier(*r) != Tier::Leaf),
            "domain {d} reads a leaf"
        );
    }
}
