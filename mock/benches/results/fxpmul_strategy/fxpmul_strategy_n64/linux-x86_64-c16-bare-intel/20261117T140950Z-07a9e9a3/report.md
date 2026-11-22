# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 24% faster than the next best (fxpmul_precise)

fxpmul_hot (11 ns) leads fxpmul_precise (14 ns) by 24%, a clear separation rather than a photo finish. CV 102.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (11 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (11 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_precise is inconsistent: worst-20% is 3.6x its best-20%

fxpmul_precise's best 20% of batches run at 14 ns but its worst 20% at 50 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 11.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.24x (fastest 11.2 ns, slowest 13.9 ns)

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
| Counter frequency | 2112548743 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 2579ns | 1555ns | 1549ns | 1965ns | 5451ns | base |
| fxpmul_precise | 2818ns | 1563ns | 1553ns | 2267ns | 5734ns | +9.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 18ns | 11ns | 39ns | base | 3.483 |
| fxpmul_precise | 25ns | 14ns | 50ns | +35.32% | 2.574 |

## Performance model

- Peak throughput: **5.789 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 5.714 | 98.7% |
| fxpmul_precise | 4.604 | 79.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2579ns | 2579ns | base |
| fxpmul_precise | 2818ns | 2818ns | +9.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 11ns | base | --- | [11, 11] | --- | --- | --- | --- |
| fxpmul_precise | 14ns | +2.8ns (+25.0%) | [+3, +3]ns | [14, 17] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 11ns | +25.2% |
| 1 | 2 | 11ns | +24.1% |
| 1 | 3 | 11ns | +24.1% |
| 1 | 4 | 11ns | +25.2% |
| 1 | 5 | 11ns | +281.7% |
| 1 | 6 | 11ns | +25.2% |
| 1 | 7 | 14ns | +25.0% |
| 1 | 8 | 11ns | +24.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.188 | ok |
| fxpmul_precise | -0.201 | moderate- |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 33.2ns | 18.4ns | 180.6% | HIGH |
| fxpmul_precise | 35.8ns | 24.9ns | 144.2% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 11.1-38.5 ns)
     11.1 |########################################
     12.4 |########
     13.8 |
     15.2 |
     16.6 |
     17.9 |
     19.3 |
     20.7 |
     22.1 |
     23.4 |
     24.8 |
     26.2 |
     27.5 |
     28.9 |
     30.3 |
     31.7 |##
     33.0 |#
     34.4 |####
     35.8 |#
     37.2 |####
  (13 below, 9 above range)

fxpmul_precise (n=160, range 13.8-50.3 ns)
     13.8 |########################################
     15.6 |######
     17.4 |
     19.3 |
     21.1 |
     22.9 |
     24.7 |
     26.6 |
     28.4 |
     30.2 |
     32.0 |
     33.9 |
     35.7 |
     37.5 |#
     39.3 |
     41.2 |
     43.0 |######
     44.8 |
     46.6 |####
     48.4 |
  (3 below, 15 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=62.5% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **fxpmul_hot**: bridge=179.5% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=61.8% (high variance, measurements may be unstable)
- **fxpmul_precise**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fxpmul_precise**: bridge=144.6% of algo (FFI overhead may distort results)
