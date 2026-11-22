# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_8_arms dominates: 18% faster than the next best (match_2_arms)

match_8_arms (28 ns) leads match_2_arms (33 ns) by 18%, a clear separation rather than a photo finish. CV 98.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_8_arms is fastest but the noisiest (CV 98.1%)

match_8_arms wins on median (28 ns) yet has the highest variance (CV 98.1%), while match_64_arms is the steadiest (CV 89.4%, 39 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (12 ns) is smaller than the fastest variant's own run-to-run std-dev (27 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### match_8_arms is inconsistent: worst-20% is 3.9x its best-20%

match_8_arms's best 20% of batches run at 23 ns but its worst 20% at 91 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: match_8_arms** at 27.8 ns median (-15.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.42x (fastest 27.8 ns, slowest 39.3 ns)

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
| Counter frequency | 2112760997 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| match_2_arms | 2743ns | 1914ns | 1566ns | 2075ns | 5922ns | base |
| match_64_arms | 2714ns | 1925ns | 1575ns | 2028ns | 5910ns | -1.06% |
| match_8_arms | 2734ns | 1582ns | 1564ns | 2140ns | 5686ns | -0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 47ns | 27ns | 101ns | base | 5.435 |
| match_64_arms | 56ns | 32ns | 121ns | +18.42% | 4.589 |
| match_8_arms | 44ns | 23ns | 91ns | -7.64% | 5.884 |

## Performance model

- Peak throughput: **10.921 Gops/s** (match_8_arms; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 7.805 | 71.5% |
| match_64_arms | 6.506 | 59.6% |
| match_8_arms | 9.209 | 84.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2743ns | 2743ns | base |
| match_64_arms | 2714ns | 2714ns | -1.06% |
| match_8_arms | 2734ns | 2734ns | -0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 33ns | base | --- | [30, 33] | --- | --- | --- | --- |
| match_64_arms | 39ns | +6.5ns (+19.7%) | [+6, +7]ns | [39, 40] | YES | 0.0000 | 0.0000 | 1 |
| match_8_arms | 28ns | -3.7ns (-11.3%) | [-4, -3]ns | [26, 31] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 27ns | +22.2% | +14.1% |
| 1 | 2 | 33ns | -1.1% | -23.9% |
| 1 | 3 | 27ns | +20.2% | -13.3% |
| 1 | 4 | 27ns | +50.6% | -11.9% |
| 1 | 5 | 27ns | +24.2% | -2.8% |
| 1 | 6 | 33ns | +19.7% | -21.2% |
| 1 | 7 | 27ns | +47.2% | +21.0% |
| 1 | 8 | 33ns | +19.4% | -22.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.461 | moderate- |
| match_64_arms | 0.106 | ok |
| match_8_arms | -0.013 | ok |

**Consistency summary:**

- **match_64_arms**: won 1/8, lost 7/8
- **match_8_arms**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 33.3ns | 47.1ns | 70.8% | HIGH |
| match_64_arms | 34.6ns | 55.8ns | 62.1% | HIGH |
| match_8_arms | 34.6ns | 43.5ns | 79.6% | HIGH |

## Distribution (algo ns)

```
match_2_arms (n=160, range 26.8-101.3 ns)
     26.8 |########################################
     30.6 |#################################
     34.3 |#
     38.0 |
     41.7 |
     45.5 |
     49.2 |#
     52.9 |
     56.6 |
     60.3 |
     64.1 |
     67.8 |#
     71.5 |
     75.2 |
     79.0 |
     82.7 |###
     86.4 |##
     90.1 |######
     93.8 |##
     97.6 |###
  (15 below, 12 above range)

match_64_arms (n=160, range 32.4-121.1 ns)
     32.4 |################################
     36.8 |########################################
     41.2 |#
     45.7 |
     50.1 |
     54.5 |
     59.0 |
     63.4 |
     67.8 |
     72.3 |
     76.7 |
     81.1 |#
     85.6 |##
     90.0 |
     94.4 |#
     98.9 |#
    103.3 |##
    107.7 |##
    112.2 |##
    116.6 |####
  (12 below, 13 above range)

match_8_arms (n=160, range 23.4-90.6 ns)
     23.4 |########################################
     26.8 |###
     30.2 |################
     33.5 |#
     36.9 |
     40.2 |
     43.6 |
     47.0 |
     50.3 |
     53.7 |
     57.0 |
     60.4 |#
     63.7 |###
     67.1 |#
     70.5 |#
     73.8 |###
     77.2 |###
     80.5 |##
     83.9 |#
     87.3 |#
  (17 below, 15 above range)

```

## Diagnostics

- **match_2_arms**: CV=64.3% (high variance, measurements may be unstable)
- **match_2_arms**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **match_2_arms**: bridge=71.0% of algo (FFI overhead may distort results)
- **match_64_arms**: CV=63.1% (high variance, measurements may be unstable)
- **match_64_arms**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **match_64_arms**: bridge=62.3% of algo (FFI overhead may distort results)
- **match_8_arms**: CV=62.7% (high variance, measurements may be unstable)
- **match_8_arms**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **match_8_arms**: bridge=72.3% of algo (FFI overhead may distort results)
