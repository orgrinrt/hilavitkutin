# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 66% faster than the next best (match_8_arms)

match_2_arms (98 ns) leads match_8_arms (162 ns) by 66%, a clear separation rather than a photo finish. CV 92.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_2_arms is fastest but the noisiest (CV 92.7%)

match_2_arms wins on median (98 ns) yet has the highest variance (CV 92.7%), while match_64_arms is the steadiest (CV 77.7%, 183 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (85 ns) is smaller than the fastest variant's own run-to-run std-dev (91 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (98 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### match_2_arms is inconsistent: worst-20% is 3.8x its best-20%

match_2_arms's best 20% of batches run at 83 ns but its worst 20% at 311 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 97.9 ns median
- 2 variants significantly slower than baseline
- Spread: 1.87x (fastest 97.9 ns, slowest 183.1 ns)

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
| match_2_arms | 2631ns | 1978ns | 1619ns | 1874ns | 5916ns | base |
| match_64_arms | 2917ns | 2066ns | 1897ns | 2268ns | 5886ns | +10.86% |
| match_8_arms | 2941ns | 2048ns | 1880ns | 2274ns | 6001ns | +11.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 138ns | 83ns | 311ns | base | 7.396 |
| match_64_arms | 253ns | 166ns | 510ns | +82.45% | 4.054 |
| match_8_arms | 229ns | 148ns | 473ns | +65.19% | 4.477 |

## Performance model

- Peak throughput: **12.390 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 10.460 | 84.4% |
| match_64_arms | 5.593 | 45.1% |
| match_8_arms | 6.307 | 50.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2631ns | 2631ns | base |
| match_64_arms | 2917ns | 2917ns | +10.86% |
| match_8_arms | 2941ns | 2941ns | +11.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 98ns | base | --- | [96, 107] | --- | --- | --- | --- |
| match_64_arms | 183ns | +93.8ns (+95.8%) | [+89, +96]ns | [182, 184] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 162ns | +72.0ns (+73.5%) | [+69, +74]ns | [162, 163] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 107ns | +69.4% | +50.2% |
| 1 | 2 | 86ns | +201.8% | +159.0% |
| 1 | 3 | 107ns | +70.3% | +52.6% |
| 1 | 4 | 108ns | +67.7% | +23.5% |
| 1 | 5 | 89ns | +105.3% | +81.7% |
| 1 | 6 | 88ns | +108.7% | +81.2% |
| 1 | 7 | 87ns | +105.3% | +85.0% |
| 1 | 8 | 107ns | +38.1% | +48.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.184 | ok |
| match_64_arms | -0.058 | ok |
| match_8_arms | -0.037 | ok |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 31.4ns | 138.4ns | 22.7% | HIGH |
| match_64_arms | 34.3ns | 252.6ns | 13.6% | HIGH |
| match_8_arms | 35.3ns | 228.7ns | 15.4% | HIGH |

## Distribution (algo ns)

```
match_2_arms (n=160, range 82.6-311.2 ns)
     82.6 |########################################
     94.1 |######################
    105.5 |#############################
    116.9 |
    128.4 |
    139.8 |
    151.2 |
    162.6 |
    174.1 |
    185.5 |
    196.9 |
    208.3 |
    219.8 |
    231.2 |
    242.6 |
    254.0 |
    265.5 |###
    276.9 |
    288.3 |####
    299.7 |
  (15 below, 17 above range)

match_64_arms (n=160, range 165.7-510.3 ns)
    165.7 |########################################
    182.9 |######################
    200.2 |
    217.4 |
    234.6 |
    251.8 |###########
    269.1 |
    286.3 |
    303.5 |
    320.8 |
    338.0 |
    355.2 |
    372.5 |
    389.7 |
    406.9 |
    424.2 |
    441.4 |
    458.6 |#
    475.9 |#
    493.1 |##
  (14 below, 17 above range)

match_8_arms (n=160, range 147.6-473.2 ns)
    147.6 |########################################
    163.9 |##
    180.2 |
    196.5 |
    212.7 |########
    229.0 |
    245.3 |
    261.6 |
    277.9 |
    294.2 |
    310.4 |
    326.7 |
    343.0 |
    359.3 |
    375.6 |
    391.8 |
    408.1 |
    424.4 |
    440.7 |
    457.0 |
  (14 below, 19 above range)

```

## Diagnostics

- **match_2_arms**: CV=65.5% (high variance, measurements may be unstable)
- **match_2_arms**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **match_2_arms**: bridge=24.2% of algo (FFI overhead may distort results)
- **match_64_arms**: CV=56.4% (high variance, measurements may be unstable)
- **match_64_arms**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **match_64_arms**: bridge=13.4% of algo (FFI overhead may distort results)
- **match_8_arms**: CV=57.4% (high variance, measurements may be unstable)
- **match_8_arms**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **match_8_arms**: bridge=15.0% of algo (FFI overhead may distort results)
