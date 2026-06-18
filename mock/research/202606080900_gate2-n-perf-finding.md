# GATE-2 G2-N perf finding: per-trunk dispatch lands the fan-out win; accumulator is a separate path

**Date:** 2026-06-08
**Round:** 202606080600 (G2-N core-pinned per-trunk dispatch)
**Hypothesis:** re-pointing `run_parallel`'s worker dispatch from the runtime-mask `run_gated` path onto the G-e compiled per-trunk monos moves the `#664` parallel perf arms toward green.

> **CORRECTION (2026-06-08, op caught it):** the "wide_parallel wins ~3.5x" claim below compared the N-core engine against a SINGLE-threaded std loop, an unfair bar. Against optimal MULTI-threaded std the engine is at PARITY, not a 3.5x win. The fair-baseline result and the gate recalibration are in `202606081100_gate2-parallel-bench-fairness.md`. The structural finding below (the re-point is correct, bit-identical; the accumulator runs the §9 unit-outer path bypassing dispatch_core) stands; only the "win" framing was wrong.

## Outcome: WORKS for the fan-out arm; accumulator is out of this path

`cargo test --release --test perf_gate -- --ignored parallel_` after the re-point (8-core host, nightly-2026-05-28):

`wide_parallel` (K column-disjoint trunks in one phase, the canonical multi-trunk fan-out the per-trunk dispatch targets):

| N | parallel ratio | expect | verdict |
|---|---|---|---|
| 4096 | 2.629x | <= 1.60x | RED (thread-spawn floor at tiny N) |
| 65536 | 0.585x | <= 0.85x | ok (engine wins) |
| 1048576 | 0.280x | <= 0.55x | ok (engine wins ~3.5x) |
| 4194304 | 0.286x | <= 0.55x | ok (engine wins ~3.5x) |

`accumulator` (single unit-outer accumulator trunk):

| N | parallel ratio | expect | verdict |
|---|---|---|---|
| 4096 | 31.297x | <= 1.30x | RED |
| 65536 | 7.429x | <= 1.20x | RED |
| 1048576 | 2.590x | <= 1.00x | RED |
| 4194304 | 1.932x | <= 1.00x | RED |

## Reading

The per-trunk dispatch re-point is the fan-out win path. At scale `wide_parallel` wins decisively (engine beats single-threaded std ~3.5x), which is the GATE-2 multi-trunk parallelism landing. The N=4096 red is the fixed thread-coordination cost dominating when the work per core is tiny; it is the inherent parallel floor, not a dispatch defect, and a threshold-calibration / spawn-amortisation concern, not a stopgap target.

The accumulator parallel arm does NOT flow through `run_core_phase` / `dispatch_core`. An accumulator-bearing carrier is detected as unit-outer (`carrier_unit_outer`) and routed to `worker_accum_unit_outer`, which dispatches the whole carrier once over a per-core record slice via `run()` (the §9 threaded-accumulator path), bypassing the trunk dispatch entirely. So G2-N's re-point neither moves nor regresses the accumulator arm; its red is the §9 unit-outer path's own optimisation axis (the per-core region + merge cost at small N, the merge serialisation, the rebase overhead), a separate later concern. The G2-N round's initial test-plan line ("branching / accumulator arms move toward green via the compiled per-trunk parallelism") conflated the two paths; this finding corrects it.

`branching` is a single-core `runtime_*` gate (it measures `sched.run()`, not `run_parallel`), so it is also outside G2-N's parallel re-point.

## Follow-ups (not G2-N scope)

- Accumulator parallel perf: profile the §9 unit-outer path (rebase + per-core region + merge) against std at large N; the ~2x gap at N=4M is the optimisation target. Separate round.
- `wide_parallel` small-N floor: spawn-amortisation / threshold calibration so tiny-N does not pay full thread coordination. Separate concern (parallel startup, not dispatch).
