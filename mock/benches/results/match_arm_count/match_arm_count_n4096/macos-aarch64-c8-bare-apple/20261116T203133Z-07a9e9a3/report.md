# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 78% faster than the next best (match_8_arms)

match_2_arms (348 ns) leads match_8_arms (619 ns) by 78%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 2.5x slower than the field

match_64_arms (880 ns) is 2.5x the fastest (348 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (348 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 347.9 ns median
- 2 variants significantly slower than baseline
- Spread: 2.53x (fastest 347.9 ns, slowest 880.0 ns)

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
| match_2_arms | 2428ns | 2422ns | 2414ns | 2422ns | 2459ns | base |
| match_64_arms | 2975ns | 2956ns | 2944ns | 2957ns | 3064ns | +22.55% |
| match_8_arms | 2721ns | 2698ns | 2677ns | 2702ns | 2823ns | +12.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 349ns | 342ns | 358ns | base | 11.738 |
| match_64_arms | 885ns | 871ns | 914ns | +153.68% | 4.627 |
| match_8_arms | 623ns | 602ns | 654ns | +78.63% | 6.571 |

## Performance model

- Peak throughput: **11.966 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 11.773 | 98.4% |
| match_64_arms | 4.655 | 38.9% |
| match_8_arms | 6.617 | 55.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2428ns | 2428ns | base |
| match_64_arms | 2975ns | 2975ns | +22.55% |
| match_8_arms | 2721ns | 2721ns | +12.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 348ns | base | --- | [347, 349] | --- | --- | --- | --- |
| match_64_arms | 880ns | +530.4ns (+152.5%) | [+529, +532]ns | [878, 881] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 619ns | +272.1ns (+78.2%) | [+269, +273]ns | [617, 623] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 348ns | +161.8% | +89.4% |
| 1 | 2 | 349ns | +150.2% | +74.4% |
| 1 | 3 | 347ns | +154.5% | +82.9% |
| 1 | 4 | 347ns | +152.5% | +78.6% |
| 1 | 5 | 346ns | +152.8% | +73.6% |
| 1 | 6 | 351ns | +150.5% | +80.9% |
| 1 | 7 | 349ns | +151.4% | +74.7% |
| 1 | 8 | 348ns | +156.0% | +77.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.122 | ok |
| match_64_arms | -0.199 | ok |
| match_8_arms | -0.394 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.8ns | 349.0ns | 0.8% |  |
| match_64_arms | 2.9ns | 885.2ns | 0.3% |  |
| match_8_arms | 2.3ns | 623.3ns | 0.4% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 342.3-358.3 ns)
    342.3 |############
    343.1 |######################
    343.9 |##############################
    344.7 |########################################
    345.5 |####################
    346.3 |###########################
    347.1 |#################
    347.9 |######################
    348.7 |#######
    349.5 |#################
    350.3 |###################################
    351.1 |################################
    351.9 |#################
    352.7 |############
    353.5 |####################
    354.3 |
    355.1 |##
    355.9 |#####
    356.7 |##
    357.5 |##########
  (13 below, 5 above range)

match_64_arms (n=160, range 870.8-913.8 ns)
    870.8 |#################################
    872.9 |###########################
    875.1 |#####################################
    877.2 |###################################
    879.4 |########################################
    881.5 |#######################
    883.7 |############
    885.8 |################
    888.0 |####
    890.1 |############
    892.3 |##########
    894.4 |##
    896.6 |
    898.7 |######
    900.9 |
    903.0 |######
    905.2 |##
    907.3 |########
    909.5 |##
    911.6 |####
  (12 below, 12 above range)

match_8_arms (n=160, range 601.5-653.5 ns)
    601.5 |##############################
    604.1 |########################################
    606.7 |#####################
    609.3 |##############################
    611.9 |########################################
    614.5 |########################
    617.1 |####################################
    619.7 |############
    622.3 |###########################
    624.9 |##################
    627.5 |############
    630.1 |########################
    632.7 |###############
    635.3 |#####################
    637.9 |###############
    640.5 |##################
    643.1 |############
    645.7 |#########
    648.3 |
    650.9 |###
  (12 below, 13 above range)

```
