# Match arm count: 2 vs 8 vs 64 — jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 89% faster than the next best (match_8_arms)

match_2_arms (1.40 us) leads match_8_arms (2.65 us) by 89%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 8.6x slower than the field

match_64_arms (12.03 us) is 8.6x the fastest (1.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (1.40 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 8.6x the fastest

Fastest match_2_arms (1.40 us) to slowest match_64_arms (12.03 us): 8.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 1396.2 ns median
- 2 variants significantly slower than baseline
- Spread: 8.62x (fastest 1396.2 ns, slowest 12030.6 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| match_2_arms | 3564ns | 3551ns | 3423ns | 3559ns | 3719ns | base |
| match_64_arms | 14050ns | 14137ns | 12824ns | 14118ns | 15073ns | +294.22% |
| match_8_arms | 4832ns | 4871ns | 4536ns | 4845ns | 5089ns | +35.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 1406ns | 1350ns | 1476ns | base | 11.657 |
| match_64_arms | 11927ns | 10722ns | 12902ns | +748.59% | 1.374 |
| match_8_arms | 2625ns | 2456ns | 2784ns | +86.73% | 6.243 |

## Performance model

- Peak throughput: **12.141 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 11.734 | 96.7% |
| match_64_arms | 1.362 | 11.2% |
| match_8_arms | 6.193 | 51.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 3564ns | 3564ns | base |
| match_64_arms | 14050ns | 14050ns | +294.22% |
| match_8_arms | 4832ns | 4832ns | +35.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 1396ns | base | --- | [1386, 1405] | --- | --- | --- | --- |
| match_64_arms | 12031ns | +10630.0ns (+761.3%) | [+10549, +10750]ns | [11958, 12110] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 2646ns | +1226.9ns (+87.9%) | [+1212, +1233]ns | [2622, 2651] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 1388ns | +697.5% | +95.5% |
| 1 | 2 | 1375ns | +784.9% | +89.3% |
| 1 | 3 | 1412ns | +793.6% | +88.4% |
| 1 | 4 | 1450ns | +770.9% | +79.7% |
| 1 | 5 | 1403ns | +659.5% | +89.9% |
| 1 | 6 | 1397ns | +766.0% | +89.0% |
| 1 | 7 | 1442ns | +722.2% | +83.8% |
| 1 | 8 | 1352ns | +791.3% | +80.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.215 | moderate- |
| match_64_arms | -0.197 | ok |
| match_8_arms | -0.159 | ok |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.9ns | 1405.5ns | 0.2% |  |
| match_64_arms | 3.1ns | 11927.2ns | 0.0% |  |
| match_8_arms | 3.0ns | 2624.5ns | 0.1% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 1349.5-1475.8 ns)
   1349.5 |###############################
   1355.8 |######################
   1362.1 |########
   1368.4 |####################
   1374.8 |##########################
   1381.1 |###########
   1387.4 |#################
   1393.7 |####################
   1400.0 |###############
   1406.3 |###########
   1412.6 |########
   1418.9 |##
   1425.3 |##
   1431.6 |
   1437.9 |###############################
   1444.2 |########################################
   1450.5 |#################################
   1456.8 |
   1463.1 |
   1469.4 |
  (15 below, 9 above range)

match_64_arms (n=160, range 10722.3-12901.7 ns)
  10722.3 |##################
  10831.2 |#######
  10940.2 |#######
  11049.2 |#######
  11158.2 |
  11267.1 |#######
  11376.1 |#########
  11485.1 |##################
  11594.0 |
  11703.0 |#######################
  11812.0 |###########
  11921.0 |########################################
  12029.9 |##############################
  12138.9 |#####################
  12247.9 |##############################
  12356.8 |##################
  12465.8 |##############
  12574.8 |################
  12683.8 |#########
  12792.7 |#########
  (16 below, 16 above range)

match_8_arms (n=160, range 2455.5-2784.5 ns)
   2455.5 |###
   2472.0 |######
   2488.4 |#
   2504.9 |####
   2521.3 |####
   2537.8 |###
   2554.2 |#
   2570.6 |##
   2587.1 |##############
   2603.5 |#############
   2620.0 |#########
   2636.4 |#########################
   2652.9 |########################################
   2669.3 |######
   2685.8 |##
   2702.2 |#########
   2718.7 |
   2735.1 |#
   2751.6 |
   2768.0 |##
  (20 below, 14 above range)

```
