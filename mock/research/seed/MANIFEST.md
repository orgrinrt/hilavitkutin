# Seed Manifest

```
frozen = false
```

State: collection pass complete; the pre-freeze open-question resolution
batch is delivered into the chapters (see the resolution-batch accounting
below); verification pass 1 and the op-ordered redo pass ran with their fix
batches applied (records `202607210010`, `202607210300`). The freeze awaits
the op rulings listed in `governance.md#lifecycle`.

This manifest proves losslessness: per chapter, which sources drained into
it, and what was deliberately left out because a later canon source
supersedes it. "Source" means the tier-1/tier-2 canon per the A2-1
precedence order ([[governance]]); intermediate artifacts appear only where
they are the registered record of a ruling. Paths are as they stood at
consolidation time; the amendment files now live under
`mock/research/pre_seed/`, the founding round under
`mock/design_rounds/202604200055/`.

## Per-chapter drain record

### governance.md

Drains A2 (`202607193200`) in full: the A2-1 precedence order, the A2-4
evidence-then-bless standard, the A2-5 mis-citation record reduced to its
surviving consequence (swap semantics unspecified, round commissioned).
Drains A1's amendment-chain registry as the source list. Adds the seed's own
lifecycle rules (op mandate, 2026-07-19) and the open-question inventory
collected from A1-1, A2-3/4/5, the founding spec's open items, and r8's
op-question section (Q1 resolved by A2-1; Q4 resolved by the executed revert
round `202607193110`; Q2/Q3 carried as open items 3 and 6).

Left out: A2-5's narrative of which rounds mis-cited what (paper trail, not
design); A2-2's registration act (consumed into the source list).

### identity.md

Drains: the spec's header (purpose, source topics), crate structure section,
vocabulary section, and R6/R8/R9 resolutions; the unified-engine amendment
(`202606061000`) (the ruling and its verification; the two-gate re-read's
FiberShape consequence is superseded by r4's const-eval mechanism and left
out); r2 section 1 (the static/adaptive split, the two PlanStage cases,
the extensibility surfaces, the mirror verdict that no adaptive trigger
reorders dispatch); the current twelve-crate structure and plugin-host layer
contracts from the repo's design templates (the shipped structure of
record, postdating the spec's three-crate sketch; the templates are tier-4
under A2-1, so the structure's tier-2 registration is owed as a registry
row at drain time); the engine-scope boundary from
the spec's foundational-context notes.

Left out: the spec's three-crate table where the twelve-crate structure
supersedes it (the arvo family table moved to foundations); the per-project
consumer table (the polka/saalis/loimu scale profiles fold into execution's
strategy discussion as the three-scale target envelope; the per-repo scope
split, full pipeline versus scheduler abstractions only, is historical
context); T1-T7 source-topic anatomy.

### foundations.md

Drains: spec domain 01 (arvo crates, dependency flow, engine mapping, build
integration), domain 02 (platform tiers, traits, hardware detection, the
no-alloc statement), domain 03 (build model, pragmas, LLVM passes, PGO/BOLT,
profiles, the ExpandedLto requirement); A1 constraint notes 6 (tiers os and
no_os only) and 7 (clock builder slot, value-carrying providers); the
caps-are-defaults application to substrate capacities; the workspace
fix-upstream rule as it binds the engine to arvo.

Left out: the spec's std-tier column (superseded by A1 note 6; noted as
deferred); the spec's per-crate arvo source citations (arvo's own design
rounds are the authority for arvo internals); nightly-gate list from T4
(superseded, see constraints).

### data-model.md

Drains: spec domain 04 (R3 type-native stride, co-located arena, 64-byte
alignment, the arena-addressing bench resolution with its numbers), domain
05 (R4 ColumnValue as amended by #631's de-specialization, round-level
amendment in A1 item 6), domain 06 (three store types, StoreId/AccessMask,
AccessSet lowering, ZST erasure, R1), domain 07's ColumnStorage half (raw
pointers, consumer memory, release-advisory counts, separate Seq/Map arena
pointer, R2 pointer), domain 08 (ordering from data flow, commutativity,
no-lineariser guarantee, record independence, no partial writes); the
never-reallocated column invariant (storage addendum consequence, perf memo
lineage).

