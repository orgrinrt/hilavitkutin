# Match arm count: 2 vs 8 vs 64 — jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 82% faster than the next best (match_8_arms)

match_2_arms (371 ns) leads match_8_arms (676 ns) by 82%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 2.5x slower than the field

match_64_arms (941 ns) is 2.5x the fastest (371 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_2_arms is fastest but the noisiest (CV 5.8%)

match_2_arms wins on median (371 ns) yet has the highest variance (CV 5.8%), while match_64_arms is the steadiest (CV 4.6%, 941 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (371 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 370.8 ns median
- 2 variants significantly slower than baseline
- Spread: 2.54x (fastest 370.8 ns, slowest 941.0 ns)

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
| match_2_arms | 2586ns | 2583ns | 2418ns | 2574ns | 2792ns | base |
| match_64_arms | 3196ns | 3154ns | 3019ns | 3186ns | 3404ns | +23.60% |
| match_8_arms | 2948ns | 2892ns | 2770ns | 2952ns | 3117ns | +14.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 374ns | 348ns | 406ns | base | 10.961 |
| match_64_arms | 953ns | 898ns | 1017ns | +155.02% | 4.298 |
| match_8_arms | 679ns | 635ns | 727ns | +81.72% | 6.032 |

## Performance model

- Peak throughput: **11.765 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 11.046 | 93.9% |
| match_64_arms | 4.353 | 37.0% |
| match_8_arms | 6.055 | 51.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2586ns | 2586ns | base |
| match_64_arms | 3196ns | 3196ns | +23.60% |
| match_8_arms | 2948ns | 2948ns | +14.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 371ns | base | --- | [369, 374] | --- | --- | --- | --- |
| match_64_arms | 941ns | +576.2ns (+155.4%) | [+569, +582]ns | [939, 943] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 676ns | +302.5ns (+81.6%) | [+295, +308]ns | [667, 686] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 349ns | +171.1% | +106.0% |
| 1 | 2 | 360ns | +159.9% | +93.8% |
| 1 | 3 | 375ns | +161.1% | +93.3% |
| 1 | 4 | 376ns | +150.6% | +67.1% |
| 1 | 5 | 375ns | +151.0% | +72.5% |
| 1 | 6 | 374ns | +155.4% | +81.3% |
| 1 | 7 | 364ns | +175.7% | +84.0% |
| 1 | 8 | 378ns | +135.6% | +76.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | 0.232 | moderate+ |
| match_64_arms | -0.446 | moderate- |
| match_8_arms | 0.141 | ok |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.8ns | 373.7ns | 0.7% |  |
| match_64_arms | 3.2ns | 953.0ns | 0.3% |  |
| match_8_arms | 2.4ns | 679.1ns | 0.4% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 348.2-405.8 ns)
    348.2 |##################
    351.0 |##############
    353.9 |#####################
    356.8 |##############
    359.7 |##
    362.6 |############################
    365.4 |##################
    368.3 |########################################
    371.2 |#####################
    374.1 |############################
    377.0 |#######################
    379.9 |#######
    382.7 |##
    385.6 |###########
    388.5 |
    391.4 |####
    394.3 |#######
    397.1 |#####################
    400.0 |#####################
    402.9 |################
  (16 below, 7 above range)

match_64_arms (n=160, range 898.5-1017.0 ns)
    898.5 |#
    904.4 |#
    910.3 |
    916.2 |#
    922.2 |###
    928.1 |########
    934.0 |########################################
    940.0 |##################################
    945.9 |######
    951.8 |####
    957.8 |#
    963.7 |#
    969.6 |#
    975.5 |#
    981.5 |
    987.4 |
    993.3 |
    999.3 |######
   1005.2 |##################
   1011.1 |###########
  (19 below, 10 above range)

match_8_arms (n=160, range 634.8-727.1 ns)
    634.8 |########
    639.4 |########################
    644.0 |########################################
    648.6 |#####################################
    653.2 |##########
    657.9 |#####################
    662.5 |##########
    667.1 |################
    671.7 |#############
    676.3 |##################
    681.0 |#############
    685.6 |#####
    690.2 |##########
    694.8 |##################
    699.4 |#####################
    704.0 |#############
    708.7 |#############
    713.3 |##################
    717.9 |#############################
    722.5 |##########
  (12 below, 15 above range)

```
