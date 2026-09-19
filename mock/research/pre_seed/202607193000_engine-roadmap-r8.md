# Engine roadmap r8: charted from the design record

**Date:** 2026-07-19
**Status:** chart-the-path conclusions for the engine-completion arc.
**Supersedes:** `202607191500_engine-roadmap-r7.md` entirely.
**Grounded on:** the design record read directly (the consolidation spec
`mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`, the
standalone canonical spec `202606111800`, addendum A1 `202606111400`, the unified-engine amendment
`202606061000`, the storage addendum `202606210600`, the GATE-2 deviation ledger `202606072100`),
plus call-site verification against source at HEAD `e0876521`. r7 and the three-way audit
`202607191400` were read as evidence of one agent's reasoning; no status in this document is
inherited from either. Where a claim of theirs is repeated here, it was re-established against the
spec text or the source; where one was found wrong, the correction is stated in section 8 with what
was checked.

## 1. What the engine is when complete, per canon

The canonical engine is the standalone spec's Part I, which restates the consolidation spec's 22
domains as amended. Condensed to the load-bearing commitments, each of which is a completion
criterion for this arc:

1. **Monomorphisation is the dispatch.** The registered units form a flat type-level carrier; const
   evaluation computes the grouping (phases by waist, trunks by disjoint-column components, the
   lifecycle rank renumber); a const-gated walk dead-code-eliminates non-members so each trunk gets a
   member-only program with zero indirect calls. Type-keyed access uses index witnesses, never
   specialization. (Spec section 5; A1 constraint notes 1 and 2.)
2. **One unified engine.** No single-core versus multi-core code fork anywhere. Core count is
   configuration; the same plan pipeline computes the best per-core programs for whatever count is
   configured, and at one core that is a serial sequence as a natural degenerate case.
   (`202606061000`, op ruling, canon-level authority.)
3. **The full plan chain.** Access matrix, topo sort, waist detection, RCM (row ordering as the
   work-unit execution order, column ordering as the arena layout), block-diagonal plus
   Dulmage-Mendelsohn validation with dead-column elimination, trunk formation (spectral over the
   fiber-conflict graph when a phase has more than 5 fibers, otherwise single trunk), fiber grouping
   (greedy at 10 or fewer ops, matrix-chain DP above), morsel sizing from the cache model, dirty
   seed. Plan output stored back onto the consumer's columnar storage. (Spec section 4; consolidation
   domain 15 steps 1 through 9.)
4. **Parallelism from trunks.** Core-pinned trunks joined only at waists and bridges; single-fiber
   record splitting exists only as two-way head+tail convergence in a single-trunk commutative
   phase, never as an N-way partition; phase overlap through per-fiber progress counters (plain
   store/release, acquire on the consumer side); the accumulator path runs unit-outer with per-core
   regions merged in append order. (Spec section 9; consolidation domains 11, 17, 20.)
5. **Incremental execution.** Dirty-skip on every path including parallel; per-morsel generation
   counters propagating through the DAG; version stamps incremented only on completion.
   (Spec section 7; consolidation domains 8, 12, 23.)
6. **The self-hosting meta pipeline.** PlanStage, ScheduleReady, PassStart, consumer band,
   ScheduleEnd, on the same carrier through the same const-gated walk, with engine-owned mutable
   meta state behind the MetaAccess gate (a recorded, contained deviation from the uniform data
   model). (Spec section 8; A1 constraint note 5.)
7. **Adaptation of parameters, never structure.** EMA-driven: morsel-size recompute, per-phase
   config re-selection among pre-monomorphised variants, record-count parameter recompute, resource
   replacement marking the plan dirty, predictive parking at waists by predicted wait. (Spec
   section 10; consolidation domain 22.)
8. **The resource storage model.** `Resource<T>` is a handle; the value is a one-record contiguous
   blob; scalar members snapshot to a stack local before the morsel loop; `Seq`/`Map` members are
   live-streamed ptr+len; handle store has separate provenance from value columns; addressing goes
   through the erased static-shape descriptor, global-capable. Bench-decided and op-ruled; no open
   storage fork remains. (`202606210600` addendum.)
9. **Platform tiers os and no_os only**, providers as trait contracts, the clock as a builder slot.
   Caps are defaults, never policy: every fixed capacity either routes through the
   `Capacity`/`Dim<N>` pattern or carries a tracked lift condition. (Spec section 11; A1-2.)
