# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 83% faster than the next best (match_8_arms)

match_2_arms (1.38 us) leads match_8_arms (2.52 us) by 83%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 9.3x slower than the field

match_64_arms (12.88 us) is 9.3x the fastest (1.38 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_8_arms shows alternating (throttle bounce) (autocorr -0.53)

match_8_arms's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (1.38 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.3x the fastest

Fastest match_2_arms (1.38 us) to slowest match_64_arms (12.88 us): 9.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 1378.3 ns median
- 2 variants significantly slower than baseline
- Spread: 9.34x (fastest 1378.3 ns, slowest 12876.5 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| match_2_arms | 3504ns | 3486ns | 3477ns | 3488ns | 3576ns | base |
| match_64_arms | 14970ns | 15014ns | 14360ns | 15020ns | 15427ns | +327.27% |
| match_8_arms | 4645ns | 4633ns | 4590ns | 4638ns | 4720ns | +32.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 1382ns | 1372ns | 1405ns | base | 11.851 |
| match_64_arms | 12845ns | 12237ns | 13298ns | +829.15% | 1.276 |
| match_8_arms | 2523ns | 2478ns | 2586ns | +82.51% | 6.494 |

## Performance model

- Peak throughput: **11.943 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 11.887 | 99.5% |
| match_64_arms | 1.272 | 10.7% |
| match_8_arms | 6.503 | 54.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 3504ns | 3504ns | base |
| match_64_arms | 14970ns | 14970ns | +327.27% |
| match_8_arms | 4645ns | 4645ns | +32.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 1378ns | base | --- | [1377, 1380] | --- | --- | --- | --- |
| match_64_arms | 12876ns | +11505.2ns (+834.7%) | [+11416, +11565]ns | [12793, 12948] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 2520ns | +1136.0ns (+82.4%) | [+1132, +1140]ns | [2515, 2523] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 1376ns | +867.2% | +87.1% |
| 1 | 2 | 1379ns | +852.5% | +79.6% |
| 1 | 3 | 1402ns | +768.5% | +80.3% |
| 1 | 4 | 1378ns | +831.5% | +81.0% |
| 1 | 5 | 1376ns | +834.5% | +83.7% |
| 1 | 6 | 1377ns | +858.3% | +82.5% |
| 1 | 7 | 1378ns | +826.5% | +82.9% |
| 1 | 8 | 1378ns | +816.0% | +80.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.066 | ok |
| match_64_arms | -0.059 | ok |
| match_8_arms | -0.530 | HIGH- (thermal bounce) |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.8ns | 1382.5ns | 0.2% |  |
| match_64_arms | 3.0ns | 12845.2ns | 0.0% |  |
| match_8_arms | 2.9ns | 2523.1ns | 0.1% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 1371.8-1404.9 ns)
   1371.8 |################################
   1373.5 |###########################
   1375.1 |##############################
   1376.8 |########################################
   1378.4 |######################################
   1380.1 |##############
   1381.7 |################
   1383.4 |##############
   1385.0 |#
   1386.7 |#####
   1388.3 |#
   1390.0 |#
   1391.6 |
   1393.3 |###
   1395.0 |###
   1396.6 |
   1398.3 |#
   1399.9 |#######
   1401.6 |#########
   1403.2 |#
  (11 below, 10 above range)

match_64_arms (n=160, range 12237.3-13297.7 ns)
  12237.3 |###########################
  12290.3 |######
  12343.3 |###
  12396.3 |###
  12449.4 |###
  12502.4 |############
  12555.4 |############
  12608.4 |#####################
  12661.4 |########################
  12714.5 |########################################
  12767.5 |############
  12820.5 |###########################
  12873.5 |#####################
  12926.6 |########################
  12979.6 |###
  13032.6 |###############
  13085.6 |########################################
  13138.6 |########################################
  13191.7 |#################################
  13244.7 |###############
  (16 below, 18 above range)

match_8_arms (n=160, range 2478.1-2585.8 ns)
   2478.1 |######################
   2483.5 |####################
   2488.9 |###############
   2494.3 |#################
   2499.7 |
   2505.1 |###############
   2510.4 |#################################
   2515.8 |##########################
   2521.2 |########################################
   2526.6 |###################################
   2532.0 |#################
   2537.3 |##
   2542.7 |######
   2548.1 |##
   2553.5 |
   2558.9 |##
   2564.2 |########
   2569.6 |####################
   2575.0 |###############
   2580.4 |######
  (15 below, 6 above range)

```
