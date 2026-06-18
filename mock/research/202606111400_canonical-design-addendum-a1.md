# Canonical Design Addendum A1: Post-GATE-2 Reconciliation and Rulings

**Date:** 2026-06-11
**Status:** locked (op rulings recorded live, 2026-06-11)
**Amends:** the consolidation spec (`mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`) as modified by the amendment chain (r2 `202606081500`, rechart `202606070100`, r3 `202606070200`, r4 `202606070700`, fairness `202606081100`, r5 `202606081600`)
**Scope:** the engine state at branch `feat/hilavitkutin-parallel-engine-gate2` HEAD `0e4369d`: parallel exec-core, E4 meta pipeline, and E8 slice 1 complete

## Why this addendum exists

The build reached the point where the spec's load-bearing mechanisms are no
longer hypotheses: the const-eval grouping with const-gated DCE dispatch, the
core-pinned trunk parallelism, the self-hosting meta pipeline, and the first
live adapt datum all ship and hold green under fair benchmarks. Contact with
the toolchain also surfaced constraints the founding spec could not have
known. This addendum reconciles canon against the shipped reality, records
the rulings op made on the surfaced forks, and registers the amendment chain
in one place so any later document (including the planned standalone spec)
has a single authoritative pointer.

Per `canonical-design-outranks-intermediate-rounds.md`, this document is an
amendment to canon, not an intermediate round: where it speaks, it modifies
the effective design. Where it is silent, the consolidation spec plus the
registered chain stand unchanged.

## State assessment (truth-of-impl, 2026-06-11)

Complete and proven: single-core devirt dispatch (zero indirect calls, ASM
gate green); const-eval grouping (`BundleMasks` fold, `compute_phases_waist`,
`compute_trunks`, rank renumber) with the const-gated carrier walk
(`RunTrunkDispatch` with `IsRoot` / `PhaseAt` associated consts); core-pinned
per-trunk parallel dispatch with waist barriers and the frame publish/await
protocol; incremental dirty-skip (store seed, predecessor propagation);
accumulator unit-outer path with per-core regions and merge; plan store-back
(`PlanHandle`, six columns); E4 meta pipeline end to end (virtual epoch
firing, lifecycle bands, the `MetaBlock` bridge, parallel parity with
main-thread designated meta bands); pass-duration EMA through a builder
clock slot (E8 slice 1).

Partial or inert: eight of nine adapt axes carry types but no data;
`replace_resource` / `replace_value` mark dirty without writing the value;
`PlanCache` is never populated; `steal_fallback` is `todo!()`; RCM rides only
the arena-layout column; sub-byte bitpacking (`ColumnValue::BIT_WIDTH`) is
declared but stride is `size_of::<T>()`; `topo_order` / `topo_count`,
`synthesise_core_programs` / `CoreProgram`, and `RecommendedOrder` are dead
or error-path-only weight.

## Rulings (op, 2026-06-11)

### A1-1: RCM dispatch order is bench-decided

The spec's Step 5/8 dual-ordering position (RCM row order = WU execution
order) versus the shipped waist-phase order is resolved by bench, not by
fiat. Both orders are buildable behind the const grouping mechanism (the
guarded-walk relaxation proven in r2 section 7-6 carries an arbitrary
const-side order). A cache-locality bench on wide fan-out DAGs decides which
wording canon keeps. Until the bench lands, neither order is drift; the fork
is registered open with a bench oracle.

### A1-2: Consumer-tunable caps get a redesign arc now

The fixed caps (`GATE2_MAX_UNITS = 256`, `GATE2_MAX_ACCUMS = 16`,
`plan_dirty: [AtomicBool; 256]`, `MAX_CORES`) exist because current rustc
rejects field access on generic constants under `generic_const_exprs` and
the trait solver overflows on `Cfg`-driven sizes. The substrate rule (caps
are defaults, never policy) outranks the convenience of the workaround: a
sketch arc investigates cap-lifting shapes (macro-generated per-cap
instantiations, associated-const indirection, alternative bound layering)
before more code accretes on the fixed sizes. Documented fixed caps remain
acceptable only as the proven-infeasible fallback, each carrying a tracked
lift condition.

### A1-3: GATE-2 closes; the remainder is one completion arc

