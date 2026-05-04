**Date:** 2026-05-05
**Phase:** TOPIC
**Scope:** Sweep all remaining `USize(0)` literal sites in hilavitkutin engine, persistence, and providers crates. Replace with canonical `USize::ZERO`. Same pattern as round 202605050200; this round picks up the out-of-scope sites from #328 / #351.
**Source topics:** Task #351 (USize(0) -> USize::ZERO sweep across engine/persistence/providers).

# Topic: USize(0) -> USize::ZERO sweep across engine, persistence, providers

Round 202605050200 closed three sites in `hilavitkutin-api`. The same pattern still exists in roughly 50 sites across 21 files in the engine, persistence, and providers crates. This round sweeps them all.

## Sites (21 files, ~50 occurrences)

- engine: `thread/assignment.rs`, `thread/convergence.rs`, `thread/pool.rs`, `plan/inputs.rs`, `plan/fiber.rs`, `plan/phase.rs`, `plan/dirty.rs`, `plan/access.rs`, `scheduler/metrics.rs`, `scheduler/plan.rs`, `dispatch/core_dispatch.rs`, `dispatch/fiber_dispatch.rs`, `dispatch/morsel.rs`, `adapt/metrics.rs`
- persistence: `src/manifest.rs`, `src/sieve.rs`, plus tests `archive_str.rs`, `cold_store.rs`, `manifest.rs`, `sieve.rs`
- providers: `src/interner.rs`

## Decision

Mechanical replacement. `USize(0)` -> `USize::ZERO`. Where the file lacks the `arvo::strategy::Identity` import, add it. No DESIGN.md.tmpl semantic changes; identical to the prior round's pattern.

## Per-rule compliance

- `no-bare-primitives.md`: literal `0` removed; canonical arvo constant used.
- `writing-style.md`: doc comments unchanged.
- `cl-claim-sketch-discipline.md`: structured `## CHANGE:` blocks belong in the SRC CL. Given the volume, the SRC CL groups changes by file with verification commands.
