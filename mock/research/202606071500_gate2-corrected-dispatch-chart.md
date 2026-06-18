# GATE-2 corrected dispatch chart (waist-bounded phases, const waist in arvo)

**Date:** 2026-06-07
**Status:** chart-the-path synthesis + roadmap DRAFT (steps 2-5 done; mirror + granularity expert passes + sketches still pending). NOT yet a finalised roadmap.
**Branch:** feat/hilavitkutin-parallel-engine-gate2 (engine); arvo work on a feature branch off arvo dev.
**Oracle:** canonical consolidation spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`; grammar [[reference-engine-dispatch-grammar]].
**Supersedes:** the depth-as-phase approach (round 2b, dropped); finding `202606071400_gate2-phase-axis-course-correction.md`.

## What is actually being built (the complete shape)

The canonical GATE-2 dispatch runs each phase's column-disjoint TRUNKS on separate cores with ZERO cross-trunk synchronisation, joined only by a WAIST barrier between phases (and bridge fibers for fan-in). A trunk is a sequential critical path within a phase; a fiber inside a trunk is a linear read-after-write chain dispatched morsel-outer (intermediates cache-resident). Single-core is the degenerate: all trunks on one core, in phase order, with degenerate (one-arriver, no-op) waists. There is no separately-designed single-core path.

The materialization mechanism (op-approved Option 1, de-risked) is the const-gated flat-carrier walk: a compile-time grouping assigns each unit a `(phase, trunk)`; `run_one_trunk::<PHASE, TRUNK>` (shipped, round 2a) walks the carrier gated on `const { member of (PHASE,TRUNK) }`, and DCE collapses each monomorphisation to that trunk's member-only program. One mono per core, zero sync.

The single load-bearing correction from the drift: **the grouping's "phase" axis must be the waist-bounded phase (DAG section between waist points), NOT topological depth.** Round-1 `compute_phases` computed depth and mislabeled it phase. Depth is the within-phase ordering/level input to trunk grouping; barriers (waists) belong only at concurrency-minimum narrow points. With waist-bounded phases, a producer→consumer chain is one phase / one trunk / one fiber, dispatched morsel-outer, which keeps `morsel_outer.rs` green.

## The parts and their dependency order

1. **arvo: additive const `waist_detect_const`** (foundational; everything waits on it). An ADDITIONAL const fn in arvo-graph alongside the untouched runtime `waist_detect`. Computes the identical waist result (topo depth → level widths → occupied depths → strict-local-minimum waists → topo positions) at compile time, `#![no_std]` / no-alloc. **Generic over the `Capacity` CONTRACT (`C: Capacity`, `C::Array<T>` = `[T; N]` via `Dim<N>`), mirroring the runtime `waist_detect<C: Capacity, B>` — NOT native `[W; N]` arrays.** This is op's directive (2026-06-07) and the numeric-position-convention ([[numeric-position-convention]], #649): the `Capacity`-type form is GCE-free type-dispatch, whereas native arrays sized by a free `const N` (or `cap_size(N)`) explode the trait solver / ICE rustc's `generic_const_exprs` from generic code. `harness-the-type-system.md` (contracts over concrete) is the parent rule. The row type `W` carries the already-const bit contracts (`[const] BitAccess + [const] BitLogic + [const] BitSequence`); the const fn uses while-loops instead of `iter_set_bits`. The non-const blockers (`Capacity::filled`, `AsRef`/`AsMut` array access) are addressed by ADDING minimal const-capable access to the `Capacity` contract (additive const methods, e.g. const construct + const index get/set), which every const-context consumer benefits from. (Open: also add additive const `topo_depth` + `connected_components` so the engine stops reimplementing them. See open questions.)

2. **engine: const grouping rewrite (phase axis → waist-bounded).** Replace `compute_phases`' depth-as-phase with: compute depth (keep, as the level input), build the const adjacency from the access masks (already done in the const fold), call arvo `waist_detect_const` over it to get waist positions, derive each unit's waist-bounded phase. Keep `compute_trunks` (within-phase column-conflict union-find) but key "within-phase" on the waist-bounded phase. `is_member` / `phase_of` / `trunk_of` then carry the canonical phase. Update `tests/gate2_const_grouping.rs` (depth-based expectations → waist-based) and keep `morsel_outer.rs` green.

