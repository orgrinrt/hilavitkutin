# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 27% faster than the next best (fxpmul_precise)

fxpmul_hot (97 ns) leads fxpmul_precise (124 ns) by 27%, a clear separation rather than a photo finish. CV 88.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (26 ns) is smaller than the fastest variant's own run-to-run std-dev (86 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (97 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_hot is inconsistent: worst-20% is 3.7x its best-20%

fxpmul_hot's best 20% of batches run at 80 ns but its worst 20% at 292 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 97.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.27x (fastest 97.1 ns, slowest 123.5 ns)

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
| fxpmul_hot | 2666ns | 1968ns | 1613ns | 1910ns | 5988ns | base |
| fxpmul_precise | 2539ns | 1676ns | 1657ns | 1828ns | 5555ns | -4.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 131ns | 80ns | 292ns | base | 7.830 |
| fxpmul_precise | 188ns | 123ns | 407ns | +43.45% | 5.458 |

## Performance model

- Peak throughput: **12.880 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.546 | 81.9% |
| fxpmul_precise | 8.291 | 64.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2666ns | 2666ns | base |
| fxpmul_precise | 2539ns | 2539ns | -4.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 97ns | base | --- | [80, 97] | --- | --- | --- | --- |
| fxpmul_precise | 124ns | +43.9ns (+45.2%) | [+44, +50]ns | [123, 151] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 80ns | +89.4% |
| 1 | 2 | 97ns | +55.0% |
| 1 | 3 | 80ns | +55.0% |
| 1 | 4 | 80ns | +55.0% |
| 1 | 5 | 97ns | +55.1% |
| 1 | 6 | 97ns | +26.8% |
| 1 | 7 | 80ns | +55.0% |
| 1 | 8 | 80ns | +55.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.142 | ok |
| fxpmul_precise | 0.057 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 32.4ns | 130.8ns | 24.8% | HIGH |
| fxpmul_precise | 30.0ns | 187.6ns | 16.0% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 79.5-292.1 ns)
     79.5 |########################################
     90.1 |###################################
    100.8 |#
    111.4 |
    122.0 |
    132.7 |
    143.3 |
    153.9 |
    164.6 |
    175.2 |
    185.8 |
    196.4 |
    207.1 |
    217.7 |
    228.3 |###
    239.0 |##
    249.6 |##
    260.2 |###
    270.9 |
    281.5 |####
  (29 below, 13 above range)

fxpmul_precise (n=160, range 123.3-407.0 ns)
    123.3 |########################################
    137.5 |#################################
    151.7 |
    165.9 |
    180.0 |
    194.2 |
    208.4 |
    222.6 |
    236.8 |
    251.0 |
    265.2 |
    279.4 |
    293.5 |
    307.7 |
    321.9 |
    336.1 |
    350.3 |
    364.5 |
    378.7 |
    392.9 |
  (28 below, 21 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=65.5% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fxpmul_hot**: bridge=24.7% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=68.6% (high variance, measurements may be unstable)
- **fxpmul_precise**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **fxpmul_precise**: bridge=16.1% of algo (FFI overhead may distort results)
