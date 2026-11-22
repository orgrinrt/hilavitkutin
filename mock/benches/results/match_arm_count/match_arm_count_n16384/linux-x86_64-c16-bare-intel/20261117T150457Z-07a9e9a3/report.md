# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 636% faster than the next best (match_64_arms)

match_2_arms (1.30 us) leads match_64_arms (9.59 us) by 636%, a clear separation rather than a photo finish. CV 40.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_8_arms is an outlier: 9.0x slower than the field

match_8_arms (11.68 us) is 9.0x the fastest (1.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_2_arms is fastest but the noisiest (CV 40.2%)

match_2_arms wins on median (1.30 us) yet has the highest variance (CV 40.2%), while match_8_arms is the steadiest (CV 6.8%, 11.68 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### match_64_arms shows alternating (throttle bounce) (autocorr -0.54)

match_64_arms's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (1.30 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.0x the fastest

Fastest match_2_arms (1.30 us) to slowest match_8_arms (11.68 us): 9.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### match_2_arms is inconsistent: worst-20% is 1.6x its best-20%

match_2_arms's best 20% of batches run at 1.29 us but its worst 20% at 2.13 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 1304.0 ns median
- 2 variants significantly slower than baseline
- Spread: 8.95x (fastest 1304.0 ns, slowest 11677.2 ns)

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
| match_2_arms | 3351ns | 2847ns | 2834ns | 3076ns | 4693ns | base |
| match_64_arms | 11950ns | 11168ns | 10698ns | 11751ns | 13800ns | +256.59% |
| match_8_arms | 13417ns | 13578ns | 12023ns | 13566ns | 14363ns | +300.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 1528ns | 1294ns | 2130ns | base | 10.724 |
| match_64_arms | 10247ns | 9130ns | 11874ns | +570.68% | 1.599 |
| match_8_arms | 11552ns | 10335ns | 12439ns | +656.12% | 1.418 |

## Performance model

- Peak throughput: **12.657 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 12.564 | 99.3% |
| match_64_arms | 1.708 | 13.5% |
| match_8_arms | 1.403 | 11.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 3351ns | 3351ns | base |
| match_64_arms | 11950ns | 11950ns | +256.59% |
| match_8_arms | 13417ns | 13417ns | +300.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 1304ns | base | --- | [1297, 1580] | --- | --- | --- | --- |
| match_64_arms | 9594ns | +8093.4ns (+620.7%) | [+7974, +8274]ns | [9441, 10026] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 11677ns | +10191.3ns (+781.5%) | [+10124, +10271]ns | [11583, 11738] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 1296ns | +614.5% | +816.3% |
| 1 | 2 | 1584ns | +621.5% | +653.3% |
| 1 | 3 | 1296ns | +614.9% | +816.5% |
| 1 | 4 | 1584ns | +495.6% | +622.3% |
| 1 | 5 | 1584ns | +510.0% | +649.4% |
| 1 | 6 | 1296ns | +787.4% | +785.4% |
| 1 | 7 | 1295ns | +602.3% | +643.8% |
| 1 | 8 | 1296ns | +795.9% | +782.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.139 | ok |
| match_64_arms | -0.543 | HIGH- (thermal bounce) |
| match_8_arms | 0.096 | ok |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 23.1ns | 1527.8ns | 1.5% |  |
| match_64_arms | 24.9ns | 10246.8ns | 0.2% |  |
| match_8_arms | 24.0ns | 11552.2ns | 0.2% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 1294.4-2129.6 ns)
   1294.4 |########################################
   1336.2 |
   1377.9 |
   1419.7 |
   1461.5 |
   1503.2 |
   1545.0 |#######################
   1586.7 |#####
   1628.5 |
   1670.3 |
   1712.0 |
   1753.8 |
   1795.5 |
   1837.3 |
   1879.1 |
   1920.8 |
   1962.6 |
   2004.3 |
   2046.1 |
   2087.9 |
  (12 below, 9 above range)

match_64_arms (n=160, range 9129.9-11873.7 ns)
   9129.9 |####################################
   9267.1 |########################################
   9404.3 |################
   9541.5 |#########
   9678.7 |#########
   9815.9 |###
   9953.1 |####
  10090.3 |###
  10227.4 |#
  10364.6 |###
  10501.8 |
  10639.0 |
  10776.2 |#
  10913.4 |
  11050.6 |
  11187.8 |#########
  11325.0 |###########################
  11462.1 |###############
  11599.3 |##################
  11736.5 |###
  (16 below, 12 above range)

match_8_arms (n=160, range 10334.8-12439.2 ns)
  10334.8 |#####
  10440.1 |
  10545.3 |
  10650.5 |##
  10755.7 |
  10860.9 |
  10966.1 |
  11071.4 |#######
  11176.6 |#####################################
  11281.8 |####################
  11387.0 |#########################
  11492.2 |#####################################
  11597.5 |###################################
  11702.7 |########################################
  11807.9 |################################
  11913.1 |####################
  12018.3 |################################
  12123.5 |############
  12228.8 |###############
  12334.0 |#######
  (17 below, 11 above range)

```

## Diagnostics

- **match_2_arms**: CV=34.3% (high variance, measurements may be unstable)
