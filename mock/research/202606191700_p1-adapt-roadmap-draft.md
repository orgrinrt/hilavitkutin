# P1 adapt + E8 roadmap draft (chart-the-path phase 5)

**Date:** 2026-06-19
**Phase:** chart-the-path phase 5 (roadmap draft, pre-mirror, pre-sketches)
**Input:** `202606191600_p1-adapt-comprehension.md` (phases 2-4)
**Canonical oracle:** consolidation spec domain 22 (:2006-2076), E8 trigger (:2042-2048)

Ordered steps from current `dev` state to the complete domain-22 adapt + E8
subsystem. Each step is one mockspace round (or a short sequence on one branch).
Marked PROVEN (an existing sketch/bench/shipped pattern confirms it) or UNPROVEN
(needs a phase-10 sketch before implementation). The mirror (phase 6) and
granularity (phase 8) passes refine this; it is a draft.

## Step R1: AdaptArena field layout + runtime population

The per-frame scratch home every axis writes into. Hot lines per fiber, cold SoA
per pass (per `adapt/arena.rs:17` doc). Sized by `PlanDims` capacity types
(`Cores`, `Units`, `AccumsPerCore` precedent from P0.1c). Allocated via the
scheduler's `MemoryProvider`, owned by the scheduler (engine-owned, like
`MetaBlock`), not a consumer store.

UNPROVEN. Sketch: does a `Capacity`-generic `AdaptArena<D>` field on the
`Scheduler` populate per-frame without a GCE/array-length wall (same pattern as
the P0.1c grouping arrays, so low risk, but the hot/cold split shape is new).

## Step R2 (KEYSTONE): AdaptWu real engine Ctx via CtxFor into OnMeta dispatch

Replace `AdaptCtxUnimplementedStub` (`adapt_wu.rs`) with the real engine Ctx:
`type Ctx<'frame> = CtxFor<'frame, AdaptWu::Read, AdaptWu::Write, OnMeta<ScheduleEnd>>`
computed from AdaptWu's nine-axis access set (the just-merged P0.2 CtxFor). Wire
it through the meta-WU `OnMeta` dispatch path the E4 slices built (slice-2 meta
lifecycle bands + slice-3 MetaBlock bridge). Everything else depends on this.

UNPROVEN, highest-risk. Sketch FIRST (per the routine: keystone before the rest).
The sketch must prove: AdaptWu's CtxFor-computed Ctx, carrying nine metrics
Resources in its access set, dispatches through the `OnMeta<ScheduleEnd>` band on
a real scheduler frame, and the execute body can read/write those metrics via the
Ctx accessors. Leeway: the exact schedule marker (`ScheduleEnd` vs a new
`ScheduleAdapt` meta virtual) is open; the sketch proves the family works, the
mirror/granularity passes pick the marker.

## Step R3: AdaptWu::execute body (nine-axis sample + threshold + anomaly)

Walk the nine metrics Resources, threshold-compare each enabled axis (cheap
bitmask compares per spec :2042), set the `anomaly: Bool` on drift, fire the
anomaly virtual. Per-axis sampling logic lands here (timing axes read the
MetaBlock EMA pattern pass_duration established).

UNPROVEN (depends R1+R2). Decomposes per-axis: the timing axes (phase_ema,
fiber_ema) follow the shipped pass_duration pattern (PROVEN pattern); the others
(change_class via domain-12 gen counters, throughput, core_idle_time,
memory_watermark) are new sampling logic.

## Step R4: ema_update bench-validated body

The vectorized per-unit EMA batch update (spec :2040, NEON 4xU32 / AVX2 8xU32 /
scalar). A 7-instruction NEON kernel was bench-sketched historically
(`adapt_ema.rs` doc references it). Needs a per-unit EMA storage array on
`SchedulerMetrics` first (spec :2038; currently only the scalar
`ema_pass_duration_ns` exists).

PARTIALLY PROVEN (NEON kernel sketched before; re-confirm against current
nightly + the new per-unit storage). Bench-gated per arvo-always-optimal: the
SIMD vs scalar choice is a bench decision, not a design call.