Left out: R4's min_specialization blanket-impl mechanics (superseded by
#631; the surviving contract is stated); superseded stride designs (named
only as "superseded universal-stride designs"); the Payload/SlotSize system
(dead, R4).

### storage.md

Drains: the resource-storage canonical addendum (`202606210600`, revised
2026-07-02) in full: the R5 field-type model, the one-record blob with the
six-variant bench numbers, scalar snapshot with the honest wall-clock
finding, live-streamed collections with the 2.5x/4MiB/64MiB numbers, handle
store and noalias, erased static-shape addressing with op's hybrid ruling
and parity numbers, the morsel-budget interaction; A2-3 (PlanAffecting open
marker); A2-5's surviving consequence (swap unspecified, round commissioned,
mark-dirty-only interim); the founding spec's open item 1 (collection
accessor shape).

Left out: the addendum's retraction narrative (the refuted shape-bound
reading; the manifest records that it was refuted and dropped); the
DrainStores drift discussion (implementation status, registry material, not
design).

### contracts.md

Drains: spec domain 09 (trait signature, schedule conditions, Context
access, inner-loop contract, registration, authoring guideline), domain 10
(virtual flag system in full: bit-packing, affinity assignment,
per-(virtual,consumer) bits, clear-on-dispatch, epoch reset, atomicity
split, pure-flags rule), the cross-domain ColumnStorage checkpoint (raw
pointers, `&self`); A1-6 (computed CtxFor, shipped); A1 constraint note 2
(index witnesses); r2's D1 resolution reduced to its surviving contract (the
provisional producer-before-consumer registration constraint, op decision
(b), with the sketch-proven guarded-walk relaxation); the meta OnMeta
condition pointer.

Left out: the E0119/specialization wall narratives (constraint notes carry
the surviving fact); the signal-ordered execution theory (noted explicitly
as never validated, not committed); dispatch-approach content (in dispatch).

### plan.md

Drains: spec domain 11 (phase decomposition, cache principles, execution
strategies, shared-read-column approaches, column-count strategy), domain 12
(morsel formula, 3D cube, change detection, dispatch integration), domain 14
(fiber definition, co-located arena, holistic feasibility check, column
classification, temperature, spanning tree, adaptive configs, unique-column
counting), domain 15 (the nine steps in full with their arvo mapping),
domain 16 (data loading, loaders as WUs, plan recompute, no backpressure);
A1-1 (the RCM row-order bench fork); r2 section 2 (the C3 static/adaptive
split of RCM's two outputs); r8's spectral role deviation with A2-4's
standard applied; the deferred-algorithms list (spec domain 15).

Left out: the spec's worked game-world example traces and bench tables
(evidence, cited by the registry bench rows, not restated as design); T6
data-structure rename notes (vocabulary already canonical).

### dispatch.md

Drains: spec domain 17 (devirt rules, approach menu with penalties, the
flattener/rust-pipe pattern, inlining discipline, compiled per-core intent,
progress counters, lock-free guarantee, ASM checklist), domain 13
(intrinsics: what helps, what hurts, the day-one microkernel table with the
prefetch removal, the likely/unlikely ban), domain 19 (pointer separation,
stack-local resource caching, accumulation under convergence); r4's
const-eval grouping + const-gated DCE mechanism in full (the four-step
chain); A1 constraint notes 1-3; r2 section 6 (Approach-2 as the
devirt-preserving realisation of the spec's record-count approach
selection); the deviation-ledger section 1 disposition (runtime ownership
mask, op-blessed, escalation path) as it bears on the dispatch design.

Left out: r3's nested-carrier G2-0c construction (superseded by r4; the
wall is carried as constraint note 1); the sketch-by-sketch proof narrative
(registry sketch rows carry it); approach-C preference history.

### execution.md

