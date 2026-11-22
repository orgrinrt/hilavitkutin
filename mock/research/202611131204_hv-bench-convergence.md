Source: kaski channel (coord/kaski of clause-dev), att/20261113T120458243777553-hv-bench-convergence-799c98/report-hv-bench-convergence.md

# hv-bench-convergence report

Branch `work/bench-convergence` on hilavitkutin, head `cc96a4c`, 11 commits over `dev`, pushed. Not landed.

## Done

- `mock/benches/single-trunk-convergence/`: the bench under the harness, seven arms as cdylibs (sequential, nway, head_tail, head_tail_dyn, kernel_seq, kernel_nway, std_parallel), 28 sections over fold_light, fold_heavy, append and short fibers, W in {1, 2, 4, 8}, n from 16 to 134217728.
- Head-and-tail is implemented in the bench, because the engine never emits `Head` or `Tail` ranges. It runs on a crew built from the engine's `PoolFrame` and frame protocol, with N-way controls on the same crew.
- `support/tests/correctness.rs`: 6 tests, green. They cover every arm against sequential and a reference, the append order with and without the fallback, the floating meeting point's coverage, and a negative control.
- Results: all four sweeps, each run on an idle machine, plus short fibers pinned to one CPU. `--summary` writes `results/summary.csv` and `summary.md`. `FINDINGS.md` opens with the three answers.
- Not scheduler source: `engine_vs_std/src/executor.rs` gained `with_workers`. Commit `df97140` fixes two repo lints broken by the mockspace bump (ticket 68636ad9ba36).

## Answers, in short (FINDINGS.md has the predicates)

- **Head-and-tail's 2x:** it reaches 1.8x to 1.9x over one walker where a frame holds about 150 us of work or more, and loses at 4096 records.
- **N-way vs head-and-tail:** N-way ties at W = 2. It passes by up to 1.81x at W = 4, up to 2.05x at W = 8 on folds, and up to 3.22x on the append.
- **Short fibers:** below one to four morsels, no parallel arm beats the engine's sequential frame (7 to 11 us). Head-and-tail costs 20 to 34 us and the engine's N-way 34 to 97 us. Most of that is a cross-vCPU wake (1.7 us when pinned to one CPU).

## Not established

The mechanism of the bimodal wake, why compute-bound scaling stops near 2x at W = 4, the spread at large n, head-and-tail inside the engine, and fibers in a full pipeline.

## Tickets

- c634312bdc34: 3 s TSC calibration per worker subprocess.
- 7ddf5ef52d08: background builds under a running bench.
- 68636ad9ba36: lints against mockspace a81e195.
- 2546c9e02a5e: the long run.

## Review

Not run yet. The dispatch of `pr-reviewer-senior` was refused by the auto-mode classifier, so the branch is unreviewed.
