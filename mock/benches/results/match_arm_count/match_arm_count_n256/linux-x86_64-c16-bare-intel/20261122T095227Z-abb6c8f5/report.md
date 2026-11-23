# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 12% faster than the next best (match_8_arms)

match_2_arms (27 ns) leads match_8_arms (30 ns) by 12%, a clear separation rather than a photo finish. CV 108.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_2_arms is fastest but the noisiest (CV 108.9%)

match_2_arms wins on median (27 ns) yet has the highest variance (CV 108.9%), while match_8_arms is the steadiest (CV 86.4%, 30 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### match_64_arms shows alternating (throttle bounce) (autocorr -0.61)

match_64_arms's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (12 ns) is smaller than the fastest variant's own run-to-run std-dev (29 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (27 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### match_64_arms is inconsistent: worst-20% is 3.9x its best-20%

match_64_arms's best 20% of batches run at 32 ns but its worst 20% at 123 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 27.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.45x (fastest 27.1 ns, slowest 39.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3c8715d9a4b682f1 |
| Counter frequency | 2112785978 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| match_2_arms | 2433ns | 1571ns | 1560ns | 1698ns | 5512ns | base |
| match_64_arms | 2640ns | 1914ns | 1565ns | 1887ns | 5976ns | +8.51% |
| match_8_arms | 2600ns | 1911ns | 1562ns | 1905ns | 5720ns | +6.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 42ns | 27ns | 96ns | base | 6.073 |
| match_64_arms | 55ns | 32ns | 123ns | +29.36% | 4.695 |
| match_8_arms | 41ns | 24ns | 90ns | -1.82% | 6.186 |

## Performance model

- Peak throughput: **10.613 Gops/s** (match_8_arms; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 9.464 | 89.2% |
| match_64_arms | 6.539 | 61.6% |
| match_8_arms | 8.477 | 79.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2433ns | 2433ns | base |
| match_64_arms | 2640ns | 2640ns | +8.51% |
| match_8_arms | 2600ns | 2600ns | +6.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 27ns | base | --- | [27, 32] | --- | --- | --- | --- |
| match_64_arms | 39ns | +6.2ns (+23.1%) | [+6, +7]ns | [33, 40] | YES | 0.0000 | 0.0000 | 1 |
| match_8_arms | 30ns | no significant difference | [-2, +2]ns | [29, 31] | no | 0.4769 | 0.4769 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 33ns | +0.0% | -19.7% |
| 1 | 2 | 27ns | +49.1% | +15.0% |
| 1 | 3 | 27ns | +18.5% | -13.3% |
| 1 | 4 | 27ns | +49.3% | +6.9% |
| 1 | 5 | 27ns | +23.9% | +0.6% |
| 1 | 6 | 33ns | +19.5% | -20.0% |
| 1 | 7 | 27ns | +19.8% | +19.3% |
| 1 | 8 | 27ns | +19.1% | +14.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.202 | moderate- |
| match_64_arms | -0.615 | HIGH- (thermal bounce) |
| match_8_arms | -0.262 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 7/8
- **match_8_arms**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 29.9ns | 42.2ns | 70.9% | HIGH |
| match_64_arms | 33.5ns | 54.5ns | 61.5% | HIGH |
| match_8_arms | 32.8ns | 41.4ns | 79.2% | HIGH |

## Distribution (algo ns)

```
match_2_arms (n=160, range 26.8-96.1 ns)
     26.8 |########################################
     30.3 |################
     33.8 |#
     37.2 |
     40.7 |
     44.2 |
     47.6 |
     51.1 |
     54.5 |
     58.0 |
     61.5 |
     64.9 |
     68.4 |
     71.9 |
     75.3 |
     78.8 |
     82.2 |
     85.7 |
     89.2 |#
     92.6 |
  (11 below, 17 above range)

match_64_arms (n=160, range 31.8-123.3 ns)
     31.8 |########################################
     36.4 |################################
     41.0 |
     45.5 |
     50.1 |
     54.7 |
     59.3 |
     63.8 |#
     68.4 |
     73.0 |
     77.5 |
     82.1 |
     86.7 |
     91.3 |##
     95.8 |##
    100.4 |
    105.0 |#
    109.6 |##
    114.1 |
    118.7 |##
  (14 below, 17 above range)

match_8_arms (n=160, range 24.1-90.5 ns)
     24.1 |########################################
     27.4 |##################################
     30.8 |#####################################
     34.1 |
     37.4 |
     40.7 |
     44.0 |
     47.3 |
     50.7 |
     54.0 |
     57.3 |
     60.6 |
     63.9 |#
     67.2 |##
     70.6 |###
     73.9 |###
     77.2 |#
     80.5 |#######
     83.8 |###
     87.1 |####
  (15 below, 11 above range)

```

## Diagnostics

- **match_2_arms**: CV=69.9% (high variance, measurements may be unstable)
- **match_2_arms**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **match_2_arms**: bridge=71.0% of algo (FFI overhead may distort results)
- **match_64_arms**: CV=66.5% (high variance, measurements may be unstable)
- **match_64_arms**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **match_64_arms**: bridge=62.3% of algo (FFI overhead may distort results)
- **match_8_arms**: CV=63.0% (high variance, measurements may be unstable)
- **match_8_arms**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **match_8_arms**: bridge=79.8% of algo (FFI overhead may distort results)
