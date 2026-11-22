# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 79% faster than the next best (match_8_arms)

match_2_arms (348 ns) leads match_8_arms (622 ns) by 79%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 2.5x slower than the field

match_64_arms (877 ns) is 2.5x the fastest (348 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (348 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 347.5 ns median
- 2 variants significantly slower than baseline
- Spread: 2.52x (fastest 347.5 ns, slowest 877.1 ns)

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
| match_2_arms | 2428ns | 2419ns | 2413ns | 2420ns | 2467ns | base |
| match_64_arms | 2953ns | 2950ns | 2943ns | 2951ns | 2968ns | +21.62% |
| match_8_arms | 2710ns | 2701ns | 2680ns | 2701ns | 2768ns | +11.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 349ns | 342ns | 357ns | base | 11.748 |
| match_64_arms | 878ns | 870ns | 887ns | +151.75% | 4.667 |
| match_8_arms | 622ns | 602ns | 640ns | +78.43% | 6.584 |

## Performance model

- Peak throughput: **11.966 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 11.787 | 98.5% |
| match_64_arms | 4.670 | 39.0% |
| match_8_arms | 6.582 | 55.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2428ns | 2428ns | base |
| match_64_arms | 2953ns | 2953ns | +21.62% |
| match_8_arms | 2710ns | 2710ns | +11.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 348ns | base | --- | [347, 349] | --- | --- | --- | --- |
| match_64_arms | 877ns | +528.7ns (+152.1%) | [+528, +529]ns | [876, 878] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 622ns | +274.6ns (+79.0%) | [+273, +277]ns | [621, 625] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 347ns | +152.0% | +79.5% |
| 1 | 2 | 347ns | +151.9% | +75.7% |
| 1 | 3 | 347ns | +155.5% | +81.1% |
| 1 | 4 | 347ns | +151.9% | +78.8% |
| 1 | 5 | 348ns | +151.8% | +72.6% |
| 1 | 6 | 350ns | +151.2% | +81.3% |
| 1 | 7 | 349ns | +151.7% | +81.4% |
| 1 | 8 | 348ns | +152.8% | +78.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | 0.340 | moderate+ |
| match_64_arms | -0.402 | moderate- |
| match_8_arms | -0.275 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.5ns | 348.6ns | 0.7% |  |
| match_64_arms | 2.7ns | 877.7ns | 0.3% |  |
| match_8_arms | 2.3ns | 622.1ns | 0.4% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 342.3-356.8 ns)
    342.3 |##############
    343.0 |###########
    343.7 |####################
    344.5 |############################
    345.2 |########################################
    345.9 |#################
    346.6 |########################################
    347.4 |#####################################
    348.1 |##############
    348.8 |##############
    349.5 |######################
    350.2 |##################################
    351.0 |########
    351.7 |######################
    352.4 |##############
    353.1 |##############
    353.9 |#####
    354.6 |########
    355.3 |########
    356.0 |###########
  (14 below, 10 above range)

match_64_arms (n=160, range 870.5-886.6 ns)
    870.5 |##################
    871.3 |###############
    872.1 |###########################
    872.9 |#####################
    873.7 |#########
    874.5 |########################################
    875.3 |##############################
    876.1 |#################################
    876.9 |#################################
    877.7 |###########################
    878.5 |#####################
    879.3 |##############################
    880.1 |##############################
    880.9 |###
    881.7 |############
    882.5 |############
    883.4 |############
    884.2 |###############
    885.0 |######
    885.8 |######
  (14 below, 13 above range)

match_8_arms (n=160, range 602.3-640.5 ns)
    602.3 |############
    604.2 |##################
    606.1 |######
    608.0 |############
    609.9 |##################
    611.8 |###
    613.7 |#####################
    615.6 |##################
    617.6 |###########################
    619.5 |########################################
    621.4 |########################################
    623.3 |##############################
    625.2 |#####################
    627.1 |####################################
    629.0 |#################################
    630.9 |########################
    632.8 |###########################
    634.7 |###############
    636.6 |##################
    638.6 |#########
  (14 below, 4 above range)

```
