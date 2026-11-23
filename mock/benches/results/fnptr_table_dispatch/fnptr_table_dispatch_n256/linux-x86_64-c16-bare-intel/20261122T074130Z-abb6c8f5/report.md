# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fpt_match_16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fpt_match_16 has the worst median (31 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest fpt_table_16 at 30 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (28 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### fpt_table_16 is inconsistent: worst-20% is 3.8x its best-20%

fpt_table_16's best 20% of batches run at 25 ns but its worst 20% at 94 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fpt_table_16** at 30.4 ns median (-1.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 30.4 ns, slowest 30.7 ns)

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
| Counter frequency | 2112768962 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 2517ns | 1581ns | 1567ns | 1873ns | 5403ns | base |
| fpt_table_16 | 2884ns | 1907ns | 1558ns | 2348ns | 5817ns | +14.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 46ns | 28ns | 100ns | base | 5.514 |
| fpt_table_16 | 46ns | 25ns | 94ns | -0.09% | 5.519 |

## Performance model

- Peak throughput: **10.428 Gops/s** (fpt_table_16; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 8.339 | 80.0% |
| fpt_table_16 | 8.435 | 80.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2517ns | 2517ns | base |
| fpt_table_16 | 2884ns | 2884ns | +14.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 31ns | base | --- | [30, 34] | --- | --- | --- | --- |
| fpt_table_16 | 30ns | -1.0ns (-3.3%) | [-3, -0]ns | [30, 31] | YES | 0.0314 | 0.0314 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 35ns | -29.0% |
| 1 | 2 | 29ns | +130.3% |
| 1 | 3 | 28ns | -9.2% |
| 1 | 4 | 28ns | +11.8% |
| 1 | 5 | 31ns | +0.7% |
| 1 | 6 | 34ns | -25.1% |
| 1 | 7 | 30ns | +0.0% |
| 1 | 8 | 30ns | -17.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | 0.046 | ok |
| fpt_table_16 | -0.314 | moderate- |

**Consistency summary:**

- **fpt_table_16**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 31.8ns | 46.4ns | 68.5% | HIGH |
| fpt_table_16 | 37.0ns | 46.4ns | 79.7% | HIGH |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 28.0-99.7 ns)
     28.0 |########################################
     31.6 |################
     35.2 |####
     38.7 |
     42.3 |
     45.9 |
     49.5 |
     53.1 |
     56.7 |
     60.3 |
     63.8 |
     67.4 |
     71.0 |
     74.6 |#
     78.2 |#
     81.8 |#
     85.4 |####
     88.9 |##
     92.5 |##
     96.1 |
  (14 below, 11 above range)

fpt_table_16 (n=160, range 24.6-94.5 ns)
     24.6 |########################################
     28.0 |#######################################
     31.5 |###
     35.0 |
     38.5 |
     42.0 |#
     45.5 |
     49.0 |
     52.5 |
     56.0 |
     59.5 |
     63.0 |##
     66.5 |##
     70.0 |#####
     73.5 |#
     77.0 |###
     80.5 |#
     84.0 |###
     87.5 |######
     91.0 |##
  (14 below, 14 above range)

```

## Diagnostics

- **fpt_match_16**: CV=63.2% (high variance, measurements may be unstable)
- **fpt_match_16**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fpt_match_16**: bridge=65.5% of algo (FFI overhead may distort results)
- **fpt_table_16**: CV=60.4% (high variance, measurements may be unstable)
- **fpt_table_16**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **fpt_table_16**: bridge=80.4% of algo (FFI overhead may distort results)
