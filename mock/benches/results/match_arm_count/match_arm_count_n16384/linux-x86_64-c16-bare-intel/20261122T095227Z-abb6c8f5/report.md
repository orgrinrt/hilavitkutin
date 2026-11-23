# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 637% faster than the next best (match_64_arms)

match_2_arms (1.55 us) leads match_64_arms (11.46 us) by 637%, a clear separation rather than a photo finish. CV 30.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_8_arms is an outlier: 7.5x slower than the field

match_8_arms (11.61 us) is 7.5x the fastest (1.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_2_arms is fastest but the noisiest (CV 30.1%)

match_2_arms wins on median (1.55 us) yet has the highest variance (CV 30.1%), while match_64_arms is the steadiest (CV 8.0%, 11.46 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (1.55 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 7.5x the fastest

Fastest match_2_arms (1.55 us) to slowest match_8_arms (11.61 us): 7.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### match_2_arms is inconsistent: worst-20% is 1.6x its best-20%

match_2_arms's best 20% of batches run at 1.28 us but its worst 20% at 2.10 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 1554.4 ns median
- 2 variants significantly slower than baseline
- Spread: 7.47x (fastest 1554.4 ns, slowest 11605.5 ns)

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
| match_2_arms | 3415ns | 3435ns | 2819ns | 3211ns | 4624ns | base |
| match_64_arms | 13022ns | 13372ns | 11090ns | 13326ns | 14040ns | +281.28% |
| match_8_arms | 13153ns | 13504ns | 11199ns | 13418ns | 14315ns | +285.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 1553ns | 1278ns | 2098ns | base | 10.549 |
| match_64_arms | 11181ns | 9524ns | 12104ns | +619.89% | 1.465 |
| match_8_arms | 11333ns | 9646ns | 12389ns | +629.69% | 1.446 |

## Performance model

- Peak throughput: **12.819 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 10.540 | 82.2% |
| match_64_arms | 1.430 | 11.2% |
| match_8_arms | 1.412 | 11.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 3415ns | 3415ns | base |
| match_64_arms | 13022ns | 13022ns | +281.28% |
| match_8_arms | 13153ns | 13153ns | +285.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 1554ns | base | --- | [1364, 1564] | --- | --- | --- | --- |
| match_64_arms | 11457ns | +9974.2ns (+641.7%) | [+9882, +10023]ns | [11397, 11537] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 11606ns | +10126.4ns (+651.4%) | [+10071, +10201]ns | [11564, 11681] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 1285ns | +781.8% | +809.1% |
| 1 | 2 | 1583ns | +640.1% | +529.2% |
| 1 | 3 | 1340ns | +767.8% | +772.9% |
| 1 | 4 | 1578ns | +631.3% | +648.9% |
| 1 | 5 | 1583ns | +629.6% | +636.0% |
| 1 | 6 | 1295ns | +632.7% | +635.7% |
| 1 | 7 | 1295ns | +631.2% | +799.5% |
| 1 | 8 | 1565ns | +636.9% | +652.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.319 | moderate- |
| match_64_arms | 0.310 | moderate+ |
| match_8_arms | -0.313 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 23.5ns | 1553.2ns | 1.5% |  |
| match_64_arms | 27.2ns | 11181.2ns | 0.2% |  |
| match_8_arms | 23.4ns | 11333.4ns | 0.2% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 1278.1-2098.2 ns)
   1278.1 |###################################
   1319.1 |####
   1360.1 |###
   1401.1 |
   1442.1 |#
   1483.1 |
   1524.1 |#################
   1565.1 |########################################
   1606.1 |####
   1647.1 |
   1688.2 |
   1729.2 |
   1770.2 |
   1811.2 |
   1852.2 |
   1893.2 |
   1934.2 |
   1975.2 |
   2016.2 |
   2057.2 |
  (16 below, 9 above range)

match_64_arms (n=160, range 9523.9-12104.3 ns)
   9523.9 |##
   9652.9 |#
   9781.9 |####
   9911.0 |####
  10040.0 |#
  10169.0 |
  10298.0 |#
  10427.0 |#
  10556.1 |#
  10685.1 |####
  10814.1 |
  10943.1 |
  11072.2 |##
  11201.2 |####################
  11330.2 |########################################
  11459.2 |#############################
  11588.2 |###################################
  11717.3 |#################
  11846.3 |########
  11975.3 |####
  (22 below, 14 above range)

match_8_arms (n=160, range 9646.3-12389.1 ns)
   9646.3 |########
   9783.5 |####
   9920.6 |
  10057.8 |#######
  10194.9 |#
  10332.0 |##
  10469.2 |#
  10606.3 |
  10743.5 |#
  10880.6 |
  11017.7 |
  11154.9 |#
  11292.0 |#########
  11429.2 |#######################
  11566.3 |########################################
  11703.4 |#################
  11840.6 |###############
  11977.7 |########
  12114.9 |########
  12252.0 |#####
  (17 below, 11 above range)

```

## Diagnostics

- **match_2_arms**: CV=30.1% (high variance, measurements may be unstable)
