# Sketch S2: witness-propagation cost through engine-shaped caller graph

**Date:** 2026-05-13
**Sequel to:** `202605121400-call-site-cap-turbofish` (validated `cap_of` bridge in isolation; left witness propagation as the load-bearing unresolved question)

## Hypothesis

A `where [(); cap_size(cap_of(MAX_UNITS))]:` clause that originates on
a leaf wrapper (the engine-side bridge to arvo) and propagates through
two or three levels of generic callers does NOT cause super-linear
trait-solver work, given a caller graph shaped like the real
`compute_execution_plan` (eleven const generics, multi-generic
intermediate types in returns and parameters, the propagating witness
referencing one of the generics).

The hypothesis is FALSIFIED if `cargo check` warm wall-clock for a
concrete instantiation exceeds ~30 seconds, or if rustc reports
`-Z self-profile` trait-solver counts that scale super-linearly with
the depth of the generic call chain.

## Why the engine-shape matters

The first sketch (S1) had a single-level wrapper with one witness
clause and zero callers. Trait-solver work was bounded by the wrapper
body alone. Warm `cargo check` finished in 2.88s.

The engine differs in three structural ways that may interact
multiplicatively:

1. **Caller chain depth.** `compute_execution_plan` is two levels above
   the arvo wrapper (engine fn → `steps::rcm_reorder` → arvo call).
2. **Const-generic count.** The orchestrator carries 11 const generics;
   intermediate types carry 10. Each generic position is a
   monomorphisation dimension.
3. **Intermediate-type returns.** `ExecutionPlan<G1, G2, ..., G10>` is
   returned through the chain. Every where-clause on the witness-
   carrying type interacts with the trait solver's substitution table
   for the return type.

This sketch models all three.

## Acceptance scorecard

| Criterion | Threshold |
|---|---|
| `cargo check` warm wall-clock | < 30 seconds |
| `cargo check` cold wall-clock | < 6 minutes (cold-build floor matches S1) |
| Trait-solver work growth from 1-level to 3-level chain | sub-linear or linear |
| Per-level witness duplication count | exactly 1 per fn (no auto-multiplication) |

If all four pass, witness propagation through the engine's caller
graph is viable. The path-1 wire-up (engine-side bridge) becomes the
recommended path.

If wall-clock fails but trait-solver count is linear, the failure is
in monomorphisation, not in the solver, and the wire-up needs the
const-generic count brought down or the witness narrowed.

If trait-solver count is super-linear, path-1 is dead. Pivot to path-2
(arvo-side `*_usize` adapter shims).

## What this sketch does NOT test

- The CSR-to-BitMatrix conversion at the wrapper site. Future sketch S3.
- The UnitId/NodeId width adaptation. Future sketch S4.
- The array reshape `[T; MAX_UNITS]` vs `[T; cap_size(cap_of(MAX_UNITS))]`.
  Future sketch S5.
- Whether multiple witness clauses on different MAX_X generics (e.g.
  MAX_UNITS + MAX_PHASES + MAX_FIBERS) interact non-linearly when
  arvo wrappers consume more than one Cap generic. The three real
  stubs only consume MAX_UNITS, so this is deferred.

## Outcome

See `OUTCOME.md` after `cargo check` + measurements.
