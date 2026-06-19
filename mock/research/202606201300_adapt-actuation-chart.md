# Adapt actuation: the consumption surfaces are dead (chart before building)

**Date:** 2026-06-19
**Status:** finding + roadmap for the adapt actuation arc (domain 22, post-#148)

## The finding

The adapt subsystem now has its signals (pass-duration EMA, per-phase
`phase_ema`, per-core `idle_ns`, throughput, change rate) and its tuning DECISION
(`select_adapt_config` sets the `adapt_reconfigure` trigger on phase imbalance,
PR #148). The next step is the ACTUATION: act on the trigger to rebalance. But
both surfaces an actuation would drive are computed-but-unconsumed by dispatch:

- **`plan.morsel_sizes`** (`plan/mod.rs:89`, `<D::Fibers as Capacity>::Array<USize>`,
  populated by `size_morsels` at `plan/mod.rs:391`, stored on `ColumnStorage` at
  `scheduler:568`) is read ONLY at plan-build (`plan/core_program.rs:72`, to count
  nonzero fibers). The dispatch morsel loop does NOT read it: `run` windows the
  whole record range uniformly by the const `Cfg::MORSEL_SIZE` (`scheduler:1342`,
  also `843` / `2080`). So re-chunking `morsel_sizes` changes nothing the
  dispatch observes.
- **`phase.strategy`** (`plan/phase.rs:97`, runtime enum, default Balanced) is
  read NOWHERE (confirmed in a prior round: not dispatch, scheduler, plan, or
  thread). A dead field. Domain-14 strategy-shaping (MAX_FUSE / BALANCED /
  MAX_SPLIT influencing fiber grouping + morsel sizing at plan-BUILD) is itself
  unbuilt.

So an actuation that mutates `morsel_sizes` or `phase.strategy` would be a no-op
on the running dispatch: a fake-green stopgap that could not satisfy the
catalogued contract "morsel re-chunk reduces idle / improves the imbalanced
workload". Building it now violates the no-stopgap discipline.

## The fork (what must be wired first)

The actuation arc is gated on FIRST wiring a consumption surface. This is the
real work, and it is a genuine design fork (picking wrong is expensive), so it is
charted here rather than picked on the fly.

1. **Per-fiber morsel sizing into the dispatch loop.** The current morsel loop is
   record-range-uniform (one `Cfg::MORSEL_SIZE` for the whole frame). Per-fiber
   `morsel_sizes` is a DIFFERENT model (each fiber windowed by its own size); it
   does not map onto the uniform loop mechanically. Wiring it means the dispatch
   reads `plan.morsel_sizes[fiber]` instead of (or in addition to) the const, and
   the morsel loop becomes per-fiber-size-aware. Design questions: does
   single-core `run` switch to per-fiber sizing or keep the const fast path with
   an adaptive override? what is the const-vs-runtime-size precedence? is
   `Cfg::MORSEL_SIZE` then a default the plan refines? This unblocks the tier-1
   morsel re-chunk actuation (the lightest, since it is a runtime-field mutation
   once the loop reads it).

2. **Strategy into plan-shaping (domain-14).** Wire `phase.strategy` into the
   plan algorithm so MAX_FUSE / BALANCED / MAX_SPLIT shape fiber grouping +
   morsel sizing at build. Then a between-frame strategy reselect rides the
   tier-3 plan-recompute path (OnMeta<PlanStage>), NOT a lightweight field write
   (the field is build-time-consumed once wired). Bigger than re-chunk; depends
   on the domain-14 shaping landing first.

## Recommended sequence

1. Wire consumption surface #1 (per-fiber morsel sizing into the dispatch loop) as
   its own slice, design-reviewed (the uniform-vs-per-fiber loop model is the
   crux). This is the smaller, more self-contained of the two and unblocks the
   first real actuation.
2. Then the tier-1 morsel re-chunk actuation: on `adapt_reconfigure`, recompute
   `morsel_sizes` to rebalance the imbalanced phase/fiber, which the now-wired
   loop observes. This flips the catalogued `morsel_rechunk_reduces_idle` /
   `ema_adaptation_improves_imbalanced_workload` contracts from red (once
   bench-verified it actually improves; the improvement is a bench question, per
   the gate-red-is-not-an-op-decision discipline).
3. Domain-14 strategy-shaping + strategy reselect (tier-3) as a later arc.
4. fiber_ema, active_units, parallel-path phase_ema, AdaptArena option-B storage
   in parallel as their own slices (they do not depend on the actuation).

## Why this is the honest next step

The decision (`select_adapt_config`) is shipped and unit-tested. The actuation
needs a consumption surface that does not exist yet; building the actuation
against a dead surface would produce a no-op that fakes the contracts green. The
real work is wiring the consumption, which is a design fork (uniform-vs-per-fiber
morsel model) that the spec/design must settle. The catalogued performance
contracts stay red until the consumption is wired AND the actuation is shown by
bench to improve, which is exactly the strict-by-design red-until-real pattern.
