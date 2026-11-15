# Match arm count: 2 vs 8 vs 64 — jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (9 ns) is smaller than the fastest variant's own run-to-run std-dev (28 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### match_8_arms is inconsistent: worst-20% is 3.7x its best-20%

match_8_arms's best 20% of batches run at 27 ns but its worst 20% at 99 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: match_8_arms** at 30.5 ns median (-7.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.29x (fastest 30.5 ns, slowest 39.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Match arm count: 2 vs 8 vs 64 — jump-table dispatch scaling |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3c8715d9a4b682f1 |
| Counter frequency | 2112766500 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| match_2_arms | 2821ns | 1913ns | 1906ns | 1955ns | 6335ns | base |
| match_64_arms | 2642ns | 1917ns | 1566ns | 1983ns | 5694ns | -6.35% |
| match_8_arms | 2811ns | 1909ns | 1753ns | 2027ns | 6220ns | -0.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 49ns | 33ns | 110ns | base | 5.259 |
| match_64_arms | 54ns | 32ns | 116ns | +10.78% | 4.747 |
| match_8_arms | 44ns | 27ns | 99ns | -8.72% | 5.761 |

## Performance model

- Peak throughput: **9.561 Gops/s** (match_8_arms; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 7.758 | 81.1% |
| match_64_arms | 6.506 | 68.0% |
| match_8_arms | 8.393 | 87.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2821ns | 2821ns | base |
| match_64_arms | 2642ns | 2642ns | -6.35% |
| match_8_arms | 2811ns | 2811ns | -0.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 33ns | base | --- | [33, 33] | --- | --- | --- | --- |
| match_64_arms | 39ns | no significant difference | [-0, +6]ns | [39, 40] | no | 0.1304 | 0.1304 | 2 |
| match_8_arms | 30ns | -3.2ns (-9.8%) | [-4, -3]ns | [30, 31] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 33ns | +21.6% | -7.3% |
| 1 | 2 | 33ns | -1.4% | -6.6% |
| 1 | 3 | 33ns | -3.9% | -12.6% |
| 1 | 4 | 33ns | +19.8% | -14.7% |
| 1 | 5 | 33ns | +22.5% | -22.8% |
| 1 | 6 | 33ns | -3.0% | -8.2% |
| 1 | 7 | 33ns | -2.7% | -3.5% |
| 1 | 8 | 33ns | +16.8% | -6.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.371 | moderate- |
| match_64_arms | -0.099 | ok |
| match_8_arms | 0.299 | moderate+ |

**Consistency summary:**

- **match_64_arms**: won 4/8, lost 4/8
- **match_8_arms**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 34.2ns | 48.7ns | 70.2% | HIGH |
| match_64_arms | 33.4ns | 53.9ns | 62.0% | HIGH |
| match_8_arms | 35.4ns | 44.4ns | 79.6% | HIGH |

## Distribution (algo ns)

```
match_2_arms (n=160, range 32.7-109.8 ns)
     32.7 |########################################
     36.5 |
     40.4 |
     44.2 |
     48.1 |
     51.9 |
     55.8 |
     59.7 |
     63.5 |
     67.4 |
     71.2 |
     75.1 |
     78.9 |
     82.8 |
     86.7 |
     90.5 |
     94.4 |#
     98.2 |
    102.1 |
    105.9 |#
  (12 below, 17 above range)

match_64_arms (n=160, range 31.6-115.7 ns)
     31.6 |#################################
     35.8 |########################################
     40.0 |#############
     44.2 |
     48.4 |
     52.6 |
     56.8 |
     61.0 |
     65.2 |
     69.5 |
     73.7 |
     77.9 |#
     82.1 |
     86.3 |
     90.5 |###
     94.7 |###
     98.9 |####
    103.1 |
    107.3 |#####
    111.5 |
  (14 below, 13 above range)

match_8_arms (n=160, range 26.8-98.6 ns)
     26.8 |########################################
     30.4 |##############################
     34.0 |
     37.6 |
     41.1 |
     44.7 |
     48.3 |
     51.9 |
     55.5 |
     59.1 |
     62.7 |
     66.3 |
     69.9 |
     73.5 |
     77.1 |
     80.7 |#
     84.3 |###
     87.9 |#
     91.5 |##
     95.0 |
  (13 below, 17 above range)

```

## Diagnostics

- **match_2_arms**: CV=64.1% (high variance, measurements may be unstable)
- **match_2_arms**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **match_2_arms**: bridge=70.6% of algo (FFI overhead may distort results)
- **match_64_arms**: CV=61.7% (high variance, measurements may be unstable)
- **match_64_arms**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **match_64_arms**: bridge=62.0% of algo (FFI overhead may distort results)
- **match_8_arms**: CV=63.8% (high variance, measurements may be unstable)
- **match_8_arms**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **match_8_arms**: bridge=79.0% of algo (FFI overhead may distort results)
