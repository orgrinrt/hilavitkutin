# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 44% faster than the next best (fxpmul_precise)

fxpmul_hot (30 ns) leads fxpmul_precise (44 ns) by 44%, a clear separation rather than a photo finish. CV 89.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 89.2%)

fxpmul_hot wins on median (30 ns) yet has the highest variance (CV 89.2%), while fxpmul_precise is the steadiest (CV 89.2%, 44 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (13 ns) is smaller than the fastest variant's own run-to-run std-dev (27 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (30 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_hot is inconsistent: worst-20% is 3.7x its best-20%

fxpmul_hot's best 20% of batches run at 25 ns but its worst 20% at 93 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 30.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.44x (fastest 30.2 ns, slowest 43.6 ns)

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
| fxpmul_hot | 2638ns | 1901ns | 1558ns | 1908ns | 5910ns | base |
| fxpmul_precise | 2552ns | 1917ns | 1567ns | 1811ns | 5759ns | -3.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 42ns | 25ns | 93ns | base | 6.143 |
| fxpmul_precise | 58ns | 36ns | 129ns | +38.35% | 4.440 |

## Performance model

- Peak throughput: **10.333 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 8.477 | 82.0% |
| fxpmul_precise | 5.872 | 56.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2638ns | 2638ns | base |
| fxpmul_precise | 2552ns | 2552ns | -3.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 30ns | base | --- | [25, 30] | --- | --- | --- | --- |
| fxpmul_precise | 44ns | +13.2ns (+43.7%) | [+11, +13]ns | [44, 44] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 30ns | +43.8% |
| 1 | 2 | 30ns | +44.2% |
| 1 | 3 | 25ns | +76.2% |
| 1 | 4 | 25ns | +44.0% |
| 1 | 5 | 25ns | +44.0% |
| 1 | 6 | 25ns | +44.4% |
| 1 | 7 | 25ns | +43.8% |
| 1 | 8 | 30ns | +43.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | 0.263 | moderate+ |
| fxpmul_precise | 0.375 | moderate+ |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 33.4ns | 41.7ns | 80.2% | HIGH |
| fxpmul_precise | 32.2ns | 57.7ns | 55.8% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 24.8-92.7 ns)
     24.8 |########################################
     28.2 |##########################
     31.6 |
     35.0 |
     38.4 |
     41.7 |
     45.1 |
     48.5 |#
     51.9 |
     55.3 |
     58.7 |
     62.1 |
     65.5 |
     68.9 |
     72.3 |
     75.7 |##
     79.1 |#
     82.5 |##
     85.9 |#
     89.3 |####
  (8 below, 13 above range)

fxpmul_precise (n=160, range 35.7-129.1 ns)
     35.7 |#####################################
     40.4 |########################################
     45.0 |
     49.7 |
     54.4 |
     59.0 |
     63.7 |
     68.4 |
     73.1 |#
     77.7 |
     82.4 |
     87.1 |
     91.7 |
     96.4 |
    101.1 |
    105.8 |
    110.4 |#
    115.1 |
    119.8 |
    124.4 |
  (3 below, 20 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=64.6% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fxpmul_hot**: bridge=78.8% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=67.5% (high variance, measurements may be unstable)
- **fxpmul_precise**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **fxpmul_precise**: bridge=54.8% of algo (FFI overhead may distort results)