## Step R5: select_adapt_config decision loop

The between-frame decision function (spec :2044-2048): compare EMAs to thresholds,
emit the three trigger tiers (recompute morsel sizes / re-select per-phase config
/ full plan recompute). This is the core unbuilt piece, named only in BACKLOG.

UNPROVEN, second-highest-risk after R2. Sketch: the decision-fn shape + how it
reads the arena/metrics and emits triggers (a `Virtual` per tier, or a returned
trigger enum the meta kernel acts on). Retire the legacy `AdaptMode =
PhaseStrategy` alias here (per-axis migration, the adapt/mod.rs Pass-7 note).

## Step R6: E8 plan-recompute body (#345)

Consume `plan_dirty` in `run` (currently `let _ = &self.plan_dirty` at
scheduler:1309), gate an `OnMeta<PlanStage>` rebuild. Tier-3 trigger (record
count changed → full recompute) + the morsel-size/config-reselect cases from R5.
`replace_resource` already sets the dirty bit; this is the consume+rebuild path.

UNPROVEN. Sketch: does an `OnMeta<PlanStage>` firing gated on plan_dirty rebuild
the ExecutionPlan from the plan_cache on the next frame, single-core first, then
the parallel path. Shares domain-22 with R5; charted together.

## Step R7: PMC / perf_event sampling for cache_residency

Axis 5 needs hardware perf counters. Depends on platform-tier threading work
(os/no_os tiers). Lowest priority, can land last or defer to a follow-up.

UNPROVEN, platform-gated. Likely a separate arc; flag as the tail.

## Sequencing

R1 → R2 (keystone) → R3 → {R4, R5} → R6 → R7. R4 and R5 can proceed in parallel
once R3 lands (R4 is the SIMD kernel, R5 is the decision loop; they meet at R6).
StandardAdaptKit (6 axes) drives the default-on set; R3 wires those six first,
the opt-in three (cache_residency, throughput, memory_watermark) follow.

## Open questions for the mirror + granularity passes

1. Schedule marker for AdaptWu: reuse `OnMeta<ScheduleEnd>` or introduce a
   dedicated `ScheduleAdapt` meta virtual after ScheduleEnd? (R2)
2. Trigger emission shape from select_adapt_config: per-tier `Virtual`s vs a
   returned trigger the meta kernel acts on? (R5)
3. Per-unit EMA storage: a `<Units as Capacity>::Array<Nanos>` on the MetaBlock,
   or a separate adapt-arena cold-SoA region? (R4)
4. Does the morsel-size recompute (tier-1 trigger) need a full OnMeta<PlanStage>
   or a lighter in-place morsel re-chunk without a plan rebuild? (R5/R6)
5. E8 + R5 round boundary: one round or two? (they share the trigger taxonomy)

## Phase 7 revisions (post canonical-mirror, 2026-06-19)

The phase-6 canonical mirror (expert dispatch against spec domain 22) resolved
the open questions and surfaced fixes + three omitted spec requirements. Applied:

### Open-question resolutions (spec-dictated)

- **OQ1 (schedule marker): RESOLVED → reuse `OnMeta<ScheduleEnd>`.** The meta-
  virtual set is closed at four (spec :2124-2128: PlanStage, ScheduleReady,
  PassStart, ScheduleEnd). `ScheduleEnd` is the only between-frames lifecycle
  point. No new `ScheduleAdapt` virtual; introducing one would violate the
  closed set. R2 uses `OnMeta<ScheduleEnd>`.
- **OQ3 (per-unit EMA storage): RESOLVED → `meta::SchedulerMetrics` resource.**
  Spec :2134 names SchedulerMetrics as the home for "EMA timing data (domain
  22)". The per-unit EMA array lands as a field on SchedulerMetrics, NOT on a
  separate arena cold-SoA region and NOT a bare MetaBlock field. R4 targets it.
