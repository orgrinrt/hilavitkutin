# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fpt_match_16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fpt_match_16 has the worst median (30 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest fpt_table_16 at 30 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (28 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### fpt_table_16 is inconsistent: worst-20% is 3.9x its best-20%

fpt_table_16's best 20% of batches run at 25 ns but its worst 20% at 96 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fpt_table_16** at 30.2 ns median (-0.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 30.2 ns, slowest 30.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 2112762479 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 2538ns | 1579ns | 1569ns | 1864ns | 5527ns | base |
| fpt_table_16 | 2597ns | 1913ns | 1565ns | 1862ns | 5836ns | +2.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 47ns | 28ns | 101ns | base | 5.496 |
| fpt_table_16 | 42ns | 25ns | 96ns | -9.90% | 6.100 |

## Performance model

- Peak throughput: **10.363 Gops/s** (fpt_table_16; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 8.449 | 81.5% |
| fpt_table_16 | 8.463 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2538ns | 2538ns | base |
| fpt_table_16 | 2597ns | 2597ns | +2.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 30ns | base | --- | [30, 31] | --- | --- | --- | --- |
| fpt_table_16 | 30ns | -3.8ns (-12.5%) | [-4, -4]ns | [26, 30] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 29ns | +7.6% |
| 1 | 2 | 39ns | -21.4% |
| 1 | 3 | 28ns | -11.0% |
| 1 | 4 | 28ns | -10.0% |
| 1 | 5 | 31ns | -19.2% |
| 1 | 6 | 34ns | -10.3% |
| 1 | 7 | 30ns | -17.2% |
| 1 | 8 | 30ns | -14.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.336 | moderate- |
| fpt_table_16 | 0.084 | ok |

**Consistency summary:**

- **fpt_table_16**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 32.7ns | 46.6ns | 70.2% | HIGH |
| fpt_table_16 | 32.9ns | 42.0ns | 78.4% | HIGH |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 28.0-101.0 ns)
     28.0 |########################################
     31.6 |############
     35.3 |
     38.9 |
     42.6 |
     46.2 |
     49.9 |
     53.5 |
     57.2 |
     60.8 |
     64.5 |
     68.1 |#
     71.8 |
     75.4 |#
     79.1 |#
     82.7 |#
     86.4 |###
     90.0 |#
     93.7 |##
     97.3 |#
  (13 below, 14 above range)

fpt_table_16 (n=160, range 24.7-95.5 ns)
     24.7 |########################################
     28.2 |#################################
     31.8 |
     35.3 |
     38.9 |
     42.4 |
     45.9 |
     49.5 |
     53.0 |
     56.6 |
     60.1 |##
     63.6 |
     67.2 |
     70.7 |#
     74.3 |
     77.8 |##
     81.3 |#
     84.9 |####
     88.4 |##
     92.0 |#
  (17 below, 14 above range)

```

## Diagnostics

- **fpt_match_16**: CV=63.3% (high variance, measurements may be unstable)
- **fpt_match_16**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fpt_match_16**: bridge=66.3% of algo (FFI overhead may distort results)
- **fpt_table_16**: CV=67.2% (high variance, measurements may be unstable)
- **fpt_table_16**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **fpt_table_16**: bridge=80.3% of algo (FFI overhead may distort results)
