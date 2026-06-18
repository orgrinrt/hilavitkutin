# Completion-Arc Research Synthesis

**Date:** 2026-06-11
**Status:** LOCKED (all five pre-chart sketch findings recorded, section 6)
**Role:** the chart-the-path research document (routine step 4) for the
engine completion arc ruled in addendum A1-3
**Sources:** the consolidation spec plus the amendment chain registered in
`202606111400_canonical-design-addendum-a1.md`; the 2026-06-11 truth-of-impl
walk and canon inventory (agent passes, synthesised here); the bench and
sketch corpus under `mock/research/` and `mock/research/sketches/`

## 1. What is being built

One pipeline execution engine in which monomorphisation is the dispatch.
A consumer declares WorkUnits with typed read/write access sets and
registers them, plus their stores, on a builder. At build time the engine
computes the whole execution shape: phases bounded by waists, column-disjoint
trunks inside each phase, fibers inside trunks, and the morsel windows the
records flow through. At compile time, const evaluation folds the access
sets into masks, derives the same grouping, and a const-gated walk over the
flat carrier dead-code-eliminates everything but each trunk's member program,
so the shipped binary contains per-trunk monomorphised programs with zero
indirect calls. At run time, frames dispatch those programs: single-core in
phase order, or parallel with one core per trunk, joined only at waist
barriers and bridges, workers parked between frames on a spawn-once pool.
The scheduler schedules itself through the same machinery (meta WorkUnits on
lifecycle virtuals), observes itself (engine-owned metrics behind a gated
accessor), skips clean work (dirty masks propagated over the dependency
graph), and adapts parameters between frames (EMA-driven triggers) without
ever mutating structure. Everything is no_std, no alloc, no dyn, on arvo
numerics, with consumer-supplied platform providers (memory, threads, clock)
on os and no_os tiers.

## 2. Built versus unbuilt, in dependency order

Layer 0, foundations (BUILT): arvo numerics, Capacity/PlanDims sizing,
access-set cons machinery, witness projection (`Here`/`There`), the
ColumnStorage contract with plan store-back.

Layer 1, single-core execution (BUILT, GATE-1): devirt fiber walk, fusion
(`run_fused`), morsel windowing, accumulator append surface with per-frame
reset, incremental dirty-skip, ASM-verified zero indirect calls.

Layer 2, compile-time grouping (BUILT, GATE-2 mechanism): `BundleMasks`
const fold, const grouping fns (phases by waist, trunks by union-find, rank
renumber, band bounds), const-gated trunk dispatch with `IsRoot`/`PhaseAt`
associated consts.

Layer 3, parallel execution (BUILT, GATE-2): spawn-once pool, frame
publish/await protocol, core-pinned trunk ownership (`rank % ncores`),
waist barriers, unit-outer accumulator regions plus merge, plan-band skip,
fair-benched at parity with optimal multi-threaded std.

Layer 4, self-hosting meta (BUILT, E4): virtual epoch firing with `On<V>`
gating, lifecycle bands (`OnMeta<V>`, rank-outer renumber), the engine-owned
`MetaBlock` bridge with the `MetaAccess`-gated accessor, parallel parity via
main-thread designated meta bands. Known wart: meta units run per-morsel on
record-bearing paths (A1-7 decides the cure shape).

Layer 5, adaptation (STARTED, E8): pass-duration EMA through a builder clock
slot is live (slice 1, shipped 2026-06-11). Unbuilt: per-fiber/per-phase
EMAs, the three reorganisation triggers (morsel recompute, per-phase config
re-select, record-count plan recompute including `replace_resource` actually
writing values), predictive parking, stolen/idle counters. Eight of nine
adapt axis types are inert carriers.

Layer 6, perf substance (UNBUILT): micro-morsel inner tiling, branch
dispatch shape, shared-read-column strategy, sub-byte bitpacking stride
(`ColumnValue::BIT_WIDTH` is declared, stride is `size_of`), intrinsics and
microkernels, RCM-as-dispatch-order (bench-decided, A1-1).

