# Findings: E4 slice-2 rank-outer phase renumber

**Date:** 2026-06-08
**Round:** 202606081900 (E4 slice-2, task #685), DRAFT phase (doc CL locked)
**Outcome:** WORKS.

## Hypothesis

Shape A (locked in the slice-2 DOC CL) orders meta work units into lifecycle
bands around consumers by making each unit's lifecycle RANK the outer phase key
and the existing waist-phase the inner key, then renumbering distinct
`(rank, waist_phase)` pairs to contiguous phase ids. This sketch proves the
renumber, in the const-fn / fixed-array form the engine's `compute_phases_waist`
uses, produces correct band ordering, preserved within-rank waist order,
contiguous ids, and shared ids for equal pairs.

## The five ranks

The de-risk for lifecycle CLASSIFICATION (sketch `202606082000`) used a 3-rank
simplification (plan / consumer / epilogue) sufficient to prove disjoint
`OnMeta<V>` classification. The real kernel order has FIVE ordered lifecycle
points, so `MetaVirtual::RANK` carries five values:

- `PlanStage` = 0
- `ScheduleReady` = 1
- `PassStart` = 2
- consumer (`Always` / `On<V>`) = 3
- `ScheduleEnd` = 4

Consumers slot at rank 3, between `PassStart` and `ScheduleEnd`. This is
consistent with the DOC CL prose (plan-stage early, consumer middle, epilogue
final); the prose's "(plan-stage, consumer, or epilogue)" is illustrative of the
three coarse bands, not a closed enumeration of exactly three rank integers.

## The renumber (WORKS)

`phase_out[i]` = the count of DISTINCT pairs present in the unit set that are
lex-strictly-less than unit i's `(rank, waist_phase)` pair, where "distinct" is
counted by first occurrence (a pair contributes once, at its lowest-indexed
unit). Pure const fn over fixed arrays, no alloc, no set type, so it drops into
the engine grouping.

Proven properties (assertions, all pass; `cargo run` prints WORKS):

1. Band ordering: a one-WU-per-lifecycle-point scenario yields phases
   `[0,1,2,3,4,5]` (plan < scheduleReady < passStart < consumerA < consumerB <
   epilogue).
2. Within-rank waist order preserved: two consumers with a real RAW edge A->B
   (waist-phases 0 and 1) keep `phase[A] < phase[B]`.
3. Contiguous 0..k: every id in `0..=max` is present (one phase per distinct
   pair).
4. Equal pairs share an id: two `PassStart` WUs with identical waist-phase land
   in the same phase, so within-phase trunk grouping still sees them as
   same-phase. Scenario `[plan, passStart, passStart, consumer]` -> `[0,1,1,2]`.
5. Const-context proof: the renumber runs at const-eval (array-length proof), so
   it composes with the engine's `generic_const_exprs` const grouping.

## Why the rank order never inverts a real edge

Lifecycle data flow runs in increasing rank: plan-stage produces the plan,
consumers read it, the epilogue observes consumer output. So a real
producer->consumer edge that crosses ranks already agrees with the rank order
(lower rank is the producer). Within a rank, the waist-phase carries the real
edges unchanged. So making rank the outer key cannot reorder a producer after
its consumer.

## What this proves for the engine SRC CL

`compute_phases_waist` keeps computing the waist-phase from the RAW DAG, then a
new pass folds each unit's `Lifecycle::RANK` (via a `BundleRanks`-style const
fold mirroring `BundleMasks`) and renumbers `(rank, waist_phase)` into the final
contiguous phase array. `phase_of` / `phase_count` / `trunk_of` then read the
renumbered phases unchanged; `compute_trunks` keys on phase equality, which the
renumber preserves for equal pairs. No new trait-solver risk: the rank fold is
the same shape as the proven mask fold.

## Leeway

SOME-SHAPE: the rank integers (0..4) and the exact renumber inner loop are a
SRC-CL choice; the load-bearing facts (rank is the outer phase key, the
renumber is a const fn producing contiguous lifecycle-ordered bands that
preserve within-rank waist order and share ids for equal pairs) are settled.

## Next

SRC CL: api `meta` module (`OnMeta<V>`, `MetaVirtual` + 4 impls, `Lifecycle` +
3 impls, `MetaAccess` + 4 meta resource markers, `ScheduleGate: Lifecycle`
supertrait), engine grouping `BundleRanks` fold + rank-outer renumber in
`compute_phases_waist`, kernel firing the 4 meta virtuals at band transitions in
`scheduler/mod.rs` `run` (single-core first), TDD. Then `run_parallel` parity.
