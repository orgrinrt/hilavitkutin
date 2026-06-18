# P1 adapt + E8 comprehension (chart-the-path phase 2-4)

**Date:** 2026-06-19
**Phase:** chart-the-path comprehension + synthesis (phases 2 to 4)
**Scope:** runtime-adaptation subsystem (domain 22, #341) + E8 plan-recompute-on-resource-swap (#345)
**Canonical oracle:** `design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` domain 22 (lines 2006 to 2076), E8 trigger (:2042 to 2048)

This is the durable comprehension artifact for the P1 arc. It captures the
expert comprehension pass (chart-the-path phase 2) plus synthesis (phase 3 to 4).
The roadmap draft (phase 5), the canonical mirror (phase 6), the granularity pass
(phase 8), and the sketches (phase 10) are the remaining routine steps; this
document is their input. Truth-of-impl as of `dev` HEAD after the P0 foundation
merges (PR #133/#134/#135/#136).

## What the subsystem IS (canonical)

Two analysis tiers (spec :2013 to 2027). Static tier at plan construction:
cache-pressure profile, data-flow volume map, column-lifetime map, memory
watermark. Runtime tier between frames: nine measurement axes feeding plan
parameters (per-morsel timing, change-frequency via domain-12 generation
counters, cache residency, EMA of pass duration for frame-budget prediction,
throughput trend).

Output is a set of adjustment triggers (spec :2042 to 2048), three tiers, cheapest
first, all bitmask threshold compares between frames:
1. fiber morsel timing shifted -> recompute morsel sizes.
2. phase balance shifted -> re-select per-phase adaptive config (domain 14:
   MAX_FUSE / BALANCED / MAX_SPLIT).
3. record count changed -> full plan recompute (this is the E8 / domain-23 path).

EMA decay is `(ema * 7 + measured) / 8` (spec :2039); spec calls for a vectorized
batch update (NEON 4xU32 / AVX2 8xU32) over all per-unit EMAs in one pass
(:2040). Metrics live in `meta::SchedulerMetrics` (:2134). Predictive parking
(:2050 to 2058) estimates wait from other cores' remaining compute weight and
picks spin/backoff/park, tied to domain 17 (compiled programs) + domain 20 (core
assignment). Morsel temperature (hot/warm/cold) feeds P-core vs E-core assignment
(:2028 to 2033, domain 20).

E8: `replace_resource` on a `PlanAffecting` store sets a per-store dirty bit in
`plan_dirty`; the next `run` consumes it to gate a plan rebuild (trigger tier 3).

## Built vs unbuilt (truth-of-impl, cited)

Built and shipped:
- `fold_ema` scalar EMA kernel (`meta.rs:62`), 4 unit tests (`meta.rs:75-108`),
  fired at frame end in `run` (`scheduler/mod.rs:1373`), `run_parallel` (:1794),
  `run_fused` (:2023). `SchedulerMetrics.ema_pass_duration_ns`
  (`hilavitkutin-api/src/meta.rs:96`) is the one live axis.
- All nine axis config types (`hilavitkutin-api/src/adapt.rs`, e.g.
  `PassDurationAxis` :128, `PredictiveParkingAxis` :158) with `is_enabled()` /
  `sample_skip()`; `AdaptAxis` trait + `AdaptAxisDispatch`. Engine re-exports thin
  (`hilavitkutin/src/adapt/*`).
- All nine `*Metrics` Resources (`hilavitkutin-providers/src/metrics/`,
  `metrics_resource!` macro `metrics/mod.rs:51`), each `last_sample: USize`,
  `anomaly: Bool`.
- `AdaptWu` WorkUnit declaration with the full nine-axis access set
  (`hilavitkutin-providers/src/adapt_wu.rs:64`).
- `StandardAdaptKit` (6 of 9 axes) + `OffAdaptKit` (`adapt_kits.rs:30,61`).
- `ema_update` batch struct + cfg-gated NEON/SSE2/scalar dispatch shape
  (`adapt_ema.rs:87`); a 7-instruction NEON kernel was bench-sketched.
- `AdaptArena` capacity-generic type carrier (`adapt/arena.rs:30`).
- E8 substrate: `Replaceable` (`api/src/store.rs:276`), `PlanAffecting` sealed
  marker (`api/src/run_cfg.rs:43`), `replace_resource` -> `mark_dirty`
  (`scheduler/mod.rs:1141`), `plan_dirty` bitarray field + `plan_cache`
  (`scheduler/mod.rs:~645`).

Stubbed / unbuilt:
- `AdaptWu::execute` body is a stub (`adapt_wu.rs:91-99`); its Ctx is
  `AdaptCtxUnimplementedStub` pending real engine-Ctx-into-meta-WU dispatch.
- `ema_update` body: all three SIMD paths delegate to a no-op (`adapt_ema.rs:119`).
- `select_adapt_config`: named only in BACKLOG (:180) + an old locked CL; NO
  implementation anywhere. The core between-frame decision loop (spec :2044-2048).
- Per-unit EMA storage: `SchedulerMetrics` carries only the scalar
  `ema_pass_duration_ns`; the spec's per-unit EMA array (:2038) has no home.
- E8 plan-recompute body: `plan_dirty` is set but ignored in `run`
  (`scheduler/mod.rs:1309` `let _ = (&self.plan_dirty, &self.plan_cache)`).
- Morsel-size recompute hook, per-phase config re-selection hook, morsel-temp ->
  core-assignment hook, predictive-parking write to `PoolFrame.predicted_wait_ns`:
  all named in spec, no code path.
- `AdaptMode` is still the legacy `crate::strategy::PhaseStrategy` alias
  (`adapt/mod.rs:50`), pending per-axis migration.

## Dependency order (expert + synthesis)

1. `AdaptArena` field layout + runtime population (the per-frame scratch home).
2. **Keystone: real engine Ctx wired into meta-WU dispatch** (replace
   `AdaptCtxUnimplementedStub`). Every axis execute body depends on this. The
   just-merged **P0.2 `CtxFor`** is the enabler: `AdaptWu`'s Ctx is
   `CtxFor<'frame, AdaptWu::Read, AdaptWu::Write, OnMeta<...>>` computed from its
   nine-axis access set, instead of the stub. This is the concrete first
   implementation slice and the reason P0.2 was sequenced before P1.
3. `AdaptWu::execute` body: walk the nine metrics Resources, threshold-compare,
   set anomaly bools, fire the anomaly virtual. Needs 1 + 2.
4. `ema_update` bench-validated body (the NEON/scalar kernel). Needed before the
   per-phase / per-fiber EMA axes carry meaningful data.
5. `select_adapt_config`: the between-frame decision fn comparing EMAs to
   thresholds, emitting the three trigger tiers. Needs EMA data from 3 + 4.
6. E8 plan-recompute body: consume `plan_dirty` in `run`, gate an
   `OnMeta<PlanStage>` rebuild. Needs `select_adapt_config` for the morsel-size
   and config-reselect cases; full rebuild for the record-count case.
7. PMC / perf_event sampling for cache_residency (axis 5): platform-tier work.

## What the pass_duration wiring establishes

The EMA formula is shipped + tested + bench-independent. The frame-timing tap
point (between `frame_start` at run entry and the `fold_ema` at run exit) is the
right hook for all timing axes. The engine-owned `MetaBlock` is the right owner
for engine-written per-pass metrics, reachable by `OnMeta<ScheduleEnd>` consumers
via the `MetaAccess`-gated Ctx accessor. `MetaField` projection extends to more
metrics fields without specialization or new Ctx params. So axes 1 to 3 (the
timing/EMA axes) follow the established pattern; the novel work is the decision
loop (5) and the replan path (6).

## Gaps the spec requires with no current hook

`select_adapt_config` (the decision loop, :2044-2048); morsel-size recompute path;
per-phase config re-selection (domain 14 configs); per-unit EMA array storage
(:2038); morsel-temperature -> core-assignment path (:2028-2033); predictive-parking
write to `PoolFrame.predicted_wait_ns`; the E8 plan-recompute body proper.

## Next routine steps (for the continuation)

Phase 5: roadmap draft ordering the seven dependency steps into mockspace rounds,
marking each proven/unproven. Phase 6: canonical mirror (second expert) against
domain 22 :2006-2076. Phase 8: granularity pass (third expert) splitting the
keystone step 2 (engine-Ctx-into-meta-WU via CtxFor) and step 5
(`select_adapt_config`), the two likely to hide hard sub-problems. Phase 10:
sketches, first for step 2 (does `AdaptWu`'s `CtxFor`-computed Ctx dispatch
through the meta-WU `OnMeta` path the E4 slices built?) since it is the keystone
the rest depend on. E8 (#345) shares domain 22 and is charted in the same arc
(step 6).