GATE-2 (#662) closes with its cleanup and a perf-gate re-run. Everything
remaining (E5/E6/E8, the unslotted canon features below) is one completion
arc charted to the full canonical design, without further gate ceremony.
The chart-the-path routine produces the arc's roadmap.

All four unslotted feature groups chart into the arc:

1. Perf substance: micro-morsels, branch dispatch shape, shared-read-column
   strategy, sub-byte bitpacking stride, intrinsics/microkernels.
2. Adapt completion: the three reorganisation triggers (morsel recompute,
   per-phase config re-select, record-count plan recompute including
   `replace_resource` writing the value, task #345), predictive parking,
   stolen/idle counters.
3. Ecosystem bridges: facade/plugin-host engine integration, kits and
   providers polish, persistence spine wiring (R2 evict/inject).
4. Ops surface: `PipelineResult` status surface (Completed/Failed/Poisoned
   with dependent poisoning), the work-stealing `Executor` extension point,
   schedule introspection (#183).

### A1-4: Dead-weight cleanup is deferred into touching slices

The dead inventory (`topo_order` / `topo_count`, `PlanCache`,
`synthesise_core_programs` / `CoreProgram`, `RecommendedOrder`'s run-path
absence, the `AdaptMode` alias) is not a standalone cleanup round. Each item
is deleted by the first charted slice that touches its file, with the
no-legacy-shims rule applying at that point. The inventory above is the
checklist those slices consult.

### A1-5: Facade feasibility sketches run before the chart

The two unconcluded plugin-facade sketches (section 7-4a opaque AccessSet
past `ContainsAll`; section 7-4b per-morsel ABI hop) are driven to recorded
WORKS / FAILS findings before the chart-the-path run, so the roadmap charts
the ecosystem-bridge group on proven ground rather than around a named
unknown.

### A1-6: The consumer surface adopts a computed Ctx type if feasible

The canonical consumer spelling of a WorkUnit reduces to `Read`, `Write`,
`Hint`, and schedule; the full nine-parameter `EngineCtx` shape is computed
by an api-side type function (working name `CtxFor<'frame, R, W, Sched>`)
instead of hand-spelled per WU. A feasibility sketch proves or refutes the
type function under the current toolchain (GAT normalization, GCE
interaction). If it walls, the fallback discussion re-opens with the sketch
findings; the derive-macro and keep-explicit alternatives stay on the table
only in that event. `BuilderInput` and `HasSchedule` impls stay explicit
either way.

### A1-7: The meta carrier question is sketch-decided

Whether meta WUs stay on the shared consumer carrier (with bands hoisted out
of the morsel loop on every record-bearing path, once per frame, single-core
and parallel alike) or move to a dedicated meta carrier with an independent
band walk is decided by sketching both shapes far enough to compare
complexity and codegen. The known warts either shape must cure: per-morsel
meta execution on record-bearing paths, leading-band accumulator appends
unsupported on the parallel unit-outer path, and the slice-1b
clear-on-dispatch semantics (#687).

### A1-8: The perf gate moves to per-arm bars with a stricter baseline

The flat 1.10x bar retires. The gate demands per-arm calibrated bars (tight
where the engine should win, parity tolerance mid-range), the std baseline
upgrades to a persistent-pool variant (matching the engine's spawn-once
discipline), and measurement variance is fixed with median-of-N runs. The
gate stays blocking.

## Constraint notes elevated to canon

These are facts about building this design under the pinned toolchain
(nightly-2026-05-28). The standalone spec must carry them; they are not
incidental.

1. Type-level N-way partition of a heterogeneous carrier requires the
   forbidden full `specialization`; the canonical mechanism is const data
   plus const-gated DCE over a flat carrier (r4). Partition lives in const
   evaluation, never in the carrier type.
2. Type-keyed projection out of heterogeneous lists uses inferred index
   witnesses (`Here` / `There<I>`, `Selector`-family traits), never
   type-equality specialization.
3. `generic_const_exprs` carries two practical ceilings: no field access on
   generic constants (caps cannot be `Cfg`-sized today; see A1-2) and a
   complexity limit on inline const blocks in bounds (worked around by
   associated-const carrier structs such as `IsRoot` / `PhaseAt`).
4. Accumulator appends saturate at reserved capacity (a soundness guard);
   capacity equals record count at build. Fixtures and consumers size with
   headroom; a silently dropped append means live-versus-capacity first.
5. Engine-owned mutable meta state lives in the `MetaBlock` scheduler field
   (consumer resources are `Copy` read-only and cannot carry Cells); the
   `MetaAccess`-gated accessor exists only on a `MetaRef`-bearing Ctx,
   enforced at compile time.
6. Platform tiers are os and no_os only; the std tier is deferred
   indefinitely (op decision, registered in workspace memory).
7. The clock is a builder-slot provider (`SchedulerBuilder::clock`),
   defaulting to the os-tier monotonic clock under the default feature and
   to a null clock on no_os until DI supplies one. Platform inputs routed
   through `.with(...)` drop their values; value-carrying providers get
   dedicated slots.

## Amendment-chain registry

The effective canon is the consolidation spec as modified, in order, by:

1. r2 (`202606081500`): supersedes r1 Phase D and C3; declares type-level
   auto-derived fiber grouping dead (E0119/specialization wall); splits C3
   into arena-layout and execution-order halves; promotes the Approach-2
   variant menu.
2. GATE-2 rechart (`202606070100`) + r3 (`202606070200`): corrects r1/r2
   parallelism framing to core-pinned column-disjoint trunks joined only at
   waists and bridges; pins "compile-time" as relative to plan execution.
3. r4 (`202606070700`): replaces r3's G2-0c nested-carrier type construction
   with the const-eval grouping + const-gated DCE mechanism.
4. Fairness correction (`202606081100`): parallel-arm baseline is optimal
   multi-threaded std; the earlier 3.5x claim is superseded by parity.
5. r5 (`202606081600`): state-map corrections over the earlier overview and
   deviation ledger.
6. Round-level amendments: ColumnValue de-specialized (#631); arvo-graph
   row-width parameterisation (#663/#668); the E4 GateWith witness-slot
   deviation and engine-owned MetaBlock (rounds `202606081300`,
   `202606090000`); run_parallel designated-core meta bands
   (`202606110855`); the E8 clock slot and EMA spine (`202606110948`).
7. This addendum (A1): the rulings and constraint notes above.

## Sequence from here

1. Pre-chart feasibility sketches, findings recorded in VCS: facade 7-4a and
   7-4b (A1-5), `CtxFor` computed Ctx (A1-6), meta-carrier both shapes
   (A1-7), cap-lifting shapes (A1-2). The RCM order bench (A1-1) and
   per-arm gate recalibration (A1-8) ride the chart as early steps.
2. The chart-the-path routine runs against canon-plus-A1 and produces the
   completion-arc roadmap (A1-3 scope).
3. A new standalone canonical specification is written: fully
   self-contained, readable with no prior project knowledge, with related
   notko and arvo features included inline as attachments, superseding the
   need to cross-read the chain registered above.
4. Build work resumes on the charted arc.
