# P1 adapt + E8 granularity + sketch plan (chart-the-path phases 8-9)

**Date:** 2026-06-19
**Phase:** chart-the-path phase 8 (granularity) + phase 9 (finalise + plan sketches)
**Inputs:** `202606191600_p1-adapt-comprehension.md`, `202606191700_p1-adapt-roadmap-draft.md`
**Canonical oracle:** consolidation spec domain 22 (:2006-2076)

The phase-8 granularity pass (expert dispatch) split each roadmap step into
sketch-able sub-steps and ranked risk. This finalises the roadmap into the
sub-step list and plans the phase-10 sketches with pinned success criteria +
accepted leeway. Phase 10 (writing the sketches) is the next step; the keystone
R2a sketch is first.

## Sub-step decomposition

- **R1a** AdaptArena<D> type (hot/cold SoA split, Capacity-generic fields, ctor). Low risk (P0.1c pattern). One sketch.
- **R1b** Scheduler field + per-frame population across run/run_parallel/run_fused. Risk: scheduler-generic-to-arena-capacity binding. Sketch before.
- **R2a** (KEYSTONE, hard) AdaptCtx = EngineCtx over the nine-resource access set + one Virtual write; close all index-witness chains at once. The largest access set the projection has seen; if GCE normalization / trait-solver depth bites, R2 dies here. SKETCH FIRST.
- **R2b** confirm consumer-registered OnMeta<ScheduleEnd> WU dispatches through the existing meta-band (band keyed by phase rank, not access-set width; `dispatch_trunks` ~1455-1496). Mostly proven (gate2_meta_metrics EndWu template); fold the `.with(AdaptWu::default())` compile-check into the R2a sketch.
- **R2c** MetaPtrFor<OnMeta<ScheduleEnd>> -> MetaRef 9th param. Proven (engine_ctx.rs:1015 + EndCtx). Atomic.
- **R2d** execute reads one resource + fires one virtual via real Ctx (not stub). Prove in the R2a sketch with a minimal non-no-op body.
- **R3a** timing axes (phase_ema/fiber_ema/pass_duration) via ctx.meta::<SchedulerMetrics>() threshold. Proven pattern. Atomic after R2.
- **R3b** change_class via domain-12 generation counters. INVESTIGATE FIRST: gen-counter source is not MetaBlock nor the nine metrics resources; locate it.
- **R3c** throughput + core_idle_time: new per-pass instrumentation tap points (engine emits pass_count + ema_pass_duration_ns only). INVESTIGATE FIRST: locate/define the tap points in the dispatch paths.
- **R3d** fire AnomalyFired once if any axis tripped. Trivial tail.
- **R4a** per-unit EMA array on SchedulerMetrics (<Units as Capacity>::Array<Sample>, spec :2038). INVESTIGATE: is Capacity::Array Copy? SchedulerMetrics is Copy (MetaField projection); a non-Copy array field breaks MetaBlock copy semantics.
- **R4b** port the bench-validated 7-instruction NEON kernel (sketch 202605101036, marked WORKS) into ema_update_neon + scalar-parity test. Bench-gated. Atomic after R4a.
- **R5a** (hard) decide OQ2 trigger-emission shape (bitmask/dirty-flag vs per-tier Virtual). Sketch both, evaluate call-site ergonomics, lock. Conditions all actuation steps; cannot parallelize downstream. SKETCH.
- **R5b** implement select_adapt_config in the R5a shape; per-axis EMA threshold compares -> tier-1/2/3 triggers; retire AdaptMode=PhaseStrategy alias. Atomic after R5a.
- **R6a** (hard) tier-1 in-place morsel re-chunk. INVESTIGATE/SKETCH: are morsel sizes runtime params or baked into the const-generic dispatch program? If baked, tier-1 is not lightweight and the spec's tier distinction needs a different mechanism.
- **R6b** tier-3 full plan recompute gated on OnMeta<PlanStage> + plan_dirty, rebuild from plan_cache. Mostly proven (E4 meta band + plan_cache exist; consumption is the `let _` at scheduler:1309). Atomic after R6a.
- **R7** PMC/perf_event cache_residency. Atomic, platform-gated, tail.
- **R8a** define + write per-morsel temperature carrier (field on MorselRange / plan phase / separate array?). SKETCH: where temperature lives + how the timing axis populates it.
- **R8b** (hard) read temperature in parallel core-assignment; route hot->P-core cold->E-core. SKETCH/INVESTIGATE: does ThreadPool/Executor expose P-core vs E-core? If not, this is a new contract addition (larger than it looks).
- **R9a** (hard) compute predicted wait from other cores' remaining compute weight at the park point (run_parallel ~1638). SKETCH: what data is available at the park point; can predicted wait be computed without a new global structure?
- **R9b** write tier decision to PoolFrame.predicted_wait_ns. Atomic after R9a.
- **R10a** (hard) locate per-phase strategy markers at runtime + confirm mutability between frames. If baked into the compile-time generic program, "re-select" = a plan recompute (OnMeta<PlanStage>), not a field write. Conditions whether R10 is lightweight. INVESTIGATE/SKETCH.
- **R10b** implement the swap per R10a, consuming R5's tier-2 trigger. Atomic after R10a.

