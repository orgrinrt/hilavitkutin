# Granular roadmap + sketch plan (chart-the-path phases 8-10)

**Date:** 2026-06-19
**Status:** completes the critical chart-the-path: granularity expert pass (phase 8), finalised granular roadmap (phase 9), and the per-step sketch plan (phase 10 specs; sketches land per tick, each gating its slice).
**Builds on:** `202606201700_arc-audit-and-updated-roadmap.md` (audit + resolved phase sequence), `202606201400_per-fiber-morsel-blueprint.md` (the per-fiber morsel slices).

## Two audit premises resolved against source (no sketch needed)

- **A3 step-ordering (CONFIRMED real).** `compute_fiber_morsel_windows` runs at
  `plan/mod.rs:395` (step 9), `classify_columns` at `:409` (step 11). The L1
  window formula needs per-fiber write-byte sizes, which `classify_columns`
  produces, so they are NOT available when morsel windows are sized. A3 must
  either move column classification before sizing, or compute per-fiber write
  bytes in an earlier step. This reorder is part of A3, not a separate sketch.
- **D-act mutation safety (CONFIRMED).** `morsel_windows` is read only at build
  (`derive_phase_dispatch_order` populates `FiberDispatch.morsel_size`, and
  `store_column` copies the pool); never per-frame in `run`. So the adapt
  re-chunk actuation cannot just mutate `plan.morsel_windows`; it must refresh the
  `FiberDispatch` descriptors (re-run the derive, or write `morsel_size` on the
  descriptors directly). D-act-1 is therefore a descriptor-refresh path, not a
  field write.

## Granularity splits (phase 8 findings, applied)

Each leaf is small enough that one sketch proves its premise.

