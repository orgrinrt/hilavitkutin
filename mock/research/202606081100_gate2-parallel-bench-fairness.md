# GATE-2 parallel bench fairness: engine vs optimal MULTI-threaded std

**Date:** 2026-06-08
**Supersedes the perf claim in:** `202606080900_gate2-n-perf-finding.md` (which compared `run_parallel` against single-threaded std and overstated the result as a "3.5x win").
**Trigger:** op asked whether the parallel benches compare against optimal *parallel* std or just the single-threaded baseline.

## The gap

The `engine_vs_std` perf gate's parallel arms (`parallel_wide_parallel`, `parallel_accumulator`) divided the N-core `run_parallel` time by the **single-threaded** std time (`lib.rs` `par_ratio` was `eng_runtime_par / std_runtime`, doc literally "Multi-threaded engine vs single-threaded optimal std"). That is not a fair bar: an 8-core engine beating a 1-thread loop by 3.5x says nothing about whether the engine's parallelism is good, because a competent hand-threaded std loop on the same cores would also approach Nx. The "wide_parallel wins 3.5x" claim was an artifact of that unfair baseline.

## The fix

Added an optimal multi-threaded std baseline (`std_runtime_par`): idiomatic `std::thread::scope` across `std_threads()` (machine parallelism, matching the engine's `OsThreadPool` worker count), byte-identical output. wide_parallel runs the K chains one-per-thread (matching the engine's K-trunk spread); accumulator splits the record range into per-thread chunks (matching the deviation-9 per-core split). `par_ratio()` is now `eng_runtime_par / std_runtime_par` (the FAIR bar, engine-N-core vs std-N-core); `par_speedup_vs_serial()` keeps the vs-1-thread number as report context only.

## The honest result (8-core host, nightly-2026-05-28, two runs)

`wide_parallel`, engine vs optimal parallel std:

| N | run 1 | run 2 | vs serial (context) |
|---|---|---|---|
| 4096 | 0.765x | 0.733x | ~3.4x |
| 65536 | 0.800x | 1.084x | ~0.60x |
| 1048576 | 1.541x | 1.043x | ~0.30x |
| 4194304 | 0.861x | 0.775x | ~0.28x |

`accumulator` parallel: passes at parity vs optimal parallel std (its earlier "31x red" was entirely the unfair serial baseline; the §9 per-core split is competitive with std thread::scope chunk-fill).

## Reading

The engine's parallel dispatch is **at parity** with optimal parallel std, not the 3.5x win the serial baseline suggested. It wins at the extremes (small N, where std re-spawns scope threads per frame and the engine's persistent pool amortises; very large N, where work dominates) and sits within run-to-run noise of parity in the mid-range (64K-1M). The N=1M arm is high-variance (1.04x to 1.54x across two runs), so it can flap red against a parity ceiling.

This is a GOOD result honestly stated: the engine's schedule-once + devirt + per-trunk dispatch matches hand-threaded std parallelism while also being the structured, declarative engine. It is not a blowout win, and the gate should not claim one.

## Recalibration

`expected_ratio(_, Mode::Parallel)` for both arms changed from the old sub-1.0 "win vs serial" ceilings (0.55/0.85/1.60 for wide_parallel; 1.0/1.2/1.3 for accumulator) to a flat **1.10x** (parity vs parallel std within ~±8% measurement noise). A real regression (engine clearly slower than parallel std) trips it; noise does not. The N=1M variance means wide_parallel may occasionally go red there: that is the standing oracle honestly flagging "1M parallel perf is not reliably at parity", lifeblood, not a false alarm.

## Follow-ups (not this round)

- Stabilise the N=1M parallel measurement (raise iters at large N, or pin the cause of the 1.04->1.54 variance: page faults / allocator / scheduling on the K=4 heavy trunks over 1M records each).
- If the mid-range parity gap proves real (not noise) after stabilisation, profile the per-frame barrier/morsel cost against std thread::scope at 64K-1M.
- A stricter bar still: optimal std with its OWN persistent pool (vs the engine's persistent pool), to isolate dispatch quality from pool-amortisation. The current `thread::scope` bar credits the engine's spawn-once advantage, which is legitimate but worth separating.
