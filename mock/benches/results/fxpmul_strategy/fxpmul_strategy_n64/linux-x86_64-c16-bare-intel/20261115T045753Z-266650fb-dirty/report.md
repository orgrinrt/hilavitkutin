# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 25% faster than the next best (fxpmul_precise)

fxpmul_hot (14 ns) leads fxpmul_precise (17 ns) by 25%, a clear separation rather than a photo finish. CV 95.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 95.6%)

fxpmul_hot wins on median (14 ns) yet has the highest variance (CV 95.6%), while fxpmul_precise is the steadiest (CV 90.1%, 17 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (13 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (14 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_hot is inconsistent: worst-20% is 3.9x its best-20%

fxpmul_hot's best 20% of batches run at 11 ns but its worst 20% at 43 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 13.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.25x (fastest 13.5 ns, slowest 16.9 ns)

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
| Counter frequency | 2112754895 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 2657ns | 1889ns | 1548ns | 1887ns | 6075ns | base |
| fxpmul_precise | 2578ns | 1892ns | 1547ns | 1834ns | 5842ns | -2.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 19ns | 11ns | 43ns | base | 3.385 |
| fxpmul_precise | 23ns | 14ns | 51ns | +20.70% | 2.804 |

## Performance model

- Peak throughput: **5.777 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 4.741 | 82.1% |
| fxpmul_precise | 3.798 | 65.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2657ns | 2657ns | base |
| fxpmul_precise | 2578ns | 2578ns | -2.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 14ns | base | --- | [14, 14] | --- | --- | --- | --- |
| fxpmul_precise | 17ns | +3.3ns (+24.4%) | [+3, +3]ns | [17, 17] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 14ns | +23.9% |
| 1 | 2 | 14ns | +25.2% |
| 1 | 3 | 14ns | +2.2% |
| 1 | 4 | 11ns | +24.3% |
| 1 | 5 | 14ns | +25.2% |
| 1 | 6 | 14ns | +25.2% |
| 1 | 7 | 11ns | +24.3% |
| 1 | 8 | 14ns | +25.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.361 | moderate- |
| fxpmul_precise | -0.147 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 34.1ns | 18.9ns | 180.3% | HIGH |
| fxpmul_precise | 32.9ns | 22.8ns | 144.3% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 11.1-43.0 ns)
     11.1 |##########
     12.7 |########################################
     14.3 |
     15.9 |
     17.5 |
     19.1 |
     20.6 |
     22.2 |
     23.8 |
     25.4 |
     27.0 |
     28.6 |
     30.2 |
     31.8 |
     33.4 |
     35.0 |
     36.6 |
     38.2 |
     39.8 |
     41.4 |
  (9 below, 22 above range)

fxpmul_precise (n=160, range 13.8-51.3 ns)
     13.8 |######################
     15.7 |########################################
     17.5 |
     19.4 |
     21.3 |
     23.2 |
     25.0 |
     26.9 |
     28.8 |#
     30.7 |
     32.5 |
     34.4 |
     36.3 |
     38.2 |
     40.0 |
     41.9 |
     43.8 |
     45.7 |
     47.5 |
     49.4 |##
  (4 below, 20 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=68.3% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **fxpmul_hot**: bridge=180.7% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=66.5% (high variance, measurements may be unstable)
- **fxpmul_precise**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fxpmul_precise**: bridge=144.2% of algo (FFI overhead may distort results)
