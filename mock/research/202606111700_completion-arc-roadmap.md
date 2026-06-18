# Completion-Arc Roadmap

**Date:** 2026-06-11
**Status:** LOCKED (canonical-mirror + granularity expert passes folded in)
**Role:** the chart-the-path roadmap (routine step 5, finalised through
steps 6-9) for the engine completion arc ruled in addendum A1-3
**Reads from:** `202606111600_completion-arc-synthesis.md` (the research
document), `202606111400_canonical-design-addendum-a1.md` (the rulings), the
consolidation spec, and the amendment chain registered in A1
**Oracle:** the consolidation spec as amended; the design is the truth, the
current source is the increment
**Expert passes:** the canonical-mirror pass corrected P2.1 (RCM order is
canon/op-directive-preferred, not a free pick), P1.4 (must stay
parameter-side), P4.1 (cite r2 §5 not a non-existent spec layer), the phase
ordering (P4.2 depends on P1.4), and surfaced three unavoidable-drift items
now recorded below. The granularity pass split P0.1, P1.4, P2.3, and P4.1
into their constituent rounds and surfaced three build-time feasibility
sketches (P0.1b, P1.4a, P2.3b) the draft had folded into build steps.

## Orientation

GATE-1 (single-core devirt execution) and GATE-2 (compile-time grouping plus
core-pinned trunk parallelism) ship and hold green at fair-benched parity
with optimal multi-threaded std. The E4 self-hosting meta pipeline ships end
to end (virtual firing, lifecycle bands, the engine-owned `MetaBlock` bridge,
parallel parity). E8 slice 1 (clock-sourced pass-duration EMA) ships. Per
A1-3 GATE-2 closes with cleanup plus a perf-gate re-run, and everything
remaining is this one completion arc, charted here to the full canonical
design with no further gate ceremony.

Every unproven premise this arc rests on already has a recorded sketch
outcome (the five pre-chart sketches plus the GATE-2 mechanism family). The
roadmap therefore carries no "unproven step" rows: each step cites the
sketch or bench that proves its premise, or is a mechanical application of a
shipped mechanism. New benches the arc must run (the RCM-order decision, the
per-arm gate recalibration) are themselves steps with stated oracles, not
unproven premises.

Notation per step: PROVEN-BY names the sketch/bench/shipped mechanism that
de-risks it; SLICES names the mockspace rounds it decomposes into; CANON
cites the spec or amendment section it realises.

## Phase ordering rationale

The arc orders by dependency, not by spec-domain number. Three forces set
the order: (1) foundation-first, so a slice never builds on an unlifted cap
or an un-adopted ergonomic; (2) measurement-truthful, so the perf gate is
recalibrated before perf-substance slices land against it; (3)
debt-absorbing, so each dead-weight item dies in the first slice that
touches its file (A1-4) rather than in a separate pass.

The phases:

- **P0 Foundations and truth** — cap-lifting, CtxFor adoption, perf-gate
  recalibration, the dead-weight sweep absorbed into these touches.
- **P1 Adapt completion (E8)** — the parameter-side adaptation loop the spec
  calls domain 22, on the pass-duration spine already shipped.
- **P2 Perf substance** — micro-morsels, branch dispatch, shared-read
  strategy, bitpacking stride, intrinsics; the RCM-order bench lands here.
- **P3 Ops surface** — PipelineResult status, work-stealing Executor,
  schedule introspection, plan caching made live.
- **P4 Ecosystem bridges** — facade/plugin-host engine integration, kits and
  providers polish, persistence spine wiring.

P0 precedes all others. P1 and P2 are largely independent and may interleave
once P0 lands; P2's RCM bench gates only P2's RCM slice. P3 rests on P0 and a
stable P2 perf gate but not on P1. P4 rests on P0 and P2 AND carries one
edge into P1: P4.2 (kits/providers) depends on the P1.4a/P1.4b
`replace_resource` completion (the `Replaceable` gating composes with it) and
on the P0.2 CtxFor ergonomic. So P4 is not fully P1-independent; the P4.2 ↔
P1.4 edge is real and sequenced. The standalone canonical spec (the separate
op directive) is written against canon-plus-A1 in parallel and does not
block P0.

