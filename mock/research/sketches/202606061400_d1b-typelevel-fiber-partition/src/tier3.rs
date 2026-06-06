//! Sketch (D1b Tier 3 / #340): the full type-level fiber-GROUPING fold.
//!
//! Tiers 1+2 (in `main.rs`) proved the CARRIER (a type-level cons-list of fibers
//! devirtualises to the canonical per-core-program shape, zero `blr`) and the
//! boundary PREDICATE (`SharesStore`: a Read set shares a store with a Write set,
//! resolving positives and rejecting negatives from AccessSets alone, no GCE).
//!
//! Tier 3 asks the stricter question behind the synthesis 2.4 "full type-level vs
//! hybrid" fork: can the partition itself be DERIVED at the type level, i.e. can a
//! flat WU cons-list be FOLDED into a cons-list-of-fibers by grouping consecutive
//! data-dependent WUs, purely in the trait solver, without GCE-extreme machinery?
//!
//! Hypothesis: NO without a forbidden/incomplete feature. The fold's per-step
//! decision is "does the next WU depend on the current fiber (extend) or not
//! (start a new fiber)". Encoding that as a type-level branch needs a type-level
//! boolean `DependsBool<RB, WA>::Out in {True, False}`. The `True` arm is the
//! `SharesStore` positive (Tier 2, fine). The `False` arm requires proving the
//! NEGATIVE of `SharesStore`, which needs either `specialization` (FORBIDDEN per
//! `unstable-features.md`: structurally unsound, never stabilises) or
//! `negative_impls` (WATCH: incomplete, does not disarm coherence). Neither is
//! available on nightly-2026-05-28. This file captures the exact wall.
//!
//! If this hypothesis holds, the fork resolves toward HYBRID: the fiber grouping
//! is computed by the plan (the shipped runtime `group_fibers`, which already
//! walks the DAG) and the type level CARRIES the result (Tier 1, proven). Whether
//! emitting that typed carrier from the plan counts as roadmap section-6.5 drift
//! (macro/build-time codegen vs a pure type-derived projection) is the question
//! for the domain-expert consensus, not a thing this sketch can settle alone.
//!
//! Outcome recorded at the bottom.

#![allow(dead_code)]
#![feature(marker_trait_attr)]

use hilavitkutin_api::access::{Cons, Contains, Empty};

// The Tier 2 predicate, restated (positive-only, #[marker]).
#[marker]
trait SharesStore<Other> {}
impl<H: 'static, T: 'static, Other> SharesStore<Other> for Cons<H, T> where Other: Contains<H> {}
impl<H: 'static, T: 'static, Other> SharesStore<Other> for Cons<H, T> where T: SharesStore<Other> {}

// Type-level booleans for the fold's branch.
struct True;
struct False;

// THE WALL. To fold the WU list into fibers, each step must decide depends-or-not
// as a type-level value. That is a total function `DependsBool<RB, WA>::Out`,
// which needs BOTH arms:
//   - positive: RB shares with WA  -> True
//   - negative: RB does NOT share  -> False
// The two impls below overlap on every (RB, WA): the blanket `False` impl applies
// to all pairs, the `True` impl applies to the sharing subset. Without
// `specialization` (forbidden) the solver cannot prefer the more-specific `True`
// over the blanket `False`, so this is a coherence conflict (E0119). With
// `min_specialization` it fails too (`specializing impl repeats parameter`, the
// same wall the engine EngineCtx accessor hit, see unstable-features.md drift #1).
trait DependsBool<WA> {
    type Out;
}

// Positive arm.
impl<RB, WA> DependsBool<WA> for RB
where
    RB: SharesStore<WA>,
{
    type Out = True;
}

// Negative arm as a blanket default. THIS is what cannot coexist with the
// positive arm without specialization: it overlaps the positive subset.
impl<RB, WA> DependsBool<WA> for RB {
    type Out = False;
}

fn main() {
    println!("if this compiled, Tier 3 type-level grouping is feasible; the build is the test");
}

// ---------------------------------------------------------------------
// OUTCOME: FAILS WITH E0119 (conflicting implementations of `DependsBool<_>`).
//
//   error[E0119]: conflicting implementations of trait `DependsBool<_>`
//     --> src/tier3.rs:72:1
//   63 | / impl<RB, WA> DependsBool<WA> for RB where RB: SharesStore<WA>  (first impl)
//   72 |   impl<RB, WA> DependsBool<WA> for RB                            (conflicting)
//
// The full type-level fiber-GROUPING fold is NOT feasible on nightly-2026-05-28
// without a forbidden or incomplete feature. The fold's per-step branch needs a
// total type-level boolean (depends -> extend fiber, else -> new fiber). The
// negative arm of that boolean overlaps the positive arm and cannot be made
// more-specific without `specialization` (FORBIDDEN, unstable-features.md: unsound,
// never stabilises) or `negative_impls` (WATCH: incomplete, does not disarm
// coherence). `min_specialization` fails too (`specializing impl repeats
// parameter`, the same wall the engine EngineCtx accessor hit; unstable-features
// drift #1). GCE does not help: the blocker is negative trait reasoning, not const
// arithmetic.
//
// IMPLICATION (fork resolution, synthesis 202606060900 section 2.4):
//   - The CARRIER + boundary PREDICATE are clean type-level, no GCE (Tiers 1+2,
//     main.rs: zero `blr`, E0277 rejects non-dependent pairs). The keystone's
//     load-bearing devirt premise (roadmap section 9 D1b leeway: "any type-level
//     encoding that CARRIES the per-fiber WU sequence") is PROVEN.
//   - The full type-level DERIVATION of the grouping is NOT available. So the
//     grouping is plan-computed (shipped runtime `group_fibers`, a DAG walk over
//     exactly the `SharesStore`-style dependency Tier 2 proved type-expressible)
//     and the type level CARRIES the result.
//   - OPEN for domain-expert consensus (op asleep, special rule 2026-06-06):
//     emitting the typed per-core carrier from the plan can be done by
//     `hilavitkutin-build` codegen or a macro. Is that the canonical "build time =
//     the type system" realization, or is it the roadmap section-6.5 drift
//     (macro-generated tables vs a pure type-derived projection)? The answer
//     decides whether D1b is fully GREEN (hybrid is canonical) or carries a
//     recorded drift.
// ---------------------------------------------------------------------