3. **engine: per-(phase,trunk) dispatcher + morsel-outer nesting.** The absent piece. A dispatcher that, per core (single-core: per phase in order), calls `run_one_trunk::<PHASE, TRUNK>` for its assigned trunk, morsel-outer (the morsel loop wraps the trunk walk so a trunk's fibers keep intermediates cache-resident), with the degenerate `waist_barrier` (shipped, phase_run.rs) between phases. Single-core output-equivalent to the flat walk; `#664` element_wise green, branching/accumulator RED until real N-core parallelism.

4. **engine: N-core core-pinning + real waist barrier** (the GATE-2 payoff). Pin each trunk to a pool thread; swap the degenerate waist for `thread::barrier::phase_barrier_arrive` over a `PoolFrame`; bridge fan-in. The Sketch B keystone (2.84x, proven) is this. Bench is the oracle; the red perf arms turn green here.

## Proven vs unproven

- PROVEN (committed sketches): the const-gated DCE per-trunk isolation (`202606071230`), const grouping-from-masks incl. the trunk axis (`202606071330`, `202606070950`, `202606070800`), the N-core keystone shapes (Sketch A nest `202606070300`, B trunk keystone `202606070400`, B2 2.84x `202606070500`, C bridge `202606070600`). `run_one_trunk` / `RunGatedTrunk` shipped (round 2a). `RunPhase`/`RunPipeline` + degenerate `waist_barrier` shipped (G2-0b, phase_run.rs).
- UNPROVEN (needs sketches, chart-the-path step 10):
  - **U1: const-trait-method calls inside a const fn on the pinned nightly** (arvo `waist_detect_const`). The `[const]` bounds + `impl const` ship, but calling `[const] BitSequence::trailing_zeros` / `[const] BitLogic` from a const fn body is the exact interaction with documented `const_trait_impl` ICE/normalization rough edges. PROBE FIRST.
  - **U2: const-capable access to the `Capacity` contract.** The const fn is generic over `C: Capacity` and needs to construct + index `C::Array<T>` in a const context, but `Capacity::filled` and `AsRef`/`AsMut` are non-const. The probe must establish the minimal ADDITIVE const surface on `Capacity` (e.g. a const constructor + const index get/set, or making `Dim<N>`'s `[T; N]` reachable const-generically) that lets a const fn generic over `C: Capacity` build/read its scratch arrays WITHOUT native-array/`cap_size` GCE explosion. This is the load-bearing feasibility question (combines with U1 in one probe).
  - **U3: waist-bounded phase via const `waist_detect_const` wired into the engine grouping** produces the canonical phases for representative DAGs (diamond, two-phase chain, the game-world 3-phase example) AND keeps `morsel_outer.rs` green. Sketch over real engine grouping.
  - **U4: the per-(phase,trunk) dispatcher loop** (morsel-outer, degenerate waist) is output-equivalent to the flat walk single-core. Sketch or direct TDD.

## Open questions (for the mirror + granularity expert passes)

- **Q1 (don't-reinvent scope):** add additive const versions of topo_depth + connected_components to arvo too (engine currently reimplements both), or only waist this round? `use-the-stack-not-reinvent` argues all three; blast-radius/proofing-ground argues waist-first. The engine grouping's exact needs decide it.
- **Q2 (const waist signature): RESOLVED by op (2026-06-07) → the `Capacity` contract form** (`waist_detect_const<C: Capacity, W>`), NOT native `[W; N]` arrays. Reason: native-array/`const N`/`cap_size(N)` paths explode the trait solver and ICE `generic_const_exprs` from generic code; `Capacity` type-dispatch is GCE-free (numeric-position-convention #649). The remaining feasibility (U2) is the minimal additive const surface on `Capacity` that makes generic const access work.
- **Q3 (waist metric):** arvo `waist_detect` uses LEVEL-WIDTH local minima; the spec text (:1306) phrases it as ALIVE-PATH count. The const version must match arvo's runtime version (consistency); whether arvo's level-width diverges from spec intent is a separate arvo audit, not this arc.
- **Q4 (per-core program communication):** how N-core hands each pool thread its `(PHASE,TRUNK)` mono at runtime (const generics are compile-time). r4's answer: const grouping fold → per-trunk monos indexed by trunk id. Belongs to part 4 (N-core), chart at that step.

## Roadmap draft (per-step, with proof status)

- R0 (PROBE): sketch U1+U2 in arvo — a minimal const fn GENERIC OVER `C: Capacity` that constructs + indexes `C::Array<T>` and calls const bit-contract methods, on the pinned nightly, finding the minimal ADDITIVE const surface `Capacity` needs (no native-array/`cap_size` GCE). Gate: compiles + const-evaluates over `Dim<N>`, no ICE, no trait-solver blowup. If it walls → expert review + AskUserQuestion (chart-the-path step 11).
- R1 (arvo round): add `waist_detect_const` (additive; runtime untouched), TDD against the runtime `waist_detect` for identical results on fixture DAGs. PR --base dev, reviewer, merge. Resolves U1/U2.
- R2 (engine round): rewrite the const grouping phase axis to waist-bounded via `waist_detect_const`; update gate2_const_grouping.rs; keep morsel_outer.rs green. Resolves U3.
- R3 (engine round): per-(phase,trunk) dispatcher loop + morsel-outer nesting + degenerate waist; single-core output-equiv. Resolves U4.
- R4 (engine round, the payoff): N-core core-pinning + real waist barrier + bridge; bench the red perf arms to green. (Q4 resolved here.)

Sketches (step 10) attach to R0/U1-U2 first, then U3, then U4. R4's keystone is already proven (Sketch B/B2/C).

## Mirror-pass findings (step 6) + roadmap update (step 7)

A neutral critical-mirror expert (feature-dev:code-architect, 2026-06-07) mirrored the draft above against the canonical spec. Findings, and how the roadmap absorbs them:

1. **Trunk MEMBERSHIP changes, not just its label.** `compute_trunks` (grouping.rs:175-224) union-finds same-phase column-conflicting pairs; `same_phase` is `phase[a]==phase[b]`. When `phase` flips from depth to waist-bounded, the set of same-phase pairs changes, so the trunk OUTPUT changes (e.g. a linear chain across many depths collapses to one waist-bounded phase, so its units become same-phase and can union into one trunk). The union-find logic is unchanged; its result is not. R2's test must assert the trunk outputs (group_n / PHASE / TRUNK) under waist-bounded phases, not just phases.

2. **Canonical execution-strategy menu (spec :768-779) was under-scoped.** The draft's 4 parts cover multi-trunk + the const mechanism but omit: branches (chasers within trunk morsel scope, :743-744), bridges (fan-in, :745-746), pipelined-phase overlap (AtomicUsize progress counters, total ≈ max(phases)+fill not sum, :772-774), head+tail single-trunk convergence (:770-771). Plus plan-stage pieces: column-count strategy selection (FUSE/SPLIT thresholds :788-791, spec step 7 strategy-per-phase), shared-read-column handling between trunks (:781-787 snapshot-to-local vs aligned-morsel-sync), and the spec's two-branch trunk identification (arvo-spectral for >5 fibers, connected-components ≤5, :752-753 — shipped `compute_trunks` is union-find ≈ connected-components only).

3. **GATE-2 scope boundary (the chart now states it explicitly).** GATE-2 (#662) = the N-core parallel engine that degenerates to single-core, with the #664 branching+accumulator perf arms as the parity oracle. The canonical strategies sequence into GATE-2 sub-steps (correctness/parity first, perf refinements after), NOT a later gate — the standing mandate is the COMPLETE canonical engine, no MVP. Spectral trunk identification is the one explicit perf-only refinement deferrable behind a bench (it changes trunk shape for >5-fiber phases for cache reasons, not correctness; union-find/connected-components is the correctness baseline).

### Updated parts + roadmap (supersedes the draft R0-R4 above)

- **R0 (PROBE, unchanged):** const-over-`Capacity` feasibility (U1+U2) on the pinned nightly.
- **R1 (arvo round):** additive const `waist_detect_const` generic over `C: Capacity` (runtime untouched), TDD == runtime `waist_detect` on fixtures. Resolves U1/U2. (Waist metric: match arvo's runtime level-width definition for const/runtime consistency, Q3; any spec-intent divergence is a separate arvo audit.)
- **R2 (engine round):** const grouping phase axis depth→waist-bounded via `waist_detect_const`. Test asserts trunk OUTPUTS (per finding 1), updates gate2_const_grouping.rs, keeps morsel_outer.rs green. Resolves U3.
- **R3 (engine round):** per-(phase,trunk) dispatcher loop, morsel-outer nesting (branches are subsumed: a branch shares columns with its trunk → same conflict component → same trunk → run_one_trunk runs it within the trunk's morsel scope; VERIFY in the R3 sketch). New test targets the dispatcher directly (morsel_outer.rs alone is insufficient per finding 2). Single-core output-equiv. Resolves U4.
- **R4 (engine, N-core parallel — the payoff, decomposed):**
  - R4a multi-trunk core-pinning + real waist barrier (`phase_barrier_arrive` over PoolFrame; Sketch B/B2 proven 2.84x). The #664 branching arm turns green here.
  - R4b bridge fan-in fibers (Sketch C proven).
  - R4c shared-read-column handling between trunks (snapshot-to-local copy on phase transition; spec :781-787). UNPROVEN — sketch.
  - R4d head+tail single-trunk convergence (spec :770-771; needs FiberCons #670, shipped). UNPROVEN for the const mechanism — sketch.
  - R4e pipelined-phase overlap via progress counters (spec :772-774). UNPROVEN — sketch; the accumulator perf arm parity likely needs this + accumulator handling.
  - R4f column-count-driven strategy selection per phase (spec step 7, :788-791) wiring multi-trunk-vs-single-trunk choice.
- **Later perf bench (not correctness):** spectral trunk identification for >5-fiber phases (#644-style bench vs union-find).

### Newly unproven (for granularity pass + sketches)

U5 (R4c shared-read snapshot), U6 (R4d head+tail under the const mechanism), U7 (R4e pipelined overlap + accumulator parity), U8 (R3 branch-subsumption-by-trunk holds for a branchy DAG). R4a/R4b keystones already proven (Sketch B/B2/C). The granularity pass (step 8) should split R2/R3/R4a-f where a sub-step hides a hard problem, and confirm the GATE-2-scope sequencing.

## FINALISED roadmap (steps 8-9) — supersedes all drafts above

A neutral granularity expert (feature-dev:code-architect, 2026-06-07) found R1/R2/R3 each bundle independent hard sub-problems and three steps lack a sketch. Finalised, split, ordered, each step with the ONE thing its sketch must prove:

- **R0 (PROBE sketch):** a const fn generic over `C: Capacity` that constructs + indexes `C::Array<T>` AND runs a tight loop calling `[const] BitSequence::trailing_zeros` + `[const] BitAccess::with_bit_cleared` (the `iter_set_bits` replacement), on nightly-2026-05-28. Proves: minimal additive `Capacity` const surface + const-trait calls in a const fn body, no ICE/solver-blowup. (The loop-body const-trait calls are the real risk, not just array fill/read.)
- **R1a (arvo-tensor round):** add minimal additive const surface to `Capacity` (const constructor + const index get/set); `Dim<N>`'s `[T; N]` impl satisfies it; runtime impls unaffected. Sketch = R0. Foundational; also unblocks future `topo_depth_const`/`components_const` (Q1).
- **R1b (arvo-graph round):** port `waist_detect` → additive const `waist_detect_const<C: Capacity, W>` over the R1a surface (runtime untouched). TDD byte-identical to runtime `waist_detect` on diamond / straight-chain / two-phase fixtures. Depends on R1a compiling.
- **R2-pre (SKETCH, gates R2):** a const fn builds a `BitMatrix<B, C>` (or the adjacency `waist_detect_const` consumes) from two `[AccessMask<CS>; N]` arrays. Proves U3's load-bearing unknown (const BitMatrix construction), NOT covered by R0. If it needs arvo-bitmask const additions → **R1c (arvo-bitmask round)** before R2.
- **R2 (engine round):** const grouping phase axis depth→waist-bounded: build const adjacency from `BundleMasks`, call `waist_detect_const`, derive waist-bounded phase, re-key `compute_trunks` (logic unchanged, inputs change). Test asserts trunk OUTPUTS (group_n/phase_of/trunk_of/is_member) rederived for fixtures (membership changes, finding 1); update gate2_const_grouping.rs; morsel_outer.rs stays green.
- **R3a (engine round + sketch):** wire `run_one_trunk::<PHASE,TRUNK>` into a phase-order const-generic outer loop (reuse RunPhase/RunPipeline cons-recursion shape); single-core, full-range, output-equivalent to the flat `RunFiber` walk on the 3-phase fixture. Sketch proves output-equivalence.
- **R3b (engine round + sketch):** morsel-outer nesting around each trunk walk + branch-subsumption-by-trunk (U8) on a branchy DAG. morsel_outer.rs green; new dispatcher-targeted test.
- **R4a:** multi-trunk core-pin + real `phase_barrier_arrive` waist (Sketch B/B2 proven). #664 branching arm → green.
- **R4b:** bridge fan-in (Sketch C proven).
- **R4c (sketch U5):** shared-read-column snapshot-to-local on phase transition (spec :781-787).
- **R4d (sketch U6):** head+tail single-trunk convergence (spec :770-771; FiberCons #670 shipped).
- **R4e (sketch U7):** pipelined-phase overlap via AtomicUsize progress counters (spec :772-774) + accumulator parity (the accumulator perf arm likely needs this). One sketch covers overlap+accum before any further split.
- **R4f:** column-count-driven per-phase strategy selection (spec step 7 :788-791). Plan-stage; confirm GATE-2 vs post-R4 (does not affect parallel-dispatch correctness).
- **Later perf bench:** spectral trunk identification for >5-fiber phases vs union-find (correctness baseline = union-find).

**Order:** R0 → R1a → R1b → R2-pre(+R1c?) → R2 → R3a → R3b → R4a → R4b → R4c → R4d → R4e → (R4f). Sketches gating a round: R0 (R1a/R1b), R2-pre (R2), R3a/R3b own sketches, R4c/R4d/R4e own sketches. R4a/R4b proven.

**Sketch plan (step 10), in order:** S1=R0 (const-Capacity + const-trait-loop probe), S2=R2-pre (const BitMatrix build), S3=R3a (dispatcher output-equiv), S4=U8 (branch subsumption), then R4c/R4d/R4e sketches as those rounds approach. CHART FINALISED; build is mechanical from here, each round gated by its sketch.

### Sketch results (step 10, running)

- **S1 = R0: PROVEN (WORKS), 2026-06-07.** `arvo/mock/research/sketches/202606071600_const-capacity-waist-probe/` (committed `97a9e1b` on arvo branch `feat/arvo-const-capacity-waist`). A `const fn` generic over a `Capacity`-style GAT trait (`const trait ConstCap` with const `filled`/`get`/`set`, replacing non-const `AsRef`/`AsMut`) constructs + indexes `C::Array<T>` AND calls `[const]` row-contract methods (`trailing_zeros`/`with_bit_cleared`/`is_zero`, the iterator-free set-bit scan) in the const body, const-evaluated at N=4 AND N=8 — no GCE, no ICE, no solver blowup. Syntax: `const trait` KEYWORD (not `#[const_trait]` attribute); `[const]` bounds; `impl const`. Proves U1+U2 → R1a (add the additive const surface to arvo-tensor `Capacity`) and R1b (port `waist_detect`) are mechanically sound. The waist algorithm's extra passes (level widths / occupied / strict-local-minima) use the same const moves, so carry no new toolchain risk.
- S2 (R2-pre const BitMatrix build), S3 (R3a dispatcher output-equiv), S4 (U8 branch subsumption): pending, sketched just-in-time before R2/R3.