## P0 — Foundations and truth

### P0.1 Cap-lifting to the Capacity associated-type pattern (three rounds)
PROVEN-BY: `sketches/202606111450_cap-lifting-shapes/FINDINGS.md` (shape s1
dominates for the three grouping caps; GCE-free at the consumer). The
granularity pass split this into three rounds with a real ordering
dependency and two build-time feasibility questions the s1 sketch did not
cover (the `MAX_CORES * GATE2_MAX_ACCUMS` product-array shape and the
`AdjRow`-covers-`Units` width coupling, since arvo `Bits` resolves its
container per concrete N so a symbolic-width row word is not obviously
`Sized`).
CANON: substrate-caps-are-defaults rule + A1-2.

- **P0.1a — upstream arvo const-slice.** Add the const slice accessor for a
  `Capacity` GAT array to arvo-tensor (the one upstream addition s1 needs;
  proven by the sketch-local const trait). Feature branch + PR against arvo
  `dev`, merge, then a hilavitkutin transitive bump. Separate repo, must land
  before P0.1c.
- **P0.1b — in-engine trait-solver overflow re-test (SKETCH-GATE).** A
  feasibility sketch, not a build: re-test the documented `GATE2_MAX_UNITS`
  trait-solver overflow inside the real engine's compounded obligations
  (`BundleMasks` 4-param + `MaskProject` witnesses + `GateWith`), which the
  reduction sketch could not reproduce even at depth 64. The outcome BRANCHES
  P0.1c's shape: reproduces → grouping caps stay s1-bounded at a documented
  ceiling with the tracked lift condition; does not reproduce → they lift
  fully. This is the first of the three build-time sketches.
- **P0.1c — engine `Caps` threading + cap-site migration.** Introduce a
  `Caps` trait (`type PlanDirty: Capacity`, `type Units: Capacity`,
  `type Accums: Capacity`, `MAX_CORES` if it lifts cleanly), thread it the
  way `PlanDims` already threads, migrate `GATE2_MAX_UNITS` /
  `GATE2_MAX_ACCUMS` / `plan_dirty[256]` / `worker_ctxs[MAX_CORES]` /
  `gate2_accum_live[MAX_CORES * GATE2_MAX_ACCUMS]` off the hardcoded sizes,
  scoped by P0.1b's result. Carry a confirming sketch arm inside this round
  for the product-array (`MAX_CORES * GATE2_MAX_ACCUMS` atomic publish array)
  and the `AdjRow`/`Bits` width coupling before the migration is scoped, both
  outside the s1 sketch's coverage. Absorb the
  `lint:allow(no-bare-numeric) tracked:#121` sites the lift removes.

### P0.2 CtxFor computed-Ctx adoption
PROVEN-BY: `sketches/202606111430_ctxfor-computed-ctx/FINDINGS.md` (WORKS,
zero engine/api change to add; identity-asserted, run-proven).
CANON: harness-the-type-system rule + A1-6.
SLICES:
- Engine round: add `pub type CtxFor<'frame, R, W, S = Always>` plus the four
  fold traits (ResourceBundleOf / ColBundleOf / AccumBundleOf /
  VirtBundleOf, disjoint per-cons-head impls, the shipped Project-family
  pattern) in `dispatch::engine_ctx`. Migrate the hand-spelled nine-parameter
  aliases in the test suite (`gate2_meta_metrics`, `gate2_parallel_meta`,
  `gate2_adapt_ema`, others) to `CtxFor`. No consumer-visible behaviour
  change; the identity assertions are the regression guard.

