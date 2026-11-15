# Match arm count: 2 vs 8 vs 64 — jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 51% faster than the next best (match_8_arms)

match_2_arms (106 ns) leads match_8_arms (161 ns) by 51%, a clear separation rather than a photo finish. CV 88.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_2_arms is fastest but the noisiest (CV 88.0%)

match_2_arms wins on median (106 ns) yet has the highest variance (CV 88.0%), while match_8_arms is the steadiest (CV 83.9%, 161 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (74 ns) is smaller than the fastest variant's own run-to-run std-dev (94 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (106 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### match_2_arms is inconsistent: worst-20% is 3.8x its best-20%

match_2_arms's best 20% of batches run at 88 ns but its worst 20% at 329 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 106.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.70x (fastest 106.5 ns, slowest 180.8 ns)

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
| match_2_arms | 2794ns | 1988ns | 1639ns | 2049ns | 6184ns | base |
| match_64_arms | 2767ns | 2061ns | 1685ns | 2062ns | 5961ns | -0.97% |
| match_8_arms | 2662ns | 2041ns | 1670ns | 2038ns | 5525ns | -4.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 149ns | 88ns | 329ns | base | 6.876 |
| match_64_arms | 243ns | 148ns | 523ns | +63.35% | 4.209 |
| match_8_arms | 210ns | 132ns | 435ns | +40.84% | 4.882 |

## Performance model

- Peak throughput: **11.659 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 9.615 | 82.5% |
| match_64_arms | 5.665 | 48.6% |
| match_8_arms | 6.352 | 54.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2794ns | 2794ns | base |
| match_64_arms | 2767ns | 2767ns | -0.97% |
| match_8_arms | 2662ns | 2662ns | -4.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 106ns | base | --- | [106, 107] | --- | --- | --- | --- |
| match_64_arms | 181ns | +77.0ns (+72.3%) | [+76, +80]ns | [180, 181] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 161ns | +55.2ns (+51.8%) | [+54, +56]ns | [160, 162] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 107ns | +38.8% | +22.8% |
| 1 | 2 | 108ns | +67.9% | +50.5% |
| 1 | 3 | 106ns | +40.2% | +25.4% |
| 1 | 4 | 107ns | +67.8% | +51.3% |
| 1 | 5 | 107ns | +71.1% | +50.7% |
| 1 | 6 | 107ns | +72.3% | +48.7% |
| 1 | 7 | 87ns | +105.7% | +85.2% |
| 1 | 8 | 88ns | +105.8% | +81.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | 0.459 | moderate+ |
| match_64_arms | -0.137 | ok |
| match_8_arms | -0.258 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 33.3ns | 148.9ns | 22.3% | HIGH |
| match_64_arms | 32.6ns | 243.3ns | 13.4% | HIGH |
| match_8_arms | 31.5ns | 209.8ns | 15.0% | HIGH |

## Distribution (algo ns)

```
match_2_arms (n=160, range 87.8-329.5 ns)
     87.8 |######
     99.9 |########################################
    112.0 |
    124.1 |
    136.2 |
    148.2 |
    160.3 |
    172.4 |
    184.5 |
    196.6 |
    208.7 |
    220.7 |
    232.8 |
    244.9 |
    257.0 |
    269.1 |
    281.2 |#
    293.2 |###
    305.3 |#
    317.4 |
  (22 below, 15 above range)

match_64_arms (n=160, range 148.4-523.2 ns)
    148.4 |#####
    167.2 |########################################
    185.9 |#
    204.6 |
    223.4 |
    242.1 |
    260.8 |
    279.6 |
    298.3 |
    317.1 |
    335.8 |
    354.5 |
    373.3 |
    392.0 |
    410.8 |
    429.5 |
    448.2 |
    467.0 |
    485.7 |
    504.4 |
  (19 below, 21 above range)

match_8_arms (n=160, range 132.0-435.2 ns)
    132.0 |########
    147.2 |########################################
    162.3 |########
    177.5 |
    192.7 |
    207.8 |
    223.0 |
    238.1 |
    253.3 |
    268.5 |
    283.6 |
    298.8 |
    313.9 |
    329.1 |
    344.3 |
    359.4 |
    374.6 |
    389.7 |
    404.9 |
    420.1 |
  (15 below, 19 above range)

```

## Diagnostics

- **match_2_arms**: CV=62.9% (high variance, measurements may be unstable)
- **match_2_arms**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **match_2_arms**: bridge=22.3% of algo (FFI overhead may distort results)
- **match_64_arms**: CV=63.0% (high variance, measurements may be unstable)
- **match_64_arms**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **match_64_arms**: bridge=13.5% of algo (FFI overhead may distort results)
- **match_8_arms**: CV=64.5% (high variance, measurements may be unstable)
- **match_8_arms**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **match_8_arms**: bridge=15.0% of algo (FFI overhead may distort results)
