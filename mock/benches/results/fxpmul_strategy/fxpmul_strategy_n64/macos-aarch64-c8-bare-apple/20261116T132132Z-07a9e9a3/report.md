# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot dominates: 16% faster than the next best (fxpmul_precise)

fxpmul_hot (2 ns) leads fxpmul_precise (3 ns) by 16%, a clear separation rather than a photo finish. CV 36.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (1 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (2 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fxpmul_precise's comparison is tie-heavy (14% tied pairs)

14% of paired samples for fxpmul_precise are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### fxpmul_precise is inconsistent: worst-20% is 3.3x its best-20%

fxpmul_precise's best 20% of batches run at 1 ns but its worst 20% at 5 ns (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 2.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.16x (fastest 2.5 ns, slowest 2.9 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 2096ns | 2092ns | 2088ns | 2093ns | 2114ns | base |
| fxpmul_precise | 2109ns | 2092ns | 2088ns | 2093ns | 2176ns | +0.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 2ns | 1ns | 4ns | base | 27.076 |
| fxpmul_precise | 3ns | 1ns | 5ns | +22.74% | 22.059 |

## Performance model

- Peak throughput: **55.652 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 25.600 | 46.0% |
| fxpmul_precise | 22.069 | 39.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2096ns | 2096ns | base |
| fxpmul_precise | 2109ns | 2109ns | +0.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 2ns | base | --- | [2, 2] | --- | --- | --- | --- |
| fxpmul_precise | 3ns | +0.4ns (+16.0%) | [+0, +1]ns | [3, 3] | YES | 0.0000 | 0.0000 | **22** (14%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 2ns | +26.1% |
| 1 | 2 | 2ns | +47.6% |
| 1 | 3 | 2ns | +17.4% |
| 1 | 4 | 2ns | +16.0% |
| 1 | 5 | 2ns | +38.1% |
| 1 | 6 | 2ns | +34.8% |
| 1 | 7 | 2ns | +38.1% |
| 1 | 8 | 2ns | +16.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | -0.465 | moderate- |
| fxpmul_precise | -0.397 | moderate- |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 4.3ns | 2.4ns | 181.1% | HIGH |
| fxpmul_precise | 3.2ns | 2.9ns | 108.6% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 1.1-3.7 ns)
      1.1 |##################
      1.3 |
      1.4 |
      1.5 |
      1.7 |#################################
      1.8 |
      1.9 |
      2.0 |########################################
      2.2 |
      2.3 |
      2.4 |##################################
      2.5 |
      2.7 |
      2.8 |#################################
      2.9 |
      3.0 |
      3.2 |################
      3.3 |
      3.4 |
      3.6 |
  (10 below, 18 above range)

fxpmul_precise (n=160, range 1.4-4.6 ns)
      1.4 |
      1.6 |##################
      1.7 |
      1.9 |
      2.0 |#############################
      2.2 |
      2.4 |##################
      2.5 |
      2.7 |
      2.8 |########################################
      3.0 |
      3.1 |##############################
      3.3 |
      3.5 |
      3.6 |##############
      3.8 |
      3.9 |
      4.1 |#################
      4.3 |
      4.4 |
  (16 below, 17 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=38.5% (high variance, measurements may be unstable)
- **fxpmul_hot**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **fxpmul_hot**: bridge=168.0% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=38.5% (high variance, measurements may be unstable)
- **fxpmul_precise**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **fxpmul_precise**: bridge=100.0% of algo (FFI overhead may distort results)