Layer 7, ops surface (UNBUILT): `PipelineResult` status surface with
dependent poisoning, work-stealing `Executor` extension point, schedule
introspection (#183), plan caching (`PlanCache` is an empty husk).

Layer 8, ecosystem bridges (UNBUILT): facade/plugin-host engine integration
(feasibility pending, section 6), kits and providers polish, persistence
spine wiring (R2 evict/inject).

Cross-cutting debts the arc absorbs where slices touch them (A1-4): the
dead-weight inventory (`topo_order`/`topo_count`, `PlanCache`,
`synthesise_core_programs`/`CoreProgram`, `RecommendedOrder` run-path
absence, `AdaptMode` alias), the cap-lifting redesign (A1-2), the api
warning drift (#686), E4 slice-1b clear-on-dispatch (#687), and the perf
gate recalibration (A1-8).

## 3. What the bench and sketch corpus has established

Proven by sketch: the type-level N-way carrier partition walls on forbidden
specialization, and the const-eval grouping plus const-gated DCE walk is the
working mechanism (the r4 sketch family, `202606071230`/`202606071330`/
`202606070800`); the guarded-walk relaxation can carry an arbitrary
const-side order through dispatch without devirt loss (`202606090300`, the
vehicle for both auto-RCM registration recovery and the A1-1 bench); the
witness-tuple gating slot carries schedule gates without a second generic
(E4 slice-1 rounds); dirty-gated walking holds under the carrier
(`202606111000_e1-e7-dirty-gated-walk`); fusion auto-synthesis and
min-spec-free transparent dispatch hold (the `202606091*` D4 family); the
row-word cap widening holds (`202606101000`).

Proven by bench: the engine is at parity with optimal multi-threaded std on
the parallel arms (fairness finding `202606081100`, superseding the earlier
3.5x claim that measured against single-threaded std); element-wise
single-core is green against the GATE-1 bar; the N=1M wide_parallel arm
flaps on measurement variance, which A1-8 fixes with median-of-N and a
persistent-pool std baseline.

Established negative results: full specialization is structurally required
by every type-level partition shape tried (forbidden, closed); blanket
`ColumnValue` via min_specialization was removed in favour of the spec-free
default (#631).

## 4. The muddle this chart resolves

Not a directional muddle: the mechanism bets are settled and proven. The
muddle is completeness bookkeeping. A dozen canon features have no roadmap
slot; three subsystems are partially inert (adapt axes, plan cache, work
stealing); several surfaces are husks awaiting their data sources; and the
toolchain constraints surfaced during GATE-2 (GCE field-access wall, trait
solver overflow on Cfg-driven sizes, const-block complexity ceiling) were
worked around locally without a unified posture. Without one chart, the
remaining work would be picked ad hoc and the inert surfaces would keep
reading as "done" in overviews. The arc charts everything in section 2
layers 5 through 8, plus the absorbed debts, to the full canonical design,
with no further gate ceremony (A1-3).

## 5. Rulings constraining the chart (from addendum A1)

The chart takes these as fixed inputs: RCM dispatch order is bench-decided
inside the arc, not pre-decided (A1-1); the cap-lifting redesign is an early
arc slice, not deferred (A1-2); cleanup rides touching slices (A1-4); the
consumer surface adopts the computed Ctx if the sketch proves it (A1-6); the
meta-carrier shape follows its comparative sketch (A1-7); the perf gate
recalibrates to per-arm bars with a persistent-pool std baseline and
median-of-N measurement, and stays blocking (A1-8).

## 6. Pending feasibility inputs (to fold in before the roadmap draft)

Four sketch lines are in flight; their findings complete this document:

1. Facade 7-4a (opaque AccessSet past `ContainsAll`): WORKS
   (`sketches/202606090100_facade-accessset-containsall/FINDINGS.md`,
   re-validated against the post-E4 engine through real `Scheduler::run`
   plus const-grouping introspection). The sound facade declares only its
   bridge stores (the plugin's unknown access is non-host data), so no
   over-approximating AccessSet needs to exist: `ContainsAll` is a
   registration check and passes by construction; the facade's bridge edges
   enter the dependency analysis (RAW conflict groups facade+consumer into
   one trunk; anti-topo registration rejected).
2. Facade 7-4b (per-morsel ABI hop): WORKS
   (`sketches/202606090200_facade-per-morsel-abi-hop/FINDINGS.md`, objdump
   measured). Minimal wire shape is an extern-"C" `fn(usize, usize)`
   morsel-relative range with the plugin owning its absolute cursor; the
   facade carries exactly one `blr` inside the morsel loop (8 calls for
   ceil(256/32) morsels, not per-record), host WUs stay zero-indirect.
   Additive API note for the build phase: the host-column-bridge variant
   needs a morsel-absolute slice accessor the per-WU Context does not yet
   expose (charted with the facade-integration slice).
3. `CtxFor` computed Ctx (A1-6): WORKS
   (`sketches/202606111430_ctxfor-computed-ctx/FINDINGS.md`). Four
   engine-side fold traits (ResourceBundleOf / ColBundleOf / AccumBundleOf /
   VirtBundleOf, disjoint impls per cons head kind, the shipped
   Project-family pattern) plus the shipped `MetaPtrFor` compute all six
   derived EngineCtx parameters as
   `pub type CtxFor<'frame, R, W, S = Always>` in
   `hilavitkutin::dispatch::engine_ctx` (api-side impossible: every named
   output is an engine type). Identity-asserted against the hand-spelled
   aliases across all store kinds, set interleavings, and schedule kinds;
   proven under real `Scheduler::run` on resource+column, virtual-firing,
   and OnMeta bridge arms. Consumer spelling becomes
   `type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>` (schedule
   named explicitly for non-Always). A1-6 resolves to ADOPT; the arc charts
   the adoption slice (add CtxFor + migrate test aliases).
4. Meta-carrier shapes A versus B (A1-7): RESOLVED to Shape A
   (`sketches/202606111440_meta-carrier-shapes/FINDINGS.md`, both shapes
   built and measured green). Both cure the per-morsel meta wart and the
   clean-frame meta skip identically (dispatch order `[plan, pass,
   (c1,c2)x4, end]`, zero `blr` in every driver, 1 band-run per frame after
   the fix versus the shipped 4-then-0). Shape A (shared carrier, bands
   hoisted out of the record-bearing morsel loop) needs ZERO new machinery:
   a ~50-line `run` loop restructure reusing the four shipped band const
   fns, no builder or typestate change, one grouping DAG over all units in
   one rank-renumbered phase space. Shape B (dedicated meta carrier) needs a
   ~45-line type-level router plus a projected doubling of the
   builder/scheduler surface (second carrier parameter and field, two to
   four extra witness parameters, doubled bound block), and its meta
   grouping CANNOT represent cross-carrier data edges (meta ordering becomes
   positional only). Shape B's only advantage is a smaller frame mono (1712
   versus 2222 insns). DECISION: Shape A, because the zero-machinery cost and
   the single phase space that keeps meta-to-consumer data edges
   representable both dominate; the mono-size edge does not outweigh losing
   cross-carrier data edges. Shared note for the build phase: the band const
   fns do not const-fold in the runtime-generic drivers today (they lower to
   outlined per-frame grouping recomputation), so the arc lifts band bounds
   to build-time state regardless. Neither shape resolves #687
   clear-on-dispatch or the parallel-unit-outer leading-band append window
   (single-threaded sketch); both remain charted follow-ons.
5. Cap-lifting shapes (A1-2): RESOLVED, shape s1 dominates
   (`sketches/202606111450_cap-lifting-shapes/FINDINGS.md`, eight shapes
   probed). The GCE field-access wall is exact but narrow (tuple-field
   access in anon-const grammar); every shape except the verbatim
   documented lift avoids it. s1 routes every cap through the Capacity
   associated-type pattern PlanDims already uses (`Dim<N>` binds the GAT
   array): no generic constant is constructed at all, the consumer stays
   GCE-free, threading cost is two associated types plus ~14 impl lines,
   and it covers all three caps including the const-grouping scratch. Price:
   one upstream arvo-tensor addition (a const slice accessor for the GAT
   array, proven by a sketch-local const trait). The bare-usize and
   assoc-const-indirection shapes also work but spread viral
   `where [(); ..]:` bounds and keep GCE load-bearing at the consumer
   surface, against the post-#652 migration direction. CORRECTION worth a
   re-test: the documented GATE2_MAX_UNITS trait-solver overflow did NOT
   reproduce in reduction even at depth 64; the in-engine comment is either
   specific to the full walk's compounded obligations or stale on this
   nightly. The arc charts: arvo-tensor const-slice upstream slice, the s1
   caps migration, and an in-engine overflow re-test before the grouping
   migration is scoped.

The roadmap draft (routine step 5) does not lock until every line above
carries a recorded WORKS / FAILS / INCONCLUSIVE outcome and this section is
rewritten to state them.
