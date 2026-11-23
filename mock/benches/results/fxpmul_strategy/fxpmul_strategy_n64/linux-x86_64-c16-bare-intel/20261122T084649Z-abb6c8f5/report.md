# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 24% faster than the next best (fxpmul_precise)

fxpmul_hot (14 ns) leads fxpmul_precise (17 ns) by 24%, a clear separation rather than a photo finish. CV 88.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (12 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (14 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_precise is inconsistent: worst-20% is 3.8x its best-20%

fxpmul_precise's best 20% of batches run at 14 ns but its worst 20% at 53 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 13.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.24x (fastest 13.6 ns, slowest 16.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate) |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5555666677770007 |
| Counter frequency | 2112817198 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 2902ns | 1894ns | 1547ns | 2374ns | 5838ns | base |
| fxpmul_precise | 2637ns | 1895ns | 1550ns | 1932ns | 5840ns | -9.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 21ns | 11ns | 41ns | base | 3.095 |
| fxpmul_precise | 24ns | 14ns | 53ns | +14.34% | 2.707 |

## Performance model

- Peak throughput: **5.771 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 4.706 | 81.5% |
| fxpmul_precise | 3.787 | 65.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2902ns | 2902ns | base |
| fxpmul_precise | 2637ns | 2637ns | -9.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 14ns | base | --- | [14, 14] | --- | --- | --- | --- |
| fxpmul_precise | 17ns | +3.3ns (+24.6%) | [+3, +3]ns | [17, 17] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 14ns | +25.0% |
| 1 | 2 | 11ns | +52.3% |
| 1 | 3 | 14ns | +2.2% |
| 1 | 4 | 14ns | +2.2% |
| 1 | 5 | 14ns | +25.0% |
| 1 | 6 | 11ns | +24.1% |
| 1 | 7 | 30ns | -43.4% |
| 1 | 8 | 11ns | +53.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.343 | moderate- |
| fxpmul_precise | -0.140 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 37.1ns | 20.7ns | 179.6% | HIGH |
| fxpmul_precise | 33.9ns | 23.6ns | 143.5% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 11.1-41.3 ns)
     11.1 |#######################
     12.6 |########################################
     14.1 |
     15.6 |
     17.1 |
     18.6 |
     20.2 |#
     21.7 |#
     23.2 |#
     24.7 |#
     26.2 |
     27.7 |##
     29.2 |###
     30.7 |
     32.3 |##
     33.8 |###
     35.3 |#
     36.8 |###
     38.3 |
     39.8 |###
  (4 below, 13 above range)

fxpmul_precise (n=160, range 13.8-52.7 ns)
     13.8 |#############
     15.8 |########################################
     17.7 |
     19.7 |
     21.6 |
     23.6 |
     25.5 |
     27.4 |
     29.4 |
     31.3 |
     33.3 |
     35.2 |
     37.1 |#
     39.1 |#
     41.0 |
     43.0 |#
     44.9 |
     46.8 |#
     48.8 |#
     50.7 |
  (15 below, 13 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=58.0% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fxpmul_hot**: bridge=179.4% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=66.7% (high variance, measurements may be unstable)
- **fxpmul_precise**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **fxpmul_precise**: bridge=143.8% of algo (FFI overhead may distort results)
