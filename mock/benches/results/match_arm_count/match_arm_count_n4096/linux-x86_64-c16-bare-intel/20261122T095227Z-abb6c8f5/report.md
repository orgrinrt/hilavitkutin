# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 63% faster than the next best (match_8_arms)

match_2_arms (406 ns) leads match_8_arms (664 ns) by 63%, a clear separation rather than a photo finish. CV 72.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_2_arms is fastest but the noisiest (CV 72.7%)

match_2_arms wins on median (406 ns) yet has the highest variance (CV 72.7%), while match_64_arms is the steadiest (CV 52.6%, 745 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (406 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### match_2_arms is inconsistent: worst-20% is 2.8x its best-20%

match_2_arms's best 20% of batches run at 361 ns but its worst 20% at 1.02 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 406.4 ns median
- 2 variants significantly slower than baseline
- Spread: 1.83x (fastest 406.4 ns, slowest 745.4 ns)

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
| match_2_arms | 2946ns | 2285ns | 2065ns | 2286ns | 5806ns | base |
| match_64_arms | 2832ns | 2627ns | 2152ns | 2518ns | 4454ns | -3.87% |
| match_8_arms | 2965ns | 2549ns | 2082ns | 2460ns | 5362ns | +0.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 518ns | 361ns | 1019ns | base | 7.907 |
| match_64_arms | 804ns | 612ns | 1257ns | +55.11% | 5.097 |
| match_8_arms | 772ns | 542ns | 1394ns | +49.11% | 5.303 |

## Performance model

- Peak throughput: **11.340 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 10.080 | 88.9% |
| match_64_arms | 5.495 | 48.5% |
| match_8_arms | 6.171 | 54.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2946ns | 2946ns | base |
| match_64_arms | 2832ns | 2832ns | -3.87% |
| match_8_arms | 2965ns | 2965ns | +0.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 406ns | base | --- | [404, 407] | --- | --- | --- | --- |
| match_64_arms | 745ns | +337.7ns (+83.1%) | [+309, +338]ns | [744, 746] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 664ns | +257.8ns (+63.4%) | [+255, +262]ns | [663, 665] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 406ns | +83.6% | +64.7% |
| 1 | 2 | 408ns | +83.1% | +34.6% |
| 1 | 3 | 405ns | +83.5% | +32.9% |
| 1 | 4 | 404ns | +84.5% | +63.8% |
| 1 | 5 | 407ns | +50.6% | +33.8% |
| 1 | 6 | 408ns | +83.1% | +63.2% |
| 1 | 7 | 332ns | +84.3% | +100.2% |
| 1 | 8 | 405ns | +54.8% | +64.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.167 | ok |
| match_64_arms | 0.009 | ok |
| match_8_arms | -0.137 | ok |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 30.3ns | 518.0ns | 5.8% | HIGH |
| match_64_arms | 25.9ns | 803.5ns | 3.2% |  |
| match_8_arms | 27.8ns | 772.4ns | 3.6% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 361.2-1019.5 ns)
    361.2 |#######
    394.1 |########################################
    427.0 |
    460.0 |
    492.9 |
    525.8 |#
    558.7 |
    591.6 |
    624.5 |
    657.4 |
    690.3 |
    723.3 |
    756.2 |
    789.1 |
    822.0 |
    854.9 |
    887.8 |
    920.7 |
    953.7 |
    986.6 |
  (17 below, 21 above range)

match_64_arms (n=160, range 612.3-1257.2 ns)
    612.3 |################
    644.6 |
    676.8 |
    709.1 |
    741.3 |########################################
    773.6 |
    805.8 |
    838.0 |
    870.3 |
    902.5 |
    934.8 |
    967.0 |
    999.2 |
   1031.5 |
   1063.7 |
   1096.0 |
   1128.2 |
   1160.5 |
   1192.7 |
   1224.9 |
  (17 below, 12 above range)

match_8_arms (n=160, range 541.9-1394.0 ns)
    541.9 |##################
    584.5 |
    627.1 |########################################
    669.7 |#
    712.3 |
    754.9 |
    797.5 |
    840.1 |
    882.7 |
    925.3 |
    967.9 |
   1010.6 |
   1053.2 |
   1095.8 |
   1138.4 |
   1181.0 |
   1223.6 |
   1266.2 |
   1308.8 |
   1351.4 |
  (13 below, 16 above range)

```

## Diagnostics

- **match_2_arms**: CV=57.0% (high variance, measurements may be unstable)
- **match_64_arms**: CV=48.8% (high variance, measurements may be unstable)
- **match_8_arms**: CV=54.0% (high variance, measurements may be unstable)
