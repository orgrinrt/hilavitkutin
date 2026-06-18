# GATE-2 re-chart: trunk-sectioning, then core-pinning (not fiber/record partition)

**Date:** 2026-06-07
**Scope:** the GATE-2 (parallel engine) portion of the engine-completion arc. Supersedes the Phase E framing in roadmap r1 (`202606061100_engine-completion-roadmap-draft.md` section 5) and r2 (`202606081500_engine-roadmap-r2.md` section 4) where they describe how parallelism is achieved.
**Source:** op correction (2026-06-07, live) plus re-grounding on the canonical consolidation spec (`mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`). Design is the oracle; this doc fixes a drift the roadmaps carried.

## Why this doc exists

The roadmaps drifted on the single most important GATE-2 question: what is the unit of parallelism. r1/r2 framed the parallel entry as "replace `RecordRange::Full` with real record ranges" (E1) and "workers walk phases" (E2), which reads as distributing a fiber's records or morsels across cores. That is not the canonical design. op stated it directly: the parallelism is **isolated trunks per core, bridged only by actual bridge branches/fibers**, and it cannot be attempted before trunks are established and waists are clearly sectioned. The spec says the same in plain words; the roadmaps simply did not encode it as the load-bearing mechanism. This doc re-grounds the model, maps what current code salvages, and states the corrected step sequence so the roadmap rewrite and the proving sketches descend from the oracle rather than from the drifted E-steps.

## The canonical parallelism model (what the spec actually says)

The execution hierarchy is `pipeline -> core -> phase <-> waist -> trunk -> fiber <-> branch <-> bridge -> morsel` (`:74`). The parallelism lives at the trunk level, and the spec is unambiguous about it.

A **trunk** is a "column-disjoint maximal path within a phase. Contains fibers, branches, bridges. Trunks share NO write columns with other trunks. Zero sync between trunks." (`:86`, restated `:741-742`, `:952`). A **waist** is the "narrow bottleneck in the DAG where concurrent path count drops to minimum. Defines phase boundaries." (`:87`, `:1306-1311`). A **phase** is the "wide DAG section between waists. Phases execute with pipeline parallelism." (`:88`). A **bridge** "reads from multiple trunks (fan-in). Runs after parent trunks reach required record range." (`:85`, `:745-746`). A **branch** is a side path within a trunk that shares some columns and chases within the trunk's morsel scope (`:84`, `:743-744`). A **core** is one pool thread; trunks are core-pinned (`:89`).

The execution strategies (`:768-777`) state the mechanism outright:

- **Multi-trunk phases: trunks run in parallel on separate cores.** This is the primary parallelism. Because sibling trunks share no write columns, there is **zero sync between them** within the phase (`:742`). No barrier, no atomic handshake, no shared-cell coordination during a phase: the disjointness is the license.
- **Single-trunk phases: head+tail convergence** (2 threads from opposite ends of a commutative fiber, `:770`, `:1838-1844`). This is the *only* place a single fiber's records split across cores, and it is capped at 2 (head + tail), not an N-way record partition.
- **Pipelined phases overlap** via `AtomicUsize` progress counters: phase N+1 starts when N produces one morsel (`:772-774`, `:1847-1854`).
- **Core-pinned trunks:** each trunk assigned to a specific pool thread at plan time for warm L1; leftover threads do convergence, then branches, then bridges in priority order (`:775-777`, `:1829-1837`).

Cross-trunk synchronisation happens at exactly two places, both explicit: the **waist** (a phase barrier between phases) and the **bridge** (a fan-in fiber that runs after its parent trunks reach the required record range). Nothing else crosses trunks. This is the inversion of the drifted model: parallelism is not "split the work of a fiber across cores," it is "run the column-disjoint trunks of a phase on different cores, untouched by each other, and join them only at waists and bridges."

The plan algorithm that produces this is already spec'd as seven plan-time steps (`:748-757`): build the DAG, detect phases via waist analysis, identify trunks per phase (spectral for >5 fibers else connected-components on column-disjoint paths), assign branches and bridges, form fibers within each trunk, size morsels per fiber, select per-phase strategy. The trunk skeleton is the critical path by upward rank (`:1292-1298`): highest-ranked WU is the trunk root, the trunk follows the heaviest successor, everything else is a branch or a bridge.

## Truth of the current engine (what is shipped vs ignored)

