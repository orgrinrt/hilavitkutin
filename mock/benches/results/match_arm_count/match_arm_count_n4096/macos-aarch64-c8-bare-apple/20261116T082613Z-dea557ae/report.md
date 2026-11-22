# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 78% faster than the next best (match_8_arms)

match_2_arms (355 ns) leads match_8_arms (632 ns) by 78%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 2.5x slower than the field

match_64_arms (894 ns) is 2.5x the fastest (355 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_8_arms shows alternating (throttle bounce) (autocorr -0.70)

match_8_arms's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (355 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 355.2 ns median
- 2 variants significantly slower than baseline
- Spread: 2.52x (fastest 355.2 ns, slowest 893.5 ns)

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
| match_2_arms | 2458ns | 2455ns | 2446ns | 2455ns | 2482ns | base |
| match_64_arms | 2999ns | 2993ns | 2985ns | 2994ns | 3030ns | +21.99% |
| match_8_arms | 2738ns | 2733ns | 2716ns | 2733ns | 2774ns | +11.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 355ns | 349ns | 362ns | base | 11.526 |
| match_64_arms | 895ns | 888ns | 906ns | +151.76% | 4.578 |
| match_8_arms | 631ns | 614ns | 648ns | +77.59% | 6.490 |

## Performance model

- Peak throughput: **11.726 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 11.532 | 98.3% |
| match_64_arms | 4.584 | 39.1% |
| match_8_arms | 6.476 | 55.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2458ns | 2458ns | base |
| match_64_arms | 2999ns | 2999ns | +21.99% |
| match_8_arms | 2738ns | 2738ns | +11.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 355ns | base | --- | [355, 357] | --- | --- | --- | --- |
| match_64_arms | 894ns | +537.9ns (+151.4%) | [+537, +539]ns | [892, 894] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 632ns | +276.7ns (+77.9%) | [+275, +279]ns | [629, 634] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 355ns | +151.2% | +79.0% |
| 1 | 2 | 354ns | +151.8% | +75.4% |
| 1 | 3 | 355ns | +151.2% | +80.3% |
| 1 | 4 | 355ns | +151.0% | +79.0% |
| 1 | 5 | 355ns | +151.2% | +72.0% |
| 1 | 6 | 356ns | +152.6% | +82.2% |
| 1 | 7 | 355ns | +151.7% | +75.0% |
| 1 | 8 | 357ns | +151.5% | +76.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.124 | ok |
| match_64_arms | -0.003 | ok |
| match_8_arms | -0.700 | HIGH- (thermal bounce) |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.9ns | 355.4ns | 0.8% |  |
| match_64_arms | 3.0ns | 894.7ns | 0.3% |  |
| match_8_arms | 2.5ns | 631.1ns | 0.4% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 349.3-362.1 ns)
    349.3 |############
    349.9 |###########################
    350.6 |###########################
    351.2 |##################
    351.9 |############
    352.5 |#################################
    353.1 |############
    353.8 |##################
    354.4 |########################################
    355.0 |##################
    355.7 |##################
    356.3 |#####################
    357.0 |########################################
    357.6 |###############
    358.2 |##################
    358.9 |#########
    359.5 |###########################
    360.2 |##################
    360.8 |############
    361.4 |#########
  (14 below, 12 above range)

match_64_arms (n=160, range 887.7-905.7 ns)
    887.7 |##################
    888.6 |#####################
    889.5 |########################################
    890.4 |###################################
    891.3 |################
    892.2 |#####################
    893.1 |#####################
    894.0 |#######################
    894.9 |#######################
    895.8 |################
    896.7 |############################
    897.6 |###########
    898.5 |#######
    899.4 |##############
    900.3 |###########
    901.2 |##
    902.1 |##
    903.0 |##
    903.9 |####
    904.8 |
  (12 below, 11 above range)

match_8_arms (n=160, range 614.0-647.8 ns)
    614.0 |######
    615.7 |#############
    617.4 |####################
    619.0 |####################################
    620.7 |#############
    622.4 |##########################
    624.1 |################
    625.8 |####################
    627.5 |##########
    629.2 |#######################
    630.9 |#################################
    632.6 |########################################
    634.3 |####################
    635.9 |##########################
    637.6 |##########################
    639.3 |####################################
    641.0 |################
    642.7 |#######################
    644.4 |####################
    646.1 |
  (16 below, 15 above range)

```