10. **The operations and ecosystem surface.** `PipelineResult` with per-fiber
    Completed/Failed/Poisoned and dependent poisoning; the work-stealing `Executor` extension point
    (deterministic assignment stays the default); schedule introspection; evict/inject persistence
    APIs; the facade pattern for plugin-host integration (one C-ABI hop per morsel, bridge stores
    declared normally). (Spec sections 3.6, 9, 12; consolidation domain 23 and R2; A1-3 groups.)
11. **The performance gate.** Parity with an optimally threaded, persistent-pool standard-library
    baseline at equal core count, per-arm calibrated bars, blocking. (Spec section 15; A1-8.)

### The canon record's own state

Two precedence facts a reader of this arc must hold:

- The standalone spec's header claims blanket supremacy over all older documents. r7 reports an op
  ruling of 2026-07-19 that neither the standalone spec nor the unified-engine amendment
  blanket-wins: both are amendments to the consolidation spec, each conflict resolves on its own
  merits, and the consolidation spec is the tiebreak. **That ruling exists nowhere in the design
  record itself**; its only written trace is inside r7, a roadmap draft this document supersedes.
  Recording it durably is item Q1 in section 7. Until recorded, this roadmap applies the ruling as
  reported (it changed no conclusion here: every removal and build item below is grounded in text
  the consolidation spec and the amendments state consistently).
