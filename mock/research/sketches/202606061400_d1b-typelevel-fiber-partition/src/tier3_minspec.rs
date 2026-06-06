//! Sketch (D1b Tier 3, vetted-feature retry / #340): the grouping fold's
//! depends-or-not branch under `min_specialization`.
//!
//! Op note 2026-06-06: before concluding HYBRID, check the explicitly-vetted
//! feature catalogue (`unstable-features.md`) for one that makes the full
//! type-level grouping derivation (Tier 3) work. `min_specialization` is RESOLVED
//! ALLOWED there (std-internal carve-out). The plain-`specialization` Tier 3
//! (tier3.rs) hit E0119; this retries the SAME `DependsBool` total type-level
//! boolean under `min_specialization`, the vetted subset.
//!
//! Hypothesis: `min_specialization` does NOT rescue it, because the specialization
//! is on an arbitrary added trait bound (`RB: SharesStore<WA>`) whose negation the
//! lattice cannot order. Expected: a min_specialization-specific rejection (not
//! E0119), confirming the vetted subset is also insufficient. If instead it
//! compiles, Tier 3 is feasible under a vetted feature and the fork resolves to
//! FULL type-level (better than hybrid). The build is the test.

#![allow(dead_code)]
#![feature(marker_trait_attr)]
#![feature(min_specialization)]

use hilavitkutin_api::access::{Cons, Contains, Empty};

#[marker]
trait SharesStore<Other> {}
impl<H: 'static, T: 'static, Other> SharesStore<Other> for Cons<H, T> where Other: Contains<H> {}
impl<H: 'static, T: 'static, Other> SharesStore<Other> for Cons<H, T> where T: SharesStore<Other> {}

struct True;
struct False;

trait DependsBool<WA> {
    type Out;
}

// Default (negative) arm.
impl<RB, WA> DependsBool<WA> for RB {
    default type Out = False;
}

// Specializing (positive) arm: only when RB shares with WA.
impl<RB, WA> DependsBool<WA> for RB
where
    RB: SharesStore<WA>,
{
    type Out = True;
}

fn main() {
    println!("if this compiled, min_specialization rescues Tier 3 full type-level grouping");
}