### P0.3 Perf-gate recalibration
PROVEN-BY: fairness finding `202606081100` (parity established, the flat
1.10x bar's variance characterised).
CANON: A1-8 + gate-red-is-not-an-op-decision memory.
SLICES:
- Bench round: replace the flat 1.10x bar with per-arm calibrated bars
  (tight where the engine should win at fan-out extremes, parity tolerance
  mid-range), upgrade the std baseline to a persistent-pool variant matching
  the engine's spawn-once discipline, and add median-of-N run aggregation to
  kill the N=1M wide_parallel variance flap. Gate stays blocking. This lands
  before P2 so perf-substance slices measure against the honest bar.

### P0.4 Dead-weight sweep (absorbed, not standalone)
PROVEN-BY: truth-of-impl walk (the dead inventory is enumerated).
CANON: no-legacy-shims-pre-1.0 + A1-4.
NOTE: not its own round. Each item dies in the first P0–P4 slice that edits
its file: `topo_order`/`topo_count` and `PlanCache` (a P0.1c or P3 scheduler
touch; `PlanCache` is made live by P3.4 rather than deleted), `RecommendedOrder`
run-path absence (documented, not code), the `AdaptMode` alias (the P1.x
strategy touch). EXCEPTION (canonical-mirror note):
`synthesise_core_programs`/`CoreProgram` is the one dead item the arc cannot
kill in a touching slice, because its deletion waits on the P3.3 #183
introspection decision (whether `clause explain --schedule` rebuilds from the
plan store-back columns or from this vestigial shape). The "no slice ships
leaving a touched-file dead surface behind" rule holds for every item EXCEPT
this one, which is explicitly parked until P3.3 resolves #183.

## P1 — Adapt completion (E8 / domain 22)

The pass-duration EMA spine ships (slice 1). The remaining slices complete
the parameter-side adaptation loop: measure more axes, fire the three
reorganisation triggers, and feed recomputed parameters back into the static
walk. The governing constraint is unchanged (r2 section 1, mirror-verified):
adaptation changes parameters, never structure.

### P1.1 Per-fiber and per-phase EMAs
PROVEN-BY: the slice-1 fold mechanism (`fold_ema`, the clock slot) generalises
directly; the meta-block field pattern is shipped.
CANON: spec :2035-2040 (vectorised EMA update), domain 22.
SLICES: extend `SchedulerMetrics` (or a sibling meta resource) with per-fiber
and per-phase duration cells; sample at the band boundaries the meta pipeline
already walks; fold with the shipped weight. TDD with the scripted clock.
RIDES: the engine-owned `MetaBlock` deviation (unavoidable-drift item 3): these
new cells live in the scheduler-field meta block, not a consumer `Resource`,
because consumer resources are `Copy` read-only and cannot carry the mutable
per-frame cells. This is the already-accepted deviation, not the Resource
model. Carries the E4 slice-1b clear-on-dispatch semantics (#687) if this is
the first slice re-touching meta band dispatch.

### P1.2 Reorganisation trigger: morsel-size recompute
PROVEN-BY: the morsel loop is shipped (#343); morsel size is already a plan
parameter, so recompute is a parameter swap, not a structural change.
CANON: spec :2042 (morsel-timing trigger).
SLICES: an `OnMeta` decision unit reads the per-fiber EMA, compares against
the morsel-size model (spec domain 11 `L1_usable / Σ write_sizes`), and when
the EMA crosses the hysteresis band writes a new morsel size into the plan
parameter the next frame's morsel loop reads. Hysteresis sketch
`202606062700` informs the band.

### P1.3 Reorganisation trigger: per-phase config re-select
PROVEN-BY: the Approach-2 variant menu (r2 section 6) is the devirt-preserving
realisation; bounded variant select, not regrouping.
CANON: spec :2044 (phase-balance trigger).
SLICES: a decision unit selects among the pre-monomorphised per-phase config
variants by the per-phase EMA; the selection is a runtime index into a
const-built variant set, no recompilation, no structural change.
GRANULARITY NOTE: P1.2 and P1.3 share the `OnMeta` decision-unit shape and the
hysteresis-band pattern (P1.3 is the same shape over a const variant index).
They may merge into one "parameter-recompute triggers" round carrying two
CHANGE blocks if the per-phase variant menu is already const-built; keep
separable if the variant set needs its own const-build work. Lean: one round
may carry both.

### P1.4 Reorganisation trigger: record-count parameter recompute (#345 half, two rounds)
PROVEN-BY: parameter recompute (morsel ranges, approach/config re-select over
the SAME static WU set) is shipped at build; the trigger re-runs the
parameter side between frames. The granularity pass split this; the value-
install path is a hidden hard sub-problem (a data-plane aliasing question),
not bookkeeping.
CANON: spec :2046 (record-count trigger) + domain 22 + the r2 §1 governing
principle.
GOVERNING CONSTRAINT (canonical-mirror correction): this trigger stays
PARAMETER-SIDE. Record-count crossing a threshold is r2 §1 case (i): recompute
morsel ranges and re-select approach/configs over the unchanged static WU set,
at runtime, with no fiber regrouping and no dispatch reorder. The r2 §9 mirror
verdict CONFIRMED that no adaptive trigger may regroup or reorder from a
runtime signal. Only a WU-set change is case (ii), and that is a build-time
recompile, never a between-frame action. "Full plan recompute" here means the
parameter outputs (morsel ranges, approach selection), never the structural
outputs (fiber grouping, trunk formation, dispatch order).

- **P1.4a — value-install path (SKETCH-GATE).** Complete `replace_resource` /
  `replace_value` to actually write the new value through the binding (today
  they take `_new` by an underscore parameter and only `mark_dirty`; the
  bindings expose only `__ptr()` read accessors). This is a data-plane
  aliasing question: writing through a `ResourcePtr<T>` into storage the
  reader's SAFETY note treats as "no concurrent write, proven at plan time."
  The sketch proves whether that write is sound against the existing
  read-only aliasing contract, and whether it needs a `Cell`/atomic like the
  accumulator `len` cell. Second of the three build-time sketches.
- **P1.4b — recompute gate.** Set the plan-dirty seed and gate a between-frame
  PARAMETER recompute on the record-count delta (the `plan_dirty` array /
  `PlanCache` reads are dead `let _ =` today; this makes them load-bearing,
  overlapping P3.4). Unslotted half of #345.

### P1.5 Predictive parking, stolen/idle counters
PROVEN-BY: the parking tiers (`ParkTier`, `predicted_wait_ns`) ship in
`thread/parking.rs`; this wires the prediction to the EMA.
CANON: spec :2050-2058 (predictive parking at sync points).
SLICES: feed the per-phase EMA into the park-tier prediction so a worker
about to wait on a waist barrier picks spin/spin_loop/park by the predicted
gap; add the `stolen_count` / `idle_ns` parallel counters to the meta block.

## P2 — Perf substance

### P2.1 RCM-as-dispatch-order: confirm the cache win, then apply
PROVEN-BY: the guarded-walk relaxation carries an arbitrary const-side order
(`sketches/202606090300`); both orders are buildable.
CANON: spec Step 5 (:1331-1339, "RCM produces the row reordering = WU
execution order") + Step 8 (:1403, "Process WUs in RCM-reordered topo
order") + r2 §8(a) + A1-1.
CANONICAL-MIRROR CORRECTION: the spec MANDATES RCM-row as the execution
order; it does not register a neutral fork. r2 §8(a) is a standing op
directive that RCM-row recovery is a near-term priority and "the engine
should apply the cache-optimal RCM dispatch order itself," tied to the
Approach-2 mechanism ("RCM-row recovery and Approach-2 are the same
mechanism"). A1-1's "bench it first" is a legitimate recent amendment (op was
informed the spec says RCM-row IS execution order and still chose to bench),
but the bench is NOT a free pick between RCM-row and waist-phase. The bench
CONFIRMS whether RCM-row's cache win is real and worth the Approach-2
mechanism cost on wide fan-out DAGs. If the bench shows RCM-row wins (the
canon/op-directive expectation), apply it via the Approach-2 precompiled
variant. If it does NOT win, that contradicts the standing §8(a) directive
and is surfaced to op, not silently resolved to waist-phase by the agent.
SLICES: build the RCM-row dispatch order behind the const grouping as the
Approach-2 variant (the guarded walk supplies the order); bench cache
locality on wide fan-out DAGs against the recalibrated P0.3 bar; apply per
the result with the surfacing rule above; the standalone spec records the
outcome. See the unavoidable-drift section: auto-application is
mechanism-constrained to Approach-2 (a const-fn-computed order hits the GCE
wall; a build.rs/proc-macro cannot see resolved AccessSets).

### P2.2 Micro-morsel inner tiling
PROVEN-BY: the morsel loop and the L1 model ship; micro-morsels are an inner
boundary inside the existing loop.
CANON: spec :849-856 (micro-morsels at ECS scale).
SLICES: activate inner tiling when peak live exceeds L1 (the `peak_live > L1`
test from domain 11); the inner sync points ride the existing micro-morsel
boundary the spec names. Bench-gated against the recalibrated P0.3 bar.

### P2.3 Branch dispatch shape (two rounds)
PROVEN-BY: the trunk/fiber dispatch and the const-gated walk ship; a branch
is a chaser within a trunk's morsel scope (a constrained dispatch variant).
The design premise is proven; the toolchain premise (a third const grouping
predicate against the complexity ceiling) is not, so the granularity pass
split this.
CANON: spec :84, :743-744, :1135 (branch as chaser-within-trunk).

- **P2.3a — plan-side branch detection.** A new grouping-analysis arm in
  `grouping.rs` identifies the branch shape at build time. Separable from the
  dispatch walk, build-time only.
- **P2.3b — dispatch-side const-gated branch walk (SKETCH-GATE).** Extend the
  `RunTrunkDispatch` const-gated walk with a branch predicate. The walk's
  root-test and phase-of are already forced into `IsRoot::IS` / `PhaseAt::VAL`
  associated-const carrier structs precisely because inline
  `const { trunk_of(..) == POS }` blocks hit the generic-constant complexity
  limit at this parameter count. A third predicate over the same `BundleMasks`
  carrier risks re-tripping that ceiling; the sketch proves the predicate
  threads through before the build. Third of the three build-time sketches.

### P2.4 Sub-byte bitpacking stride
PROVEN-BY: `ColumnValue::BIT_WIDTH` is declared and shipped; the stride
computation is the missing wiring.
CANON: spec domain 11/12 (bitpacked columns) + the arvo exact-width identity.
SLICES: replace the `size_of::<T>()` stride in the morsel access path with a
`BIT_WIDTH`-driven stride; the engine_ctx read/write projection computes the
bit offset; arvo's transparent-repr widths make this exact.

### P2.5 Shared-read-column strategy
PROVEN-BY: trunk isolation ships; this is a per-platform read-strategy choice
between trunks.
CANON: spec :778-786 (snapshot-to-local vs aligned-morsel-sync).
SLICES: the plan picks, per shared read column between trunks, between a
snapshot-to-local copy and an aligned-morsel-sync; bench-decided per platform.

### P2.6 Intrinsics and microkernels
PROVEN-BY: arvo ships the intrinsic surface; this is engine adoption at hot
dispatch points.
CANON: spec :878-931 (domain 13).
SLICES: identify the hot dispatch inner loops (bench-driven) and route them
through arvo microkernels where the bench shows a win; never speculative.

## P3 — Ops surface

### P3.1 PipelineResult status surface
PROVEN-BY: abort-on-failure ships; the status surface is the data shape over
it.
CANON: spec :2159-2167 (per-fiber Completed/Failed/Poisoned + dependent
poisoning) + R7.
SLICES: add the `PipelineResult` per-fiber status, propagate poisoning to
dependents through the existing predecessor masks, surface it through the
meta bridge so a consumer hook can read frame status.

### P3.2 Work-stealing Executor extension point
PROVEN-BY: the `Executor` trait surface and `steal_fallback` stub ship; this
fills the body behind the extension point.
CANON: spec :1868-1874 (consumer-pluggable stealing; not the default).
SLICES: define the steal contract (a worker that drains its trunks offers to
take a sibling's remaining morsel range), implement the default no-steal path
explicitly, leave the steal body as a consumer `Executor` impl point with a
reference implementation behind a test.

### P3.3 Schedule introspection (#183) and CoreProgram cleanup
PROVEN-BY: the plan store-back columns ship (the live data introspection
would read).
CANON: spec :1979-1983 (schedule reuse) + #183.
SLICES: decide whether introspection (`clause explain --schedule`) reads the
plan store-back columns directly; if yes, build that reader and delete the
vestigial `synthesise_core_programs` / `CoreProgram` (the A1-4 deferred
item); if the decision needs op, surface it (the one P3 item that may pause).

### P3.4 Plan caching made live
PROVEN-BY: the `PlanCache` husk and `plan_dirty` array ship; this populates
them.
CANON: spec :1486-1491, :1979-1983 (recompute only on structural change).
SLICES: populate `PlanCache` so a structurally-unchanged frame reuses the
prior plan without recompute; the cache key is the registration shape (fixed
at build) plus the plan-dirty seed; this makes the dead `let _ =
(&self.plan_dirty, &self.plan_cache)` reads load-bearing or deletes them if
the dirty-skip already covers the case.

## P4 — Ecosystem bridges

### P4.1 Facade / plugin-host engine integration (three rounds)
PROVEN-BY: both facade sketches WORK with measured evidence
(`sketches/202606090100`, `202606090200`); no op-decision wall, no new
feasibility unknown. The granularity pass split this into three rounds of
different risk profile.
CANON: r2 §5 surface 2 (the facade pattern; the consolidation spec has NO
plugin domain, so r2 §5 is the sole canon, not a spec section) + the
plugin-host layer crates (`hilavitkutin-linking` / `-extensions`).

- **P4.1a — morsel-absolute slice accessor.** Additive API on the
  Context/engine_ctx read/write projection (`read_slice` / `write_slice` /
  `morsel_range`); the current reader/writer expose only per-record
  `read(i)` / `write(i, v)` at `morsel.start + i`. Small, mechanical, mirrors
  the existing per-record projection. Named in the 7-4b findings as
  needed-but-not-built. Note: P3.2's reference steal impl also needs this
  hand-off surface, so P4.1a sequences before (or is shared with) P3.2.
- **P4.1b — facade WU shape through real dispatch.** The facade WorkUnit
  declares only its bridge stores, invokes the plugin capability once per
  morsel through the extern-"C" `fn(usize, usize)` wire shape (the plugin
  owns its absolute cursor), wired through real `Scheduler::run`. This is the
  integration round where the isolated 7-4b sketch (objdump-measured but
  standalone) meets the real grouping/dependency analysis; 7-4a already
  proved the facade's bridge edges enter the RAW conflict grouping, so the
  premise holds and the wiring is the work.
- **P4.1c — loader-layer registration.** Wire it to the shipped
  `hilavitkutin-linking` / `hilavitkutin-extensions` loader (a different
  crate group with its own forbidden-import lints) so a downstream host
  (viola) registers a plugin as a facade WU, each extension loading / running
  / dropping independently per the arbitrary-time-linking rule.

### P4.2 Kits and providers polish
PROVEN-BY: `hilavitkutin-kit` / `hilavitkutin-providers` ship; this is
completion against the engine's current builder surface (and the P0.2 CtxFor
ergonomic).
CANON: repo crate table; spec Kit/Replaceable vocabulary.
SLICES: reconcile the Kit declarative bundle and the default provider impls
against the post-CtxFor builder; add the `add_kit::<K: Default>()` ergonomic
(#299); ensure `Replaceable` gating composes with the P1.4 replace_resource
completion.

### P4.3 Persistence spine wiring
PROVEN-BY: `hilavitkutin-persistence` ships as a standalone crate; this wires
the engine's evict/inject to it.
CANON: spec R2 :2406-2414 (evict/inject), :2169-2176.
SLICES: wire the engine's cold-store eviction and injection through the
persistence crate's hot/cold bridge; the consumer owns the cold store, the
engine owns the eviction policy and the dirty/generation counters. This is
the engine half of #344/#134/#135.

## Standing items folded across phases

- The api warning drift (#686) is fixed in the P0.2 touch (the CtxFor round
  edits the api surface anyway).
- E4 slice-1b clear-on-dispatch (#687) lands in P1.1 or P1.2 (the first slice
  that re-touches the meta band dispatch).
- The meta-band const fns not const-folding (the meta-carrier finding's shared
  note) is fixed wherever P1 first lifts band bounds to build-time state.

## Unavoidable drift (recorded per the canonical-mirror pass)

Addendum A1 already records two: the type-level-partition wall (const data
plus const-gated DCE replaces a type-level N-way carrier partition, since the
latter needs forbidden full specialization) and the GCE field-access wall
(caps cannot be `Cfg`-sized via const field access; A1-2). The
canonical-mirror pass surfaces three more this arc must carry:

1. **RCM-row dispatch order is mechanism-constrained to Approach-2.** The only
   devirt-preserving auto-application of the RCM-row execution order is
   precompiling the RCM-ordered carrier as an Approach-2 variant: a
   const-fn-computed RCM order hits the GCE-in-generic wall, and a
   build.rs/proc-macro cannot see resolved AccessSets. So P2.1 cannot
   auto-apply an arbitrary computed order; it applies the RCM order through
   the Approach-2 precompiled variant or falls back to registration-order
   recovery. This is the drift on the canonical "engine applies RCM order
   itself" (spec Step 8, r2 §8a) path.
2. **Producer-before-consumer registration is provisional drift.** Canon wants
   the engine to auto-order WUs (RCM-row recovery); the shipped engine
   requires the consumer to register producer-before-consumer, validated by
   `BuildError::NonTopologicalRegistration`. r2 §8(b) marks this PROVISIONAL,
   to be relaxed when RCM-row recovery or toolchain maturity lands. P2.1 and
   P1.4b touch this surface and carry the provisional-drift marker.
3. **`MetaBlock` engine-owned mutable state deviates from the Resource/Column
   data model.** Meta mutable state lives in a scheduler field, not a consumer
   `Resource`, because consumer resources are `Copy` read-only (A1 constraint
   5, already shipped and accepted). P1.1 and P1.5 extend meta state and ride
   this accepted deviation, not the Resource model.

## Build-time sketch-gated steps

The pre-chart sketches proved every premise that could change the arc's
shape. The granularity split surfaced three NEW feasibility questions, each
localized to its mid-arc step and neither walling the arc nor changing its
shape. Each runs as a sketch at the start of its build round, per normal
mockspace discipline, not pre-chart:

- **P0.1b** — in-engine `GATE2_MAX_UNITS` trait-solver overflow re-test
  (branches P0.1c's cap-lift shape; reduction could not reproduce it at depth
  64, so the in-engine compounded-obligation case is the open question).
- **P1.4a** — `ResourcePtr<T>` data-plane value-write soundness against the
  existing read-only "no concurrent write, proven at plan time" aliasing
  contract (whether it needs a `Cell`/atomic like the accumulator `len` cell).
- **P2.3b** — a third const grouping predicate against the complexity ceiling
  that already forced the `IsRoot`/`PhaseAt` associated-const carrier
  workaround.

If any of these three sketches FAILS (the mechanism genuinely cannot be built
as the step assumes), that is a course-correction op owns per the
chart-the-path step-11 rule, surfaced with the alternatives, not silently
re-routed. None of the three is expected to fail (each has an adjacent
proven mechanism), but each is a real toolchain question, not a foregone
conclusion.

## Chart status

This roadmap is the finalised chart-the-path output (routine steps 5 through
9, with the canonical-mirror and granularity passes folded in). The build is
now mechanical: work the phases in order, run each phase's sketch-gate before
its build round where one is named, and follow each step's PROVEN-BY citation
to the proven shape. The standalone canonical specification (the separate op
directive) is written next, against canon-plus-A1-plus-this-roadmap, and the
build resumes under op's re-blessing once that spec is done.
