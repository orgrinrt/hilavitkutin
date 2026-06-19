# Engine-completion arc: critical canonical audit + updated roadmap

**Date:** 2026-06-19
**Status:** critical chart-the-path (op-requested, autonomous). Audit findings + the corrected, re-sequenced roadmap. One op-gated decision surfaced separately.
**Oracle:** consolidation spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` (22 domains, R1-R9).

## Verdict

The arc is substantially on the canonical path: shipped single-core dispatch, the
const-grouping carrier mechanism, GATE-2 trunk-per-core parallelism, and the E4
self-hosting meta pipeline are real and faithful. But the audit found two real
problems:

1. **The parallel-execution model silently narrowed.** The locked completion-arc
   roadmap (`202606111700`) dropped two spec-MANDATED parallelism mechanisms,
   head+tail convergence and pipeline-phase overlap, which survive only in the
   superseded r3 GATE-2 chart as UNBUILT. The "fair-bench parity with
   multi-threaded std" claim therefore validates a NARROWER model than the spec
   describes, not the canonical design. This is the
   `canonical-design-outranks-intermediate-rounds` failure mode: r3 reasoned from
   op's trunk-per-core rechart, the completion arc inherited that frame, and the
   spec's mandate fell out of the chart. (Op-gated, see below.)

2. **Consumer-readiness is not close.** No end-to-end consumer path exists; the
   missing surfaces are concrete and unworkable-around.

The plan-analysis chain (domain 15) and the per-fiber morsel model are the
clustered technical weakness between "runs test WUs at std-parity" and "consumers
can fully use it".

## Findings ledger (condensed; full detail in the audit transcript)

Drift: D-1 morsel-outer-shared loop vs spec per-fiber/multiple-morsels (HIGH,
charted in blueprint 202606201400, S1 rename shipped). D-2 `morsel_windows` still
a record-count partition not an L1 window (HIGH, same blueprint). D-3 dispatch is
topological not RCM-row (spec Step 5/8 :1331/:1403; shipped + memo 070100 are the
drift the canonical-outranks rule names by name; P2.1 re-grounds it). D-4
consumer must register producer-before-consumer (`NonTopologicalRegistration`,
vs domain 8 scheduler-derives-ordering; tied to D-3). D-5 ColumnValue R4
specialisation abandoned (justified, #631). D-6 MetaBlock engine-owned (accepted).
no-alloc/no-dyn/no-TypeId/no-spawn: CLEAN in shipping code.

Skips (in NEITHER code nor the locked roadmap): S-1 head+tail convergence
(domain 20:1838 / 11:771, ~2x commutative; plan record exists, never dispatched).
S-2 pipeline-phase overlap (domain 11:776 / 20:1847, progress-counter arena
exists, strict barriers shipped instead). S-3 matrix-chain DP fiber grouping
(domain 15:1410, greedy only, #339). S-4 spectral trunk formation computed but
not consumed (#644). S-5 Dulmage-Mendelsohn + dead-column elimination (domain
15:1349, components-only). S-6 per-morsel generation counters (domain 12:861,
coarse per-store dirty only). S-7 heterogeneous P/E core awareness (domain
20:1810, CoreClass stub, no detection). S-8 before/current version stamps (domain
23:2139, substituted by store_dirty).

Consumer-readiness gap: no morsel-absolute slice accessor (Context exposes only
per-record `resolve_read(i)`/`resolve_write(i,v)`); no PipelineResult status
surface; persistence ColdStore is a skeleton not engine-wired; plugin-host facade
not bridged to the engine. viola (#254) cannot run a real pipeline beyond test
WUs.

## Updated roadmap (corrected + re-sequenced)

Re-grounded on the spec. Phases ordered by dependency and by the consumer-usability
goal. Items marked [CORRECTION] fix drift; [SKIP] build an absent canonical
mechanism; [GATED] await the op decision below.

**Phase A — per-fiber morsel model [CORRECTION, prerequisite for much below].**
Blueprint `202606201400` slices 2-5: A1 FiberDispatch per-fiber size field; A2
the loop inversion (fiber-outer/morsel-inner, D-1); A3 the L1 window formula
(D-2, flips the catalogued `r6_morsel_window_formula` red green); A4 GATE-2
parallel per-fiber sizing. This is a load-bearing prerequisite for micro-morsels
(domain 12), the adapt morsel re-chunk actuation, and per-fiber locality; it
predates the locked roadmap and must be absorbed as its own phase.

**Phase B — plan-analysis chain completion (domain 15) [SKIP].** B1 matrix-chain
DP fiber grouping (S-3, #339). B2 consume the spectral partition in trunk
formation (S-4, #644). B3 Dulmage-Mendelsohn block-decomposition + dead-column
elimination (S-5; dead-column feeds morsel sizing + register pressure). These
sharpen the plan toward the spec's grouping quality.

**Phase C — RCM-row dispatch order [CORRECTION].** C1 make dispatch consume the
RCM row reordering as the WU execution order (D-3, spec Step 8), retiring the
070100 "RCM is arena-only" drift framing; this also dissolves D-4
(`NonTopologicalRegistration`) because the scheduler then derives order rather
than trusting registration. (Existing roadmap P2.1; elevated, de-drifted.)

**Phase D — adapt subsystem completion (domain 21/22) [continue].** The EMAs +
select_adapt_config decision shipped. The tier-1 morsel re-chunk ACTUATION rides
Phase A (reads/rewrites `morsel_windows` on `adapt_reconfigure`), then bench-verify
flips the catalogued adapt-perf contracts. Then fiber_ema, active_units,
parallel-path phase_ema, the AdaptArena option-B storage, per-morsel generation
counters (S-6), and strategy reselect (domain 14, after strategy plan-shaping is
wired).

**Phase E — consumer-readiness surfaces [the actual gate to viola usability].**
E1 morsel-absolute slice accessor (`read_slice`/`write_slice`/`morsel_range` on
Context) — blocks everything consumer-facing. E2 PipelineResult status surface
(per-fiber Completed/Failed/Poisoned). E3 persistence engine bridge (evict/inject
wired to ColdStore). E4 plugin-host facade bridge (linking/extensions to the
engine). E5 viola-as-hilavitkutin-app integration (#254).

**Phase F — heterogeneous + version-stamp axes [lower priority].** S-7 P/E core
detection + asymmetric morsels; S-8 before/current version stamps (domain 23) if
the coarse store_dirty proves insufficient.

**Phase G — bench + optimise + microkernels.** Once internals are real
(Phases A-E), the full bench pass + microkernels where the gate shows red arms
(branching/accumulator single-core), per op's "bench after internals ready".

**Phase H — ecosystem.** saalis / loimu / polka-dots integration, each exercising
a different subset, confirming "no missing features".

## The one op-gated decision (surfaced separately)

S-1 (head+tail convergence) and S-2 (pipeline-phase overlap) are spec-MANDATED
(domains 11/20) and PROVEN by sketch, but absent from both the shipped engine and
the locked roadmap. Per `canonical-design-outranks-intermediate-rounds`, the spec
wins over the intermediate r3 chart UNLESS the 2026-06-07 trunk-per-core rechart
(op-directed) consciously superseded them. That is op's knowledge, not derivable
from the artifacts, so it is the human-decision point this audit surfaces: are
head+tail + phase-overlap (a) back in scope as a Phase (canonical), or (b)
consciously dropped/deferred by the rechart? If (a), they slot as a Phase between
C and E (intra-trunk + cross-phase parallelism, the spec's full model). The
benches must then be re-read against the full model, not the narrowed subset.

## RESOLUTION (op decision, 2026-06-19)

**Head+tail + phase-overlap stay FULLY in scope.** Checked the recharts for a
named drop-reason (op's condition): there is none. Both are explicitly KEPT in
the r3 GATE-2 roadmap as PROVEN-but-UNBUILT, not dropped:
- head+tail convergence = G2-Nd (= E4b), sketch `202606062200` PROVEN, UNBUILT
  (`202606070200_engine-roadmap-r3-gate2.md:38-39`; rechart `202606070100:20,54`).
  Spec `:770`, `:1838-1844`. The only place a single fiber's records split across
  cores (2-way, head + tail, never N-way).
- phase-overlap via `AtomicUsize` progress counters = the E4 "overlaps phases"
  item (r3 `:45`, rechart `:21,56`). Spec `:772-774`, `:1847-1854`. NOTE: what
  shipped as "E4" was the meta-WU LIFECYCLE pipeline (PlanStage/ScheduleEnd
  virtuals); the progress-counter phase OVERLAP (phase N+1 starts when N produces
  one morsel) is the still-UNBUILT part. So the audit's "absent" stands.

They fell out of the later completion-arc roadmap (`202606111700`) P0-P4 in the
r3->completion-arc transition, unflagged. Per the canonical-design-outranks rule
and op: no named/justified reason exists, so they are canonical and return to the
active roadmap in full. The consolidation spec already mandates them (domains
11/20), so no canonical-doc change is needed; only this roadmap revision restores
them.

**Sequencing: internals first.** Op: consumer paths are mostly sugar/wiring atop
working internals and can only be built once the internals exist. So the corrected
order is internals (Phases A-D + the restored parallel mechanisms) BEFORE the
consumer surfaces (Phase E), which move to the end just before bench/ecosystem.

### Re-sequenced phases (authoritative)

1. **Phase A** per-fiber morsel model (slices A1-A4; A-S1 rename shipped PR #151).
   Prerequisite for micro-morsels + adapt morsel actuation + per-fiber locality.
2. **Phase G2C** complete the REAL GATE-2 (the rechart's term): G2-Nd head+tail
   convergence (single-trunk-phase 2-way split, sketch 202606062200) + the
   phase-overlap progress-counter mechanism (phase N+1 starts on N's first
   morsel). These finish the canonical parallel-execution model the trunk rechart
   established; the fair-bench parity must be re-read against this full model.
3. **Phase B** plan-analysis chain (DP fiber grouping, consume spectral,
   Dulmage-Mendelsohn + dead-column).
4. **Phase C** RCM-row dispatch order (correct D-3, retire the 070100 arena-only
   framing; dissolves D-4 NonTopologicalRegistration).
5. **Phase D** adapt completion (morsel re-chunk actuation rides Phase A;
   fiber_ema, active_units, parallel phase_ema, AdaptArena, per-morsel gen
   counters S-6, strategy reselect).
6. **Phase E** consumer-readiness surfaces (morsel-absolute accessor,
   PipelineResult, persistence bridge, plugin facade, viola #254) — LAST among
   feature work, built on the now-complete internals.
7. **Phase F/G/H** P/E + version stamps; full bench + microkernels; ecosystem.

Decision recorded; no further op input needed to proceed internals-first.
