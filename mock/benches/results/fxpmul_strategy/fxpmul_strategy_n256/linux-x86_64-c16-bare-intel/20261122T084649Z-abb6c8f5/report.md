# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 44% faster than the next best (fxpmul_precise)

fxpmul_hot (30 ns) leads fxpmul_precise (44 ns) by 44%, a clear separation rather than a photo finish. CV 88.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 88.3%)

fxpmul_hot wins on median (30 ns) yet has the highest variance (CV 88.3%), while fxpmul_precise is the steadiest (CV 85.2%, 44 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (13 ns) is smaller than the fastest variant's own run-to-run std-dev (27 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (30 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_precise is inconsistent: worst-20% is 3.6x its best-20%

fxpmul_precise's best 20% of batches run at 36 ns but its worst 20% at 127 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 30.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.44x (fastest 30.3 ns, slowest 43.7 ns)

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
| fxpmul_hot | 2704ns | 1909ns | 1729ns | 1932ns | 5994ns | base |
| fxpmul_precise | 2553ns | 1920ns | 1569ns | 1835ns | 5693ns | -5.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 43ns | 27ns | 94ns | base | 5.983 |
| fxpmul_precise | 58ns | 36ns | 127ns | +34.67% | 4.443 |

## Performance model

- Peak throughput: **9.328 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 8.449 | 90.6% |
| fxpmul_precise | 5.858 | 62.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2704ns | 2704ns | base |
| fxpmul_precise | 2553ns | 2553ns | -5.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 30ns | base | --- | [30, 30] | --- | --- | --- | --- |
| fxpmul_precise | 44ns | +13.3ns (+43.9%) | [+12, +13]ns | [44, 44] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 30ns | +44.2% |
| 1 | 2 | 25ns | +44.4% |
| 1 | 3 | 30ns | +17.6% |
| 1 | 4 | 30ns | +17.8% |
| 1 | 5 | 30ns | +44.2% |
| 1 | 6 | 30ns | +44.2% |
| 1 | 7 | 30ns | +43.8% |
| 1 | 8 | 30ns | +17.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.164 | ok |
| fxpmul_precise | 0.127 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 33.8ns | 42.8ns | 79.0% | HIGH |
| fxpmul_precise | 31.9ns | 57.6ns | 55.4% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 27.4-93.8 ns)
     27.4 |########################################
     30.8 |
     34.1 |
     37.4 |
     40.7 |#
     44.0 |
     47.4 |
     50.7 |
     54.0 |
     57.3 |
     60.6 |
     63.9 |
     67.3 |
     70.6 |
     73.9 |
     77.2 |
     80.5 |
     83.9 |#
     87.2 |
     90.5 |#
  (16 below, 19 above range)

fxpmul_precise (n=160, range 35.7-127.2 ns)
     35.7 |################################
     40.3 |########################################
     44.8 |
     49.4 |
     54.0 |
     58.6 |
     63.1 |
     67.7 |
     72.3 |
     76.9 |
     81.4 |
     86.0 |
     90.6 |
     95.2 |
     99.7 |#
    104.3 |##
    108.9 |#
    113.5 |#
    118.0 |##
    122.6 |
  (3 below, 14 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=62.5% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **fxpmul_hot**: bridge=79.2% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=64.6% (high variance, measurements may be unstable)
- **fxpmul_precise**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fxpmul_precise**: bridge=54.9% of algo (FFI overhead may distort results)