- The canon-amendment mechanism gap (#667) is still open: the consolidation spec is a locked
  archived round, and rulings accumulate as standalone research files. The amendment-chain registry
  in A1 is the authoritative index of which files amend canon.

## 2. Verification method

Every status below was established one of two ways. Spec claims were read in the named document at
the cited lines. Source claims were established by finding the constructing and reading call sites
at HEAD `e0876521` (grep plus enclosing-function checks), not by reading doc comments, following the
audit's own hard-won rule that a type's declaration proves nothing about whether anything builds
one. Where a trace crosses a crate boundary (api, providers, tests), the trace crossed it too.

The source moved after r7 was written. Eight rounds closed on 2026-07-19 after r7's draft
(`202607192010` through `202607192810`), and one round is open in DOC phase (`202607192900` topic,
`202607192910` doc CL). Section 3 reflects the post-round state, not r7's.

## 3. State ledger at HEAD

Five states. WIRED means reached from an entry point and matching canon. SUBSTRATE-ONLY means built,
compiling, unreached; the work is the call site. ABSENT means no implementation. DEVIATED means
shipped and reachable but not the canonical mechanism, with the deviation's recording and escalation
noted. DEFECT means a reachable body contradicting its contract.

### Wired and matching canon

The plan chain steps 1 through 5 plus morsel sizing, phase configs, and dirty seed
(`plan/mod.rs:221` `compute_execution_plan` orchestrates `build_dag`, `topo_sort`,
`compute_waists`, `rcm_reorder`, `block_diagonalise`, `project_fiber_components`,
`compute_upward_rank_and_dirty`, `compute_fiber_morsel_windows`, `select_phase_configs`,
`classify_columns`, `compute_predecessor_masks`). The const-grouping carrier and its DCE walk
(`RunTrunkDispatch`, `IsRoot`, `PhaseAt`, `Member`). `EngineCtx` projection with the
`Selector`/`Project` witness families, and the computed `CtxFor` type
(`dispatch/engine_ctx.rs:1748`; A1-6 resolved, sketch `202606111430` WORKS). Schedules
Always/On/OnMeta. Virtual firing with epoch reset. The meta pipeline end to end with the `MetaBlock`
bridge, single-core and parallel. The frame publish/await protocol with the sense-reversing
`waist_barrier` and futex parking. Core-pinned trunk parallelism under the runtime ownership rule.
The unit-outer accumulator path with per-core regions and append-order merge, its slice arithmetic
now derived from one definition (`accum_per`, round `202607192610`). Incremental dirty-skip on the
single-core and fused paths. Per-fiber L1 morsel windows on `run` and `run_core_phase`. Pass-duration
and per-phase EMAs feeding `select_adapt_config` (`scheduler/mod.rs:1560`, `:2364`). The
`DrainStores` one-record blob with scalar snapshot and live-streamed collections, per the storage
addendum. `replace_value` now installs the value through `Selector<V, Index>` (round
`202607192710`; the sketch established the old marker-typed signature could not carry an install at
all).

**Spectral is in the live chain**, contrary to the audit and r7 (section 8, correction 1):
`spectral_partition` (`plan/steps.rs:452`) is called at `:663` inside `project_fiber_components`,
which `compute_execution_plan` calls at `plan/mod.rs:309`; blocks wider than 5 units form fibers
from the spectral grouping (`steps.rs:719-733`), narrower blocks use the greedy former. Whether the
shipped role matches canon's is a separate question; see DEVIATED.

### Substrate-only (built, unreached; wiring is the work)

- Head+tail convergence. `thread::Convergence`, `plan::HeadTailConvergence`,
  `RecordRange::{Head,Tail}`. `Fiber.head_tail` is assigned `Maybe::Isnt` exactly once
  (`plan/fiber.rs:221`); `unit_meta.commutative` is written (`plan/mod.rs:422`) and read nowhere;
  `core_program.rs` always emits `Full`. The eligibility predicate is a build, not a wire.
- Phase-overlap progress counters. `ProgressCounter` with correct Release/Acquire, the arena
  accessors, the `dmb ishst` release fence, all satisfying canon's plain-store constraint.
  `PoolFrame.progress_slots` is `NonNull::dangling()` (`scheduler/mod.rs:803`).
- The parking-tier API (`pick_tier`, `spin_budget_for`, `predicted_wait_ns_*`, `WakeStrategy`);
  `waist_barrier` calls `atomic_wait` directly and carries no phase index in its signature
  (`thread/barrier.rs:104-108`). `PoolFrame` is `<MAX_CORES, 1>` (`scheduler/mod.rs:722`): the core
  axis widened, the phase axis is still 1, so per-phase predicted-wait has nowhere to live.
- Core classification. `classify_cores` returns all-P; all four `detect_into` probe arms discard
  their output (`thread/class.rs:80-116`, honestly documented since round `202607192010`).
- Per-core program synthesis. `assign_cores` (real body, test-only callers),
  `synthesise_core_programs` (test-only callers, skeleton accounting: heuristic fiber count,
  hardcoded zero trunk count, every core assumed in every phase).
- Strategy selection. `DefaultSelector::select` (`strategy/mod.rs:31`) is reached only from
  `tests/strategy.rs`. The live return is three-way (Sequential/Adaptive/Phased); `PipeChase` is
  never constructed; there is no `ChaseSteal` variant and no producer/consumer weights, which
  canon's rule requires (consolidation `:1934-1951`). Distinct from the WIRED per-phase config
  selection (`select_phase_configs`), which is canon's other half of domain 21 and does run.
- The adapt module's nine axes and `AdaptArena` (two `PhantomData`s behind a layout-describing
  module doc, now honestly labelled). `select_adapt_config` computes its decision from the EMAs
  without them, and nothing consumes the decision's output shape beyond the internal cell.
- `resource::accumulator`'s `ConvergenceBuffer`/`combine` (the head+tail merge data model, distinct
  from the live `merge_accums` compaction). Reserved for the head+tail build.
- The `dispatch::order` const fold (`topo_order`, `CarrierMasks`, `carrier_order`). Reserved: it is
  the codegen keystone's substrate, not supersession victim (see section 4).
- `steal_fallback` is `todo!()` (`thread/mod.rs:104`), the Executor extension point.

### Absent