- **OQ4 (tier-1 morsel recompute path): PARTIALLY RESOLVED → tier-1 is a
  distinct LIGHTER in-place re-chunk, NOT `OnMeta<PlanStage>`.** Spec's three-
  tier structure (:2042-2048) explicitly separates the cheap morsel-size
  recompute (tier 1) from the full plan rebuild (tier 3). Collapsing tier-1 into
  the PlanStage rebuild violates the distinction. R6 splits: tier-1 lighter
  re-chunk, tier-3 PlanStage rebuild.
- **OQ2 (trigger emission shape): GENUINELY OPEN.** Spec :2045/:2048 reference a
  "dirty propagation pattern" + "cheap bitmask comparisons", leaning toward a
  bitmask/dirty-flag shape over per-tier `Virtual`s, but does not prescribe the
  programming model. Decide at the R5 sketch (phase 10), bench/shape-driven.
- **OQ5 (round boundary): spec silent (mockspace-process).** R5+R6 land as one
  arc (they share the trigger taxonomy); split into sequential rounds on one
  branch if the implementation reveals the size warrants it.

### R3 fix: split static-tier vs runtime-tier axes

The draft conflated "nine axes" sampled in AdaptWu::execute. Spec splits them:
- **Static tier (plan-time analyses, NOT between-frame samples; spec :2013-2017):**
  cache-pressure profile, data-flow volume map, column-lifetime map, memory
  watermark. These are computed at plan construction, READ at runtime, not
  sampled by AdaptWu each frame.
- **Runtime tier (between-frame samples; spec :2019-2026):** per-morsel timing,
  change frequency (domain-12 gen counters), cache residency, EMA pass duration,
  throughput trend.
R3 samples ONLY the runtime-tier axes each frame; the static-tier values are
plan-time outputs it reads. The `memory_watermark` axis is static-tier (plan-
time), so it is NOT a per-frame AdaptWu sample; reclassify it out of the
StandardAdaptKit per-frame set if currently there.

### Added steps (omitted spec requirements the mirror found)

- **R8: morsel temperature -> core assignment (spec :2028-2033, domain 20).**
  Sampled morsel temperature (hot/warm/cold) routes hot morsels to P-cores, cold
  to E-cores. No current code path. New step: write temperature from the timing
  axis, feed it to the core-assignment decision in the parallel dispatch. UNPROVEN.
- **R9: predictive-parking wait tiers (spec :2050-2058, domain 17).** Compute
  predicted wait from other cores' remaining compute weight, pick a tier (under
  100 ns spin, 100 ns to 10 us backoff, over 10 us park), write to
  `PoolFrame.predicted_wait_ns`. The `PredictiveParkingAxis` config exists; the
  wait-tier computation + PoolFrame write is the unbuilt part. UNPROVEN. This is
  axis 7's real body (R3 only sets its anomaly flag; R9 is the actuation).
- **R10: domain-21 strategy re-selection between frames (spec :2074).** A named
  domain-22 output: re-select the per-phase strategy marker between frames on
  phase-balance shift. This is the tier-2 trigger's actuation (R5 emits the
  trigger; R10 applies the strategy swap). Retires the legacy `AdaptMode =
  PhaseStrategy` alias. UNPROVEN.

### Revised sequencing

R1 -> R2 (keystone) -> R3 (runtime-tier axes only) -> {R4 ema kernel, R5 decision
loop} -> {R6 E8 tier-1/tier-3 split, R10 strategy reselect = tier-2 actuation, R9
predictive parking, R8 morsel-temp->core} -> R7 (PMC, platform-gated, tail).
R8/R9/R10 are the actuation steps that consume R5's triggers; they can parallelize
once R5 lands. The keystone R2 sketch is still FIRST (phase 10).

### Next routine steps

Phase 8: granularity expert over this revised roadmap (split R2 keystone, R5
decision loop, and the R8/R9/R10 actuation steps if any hide hard sub-problems).
Phase 10: sketches, R2 keystone first (AdaptWu CtxFor Ctx through OnMeta<ScheduleEnd>
dispatch), then R5 (decision-fn shape + OQ2 trigger model), then per-step.
