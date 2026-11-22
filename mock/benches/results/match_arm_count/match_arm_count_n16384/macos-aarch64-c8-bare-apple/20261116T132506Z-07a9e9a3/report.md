# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 83% faster than the next best (match_8_arms)

match_2_arms (1.35 us) leads match_8_arms (2.47 us) by 83%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 9.4x slower than the field

match_64_arms (12.67 us) is 9.4x the fastest (1.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_8_arms shows alternating (throttle bounce) (autocorr -0.52)

match_8_arms's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (1.35 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.4x the fastest

Fastest match_2_arms (1.35 us) to slowest match_64_arms (12.67 us): 9.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 1350.0 ns median
- 2 variants significantly slower than baseline
- Spread: 9.39x (fastest 1350.0 ns, slowest 12674.5 ns)

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
| match_2_arms | 3433ns | 3430ns | 3424ns | 3430ns | 3447ns | base |
| match_64_arms | 14747ns | 14758ns | 14408ns | 14735ns | 15122ns | +329.62% |
| match_8_arms | 4566ns | 4554ns | 4511ns | 4549ns | 4670ns | +33.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 1351ns | 1345ns | 1361ns | base | 12.125 |
| match_64_arms | 12657ns | 12324ns | 13025ns | +836.68% | 1.294 |
| match_8_arms | 2478ns | 2430ns | 2559ns | +83.37% | 6.612 |

## Performance model

- Peak throughput: **12.185 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 12.136 | 99.6% |
| match_64_arms | 1.293 | 10.6% |
| match_8_arms | 6.628 | 54.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 3433ns | 3433ns | base |
| match_64_arms | 14747ns | 14747ns | +329.62% |
| match_8_arms | 4566ns | 4566ns | +33.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 1350ns | base | --- | [1350, 1351] | --- | --- | --- | --- |
| match_64_arms | 12675ns | +11315.8ns (+838.2%) | [+11249, +11358]ns | [12599, 12710] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 2472ns | +1120.5ns (+83.0%) | [+1117, +1125]ns | [2467, 2475] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 1350ns | +848.0% | +87.1% |
| 1 | 2 | 1350ns | +844.8% | +79.8% |
| 1 | 3 | 1350ns | +838.2% | +83.7% |
| 1 | 4 | 1350ns | +832.9% | +80.9% |
| 1 | 5 | 1350ns | +811.5% | +83.6% |
| 1 | 6 | 1350ns | +859.7% | +82.9% |
| 1 | 7 | 1350ns | +816.7% | +83.2% |
| 1 | 8 | 1353ns | +830.5% | +80.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.115 | ok |
| match_64_arms | -0.445 | moderate- |
| match_8_arms | -0.516 | HIGH- (thermal bounce) |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.9ns | 1351.3ns | 0.2% |  |
| match_64_arms | 3.3ns | 12657.3ns | 0.0% |  |
| match_8_arms | 3.1ns | 2477.9ns | 0.1% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 1344.6-1360.6 ns)
   1344.6 |####
   1345.4 |#######################
   1346.2 |#########################
   1347.0 |##########
   1347.8 |###########################
   1348.6 |##################
   1349.4 |########################################
   1350.2 |############
   1351.0 |#####################
   1351.8 |##############
   1352.6 |##################
   1353.4 |##########
   1354.2 |##################
   1355.0 |##############
   1355.8 |##############
   1356.6 |##
   1357.4 |
   1358.2 |####
   1359.0 |######
   1359.8 |
  (13 below, 10 above range)

match_64_arms (n=160, range 12324.3-13025.3 ns)
  12324.3 |###############################
  12359.4 |##############
  12394.4 |#################
  12429.5 |####################
  12464.5 |#################
  12499.6 |##############
  12534.6 |###########
  12569.7 |########################################
  12604.7 |#################
  12639.8 |#####
  12674.8 |##################################
  12709.9 |###############################
  12744.9 |############################
  12780.0 |######################
  12815.0 |###########
  12850.1 |###########
  12885.1 |#################
  12920.2 |##############
  12955.2 |###########
  12990.3 |###########
  (14 below, 12 above range)

match_8_arms (n=160, range 2429.8-2559.4 ns)
   2429.8 |##################
   2436.2 |#################################
   2442.7 |###########
   2449.2 |###
   2455.7 |########
   2462.2 |##################
   2468.7 |########################################
   2475.1 |####################################
   2481.6 |##################
   2488.1 |########
   2494.6 |
   2501.1 |#
   2507.5 |#
   2514.0 |##########
   2520.5 |###########
   2527.0 |#####
   2533.5 |#
   2539.9 |
   2546.4 |#
   2552.9 |#
  (15 below, 6 above range)

```
