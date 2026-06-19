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
