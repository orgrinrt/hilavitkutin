# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 75% faster than the next best (fxpmul_precise)

fxpmul_hot (25 ns) leads fxpmul_precise (44 ns) by 75%, a clear separation rather than a photo finish. CV 109.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fxpmul_hot is fastest but the noisiest (CV 109.1%)

fxpmul_hot wins on median (25 ns) yet has the highest variance (CV 109.1%), while fxpmul_precise is the steadiest (CV 88.0%, 44 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (19 ns) is smaller than the fastest variant's own run-to-run std-dev (27 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (25 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_precise is inconsistent: worst-20% is 3.7x its best-20%

fxpmul_precise's best 20% of batches run at 36 ns but its worst 20% at 132 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 24.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.75x (fastest 24.9 ns, slowest 43.6 ns)

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
| fxpmul_hot | 2559ns | 1569ns | 1563ns | 1792ns | 5859ns | base |
| fxpmul_precise | 2595ns | 1925ns | 1575ns | 1823ns | 5930ns | +1.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 40ns | 25ns | 92ns | base | 6.350 |
| fxpmul_precise | 58ns | 36ns | 132ns | +44.83% | 4.384 |

## Performance model

- Peak throughput: **10.341 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.281 | 99.4% |
| fxpmul_precise | 5.872 | 56.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2559ns | 2559ns | base |
| fxpmul_precise | 2595ns | 2595ns | +1.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 25ns | base | --- | [25, 25] | --- | --- | --- | --- |
| fxpmul_precise | 44ns | +18.7ns (+75.1%) | [+11, +19]ns | [44, 44] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 25ns | +76.2% |
| 1 | 2 | 25ns | +44.4% |
| 1 | 3 | 30ns | +18.0% |
| 1 | 4 | 25ns | +44.1% |
| 1 | 5 | 25ns | +76.0% |
| 1 | 6 | 25ns | +44.1% |
| 1 | 7 | 25ns | +75.9% |
| 1 | 8 | 25ns | +76.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.162 | ok |
| fxpmul_precise | -0.120 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 32.0ns | 40.3ns | 79.4% | HIGH |
| fxpmul_precise | 32.5ns | 58.4ns | 55.7% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 24.8-91.6 ns)
     24.8 |########################################
     28.1 |########
     31.4 |
     34.8 |
     38.1 |
     41.5 |
     44.8 |
     48.2 |
     51.5 |
     54.8 |
     58.2 |
     61.5 |
     64.9 |
     68.2 |
     71.6 |
     74.9 |#
     78.2 |
     81.6 |##
     84.9 |#
     88.3 |###
  (14 below, 10 above range)

fxpmul_precise (n=160, range 35.7-132.4 ns)
     35.7 |###################################
     40.5 |########################################
     45.4 |
     50.2 |
     55.0 |
     59.9 |
     64.7 |
     69.5 |
     74.4 |
     79.2 |
     84.0 |
     88.9 |
     93.7 |
     98.6 |
    103.4 |
    108.2 |
    113.1 |#
    117.9 |#
    122.7 |
    127.6 |####
  (4 below, 15 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=67.4% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fxpmul_hot**: bridge=79.1% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=65.7% (high variance, measurements may be unstable)
- **fxpmul_precise**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **fxpmul_precise**: bridge=55.0% of algo (FFI overhead may distort results)
