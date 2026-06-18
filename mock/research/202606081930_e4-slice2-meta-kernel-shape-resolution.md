# E4 slice-2 meta-kernel shape: design resolution

**Date:** 2026-06-08
**Round:** 202606081900 (GATE-2 E4 slice-2, task #685), at TOPIC
**Inputs:** canonical core-design Q9 (`mock/design_rounds/202604200055/202603141800_topic.hilavitkutin-core-design.md:716-814`) + consolidation `:2119-2137`/`:1989`; neutral `feature-dev:code-architect` read (agent a9f6a53d566ec3a4b); shipped slice-1 machinery.

The slice-2 topic flagged one load-bearing fork: how the kernel runs meta WUs in lifecycle order (plan-stage -> consumers -> epilogue) within one frame without re-running unconditional `Always` consumers in every lifecycle sub-pass. Three candidate shapes were posed (A implicit-grouping-edges, B kernel-sequenced sub-passes with a schedule-class gate, C separate meta-carrier). This note records the resolution so the DOC CL can lock it.

## Resolution: Shape A (same carrier + implicit ScheduleReady edge + kernel fires at phase boundaries)

The canonical is explicit and decides the fork: "meta units are in the same DAG, dispatched the same way" (`:781`); "all consumer work units implicitly depend on [ScheduleReady] — the scheduler adds the edge automatically; consumers never declare it" (`:726`); "the implicit ScheduleReady edge adds one virtual flag check per consumer unit per pass" (`:784`). That rules out Shape C (separate carrier contradicts "same DAG") and Shape B (a schedule-class gate is a dimension the canonical does not describe, and it needs multiple dispatch calls even on the steady-state non-plan-dirty frame). The architect read reached the same conclusion independently.

The mechanism that makes Shape A work without double-running `Always` consumers: the lifecycle ordering is carried by the EXISTING waist-based phase grouping, not by multiple dispatch passes. Meta WUs land in their natural phases:
- `On<meta::PlanStage>` WUs in early phase(s);
- consumer WUs in the middle phases;
- `On<meta::ScheduleEnd>` WUs in the final phase(s).

One phase-ordered `dispatch_trunks` runs everything in lifecycle order; the slice-1 fired-flag gate selects which run (PlanStage only on a plan-dirty frame). The kernel fires each meta virtual at the corresponding phase boundary (between phases in the phase loop), so a meta WU's gate is open exactly when its lifecycle point is reached. Three `fire()` calls per pass, the "zero meaningful overhead" the canonical promises (`:785`).

The ordering that places consumers AFTER plan-stage is the canonical's "scheduler adds the ScheduleReady edge automatically": every non-PlanStage WU gains a synthetic read-dependency on `Virtual<meta::ScheduleReady>` (which the plan-stage lifecycle writes), so the waist grouping orders plan-stage < consumers. This is also where slice-1's known limitation (the grouping does not treat a `Virtual<V>`-write -> `On<V>` as a scheduling edge) gets resolved FOR THE META EDGE specifically: the plan-construction injects the edge rather than relying on the grouping to infer it.

## Riskiest sub-mechanism to sketch before the DOC CL locks

The synthetic-ScheduleReady-edge injection in the plan-construction path (`plan_inputs_from_bundle`, `scheduler/mod.rs:54`; the const `BundleProject`/`BundleMasks` over `MaskProject`, `plan/project.rs`/`plan/grouping.rs`). The plan masks are computed at const-eval from declared `Read`/`Write` access sets. Injecting an extra read-mask bit for a virtual the consumer did not declare requires classifying each WU at plan time as consumer-class (gets the bit) vs `On<meta::PlanStage>`-class (does not), by reading `<W as HasSchedule>::Sched` (the slice-1 companion trait). The sketch must demonstrate, under the engine's `generic_const_exprs` const machinery, that the plan path can read `Sched`, distinguish `On<meta::PlanStage>` from the rest, and conditionally set a mask bit, compiling clean. If const-time type classification of the assoc type is not const-expressible, the fallback is to inject the edge at the (runtime) bindings/plan-build step rather than the const grouping, or to require consumer WUs carry the edge via a blanket-added marker; the sketch decides.

## My assessment (own read, beyond the architect's)

Agree with Shape A as canonical-faithful. Two reservations the architect under-addressed, both for the implementation phase, neither changing the shape choice:

1. **`run_parallel` boundary firing.** Single-core `dispatch_trunks` fires between phases on the main thread trivially. In `run_parallel` the phase boundaries are worker-side sense-reversing waist barriers (`thread/barrier.rs`, `run_core_phase`); there is no single between-phase main-thread moment. The meta-virtual fire at a phase boundary must be done by a designated thread (e.g. the barrier-releasing worker, or core 0 at the barrier) so the stamp is visible to the next phase's gate under the barrier's happens-before. This is a real wiring detail for the parallel path; single-core lands first, parallel parity second (the slice-1 / G2-N pattern).

2. **Const-edge-injection blast radius.** Touching the const grouping is delicate (it is the G-c/G-d generic_const_exprs machinery). The sketch above is the gate on whether the edge lives in the const path or a runtime plan-build step; prefer the smallest-blast-radius site that still produces the correct phase ordering.

## Resolved questions (from the topic)

1. Carrier shape: SAME carrier (canonical), not separate; kernel fires at phase boundaries; ordering via waist phases + synthetic ScheduleReady edge.
2. Slice-2 vs slice-3 split: slice-2 ships the `meta` module (4 virtuals + 4 meta resource marker types + `MetaAccess` ZST marker), the synthetic ScheduleReady edge, the kernel firing the 4 at lifecycle points, and TDD (On<meta::PlanStage> runs only on plan-dirty; consumer runs only after ScheduleReady; On<meta::ScheduleEnd> runs after consumers). DEFER to slice-3/E8: `MetaAccess` compile-time Context enforcement, real meta-resource bodies + EMA metrics WU, phase-overlap pipelining, the adaptation WU body.
3. `MetaAccess` mechanism: ZST marker trait on the 4 meta resource types + a Ctx access bound gated on a `meta::*` schedule (canonical `:811-814`); the ENFORCEMENT bound is slice-3, the marker + types land in slice-2.
4. "Wait for plan WUs": the plan-stage WUs occupy the early phase(s); the kernel fires ScheduleReady at the plan->consumer phase boundary (after the plan phase trunks complete), which is the "wait" expressed structurally.
5. Cadence: `PlanStage` fired only on a plan-dirty frame (the existing `plan_dirty` seed on the scheduler); `PassStart`/`ScheduleReady`/`ScheduleEnd` every pass.

## Next step

De-risk sketch (the synthetic-edge / const-classification mechanism above) -> DOC CL (resolve + the `## Self-hosting meta pipeline` DESIGN.md.tmpl section, citing this note) -> src CL (`meta` module + edge injection + kernel) -> TDD -> lock -> close. Single-core first; `run_parallel` boundary-fire parity second.

## See also

Topic `202606081900_topic.gate2-e4-slice2-self-hosting-meta-pipeline.md`; canonical Q9 `:716-814`; slice-1 round 202606081200 (closed); breadcrumb `[[engine-completion-roadmap-routine]]`.
