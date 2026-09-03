# LIGHT_THRESHOLD: the Bench Oracle and Its Real Gate

**Date:** 2026-07-19
**Status:** the item-8 resolution record of the seed pre-freeze batch.
Governance already states the resolution SHAPE ("a bench-set tunable
constant, a registry constant row, not a design question"); this record
pins the oracle and its gate so the registry row is precise.

## What the constant arbitrates

Plan-time strategy selection classifies producers by weight (WU count
times column accesses); a producer under `LIGHT_THRESHOLD` routes the
phase to the adaptive strategy, over it to pipe-chase (with chase-steal
when consumer weight exceeds half producer weight). Canon names the
constant and deliberately leaves it unvalued: the boundary is a
measured fact about real dispatch, not a design opinion.

## The oracle

The value-setting bench runs the SAME phase shape under the adaptive
and pipe-chase strategies across a producer-weight sweep and reads the
crossover: `LIGHT_THRESHOLD` is the weight at which the two medians
cross, with the surrounding regime (record counts 10K to 1M, the
strategy-gating band canon defines) as the sweep's frame.

## The real gate, stated honestly

The oracle needs both strategies runnable, and today
`strategy/mod.rs` carries the `Strategy` / `PhaseStrategy` vocabulary
and a `DefaultSelector` with no selection logic or alternative dispatch
paths behind them; the engine executes its one dispatch shape. The
crossover bench therefore cannot run yet, and no synthetic stand-in
would measure the thing the constant gates. The gate is the adapt-phase
strategy build (the r6 Phase D tail); the bench runs at its end, the
same pattern as the #644 fiber bench running at the end of B1b.

## Registry shape at freeze

One constant row: name `LIGHT_THRESHOLD`, value BENCH-PENDING by
design, oracle the adaptive-versus-pipe-chase crossover sweep above,
gate the strategy-variant build, provenance this record plus the seed
execution chapter's strategy-selection section. The item closes as
resolved-by-registration; the VALUE lands as a registry amendment when
the gate opens, exactly the post-freeze flow the registry exists for.