Drains: spec domain 20 (pool, hybrid wake, heterogeneous cores, core-pinned
trunks, convergence, pipeline parallelism, work-stealing-optional, spawn
overhead), domain 21 (strategy thresholds, shapes, per-phase selection,
plan-time selection with LIGHT_THRESHOLD, frame budget, schedule reuse,
cadence-consumer-side), domain 22 (two analysis tiers, EMA metrics,
reorganisation triggers, temperature, predictive parking, frame prediction);
the GATE-2 rechart + r3 (trunk-per-core governing correction, the two-stage
sequence's surviving model, sketch-proven results with the 2.84x number);
the fairness amendment (`202606081100`, parity baseline, superseded 3.5x
claim); A1-8 (per-arm gate); the six agent-call deviations from the ledger
(`202606072100` sections 2, 4-and-its-supersession, 5, 6, 7, 10) under
A2-4; r5's worker-side barrier correction; the round-level designated-core
meta-band amendment pointer (in scheduler); E4-slice-3/E8 one-arc finding
(adapt hangs off ScheduleEnd).

Left out: the drifted E1/E2 record-partition framing (superseded, named
only as the corrected drift); stale ledger sections 1/4/9 status prose (the
correction header's surviving facts are used); bench table bodies (registry
bench rows).

### scheduler.md

Drains: spec domain 23 (builder API, static schedule, self-hosting meta
pipeline, version stamps, error handling R7, error propagation, R2 APIs,
resource initialisation); A1 constraint notes 5 (MetaBlock) and 7 (builder
slots); the designated-core meta bands round-level amendment
(`202606110855`); r2 section 5 (extensibility surfaces, facade pattern) and
r5's facade sketch conclusions (both WORKS); the Kit/providers preset layer
from the repo's design templates; the PlanAffecting pointer (A2-3).

Left out: the E4 build-slice narratives (implementation history); the
MetaAccess sketch-gap discussion (closed by the shipped meta pipeline).

### constraints.md

Drains: the spec's cross-cutting constraints section (architectural
constraints, the 20 design principles condensed, enforcement lints, the
auto-vectorisation contract with the eight killers); A1-2 (caps are
defaults, the redesign arc); all seven A1 constraint notes; the workspace
unstable-features regime as it supersedes T4's nightly-gate list; the
strict-by-design quality regime and the evidence-then-bless and
bench-decided standards (A2-4, the arc's standing rules).

Left out: T4's always-on/where-useful/tracking nightly lists (superseded by
the vetted workspace tables); per-lint TOML details (live in the lint
config, not canon prose); the spec's three cross-project lints
(no-toml-value, no-ui-below-cli, sdk-traits-only), which bind
sibling-project crates outside the engine's scope and are deliberately
excluded.

## Round-level amendments accounting

A1 item 6 registers five round-level amendments. Their disposition here:
ColumnValue de-specialization (#631) is in data-model; the arvo-graph
row-width parameterisation (#663/#668) is subsumed by the caps-are-defaults
rule in constraints (the 64-node cap is lifted; no seed statement carries a
fixed width); the E4 GateWith witness-slot deviation and engine-owned
MetaBlock are in scheduler (MetaBlock as constraint note 5; the witness-slot
detail is implementation record, registry material); the designated-core
meta bands are in scheduler; the E8 clock slot and EMA spine are in
foundations (clock) and execution (EMA metrics).

## Post-A2 state corrections applied

The seed reflects two facts newer than r8: the D1 install revert round
(`202607193110`) executed, and the swap-semantics round has since run, so
the S1-S7 install spec is implemented and benched, awaiting op ratification
(storage chapter); and A2's rulings supersede r8's open questions Q1/Q3/Q4
as recorded in governance.

## Resolution-batch additions

Content that entered the chapters after the collection pass, from the
pre-freeze resolution batch (each with its evidence record): storage's
"Replacement semantics" S1-S7 section (round `202607200500`, record
`202607201100`); plan's A1-1 ordering resolution (`202607200800`) and the
spectral evidence status (`202607201200`); execution's deviation
resolutions and wake-policy ruling (`202607201400`, `202607201600`,
`202607202000`, `202607202200`, `202607202340`, `202607210100`
controlling) plus the `mid_record` boundary statement (`202607201500`);
governance's item 7 and 8 resolutions (`202607201500`, `202607202100`).
These are chapter updates under the lifecycle's resolution-batch rule, each
traceable to its record; the per-chapter drain records above describe the
collection-pass sources.