The plan **computes** the full trunk/waist/fiber structure and stores it. Shipped in `plan/steps.rs` + `plan/{phase,trunk,fiber}.rs`: `compute_waists` produces `PhaseBoundaries`; `block_diagonalise` produces a `BlockPartition` (connected-component blocks = column-disjoint trunk candidates); `phase_trunk_counts`, `project_fiber_components`, `group_fibers`, `fiber_grouping_from_trunks`, `compute_upward_rank_and_dirty` all exist. `PlanHandle` carries `phase_count` / `trunk_count` / `phases_id` / `trunks_id` / `morsel_sizes_id`.

Dispatch **ignores all of it.** `RunFiber` (`dispatch/fiber_run.rs`) walks the flat `WuVals` cons-list in registration/topo order, phase- and trunk-agnostic (its own doc: "runs execute over a value-carrying unit list, in cons-list order"). `Scheduler::run` drives one whole-program `RunFiber` walk (morsel-outer, or unit-outer when an accumulator is present). `synthesise_core_programs` round-robins **fibers** to cores with `RecordRange::Full` (the wrong unit, and a placeholder per its own doc comment).

So the GATE-1 dispatch is the entire DAG flattened into one sequential walk that crosses every phase and merges every would-be-parallel trunk into one thread. It is correct at one core precisely because at one core there is no parallelism to express, so collapsing everything to one walk is output-equivalent.

## The whole-program carrier is the degenerate single-trunk collapse (the salvage)

op's read is correct and precise: the flat carrier is a trunk in the load-bearing sense. Not literally (it spans all phases and all trunks), but the walk machinery (`RunFiber`: project `EngineCtx` -> `invoke_wu_in_fiber` -> morsel window -> devirt) **is exactly the body a single trunk's fiber-walk runs.** At one core with no sectioning, the whole program executing as one sequential walk is observationally identical to "one phase, one trunk, all fibers." The carrier mimics a trunk because its execution body *is* the trunk's execution body, just not yet wrapped in the trunk/waist nesting.

The salvage map:

- **Keep verbatim (the innermost level):** `RunFiber` / `run_gated`, `EngineCtx` projection, `invoke_wu_in_fiber`, the morsel loop, the dirty gate, devirt. This is what each fiber of each trunk runs. Nothing about it changes.
- **Add the two nesting levels, both already sketch-proven in isolation:** `RunTrunk` over `FiberCons`/`FiberNil` (sketch `202606061400`, a trunk = a type-level list of fibers, delegating to `RunFiber`, zero blr; this is task #670) wraps `RunFiber`. A phase-sub-carrier walk + waist barrier (sketch `202606081600`, phases as separate type-level sub-carriers walked sequentially with an `AtomicUsize` barrier between, zero blr) wraps trunks. The flat carrier becomes the degenerate instance: one phase, one trunk, one fiber = the whole `WuCons`.
- **Change (the real new work):** the carrier *construction*. `build()` today emits one flat `WuVals`; it must emit the phase/trunk/fiber-nested carrier `PhaseCons<TrunkCons<FiberCons<WuCons>>>`, built from the plan structures that already exist and are currently ignored by dispatch (`PhaseBoundaries`, `BlockPartition`, `group_fibers`, `project_fiber_components`). Wiring those plan outputs into the carrier shape is the act of "establishing trunks and sectioning waists" in the dispatch path.

Almost all GATE-1 dispatch code survives. What was missing is that the flat carrier never consumed the trunk/waist structure the plan already produces.

## The corrected step sequence (the order op pinned)

Trunks must be established and waists sectioned in the dispatch path **before** any core-pinning. Two stages.

**Stage G2-0 (single-core, output-equivalent): make dispatch consume the trunk/waist sectioning.** Build the nested `PhaseCons<TrunkCons<FiberCons<WuCons>>>` carrier from the plan's `PhaseBoundaries` + `BlockPartition` + fiber grouping. Walk it sequentially: `RunPhase` over phases (waist barrier between, degenerate at one thread), `RunTrunk` over trunks within a phase (sequential at one thread), `RunFiber` over fibers within a trunk (unchanged). Still one core, output-equivalent to today's flat walk, so it validates against the GATE-1 oracle as a refactor and the `#664` perf gate must not regress. This is where trunks get established and waists sectioned. It is the opening piece of GATE-2 (GATE-1 already merged as a complete one-PR milestone; this step's purpose is to enable parallelism, so it belongs to the parallelism gate). Composes the two proven sketches (`061400` + `081600`) but proves, for the first time, the full three-level nest built from the *real* plan structures rather than hand-built sub-carriers.

**Stage G2-N (multi-core): core-pin the trunks.** Within each phase, assign each column-disjoint trunk sub-carrier to a core (the core-pinned-trunk assignment, `:1829-1837`). Trunks run concurrently with **zero sync** (they share no write columns, `:742`); the only synchronisation is the **waist barrier** between phases (the shipped `phase_barrier_arrive` / `phase_barrier_reset`) and the **bridge** fibers that fan in after parent trunks reach the required record range. Single-trunk phases use head+tail convergence (E4b, sketch `202606062200`). Validate N-core output bit-identical to the 1-core run (E6 oracle, `:` synthesis 2.4), but partitioned by trunk, not by record range. This is the real GATE-2.

The earlier E-steps slot into this corrected spine rather than replacing it: E2 (spawn-once pool) is the worker substrate that runs each core's pinned trunk; E3 (barrier generation/sense-bit fix) hardens the waist barrier for the multi-episode case; E4 (meta-WU pipelining) overlaps phases via progress counters once trunks run concurrently; E4b (head+tail) is the single-trunk-phase strategy; E5a/E5b (P/E affinity) is the core-pinning policy; E6 is the oracle. E7 (dirty-skip) and E8 (adapt) are orthogonal runtime passes that ride on top at any core count.

## What the drift was, concretely

r1 section 5 frames E1 as "replace `RecordRange::Full` in `synthesise_core_programs` with computed half-open ranges from the morsel-size formula" and E2 as workers that "walk phases." Both describe record/morsel-range distribution, which is the wrong unit: the spec's parallel unit is the core-pinned column-disjoint trunk, and record-range splitting appears only as the 2-way head+tail convergence inside a *single-trunk* phase (`:770`). r2 section 4 says "E1..E8 stand as r1" and reframed only adaptivity; it never corrected the unit-of-parallelism question. The drift reached the code: `synthesise_core_programs` round-robins fibers (not trunks) with `RecordRange::Full`. And #670 (the `FiberCons` nesting) is mis-scoped in the task tree as "post-GATE-1 morsel-locality polish" when it is in fact the first structural step (`RunTrunk` needs `FiberCons`) of the trunk-sectioning prerequisite, on the GATE-2 critical path.

This is a textbook case of `canonical-design-outranks-intermediate-rounds.md`: the intermediate roadmap rounds (r1, r2) reasoned from the current flat-carrier code and from each other, and drifted off the spec's trunk-per-core statement. The fix is to re-derive from the spec (this doc) and rewrite the strayed roadmap section.

## Sketches the corrected model needs (Step-9/10 plan)

`061400` proved `RunTrunk`/`FiberCons` in isolation; `081600` proved phase-sub-carriers + barrier in isolation; neither proved the full nest, nor proved it built from the real plan structures, nor proved real column-disjoint trunks running concurrently with zero sync on the shipped barrier. The genuinely-unproven integration:

1. **Sketch A (G2-0 nest):** build `PhaseCons<TrunkCons<FiberCons<WuCons>>>` and walk it single-core via `RunPhase` -> `RunTrunk` -> `RunFiber`; assert output-equivalent to the flat walk over the same WUs and **zero blr** in the nested walk. Pins: the three-level nest composes and devirts. Leeway: the exact `PhaseCons`/`TrunkCons` type shapes and witness-threading may differ; the proof is "the full nest devirts and is output-equivalent."
2. **Sketch B (G2-N trunk-per-core, the keystone):** two column-disjoint trunks (disjoint write columns) within one phase, each a real `TrunkCons<FiberCons<WuCons>>`, run on two real worker threads **with no synchronisation between them** (the disjointness is the proof), a waist barrier (shipped `phase_barrier_arrive`) before a second phase, output bit-identical to the 1-core run, zero blr in each trunk walk. Pins: column-disjoint trunks parallelise with zero sync and the only cross-trunk join is the waist. This replaces the earlier (wrong) record-partition keystone.
3. **Sketch C (bridge fan-in):** a bridge fiber reading from two parent trunks' write columns, running after both reach the required record range (`:745-746`); correctness of the fan-in join. Pins: the bridge is the explicit cross-trunk data path and it composes with the nest.

Sketch A gates G2-0; B and C gate G2-N. If any walls (for example, a `PhaseCons`/`TrunkCons` nest that cannot infer its witnesses, or a trunk-disjointness the type system cannot express without record-range typing), that is a roadmap-changing finding to surface to op, not to route around.

## See also

`canonical-design-outranks-intermediate-rounds.md` (why the roadmap drifted and which design wins), `design-is-the-oracle.md`, roadmap r1 section 5 + r2 section 4 (the drifted Phase E framing this doc corrects), sketches `202606061400` (RunTrunk/FiberCons), `202606081600` (phase sub-carrier + barrier), `202606062200` (head+tail), `202606062500` (N-vs-1 oracle, to be re-grounded to trunk partition).