- **Phase A (per-fiber morsel).** A1 (descriptor field) DONE (PR #154). A2 splits:
  **A2a** sketch proving `RunGatedTrunk::run_trunk` can be driven per-fiber with
  distinct `MorselRange` values on a single-phase carrier without breaking the
  phase-gated const-DCE (the const-monomorphised carrier does not expose per-fiber
  enumeration from outside `RunGatedTrunk`; `scheduler/mod.rs` ~1522 passes one
  `MorselRange` to the whole carrier). **A2b** the loop inversion in `run` once A2a
  passes. **A3** the L1 formula (with the step reorder above). **A4** GATE-2
  parallel per-fiber sizing (deferred until A1-A3 stable).
- **Phase G2C (real GATE-2).** **G2C-1** sketch: head+tail as a const-selectable
  third dispatch mode without a per-dispatch runtime branch (the const-grouping
  mechanism exists to eliminate that branch). **G2C-2** wire head+tail into
  `worker_main`. **G2C-3** sketch: phase-overlap progress-counter protocol, proving
  a downstream worker's Acquire load on an upstream-published `ProgressCounter`
  composes correctly with the `waist_barrier` Release fence without a full barrier
  per morsel (`PoolFrame` already carries `progress_slots`). **G2C-4** wire
  phase-overlap into `run_core_phase`.
- **Phase B (plan-analysis).** **B1a** cost-function definition (is the DP cost
  from the step-6 Laplacian or a per-fiber weight) + **B1b** DP grouping. **B2**
  sketch (mandatory): spectral cluster labels thread into `FiberGrouping`
  consistently with `Trunk` (else B2 silently changes grouping semantics). **B3a**
  confirm Dulmage-Mendelsohn substrate availability in arvo-sparse (cross-repo PR
  if absent) + **B3b** integration + dead-column elimination.
- **Phase C (RCM-row dispatch).** **C1** sketch: RCM-ordered `topo_order` drives
  carrier-position const-dispatch correctly (the carrier walks by carrier
  position, NOT by `topo_order` index, `steps.rs:335-338` still calls RCM
  arena-only). **C2** relax `NonTopologicalRegistration` once C1 proves. **C3**
  retire the arena-only doc framing.
- **Phase D (adapt actuation).** **D-act-1** the descriptor-refresh path (above) +
  **D-act-2** wire the re-chunk on `adapt_reconfigure`. Remaining D axes
  (fiber_ema, active_units, parallel phase_ema, AdaptArena, per-morsel gen
  counters, strategy reselect) are fine at listed granularity.

## Sketch plan (phase 10): per unproven step, hypothesis + leeway

Each sketch lands in `mock/research/sketches/<ts>_<topic>/` per
`cl-claim-sketch-discipline.md` (hypothesis, real code vs real crates, outcome
WORKS / FAILS WITH <error> / INCONCLUSIVE, the proven shape). Leeway noted = how
exact the proven shape must be.

1. **A2a** (gates A2, the next slice). Prove: a fiber-outer loop in `run` calls the
   per-trunk monomorphised dispatch once per fiber per window with distinct
   `MorselRange`, no per-record indirect call, const-DCE intact. Leeway: some-shape
   (either `RunGatedTrunk` accepts a per-fiber range selector, OR a per-fiber outer
   wrapper drives it; prove one compiles + stays devirtualised via the asm gate).
2. **C1**. Prove: RCM-ordered `topo_order` + carrier-position const-dispatch yields
   dependency-correct output without an added indirection layer. Leeway: exact
   (the ordering contract is precise).
3. **G2C-1**. Prove: head+tail third mode is const-selectable, no runtime branch on
   the per-record path. Leeway: some-shape (gating mechanism family).
4. **G2C-3**. Prove: progress-counter Acquire/Release composes with `waist_barrier`
   for happens-before without per-morsel full barrier. Leeway: exact (memory
   ordering is a correctness contract).
5. **B2**. Prove: spectral labels map to `FiberGrouping` consistently with the
   `Trunk`/`FiberGrouping` types. Leeway: some-shape.
6. **A3**. Prove: the step reorder (classify columns before sizing) leaves the rest
   of the step chain correct, and the L1 formula computes from the per-fiber write
   bytes. Leeway: exact (formula is spec domain 12).

## Cross-cutting discipline: ASM-emission contract tests

Add (op 2026-06-19): the existing `asm_gate` (`mock/benches/asm_gate_fixtures/` +
`cargo run --bin asm_gate`, round 202606111600) disassembles four `#[no_mangle]`
fixtures and asserts zero indirect calls. Generalize it into a standing
per-typestate ASM-emission contract harness: for each codegen-load-bearing
typestate shape, a fixture exports a stable named symbol, the harness disassembles
it (nm/objdump/otool, the existing fallback chain), scopes to that symbol's body,
and asserts the instructions we EXPECT are present and the ones we DON'T are
absent. General, not fragile exact-match: assert properties (no indirect call /
no call to a named runtime helper / a specific inlined op is present), scoped to
the symbol region, so a typestate change that regresses codegen fails the test
with the REASON, where the bench would only show the symptom. Each new
typestate-bearing slice (per-fiber morsel windowing, RCM-row dispatch, head+tail,
phase-overlap) adds its fixture + present/absent assertions. This rides alongside
the perf gate, both standing red oracles until the codegen matches. Backlog note
added to `hilavitkutin` crate BACKLOG. Tracked as a cross-cutting phase, executed
incrementally as each slice lands its fixture.

## Authoritative sequence (unchanged from 202606201700, now granular)

A1 (done) -> A2a sketch -> A2b -> A3 -> A4 -> G2C-1 sketch -> G2C-2 -> G2C-3 sketch
-> G2C-4 -> B1a -> B1b -> B2 sketch -> B3a -> B3b -> C1 sketch -> C2 -> C3 ->
D-act-1 -> D-act-2 -> remaining D axes -> E consumer surfaces -> F/G/H. The
ASM-emission contract fixtures land with each typestate slice throughout.

## Status and resolutions (2026-07-02 canon-alignment pass)

Read-only mirror of this roadmap against the consolidation spec plus the source
state landed since 2026-06-20. No canon conflict found in the sequence or the
phase framings. Statuses, solved questions, and detail fill-ins below; the
sequence above stands.

### Landed since this roadmap was written

- **A2a DONE.** Sketch shipped (PR #157, WORKS-by-derivation) and the proven
  shape landed as source: `Scheduler::run_one_trunk_windowed`
  (`scheduler/mod.rs:1494`, commit `aaa6097a`), with the two sketch refinements
  (direct per-fiber entry; `fiber_mask` composed with dirty-skip). **A2b remains**:
  no caller inside `run` yet; the loop inversion (fiber-outer/morsel-inner keyed
  by `FiberDispatch.morsel_size`), the `fiber_mask` wiring, and the first
  per-fiber ASM fixture are the open slice.
- **A3 substrate DONE, formula wiring unblocked.** A3 sketch (PR #159 WORKS), A3a
  `StoreElemBytes`/`StoreSizes`/`store_sizes` (commit `9e80d835`), the A3
  re-chart onto the canonical R5 resource model (commit `272dee96`), and the
  collection-footprint substrate `CollectionBytes`/`ResourceFootprint` +
  `#[derive(ResourceFootprint)]` (#163/#164, merged) are all in. **A3b (the L1
  formula + the step reorder) is now unblocked**: the storage bench (both runs)
  decided the layout is a one-record blob, so the Resource contribution to
  per-fiber write bytes is the blob stride plus the `CollectionBytes` term for
  collection members, over blob strides (NOT a decomposed per-member column set).
  This is unaffected by the hybrid addressing choice, since the erased static-shape
  descriptor changes value addressing, not the per-store size fold.
- **Inserted arc: resource-storage drift-fix (round 202606210600), layout DECIDED.**
  Not in this roadmap's original scope (the round opened after it). The
  pressure-test + six-variant bench (both runs, 2026-07-02) decided the
  `Resource<T>` value layout: a one-record blob with a scalar stack-snapshot
  before the morsel loop and live-streamed (never snapshot-copied) `Seq`/`Map`
  members, plus the noalias provenance invariant. Decomposed (V2) and shape-bound
  (V3) layouts are rejected (lose axes B/C/E). The drift-fix is additive on the
  shipped `DrainStores` blob (add the snapshot + live-stream + erased static-shape
  addressing), not a rewrite, and lands before or with A3b. **The fork is CLOSED:
  op picked the hybrid (global-capable) on 2026-07-02**, so value access routes
  through an erased static-shape descriptor (parity in-process per the bench, buys
  plugin/wasm resource crossing) uniformly for every resource. See
  `mock/research/202606210600_storage-bench-findings.md` (run-2 confirmation
  section) and `mock/design_rounds/202606210600_topic.hybrid-decision.md`.

### Open questions solved (no sketch needed, resolved from canon + source)

- **B1a SOLVED.** The DP cost is neither the step-6 Laplacian nor a per-fiber
  weight. Spec Step 8 states it outright:
  `cost(i,j) = record_count x sum of size_of::<T_k>() over the union columns of
  ops i..j`, using type-native strides (R3) and the co-located arena model; the
  cost IS the memory bandwidth of one data walk through the candidate fiber's
  arena. Feasibility = the full domain-14 holistic check (register file, L1
  write budget, L1+L2 total, no fan-in, no pipeline breaker). Greedy mode for
  <=10 ops, DP for >10. The Laplacian edge weight (sum of shared-column bytes
  between two FIBERS) belongs to step-7 spectral trunk formation, whose nodes
  are fibers, and which runs AFTER fiber grouping. Canon subtlety worth pinning:
  the spec says step 8 runs before step 7 despite the numbering ("After fiber
  grouping (step 8, which runs first...)"); a number-order reading inverts it.
- **B3a SOLVED.** `arvo-sparse` already ships the Dulmage-Mendelsohn surface:
  `DulmageMendelsohn<C: Capacity>`, `dulmage_mendelsohn`,
  `dulmage_mendelsohn_via`, `classification_to_mask` (`arvo-sparse/src/dm.rs`),
  Capacity-parameterized to match the post-#652 PlanDims shape. No cross-repo PR.
  B3b reduces to pure integration + dead-column elimination.
- **B1b substrate confirmed.** `arvo-comb` ships
  `matrix_chain_dp<N: Capacity, W>(cost: impl Fn(USize, USize) -> W, feasible:
  impl Pred2<USize, USize>) -> (W, Array<USize, N>)` plus `greedy_group`.
  Integration = supply the spec cost closure from A3a's `store_sizes` + the
  access masks; both DP and greedy modes have their substrate.

### Detail fill-ins for the pending sketches (narrowing scope, from canon + source)

- **G2C-1 (head+tail const-selection).** The selector input already exists as a
  compile-time constant: `COMMUTATIVE: Bool` on the WU contract
  (`hilavitkutin-api/src/work_unit.rs:84`), threaded into `PlanInputs`
  (`plan/project.rs:154`), and `plan/fiber.rs:150` already computes head+tail
  ELIGIBILITY (commutative + single-trunk + spec conditions). So the sketch
  narrows to: prove the third dispatch mode folds through the const-grouping
  carrier walk with no per-record runtime branch. The commutative resource
  accumulation merge can reuse the shipped unit-outer per-core accum region +
  merge path (#683); canon (domain-08) skips head+tail for non-commutative
  accumulation, which the eligibility already encodes.
- **G2C-3 (phase-overlap ordering).** The substrate is fully shipped:
  `ProgressCounter` (Release/Acquire, `#[repr(transparent)]` over `AtomicUsize`,
  `dispatch/progress.rs`), the arena-indirection codegen shape proven WORKS
  (sketch `202605101036-progress-counter-arena`), the S3 store-store fence
  invariant (`dispatch/sync.rs::emit_progress_release_fence`), and
  `PoolFrame.progress_slots` (`hilavitkutin-api/src/platform.rs:236`) with the
  codegen slot index (`dispatch_codegen.rs:251`). Canon protocol (spec ~772,
  1326, 1478): phase N+1 starts when N publishes one morsel; counters provide
  implicit flow control (N+1 cannot outrun N); total ~= max(phases) + fill
  latency. The sketch narrows to the one unproven composition: a downstream
  Acquire on an upstream-published counter combined with the `waist_barrier`
  Release fence, without a per-morsel full barrier.
- **B2 (spectral labels -> FiberGrouping).** The hazard is now named precisely.
  `Trunk` is `{id, fiber_offset, fiber_count}` (`plan/trunk.rs:168`): a
  CONTIGUOUS range over the fiber pool. `arvo-spectral::k_way_partition`
  produces arbitrary per-fiber cluster labels, not contiguous ranges. So the
  projection must renumber fibers so each trunk's fibers are contiguous, AND
  apply the same permutation to `FiberGrouping.assignment` (`plan/fiber.rs:23`,
  the per-unit FiberId array); missing the second half is exactly the "silently
  changes grouping semantics" failure. The B2 sketch proves the
  label-to-contiguous-renumber projection keeps both views consistent.
- **C phase (RCM-row dispatch).** Aligned with canon by prior resolution (the
  RCM Step 5/Step 8 reading is canonicalized workspace-wide in
  `canonical-design-outranks-intermediate-rounds.md`). C1 sketch still required
  as specced (exact leeway).

### Sketch queue unchanged

A2b is implementation (its sketch was A2a). Remaining compile-bearing sketches
in order: G2C-1, G2C-3, C1, B2, plus the A3 reorder proof folded into A3b once
the storage decision lands. Each lands with its ASM-emission fixture per the
cross-cutting discipline above.
