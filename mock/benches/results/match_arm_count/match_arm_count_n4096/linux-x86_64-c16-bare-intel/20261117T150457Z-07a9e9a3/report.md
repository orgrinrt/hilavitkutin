# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 68% faster than the next best (match_8_arms)

match_2_arms (396 ns) leads match_8_arms (666 ns) by 68%, a clear separation rather than a photo finish. CV 77.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_2_arms is fastest but the noisiest (CV 77.6%)

match_2_arms wins on median (396 ns) yet has the highest variance (CV 77.6%), while match_8_arms is the steadiest (CV 55.0%, 666 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (396 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### match_2_arms is inconsistent: worst-20% is 3.3x its best-20%

match_2_arms's best 20% of batches run at 323 ns but its worst 20% at 1.06 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 396.1 ns median
- 2 variants significantly slower than baseline
- Spread: 1.88x (fastest 396.1 ns, slowest 746.2 ns)

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
| match_2_arms | 2895ns | 2287ns | 1867ns | 2167ns | 6107ns | base |
| match_64_arms | 3004ns | 2632ns | 2164ns | 2617ns | 5007ns | +3.76% |
| match_8_arms | 2801ns | 2554ns | 2085ns | 2486ns | 4461ns | -3.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 506ns | 323ns | 1064ns | base | 8.101 |
| match_64_arms | 852ns | 619ns | 1412ns | +68.42% | 4.810 |
| match_8_arms | 732ns | 542ns | 1166ns | +44.75% | 5.596 |

## Performance model

- Peak throughput: **12.665 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 10.340 | 81.6% |
| match_64_arms | 5.489 | 43.3% |
| match_8_arms | 6.146 | 48.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2895ns | 2895ns | base |
| match_64_arms | 3004ns | 3004ns | +3.76% |
| match_8_arms | 2801ns | 2801ns | -3.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 396ns | base | --- | [393, 404] | --- | --- | --- | --- |
| match_64_arms | 746ns | +341.7ns (+86.3%) | [+341, +343]ns | [746, 746] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 666ns | +264.6ns (+66.8%) | [+252, +275]ns | [666, 668] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 404ns | +84.4% | +65.5% |
| 1 | 2 | 331ns | +125.6% | +103.5% |
| 1 | 3 | 331ns | +84.4% | +65.1% |
| 1 | 4 | 405ns | +84.3% | +36.8% |
| 1 | 5 | 405ns | +84.4% | +34.2% |
| 1 | 6 | 405ns | +84.3% | +65.8% |
| 1 | 7 | 331ns | +126.4% | +101.0% |
| 1 | 8 | 331ns | +89.9% | +101.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | 0.128 | ok |
| match_64_arms | -0.223 | moderate- |
| match_8_arms | 0.340 | moderate+ |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 29.8ns | 505.6ns | 5.9% | HIGH |
| match_64_arms | 27.7ns | 851.6ns | 3.3% |  |
| match_8_arms | 26.3ns | 731.9ns | 3.6% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 323.4-1063.9 ns)
    323.4 |###########################
    360.4 |################
    397.5 |########################################
    434.5 |
    471.5 |
    508.5 |
    545.6 |
    582.6 |
    619.6 |
    656.6 |
    693.7 |
    730.7 |
    767.7 |
    804.7 |#
    841.8 |
    878.8 |
    915.8 |
    952.8 |####
    989.9 |
   1026.9 |#
  (27 below, 18 above range)

match_64_arms (n=160, range 619.5-1412.3 ns)
    619.5 |#######
    659.1 |
    698.7 |
    738.4 |########################################
    778.0 |
    817.7 |
    857.3 |
    897.0 |
    936.6 |
    976.3 |
   1015.9 |
   1055.5 |
   1095.2 |
   1134.8 |
   1174.5 |
   1214.1 |
   1253.8 |
   1293.4 |
   1333.0 |
   1372.7 |
  (15 below, 14 above range)

match_8_arms (n=160, range 541.7-1165.7 ns)
    541.7 |####################
    572.9 |#
    604.1 |#
    635.3 |###################
    666.5 |########################################
    697.7 |
    728.9 |
    760.1 |
    791.3 |
    822.5 |
    853.7 |
    884.9 |
    916.1 |
    947.3 |
    978.5 |
   1009.7 |
   1040.9 |
   1072.1 |
   1103.3 |#
   1134.5 |
  (10 below, 11 above range)

```

## Diagnostics

- **match_2_arms**: CV=60.8% (high variance, measurements may be unstable)
- **match_2_arms**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **match_64_arms**: CV=52.1% (high variance, measurements may be unstable)
- **match_8_arms**: CV=50.1% (high variance, measurements may be unstable)
