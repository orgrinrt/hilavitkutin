//! Sketch (D1b Tier 3, vetted-feature retry / #340): the grouping fold's
//! depends-or-not branch under `negative_impls`.
//!
//! Op note 2026-06-06: check the vetted catalogue. `negative_impls` is WATCH
//! (allowed) in `unstable-features.md`, with the known gap #133556 ("does not yet
//! disarm coherence"). This tries to give the solver an explicit negative so the
//! two `DependsBool` arms stop overlapping: a `NotShares` marker with a negative
//! impl for the sharing case, then keying the False arm on `NotShares`.
//!
//! Hypothesis: `negative_impls` does NOT rescue it, because (a) a blanket negative
//! `impl !SharesStore for ...` cannot be conditional on "Other does not contain
//! any member" (no negative-of-Contains), and (b) #133556 means even a present
//! negative impl does not disarm the positive-arm coherence overlap. Expected: a
//! coherence or unconstrained-parameter rejection. If it compiles, Tier 3 is
//! feasible under a vetted feature. The build is the test.

#![allow(dead_code)]
#![feature(marker_trait_attr)]
#![feature(negative_impls)]

use hilavitkutin_api::access::{Cons, Contains, Empty};

#[marker]
trait SharesStore<Other> {}
impl<H: 'static, T: 'static, Other> SharesStore<Other> for Cons<H, T> where Other: Contains<H> {}
impl<H: 'static, T: 'static, Other> SharesStore<Other> for Cons<H, T> where T: SharesStore<Other> {}

struct True;
struct False;

// A disjoint marker: "RB does NOT share with WA". To partition the (RB, WA) space
// into True/False arms without overlap, the negative arm must key on a trait that
// holds EXACTLY when SharesStore does not. negative_impls lets us WRITE a negative
// impl, but cannot synthesise "holds when SharesStore absent" for the positive
// arm of THIS marker. The attempt: a default-positive NotShares with a negative
// impl over the sharing case.
trait NotShares<Other> {}
impl<RB, WA> NotShares<WA> for RB {}
// Negative impl removing the sharing subset. Requires negative_impls.
impl<RB, WA> !NotShares<WA> for RB where RB: SharesStore<WA> {}

trait DependsBool<WA> {
    type Out;
}
impl<RB, WA> DependsBool<WA> for RB
where
    RB: SharesStore<WA>,
{
    type Out = True;
}
impl<RB, WA> DependsBool<WA> for RB
where
    RB: NotShares<WA>,
{
    type Out = False;
}

fn main() {
    println!("if this compiled, negative_impls rescues Tier 3 full type-level grouping");
}
