# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 55% faster than the next best (fxpmul_precise)

fxpmul_hot (97 ns) leads fxpmul_precise (151 ns) by 55%, a clear separation rather than a photo finish. CV 87.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 87.2%)

fxpmul_hot wins on median (97 ns) yet has the highest variance (CV 87.2%), while fxpmul_precise is the steadiest (CV 82.5%, 151 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (54 ns) is smaller than the fastest variant's own run-to-run std-dev (85 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (97 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_hot is inconsistent: worst-20% is 3.7x its best-20%

fxpmul_hot's best 20% of batches run at 80 ns but its worst 20% at 291 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 97.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.55x (fastest 97.2 ns, slowest 150.8 ns)

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
| fxpmul_hot | 2645ns | 1972ns | 1612ns | 1887ns | 5951ns | base |
| fxpmul_precise | 2637ns | 2026ns | 1660ns | 2024ns | 5452ns | -0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 130ns | 80ns | 291ns | base | 7.884 |
| fxpmul_precise | 196ns | 123ns | 402ns | +50.54% | 5.237 |

## Performance model

- Peak throughput: **12.873 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.535 | 81.8% |
| fxpmul_precise | 6.790 | 52.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2645ns | 2645ns | base |
| fxpmul_precise | 2637ns | 2637ns | -0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 97ns | base | --- | [97, 97] | --- | --- | --- | --- |
| fxpmul_precise | 151ns | +53.6ns (+55.1%) | [+54, +62]ns | [151, 151] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 97ns | +55.1% |
| 1 | 2 | 80ns | +89.2% |
| 1 | 3 | 80ns | +55.0% |
| 1 | 4 | 97ns | +55.0% |
| 1 | 5 | 97ns | +26.9% |
| 1 | 6 | 80ns | +89.4% |
| 1 | 7 | 97ns | +55.0% |
| 1 | 8 | 80ns | +89.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.376 | moderate- |
| fxpmul_precise | -0.375 | moderate- |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 32.3ns | 129.9ns | 24.9% | HIGH |
| fxpmul_precise | 31.1ns | 195.5ns | 15.9% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 79.5-290.7 ns)
     79.5 |#########################
     90.1 |########################################
    100.7 |
    111.2 |
    121.8 |
    132.3 |
    142.9 |
    153.5 |#
    164.0 |
    174.6 |
    185.1 |
    195.7 |
    206.3 |#
    216.8 |
    227.4 |
    237.9 |#
    248.5 |#
    259.1 |#
    269.6 |#
    280.2 |
  (17 below, 18 above range)

fxpmul_precise (n=160, range 123.4-402.5 ns)
    123.4 |#########
    137.3 |########################################
    151.3 |
    165.2 |
    179.2 |
    193.1 |
    207.1 |
    221.1 |
    235.0 |
    249.0 |
    262.9 |
    276.9 |
    290.8 |
    304.8 |
    318.8 |
    332.7 |
    346.7 |
    360.6 |
    374.6 |
    388.5 |
  (10 below, 19 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=65.3% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fxpmul_hot**: bridge=24.7% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=63.6% (high variance, measurements may be unstable)
- **fxpmul_precise**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **fxpmul_precise**: bridge=16.0% of algo (FFI overhead may distort results)