## Phase-10 sketch order (pinned criteria + leeway)

Sketches go in `mock/research/sketches/<ts>_<topic>/` per cl-claim-sketch-discipline (hypothesis at top, real code vs real crates, outcome WORKS/FAILS-WITH/INCONCLUSIVE).

1. **R2a keystone (FIRST, blocks the arc).** Hypothesis: AdaptWu's CtxFor-computed Ctx, with all nine metrics Resources + one Virtual<AnomalyFired> write in its access set, compiles and dispatches through OnMeta<ScheduleEnd> on a real scheduler frame, and a minimal execute body reads one resource + fires the virtual. Success: builds + the test frame runs + the read/fire observe correctly. LEEWAY: the family works (the exact nine-resource ordering / which concrete metrics types) is flexible; what must hold is "a nine-resource OnMeta consumer Ctx projects and dispatches". If it FAILS on trait-solver depth / GCE normalization, that is the keystone course-correction -> AskUserQuestion per the routine.
2. **R5a trigger shape.** Hypothesis: a bitmask/dirty-flag return from select_adapt_config is expressible and ergonomic vs per-tier Virtuals. Success: both shapes compile in a sketch; pick on call-site clarity (spec :2045/:2048 lean bitmask). LEEWAY: exact flag layout open; the shape family (bitmask vs Virtual) is what locks.
3. **R6a morsel re-chunk.** Hypothesis: morsel sizes are runtime-mutable plan params (not const-generic-baked), so tier-1 re-chunk is an in-place field update. Success: a sketch mutates a morsel size on an existing plan + re-runs without a rebuild. If baked: FAILS -> tier-1 collapses into tier-3, AskUserQuestion.
4. **R10a strategy mutability.** Hypothesis: per-phase config (MAX_FUSE/BALANCED/MAX_SPLIT) is a runtime param mutable between frames. Same baked-vs-runtime fork as R6a; same course-correction if baked.
5. **R8b P/E-core abstraction.** Hypothesis: the ThreadPool/Executor contract can express core-type without a breaking change. If not: INCONCLUSIVE -> a contract-addition design sub-round.
6. **R9a park-point data.** Hypothesis: remaining-compute-weight is derivable at the park point from existing per-trunk cost data. If a new global structure is needed: scope expansion, note it.

Investigate-before-sketch (no sketch, just source reads, may reveal missing abstraction): R3b gen-counter source, R3c throughput/core-idle tap points, R4a Capacity::Array Copy semantics, R8a temperature carrier.

## After phase 10

Each confirmed sketch records its proven concrete shape back here. Then P1
implementation is mechanical per the sub-step list: R1 -> R2 -> R3 -> {R4,R5} ->
{R6,R8,R9,R10} -> R7, each sub-step a mockspace round on a feature branch. The
keystone R2a result gates whether the whole CtxFor-into-OnMeta approach holds or
needs the human course-correction the routine reserves for a failed keystone.