Dulmage-Mendelsohn fine decomposition and dead-column elimination (`block_diagonalise` is
connected-components only; its own doc defers D-M "to a later round", `steps.rs:372-374`).
Matrix-chain DP fiber grouping (`arvo-comb::matrix_chain_dp` exists upstream; no engine call).
RCM row order as the execution order (computed, stored back, and used only to validate
producer-before-consumer registration via `RecommendedOrder`; the provisional
`NonTopologicalRegistration` constraint from the spec's Part II stands). Column classification
refinement (`classify_columns` lands everything Internal by documented skeleton, `steps.rs:1079`,
so DSE has nothing to act on). Per-morsel generation counters (coarse per-store dirty only).
Version stamps. Parallel-path dirty skip (`run_parallel` dispatches all-ones,
`scheduler/mod.rs:2138`). `run_fused` per-fiber morsel window (uses `Cfg::MORSEL_SIZE` const,
`:2269`). Micro-morsel tiling behaviour (the interval consts exist). Shared-read-column strategy.
Sub-byte bitpacking stride (BIT_WIDTH declared, stride is `size_of`). The day-one intrinsics
microkernel set. Predictive parking. The three adapt reorganisation triggers. `PipelineResult` and
dependent poisoning. Schedule introspection. Evict/inject persistence bridge. The facade engine
integration (both feasibility sketches WORKS: `202606090100`, `202606090200`). The morsel-absolute
slice accessor.

### Deviated, with the recording

- **Runtime-mask dispatch instead of compile-time per-core monomorphised programs.** Op-blessed
  2026-06-07 (deviation ledger section 1), bench-gated, escalation named as build.rs/proc-macro
  codegen of the real monos. In-language materialisation walls on forbidden specialization or
  const-recursion overflow, so within pure Rust this is toolchain-forced; the build-script
  escalation path remains open and is the recorded route if the bench demands it. The
  `FiberShape`-gated codegen family (zero impls) is that target's substrate.
- **`Pin`-based `run_parallel` receiver and inline `PoolFrame`** instead of arena placement
  (ledger section 2, agent-call, reconcilable; trigger is consumer ergonomics).
- **Spectral's role.** Canon's step 7 forms **trunks** by Fiedler bisection over a fiber-conflict
  graph weighted by shared column bytes, gated at more than 5 fibers (consolidation `:1373-1396`).
  Shipped: trunks come from block-diagonal connected components per phase, and spectral instead
  forms **fibers** within wide blocks, gated at more than 5 units. No ledger entry records this
  role change. It is either drift to rebuild or a reinterpretation to bless and record; op question
  Q2, with the #644 bench as the proposed oracle.
- **Engine-owned meta state** (MetaBlock): recorded and contained, A1 constraint note 5. Not owed
  further work beyond keeping the gate intact.

### Defects still open

`replace_resource` keeps its stub deliberately (round `202607192710` NOT-CHANGED entry): its
`PlanAffecting` bound has exactly one implementor (`DefaultRunCfg`, a run-config type), so the
function is callable only for a type no consumer registers as a resource. Deciding what the bound
ranges over is #696, op question Q3. `plan_dirty` by `PlanAffectingId` is stubbed alongside and
belongs with the plan-recompute wiring. The former D2 (std spawn no-op) and D3 (deallocate
alignment UB) are fixed and test-observed; the fn-pointer `spawn_fn` half was deleted from both
tiers with its trampoline ownership traced (round `202607192810`).

## 4. Removals, on supersession grounds

Per the workspace no-legacy-shims and drift rules, an item is removed when a named later design
decision replaced it; reference count is not a keep signal (a test exercising a superseded
mechanism is rot with a green checkmark). Each removal below names its superseding decision.

1. **`dispatch/phase_run.rs` entire** (`RunPhase`, `RunPipeline`, its local spin-only
   `waist_barrier`), its re-export at `dispatch/mod.rs:41`, its test
   `tests/phase_pipeline_dispatch.rs`, and the `DESIGN.md.tmpl:222-232` paragraph describing the
   nest. Ground: A1 amendment-registry item 3 records that r4 (`202606070700`) replaced the
   nested-carrier type construction with const-eval grouping plus const-gated DCE; `RunPhase`/
   `RunPipeline` are that construction, and sketch `202607191200` independently proved the nest
   unwireable from flat registration (partition-by-key walls on forbidden specialization). The open
   round `202607192900`/`202607192910` already charters exactly this; finishing that round is the
   first action of the arc.
2. **The `phase_barrier_arrive`/`_reset`/`_observe` family** and the `DESIGN.md.tmpl:231` promise
   to wire it. Ground: it is a second, incompatible reset protocol on the same `phase_arrived` word
   the live sense-reversing `waist_barrier` owns; the live barrier is the shipped realisation of
   the frame protocol the amendments record (r5 state map; ledger correction header). Canon's
   silence on the runtime barrier shape does not license two protocols on one word. Same open
   round.
3. **Already executed under the same test** (recorded here so r8 is complete): the N-way ceil-slice
   in `run_core_phase` (deleted, round `202607192410`; ground is the spec's explicit "never as an
   N-way record or morsel partition", `202606111800:447-450`, consistent with the consolidation
   spec's two-thread convergence at `:770-771` and `:1840-1841`); `thread::pool`
   (deleted, round `202607192510`; one reference, no design entry, duplicated the frame protocol's
   shutdown); the fn-pointer `spawn` (deleted both tiers, round `202607192810`, superseded by the
   generic spawn D2 restored).

**Explicitly not removals**, because supersession does not apply: `core_phase_mask`
(`core_mask.rs:3-4` records the mask form as op's chosen mechanism of 2026-06-07; the live inline
`rank % ncores` implements it); the `dispatch::order` const fold and the codegen family
(`FiberShape`, `codegen_fiber`, `codegen_core`), which are the canonical compile-time
materialisation's substrate held behind the bench-gated escalation; `RecordRange::{Head,Tail}`,
`HeadTailConvergence`, `Convergence`, `ConvergenceBuffer`, which are the canonical head+tail
mechanism's substrate that item C1 consumes; the adapt axis types, which the adapt completion
consumes. Deleting any of these would destroy a canonical target's substrate, the exact mistake
the audit caught r6 preparing.

## 5. Dependency-ordered remaining work

Bands in dependency order; benches ride their owning items (forks are bench-decided, not argued).

### Band A: finish what is open

- **A1.** Close round `202607192900`/`202607192910`: the two deletions of section 4 with their doc
  paragraphs. Everything in band B and C touches these files' neighbourhoods; carrying dead
  competing protocols into that work invites the corruption the round names.
- **A2.** #696: op decides what `PlanAffecting` ranges over (Q3), then `replace_resource` gets the
  same `Selector` install arm `replace_value` got, and `plan_dirty` keyed by `PlanAffectingId`
  wires into the plan-recompute trigger. Gates the adapt triggers and any resource-swap test.

### Band B: canon-mandated structural substrate

- **B1. Cap lifting completion (A1-2).** A1-2 orders this arc "before more code accretes on the
  fixed sizes", the sketch (`202606111450`, shape s2) proved the GCE-free consumer shape WORKS, and
  the first slice landed 2026-06-19 (plan_dirty and grouping caps onto `PlanDims`). Residuals:
  `gate2_phase`/`gate2_trunk` scratch still `[USize; GATE2_MAX_UNITS]` (#690, guard at
  `scheduler/mod.rs:116`), the accumulator publish array still sized by `MAX_CORES *
  GATE2_MAX_ACCUMS` (#649 partial), `MAX_CORES` fixed. r7 omitted this item entirely (section 8,
  correction 2).
- **B2. Widen `PoolFrame`'s phase axis** from `<MAX_CORES, 1>` to real `P`, and allocate the
  progress-slot arena (replaces `NonNull::dangling()`). This is the shared prerequisite of
  head+tail (if `mid_slot` resolves to an arena slot), phase overlap, parking tiers, and the adapt
  telemetry (deviation ledger section 3 named exactly this widening as the adapt gate).

### Band C: the parallel-substance gaps (the largest canon deltas in the run path)

- **C1. Head+tail convergence.** Four parts: the eligibility predicate (consume
  `unit_meta.commutative`, single-trunk phase, record threshold, accumulation compatibility;
  populate `Fiber.head_tail`); decide `mid_slot` semantics (the variant doc says record boundary,
  the name says arena slot, nothing constructs it; the builder of C1 decides and records); emit
  `Head`/`Tail` from the core-program synthesis once C3 gives it honest accounting; the two-walker
  dispatch with per-thread accumulators merged per domain 19 (additive add, sequential fallback
  for non-commutative), which is what `ConvergenceBuffer` exists for.
- **C2. Phase overlap.** Publish morsel completion through the progress arena (mechanical after
  B2); consumer-side acquire replacing the full barrier where the plan allows. The happens-before
  argument is provable on paper but needs a contention stress harness; this is the one item a
  sketch cannot fully prove.
- **C3. Per-core program accounting.** Replace the synthesis skeleton's heuristic fiber count,
  hardcoded zero trunk count, and all-cores-all-phases assumption with real `FiberGrouping` and
  `assign_cores` output, and make it reachable, labelled for what it is. This is the plan-side half
  of the compiled-program target; the codegen half stays behind the bench-gated escalation.
- **C4. Parallel dirty-skip.** Canon places the skip before each pass with no parallel exemption;
  depends on C2's happens-before for mask stability across publish/await.
- **C5. `run_fused` per-fiber window.** Mechanical; the other two paths already carry it.
- **C6. Parking tiers, uniform-core half.** Thread a phase index through `waist_barrier`, drive
  `pick_tier` from the per-phase predicted wait (needs B2). Canon's thresholds: under 100ns spin,
  to 10us spin-loop with backoff, above park.
- **C7. Core classification.** Real probe arms per platform (not sketch-provable off-platform;
  validation is per-target), pool construction consuming the classes, then the heterogeneous
  morsel sizing (P-cores larger, E-cores proportionally smaller, thread count
  `min(physical_cores, parallelisable width + 1)`) and C6's heterogeneous spin budgets.

### Band D: plan-analysis completion

- **D1.** Dulmage-Mendelsohn fine decomposition and dead-column elimination layered onto
  `block_diagonalise` (its doc already reserves the spot); feeds morsel sizing and register
  budget.
- **D2.** Matrix-chain DP fiber grouping above 10 ops, sharing the holistic feasibility predicate
  with the greedy former; `arvo-comb::matrix_chain_dp` is the substrate.
- **D3.** Spectral role reconciliation per op's Q2 ruling plus the #644 bench.
- **D4.** RCM row order as execution order: the A1-1 bench fork (both orders buildable behind the
  const grouping; a cache-locality bench on wide fan-out DAGs decides). Landing it dissolves the
  provisional `NonTopologicalRegistration` constraint, which canon marks as to-be-relaxed.
- **D5.** Column classification refinement (Input/Output/Internal for real), enabling DSE.
- **D6.** Strategy selection completed to canon: weights computed in the plan (WU count times
  column accesses), the `ChaseSteal` variant, the LIGHT_THRESHOLD bench (canon names the constant
  and never values it), and the selection actually consumed at dispatch.
- **D7.** Per-morsel generation counters (needs a storage sketch: where counters live under
  no-alloc when morsel count varies), then version stamps per domain 23.
- **D8.** Sub-byte bitpacking stride; shared-read-column strategy (canon states a per-target
  recommendation, snapshot-to-local on ARM, aligned-morsel on x86 NUMA); micro-morsel tiling
  (canon marks it ECS-scale; stays charted last in this band); the day-one intrinsics set
  (noting canon removed explicit prefetch and bans likely/unlikely in dispatch loops, so part of
  this item is verifying absences).

### Band E: adaptation completion

The three reorganisation triggers (morsel recompute from fiber EMA drift; per-phase re-selection
among pre-monomorphised variants by runtime index; record-count parameter recompute, which consumes
A2's real resource swap), predictive parking (consumes C6), stolen/idle counters, and real data
behind the adapt axes (`AdaptArena` becomes the layout its module doc describes). All
parameter-side; nothing here regroups structure, per the governing principle.

### Band F: operations and ecosystem surfaces

`PipelineResult` with per-fiber status and dependent poisoning. The `Executor` stealing extension
point (`steal_fallback` becomes a real trait surface; deterministic default stays default).
Schedule introspection (#183). Evict/inject persistence APIs wired to the persistence spine. The
facade plugin-host integration on the two WORKS sketches. Kits and providers polish. The
morsel-absolute slice accessor.

### Band G: the standing gate

The A1-8 per-arm perf gate (persistent-pool std baseline, calibrated bars, median-of-N) runs as
each band lands and stays blocking. Red arms that a later band resolves stay red per the
strict-by-design rule; they are the measurement, not the problem.

## 6. Why this order

Band A is open work whose files everything else touches. Band B is the substrate multiple bands
consume (B2 alone gates C1, C2, C6, and band E) and is the one item canon explicitly orders before
further accretion. Band C closes the gap between the shipped trunk-only parallelism and canon's
full intra-phase model, which the deviation ledger names as the largest not-built capability.
Band D completes the plan analysis whose outputs bands C and E consume. Band E reads telemetry that
exists only after B2/C6. Band F is consumer-facing surface that sits on a correct engine. Nothing
in any band waits on a mechanism not already proven feasible by a recorded sketch, with the two
named exceptions (C2's contention behaviour, C7's per-platform probes), both of which carry their
validation plan inline.

## 7. Questions canon does not answer, for op

- **Q1, the precedence ruling's record.** The standalone spec's header claims it wins over every
  older document; r7 reports op ruled 2026-07-19 that it does not blanket-win and the consolidation
  spec is the tiebreak. The ruling has no durable record in the design record. Confirm the ruling
  and record it as a canon artefact (the #667 mechanism or a dated amendment file), because the two
  texts currently contradict each other on their face.
- **Q2, spectral's role.** Canon step 7 says spectral forms trunks over a fiber-conflict graph
  (more than 5 fibers); shipped code forms trunks from block components and uses spectral for
  fiber grouping within wide blocks (more than 5 units). Rebuild to canon, bless-and-record the
  shipped role, or delegate to the #644 bench as the oracle? No ledger entry currently covers it.
- **Q3, `PlanAffecting`'s range** (#696). The sealed trait has one implementor and it is a
  run-config type, so canon's resource-swap-marks-plan-dirty trigger has no consumer-reachable
  carrier. What should plan-affecting resources be: an open marker consumers implement, an
  engine-inferred property, or a builder-declared set?
- **Q4, the "D1 install revert" phrase.** The open topic `202607192900` lists as out of scope "the
  D1 install revert, which is its own correction". No round, task, or note records an intended
  revert of the `replace_value` install. If a correction to that install is directed, it needs an
  artefact; if the phrase is a slip, it should be struck when the round closes, because as written
  it implies an op direction the record does not carry.

Deliberately not op questions, with their resolution channel: LIGHT_THRESHOLD (bench, D6), the RCM
order fork (bench, per A1-1's explicit ruling), `mid_slot` semantics (C1's builder decides against
the variant doc and records it), the meta-carrier question (A1-7, resolved in practice: the meta
pipeline shipped on the shared carrier and is WIRED).

## 8. Where r7 was wrong, and what was checked

r7's self-correcting discipline was real and most of its verified rows held up. Four substantive
corrections and one re-baseline:

1. **Spectral participation.** The audit listed "spectral trunk formation consumed by the runner"
   as ABSENT while flagging its own trace incomplete; r7's B3 carried the ABSENT framing. Checked:
   `spectral_partition` is called at `plan/steps.rs:663` inside `project_fiber_components`, which
   `compute_execution_plan` calls at `plan/mod.rs:309`, and the width-gate at `steps.rs:719-733`
   routes wide blocks through `spectral_grouping_in_block`. Spectral is live. The real finding is
   the role deviation (Q2) plus D-M's genuine absence, which is a different work item than "wire
   spectral".
2. **The cap-lifting arc is missing from r7.** A1-2 is a canon ruling ordering the arc before
   further accretion; the sketch proved the shape; a slice landed 2026-06-19; residuals #690, #649
   and `MAX_CORES` remain. No r7 band carries any of it. r8 band B1 does.
3. **Strategy selection's reachability.** r7 corrected the audit's "only ever returns Adaptive" to
   a three-way live return but still discussed it as reachable machinery. Checked: the only
   callers of `DefaultSelector::select` are in `tests/strategy.rs`; nothing in the plan or
   scheduler consumes any `Strategy` value. The whole selector is SUBSTRATE-ONLY, distinct from
   the wired `select_phase_configs`. r8 D6 words the item accordingly.
4. **Canon 59's framing.** r7 called the compiled per-core program "closer to cannot be done as
   specified today than to declined". Half right: in-language materialisation walls (ledger
   section headnote), but the ledger's op-blessing names build.rs/proc-macro codegen as a viable,
   open escalation, so the accurate status is op-blessed runtime mask with a defined bench-gated
   escalation path, not a dead requirement. The substrate stays reserved either way.
5. **Stale statuses.** r7's bands 0 and 1 describe pre-round state. As of HEAD: D2, D3, D4a done;
   D1 half done (`replace_value` installs; `replace_resource` is #696); R1 done; `thread::pool`
   deleted; the accumulator slice unified; the fn-pointer spawn retired on both tiers with the
   trampoline traced; the phase_run/phase-barrier deletions chartered in the open round. Section 3
   is the current baseline.

One methodological point r7 got right and r8 keeps as standing law for this arc: substrate
existence is established by construction sites and enclosing-function checks, never by doc
comments, type declarations, or re-exports. Every status in this document was established that way,
and the next roadmap revision owes the same.
