# Findings: M2, type-level dedup-on-Concat

**Date:** 2026-05-05.
**Round:** 202605042200.
**Maps to:** Audit M2 remediation.
**Outcome:** Physical dedup BLOCKED by the same coherence constraint as round-3 NotIn. Logical dedup via `#[marker] Contains` is what the typestate already gets for free; it suffices for correctness. Round-4 ships without physical dedup; trait-solver cost from duplicate accumulation is real but not pathological at the validated depth-5 / 25-WU scale.

## Setup

S1b's depth-5 sketch (`../202605050700_deep_stacking_d5/`) confirmed that the bundle accumulator's recursive Concat produces duplicate-laden AccumRead and AccumWrite chains. At depth 5 with 25 WUs touching 13 distinct stores, the resulting Cons chain has roughly 36 nodes; StringInterner appears 4 times, Clock 4 times. The audit's M2 finding asked whether type-level dedup at `Concat` time is feasible, on either of two candidate shapes:

1. **Set-difference operator.** `Difference<L, R>::Out = L \ R`. Walks L for each element of R and removes matches.
2. **Concat-dedup operator.** `ConcatDedup<R>::Out = L ++ (R \ L)`. Walks R; for each element, prepends to L if not already present.

Both rely on a type-level distinction between "L contains H" and "L does not contain H" to drive different recursive cases. That distinction is the round-3 NotIn problem.

## What was built

Three sketches under this directory:

| File | Outcome | What it tests |
|------|---------|---------------|
| `attempt_a_set_difference.rs` | Compiles (trivial no-op shape) | Set-difference; the actually-discriminating shape is commented out and would fire E0119/E0751. |
| `attempt_b_skip_concat.rs` | Compiles (trivial no-op shape) | ConcatDedup; same comment-out structure as A. |
| `attempt_b_failing_discriminating.rs` | Fails E0119 | ConcatDedup with the actually-discriminating two-impl shape. The compile failure is captured below. |
| `attempt_c_marker_logical_dedup.rs` | Compiles | Demonstrates that `#[marker] Contains` resolves correctly on duplicate-laden Cons chains; physical dedup is not required for the typestate proof. |

## The compile failure (attempt B, discriminating shape)

```
$ rustup run nightly rustc --crate-type=lib --edition=2024 \
    attempt_b_failing_discriminating.rs --emit=metadata
error[E0119]: conflicting implementations of trait `ConcatDedup<Cons<_, _>>`
  --> attempt_b_failing_discriminating.rs:50:1
   |
41 | / impl<L: AccessSet, H, T> ConcatDedup<Cons<H, T>> for L
42 | | where
43 | |     L: Contains<H>,
44 | |     L: ConcatDedup<T>,
   | |______________________- first implementation here
...
50 | / impl<L: AccessSet, H, T> ConcatDedup<Cons<H, T>> for L
51 | | where
52 | |     L: NotContains<H>,
53 | |     Cons<H, L>: ConcatDedup<T>,
   | |_______________________________^ conflicting implementation
```

The two impls of `ConcatDedup<Cons<H, T>> for L` differ only by their where-clause guards (`L: Contains<H>` vs `L: NotContains<H>`). Rust's coherence checker does not admit where-clause-based discrimination as distinguishing for trait selection; the impl signatures collide.

The fix would be one of:

a. **Negative impl on the conflict.** Round-3's NotIn sketch ruled this out: `feature(negative_impls)` cannot encode `(K, R): !NotIn<H>` for the parameterised case. The NotContains marker has no implementations to back it.
b. **Specialisation.** `feature(specialization)` is much more unstable than the substrate's other nightly features and the workspace forbids it.
c. **Type-level disequality witness.** Equivalent in power to `TypeId`, which the workspace forbids.

The audit M2 anticipated this outcome ("If the dedup operator itself has trait-solver pathology, the round-4 plan accepts the duplicate cost").

## Why physical dedup is not required for correctness

`attempt_c_marker_logical_dedup.rs` demonstrates the property the substrate actually relies on: `#[marker] Contains` resolves on duplicate-laden chains the same way it does on deduplicated chains. The two `Contains` impls (head-match and tail-recurse) are marker-marked, so overlap is permitted; the trait solver picks any matching occurrence and stops.

A chain `Cons<X, Cons<X, Cons<Y, Cons<X, Empty>>>>` satisfies:

- `Contains<X>` (resolves at the first head, stops).
- `Contains<Y>` (recurses past the X duplicates, resolves at Y).
- `ContainsAll<Cons<X, Cons<Y, Empty>>>`.
- `ContainsAll<Cons<X, Cons<X, Cons<Y, Empty>>>>` (a request list with duplicates also resolves).

The witness functions in attempt C compile clean. Implication: AccumRead/AccumWrite duplicates do not break or duplicate the build-time proof. They show up only as longer Cons chains in the trait-solver's work and in error messages.

## Cost analysis

Without physical dedup, two costs remain:

1. **Trait-solver traversal length.** ContainsAll<L> over a duplicate-laden L of length M does M iterations, each checking Contains<H_i> against Stores. With `#[marker]` Contains, each H_i check resolves at the first match; if Stores is itself duplicate-laden, each check pays at most "depth in Stores to first match" cost. For a 36-element Cons chain over a 13-element Stores, naive O(M * depth) = roughly 36 * 6 = ~216 trait-solver steps for a single .build() proof. Empirically this lands at 0.53s wall clock at depth 5 (S1b finding).
2. **Error message verbosity.** When Stores does not satisfy ContainsAll<AccumRead>, the diagnostic prints the full Cons chain. At depth 5 the chain is 36 nodes; rustc emits a "long type written to file" note. The marker-name lookup remains visible at the top of the diagnostic regardless. Workable per M3.

Neither cost is pathological at round-4's validated scale. They do compound at higher N (depth 6+, 50+ WUs); when a substrate consumer reaches that scale, dedup becomes worth revisiting via specialisation or a new approach (waiting on `feature(specialization)` to mature, or on an unforeseen alternative).

## Recommendation

Round-4 ships without physical dedup. The doc CL captures the duplicate-cost reality:

- AccumRead / AccumWrite carry duplicates by design at round-4.
- Logical dedup via `#[marker] Contains` makes duplicates correctness-irrelevant.
- Trait-solver cost grows linearly with chain length; pathology has not been observed at the validated depth-5 / 25-WU scale.
- Physical dedup is a known follow-up (round-5+ or beyond) gated on either specialisation stabilisation or a coherence-clean alternative encoding.

The audit's escape clause ("doc CL captures the inefficiency as a cost") applies. M2 closes as INVESTIGATED, DEFERRED.

## Cross-references

- `mock/research/sketches/registrable-not-in-202605051200/FINDINGS.md`. Round-3 NotIn proof; the underlying coherence problem.
- `mock/research/sketches/202605050530_deep_stacking/FINDINGS.md`. S1 depth-4 baseline; the duplicate-laden chain pattern first observed there.
- `mock/research/sketches/202605050700_deep_stacking_d5/FINDINGS.md`. S1b depth-5; chain-length data point.
- `mock/design_rounds/202605042200_topic_round_4_audit.md`. Audit topic, finding M2.
- `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md`. Sketch-and-record discipline this file follows.
