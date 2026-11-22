# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 62% faster than the next best (match_8_arms)

match_2_arms (97 ns) leads match_8_arms (157 ns) by 62%, a clear separation rather than a photo finish. CV 4.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 2.4x slower than the field

match_64_arms (232 ns) is 2.4x the fastest (97 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (97 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 96.7 ns median
- 2 variants significantly slower than baseline
- Spread: 2.40x (fastest 96.7 ns, slowest 231.7 ns)

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
| match_2_arms | 2178ns | 2171ns | 2165ns | 2171ns | 2210ns | base |
| match_64_arms | 2310ns | 2302ns | 2295ns | 2302ns | 2347ns | +6.06% |
| match_8_arms | 2240ns | 2235ns | 2228ns | 2235ns | 2266ns | +2.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 97ns | 91ns | 104ns | base | 10.530 |
| match_64_arms | 232ns | 226ns | 237ns | +138.16% | 4.421 |
| match_8_arms | 157ns | 150ns | 164ns | +61.17% | 6.534 |

## Performance model

- Peak throughput: **11.204 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 10.589 | 94.5% |
| match_64_arms | 4.420 | 39.4% |
| match_8_arms | 6.535 | 58.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2178ns | 2178ns | base |
| match_64_arms | 2310ns | 2310ns | +6.06% |
| match_8_arms | 2240ns | 2240ns | +2.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 97ns | base | --- | [96, 98] | --- | --- | --- | --- |
| match_64_arms | 232ns | +134.2ns (+138.8%) | [+134, +135]ns | [231, 232] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 157ns | +59.2ns (+61.2%) | [+59, +60]ns | [156, 158] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 96ns | +139.5% | +62.7% |
| 1 | 2 | 97ns | +138.3% | +60.3% |
| 1 | 3 | 97ns | +138.4% | +59.8% |
| 1 | 4 | 96ns | +143.2% | +65.3% |
| 1 | 5 | 96ns | +140.0% | +61.1% |
| 1 | 6 | 98ns | +136.2% | +59.8% |
| 1 | 7 | 98ns | +135.0% | +61.8% |
| 1 | 8 | 97ns | +139.2% | +62.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.004 | ok |
| match_64_arms | -0.096 | ok |
| match_8_arms | -0.386 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.8ns | 97.2ns | 2.8% |  |
| match_64_arms | 3.1ns | 231.6ns | 1.3% |  |
| match_8_arms | 3.0ns | 156.7ns | 1.9% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 91.4-103.7 ns)
     91.4 |###
     92.0 |####################################
     92.6 |######
     93.2 |##############################
     93.9 |##################
     94.5 |#################################
     95.1 |###############
     95.7 |########################################
     96.3 |########################
     96.9 |#####################
     97.6 |###############
     98.2 |#####################
     98.8 |###########################
     99.4 |##############################
    100.0 |######
    100.6 |###########################
    101.3 |######
    101.9 |###############
    102.5 |###########################
    103.1 |
  (14 below, 13 above range)

match_64_arms (n=160, range 226.0-237.5 ns)
    226.0 |##############
    226.6 |##############
    227.1 |########
    227.7 |########
    228.3 |###############################
    228.9 |#########################
    229.4 |#########################
    230.0 |########
    230.6 |######################
    231.2 |###############################
    231.7 |###############################
    232.3 |###########
    232.9 |########################################
    233.5 |#################
    234.0 |######################
    234.6 |##############
    235.2 |########
    235.8 |#################
    236.3 |#####
    236.9 |########
  (16 below, 15 above range)

match_8_arms (n=160, range 150.2-163.6 ns)
    150.2 |#########################
    150.9 |##
    151.6 |########################################
    152.2 |#################
    152.9 |##
    153.6 |###############################
    154.2 |####################
    154.9 |####################
    155.6 |###############################
    156.3 |######################
    156.9 |############################
    157.6 |#################
    158.3 |######################
    158.9 |######################
    159.6 |####################
    160.3 |######################
    161.0 |#####
    161.6 |####################
    162.3 |########
    163.0 |#####
  (10 below, 14 above range)

```
