# GATE-2 phase-axis course-correction: depth is not the waist-bounded phase

**Date:** 2026-06-07
**Status:** course-correction finding, pending op decision (chart-the-path step 11)
**Branch:** feat/hilavitkutin-parallel-engine-gate2
**Supersedes the round 2b plan in:** the engine-completion breadcrumb (round 2b "phase-sectioned single-core re-point") and `mock/design_rounds/202606071300_*` (topic + doc CL + src CL, round 2b, opened but NOT to be implemented as written).

## What was found

Grounding round 2b (re-point `Scheduler::run`'s morsel-outer path to dispatch
unit-by-unit grouped by the round-1 const grouping's per-unit "phase" number,
with a barrier between groups) against the canonical design surfaced a direct
conflict with both the spec and a shipped, currently-green test.

The round-1 const grouping's `compute_phases` (plan/grouping.rs) assigns each
unit its **longest read-after-write dependency depth** (topological depth). The
round 2b re-point sections dispatch by that number: run all depth-0 units across
every morsel, barrier, then all depth-1 units across every morsel, and so on.

For an accumulator-free two-unit chain (unit A writes a column, unit B reads it,
one RAW edge), `compute_phases` gives A depth 0 and B depth 1. Round 2b would
dispatch `[A, A, A | barrier | B, B, B]` (depth-group-outer). The shipped test
`tests/morsel_outer.rs` asserts the opposite for exactly this shape:
`[A, B, A, B, A, B]` (morsel-outer), and explicitly names and rejects the
`[A, A, A, B, B, B]` order, citing the canonical reason (intermediate columns
stay cache-resident per morsel only under morsel-outer nesting).

So round 2b as specced would break a green canonical-design test, presented as a
"no-regress" change. The src CL even claimed "#664 no-regress" while missing
morsel_outer.rs entirely.

## Why it diverges (canonical grounding)

The spec defines the two quantities as different axes:

- **Phase** = "wide DAG section between waist points (concurrent path count
  minimums)" (spec `:739`). Waist points are local minima in the count of alive
  concurrent paths, detected by waist analysis (`:1306`, `:1309-1311`); the
  worked example (`:1319-1323`) shows phases spanning units at different topo
  depths.
- **Trunk** = "sequential critical path within a phase. Shares NO write columns
  with other trunks" (`:741-742`).
- A producer to consumer RAW chain is a **sequential critical path sharing a
  column** = one trunk, within **one** phase (a linear DAG has path-count 1
  throughout, so no interior waist). Spec `:1394-1396`: "for <= 5 fibers: single
  trunk." Records within a fiber are independent (`:608`), and a single-trunk
  accumulator-free phase dispatches morsel-outer (the execution-strategy /
  morsel model), which is what morsel_outer.rs encodes.

`compute_phases`' depth-number equals the spec-phase only for a linear DAG with
no branching; in general they diverge (a unit deep inside one wide phase gets a
large depth but is still in the same spec-phase as shallower units). The
neutral domain-expert review (feature-dev:code-architect, 2026-06-07) confirmed
all three points independently with the spec citations above, and added the
load-bearing clarification:

> The depth-number is load-bearing for identifying within-phase structure
> (trunks), not for identifying where barriers go. Barriers belong at spec-phase
> transitions, which are waist points, not at depth-level transitions.

## Impact

1. **Round 2b (single-core phase-sectioned re-point) is drift.** It sections by
   the wrong axis and would break morsel_outer.rs and the canonical
   morsel-local-fiber guarantee. Do not implement it as written.

2. **The single-core flat morsel-outer walk shipped in GATE-1 is already the
   correct single-core degenerate.** morsel_outer.rs is the test that pins it.
   Single-core needs no phase-sectioned re-point.

3. **The GATE-2 mechanism's "phase" axis must be the waist-bounded phase, not
   depth.** This matters beyond round 2b: the round-2a const-gated per-trunk
   monos (`run_one_trunk::<PHASE, TRUNK>`) use the (phase, trunk) pair to isolate
   per-core programs. If "phase" is depth, a single linear trunk's units (which
   have different depths) get split across depth-monos, over-synchronising. With
   the waist-bounded phase, a linear trunk's units share one phase and run as one
   trunk program. The engine already computes the canonical structure
   (`compute_waists` to PhaseBoundaries, `block_diagonalise` to BlockPartition,
   group_fibers, project_fiber_components) but dispatch currently ignores it.

4. **Round-1's const grouping is not wasted.** `compute_trunks` (within-phase
   column-conflict components) is the right trunk axis. The depth from
   `compute_phases` is a valid *input* to trunk grouping; it is just not the
   barrier/phase axis. The correction is to source the phase (waist) axis from
   the engine's waist analysis, not from depth.

## Recommended forward path (for op decision)

Option A (recommended): drop the single-core phase-sectioned re-point. Keep the
shipped flat morsel-outer walk as the canonical single-core degenerate. Proceed
to G2-N (N-core): pin trunks to cores via the round-2a `run_one_trunk`
const-gated per-trunk monos, with waist barriers derived from the engine's
`compute_waists` (not depth). Correct the const grouping so its phase axis is the
waist-bounded phase; depth stays only as a trunk-grouping input.

Option B: keep a single-core round, but derive its barriers from `compute_waists`
(real waist points) rather than `compute_phases` depth. For waist-free DAGs (the
producer to consumer chain) this is a no-op equal to the flat walk, so
morsel_outer.rs stays green; it establishes the waist-sectioning structure
single-core as a G2-N foundation.

Either way: do not section single-core dispatch by topological depth.

## Audit-trail note

Round 2b's topic, doc CL, and src CL (`mock/design_rounds/202606071300_*`) are
left in place as the audit trail of the strayed step, to be deprecated or
rewritten once op picks the forward path. The src CL was committed (advancing the
round to IMPL) before the conflict was found during implementation grounding;
no source edits were made.
