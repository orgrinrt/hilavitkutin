# Match arm count: 2 vs 8 vs 64 — jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 64% faster than the next best (match_8_arms)

match_2_arms (404 ns) leads match_8_arms (663 ns) by 64%, a clear separation rather than a photo finish. CV 70.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_2_arms is fastest but the noisiest (CV 70.0%)

match_2_arms wins on median (404 ns) yet has the highest variance (CV 70.0%), while match_8_arms is the steadiest (CV 59.3%, 663 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### match_64_arms shows warm-up / thermal drift (autocorr +0.50)

match_64_arms's per-pass series has lag-1 autocorrelation +0.50, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (404 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### match_2_arms is inconsistent: worst-20% is 2.7x its best-20%

match_2_arms's best 20% of batches run at 360 ns but its worst 20% at 961 ns (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 403.9 ns median
- 2 variants significantly slower than baseline
- Spread: 1.85x (fastest 403.9 ns, slowest 747.2 ns)

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
| match_2_arms | 2892ns | 2286ns | 2071ns | 2288ns | 5522ns | base |
| match_64_arms | 3150ns | 2630ns | 2623ns | 2632ns | 5232ns | +8.94% |
| match_8_arms | 2949ns | 2544ns | 2086ns | 2536ns | 5053ns | +2.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 506ns | 360ns | 961ns | base | 8.089 |
| match_64_arms | 893ns | 744ns | 1479ns | +76.33% | 4.587 |
| match_8_arms | 770ns | 546ns | 1317ns | +51.99% | 5.322 |

## Performance model

- Peak throughput: **11.373 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 10.141 | 89.2% |
| match_64_arms | 5.481 | 48.2% |
| match_8_arms | 6.175 | 54.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2892ns | 2892ns | base |
| match_64_arms | 3150ns | 3150ns | +8.94% |
| match_8_arms | 2949ns | 2949ns | +2.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 404ns | base | --- | [404, 404] | --- | --- | --- | --- |
| match_64_arms | 747ns | +344.1ns (+85.2%) | [+343, +345]ns | [746, 748] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 663ns | +261.0ns (+64.6%) | [+259, +263]ns | [663, 664] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 404ns | +84.5% | +35.4% |
| 1 | 2 | 404ns | +84.4% | +66.0% |
| 1 | 3 | 404ns | +84.0% | +63.3% |
| 1 | 4 | 328ns | +127.8% | +102.2% |
| 1 | 5 | 404ns | +84.8% | +64.0% |
| 1 | 6 | 404ns | +85.4% | +65.1% |
| 1 | 7 | 404ns | +85.1% | +34.8% |
| 1 | 8 | 404ns | +85.5% | +64.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.165 | ok |
| match_64_arms | 0.500 | HIGH+ (drift/warm-up) |
| match_8_arms | -0.254 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 29.7ns | 506.4ns | 5.9% | HIGH |
| match_64_arms | 29.0ns | 892.9ns | 3.2% |  |
| match_8_arms | 27.6ns | 769.6ns | 3.6% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 360.1-961.3 ns)
    360.1 |
    390.2 |########################################
    420.3 |
    450.3 |
    480.4 |
    510.4 |
    540.5 |
    570.5 |
    600.6 |
    630.6 |
    660.7 |
    690.8 |
    720.8 |
    750.9 |
    780.9 |
    811.0 |
    841.0 |
    871.1 |
    901.2 |
    931.2 |
  (16 below, 20 above range)

match_64_arms (n=160, range 743.8-1479.2 ns)
    743.8 |########################################
    780.5 |
    817.3 |
    854.1 |
    890.9 |
    927.6 |
    964.4 |
   1001.2 |
   1038.0 |
   1074.7 |
   1111.5 |
   1148.3 |
   1185.1 |
   1221.8 |
   1258.6 |
   1295.4 |
   1332.2 |
   1368.9 |
   1405.7 |
   1442.5 |
  (15 below, 13 above range)

match_8_arms (n=160, range 545.7-1317.0 ns)
    545.7 |########
    584.3 |
    622.8 |######
    661.4 |########################################
    700.0 |
    738.5 |
    777.1 |
    815.7 |
    854.2 |
    892.8 |
    931.3 |
    969.9 |
   1008.5 |
   1047.0 |
   1085.6 |
   1124.2 |
   1162.7 |
   1201.3 |
   1239.8 |
   1278.4 |
  (16 below, 15 above range)

```

## Diagnostics

- **match_2_arms**: CV=55.8% (high variance, measurements may be unstable)
- **match_64_arms**: CV=50.9% (high variance, measurements may be unstable)
- **match_64_arms**: autocorrelation=0.50 (measurement drift or warm-up artifact)
- **match_8_arms**: CV=51.1% (high variance, measurements may be unstable)
