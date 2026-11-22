# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 61% faster than the next best (match_8_arms)

match_2_arms (100 ns) leads match_8_arms (160 ns) by 61%, a clear separation rather than a photo finish. CV 4.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 2.4x slower than the field

match_64_arms (237 ns) is 2.4x the fastest (100 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (100 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 99.6 ns median
- 2 variants significantly slower than baseline
- Spread: 2.38x (fastest 99.6 ns, slowest 236.7 ns)

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
| match_2_arms | 2204ns | 2201ns | 2190ns | 2201ns | 2227ns | base |
| match_64_arms | 2338ns | 2334ns | 2325ns | 2334ns | 2364ns | +6.11% |
| match_8_arms | 2267ns | 2264ns | 2255ns | 2263ns | 2288ns | +2.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 100ns | 93ns | 106ns | base | 10.290 |
| match_64_arms | 237ns | 232ns | 243ns | +138.05% | 4.323 |
| match_8_arms | 161ns | 153ns | 168ns | +61.38% | 6.376 |

## Performance model

- Peak throughput: **10.979 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 10.281 | 93.6% |
| match_64_arms | 4.326 | 39.4% |
| match_8_arms | 6.384 | 58.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2204ns | 2204ns | base |
| match_64_arms | 2338ns | 2338ns | +6.11% |
| match_8_arms | 2267ns | 2267ns | +2.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 100ns | base | --- | [99, 100] | --- | --- | --- | --- |
| match_64_arms | 237ns | +137.1ns (+137.7%) | [+137, +138]ns | [236, 238] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 160ns | +60.8ns (+61.0%) | [+60, +62]ns | [159, 162] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 99ns | +136.0% | +63.3% |
| 1 | 2 | 101ns | +135.3% | +55.7% |
| 1 | 3 | 99ns | +139.4% | +58.4% |
| 1 | 4 | 100ns | +140.1% | +65.1% |
| 1 | 5 | 99ns | +140.6% | +63.4% |
| 1 | 6 | 100ns | +136.1% | +58.7% |
| 1 | 7 | 100ns | +135.5% | +62.2% |
| 1 | 8 | 100ns | +137.6% | +59.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.497 | moderate- |
| match_64_arms | -0.060 | ok |
| match_8_arms | -0.260 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.7ns | 99.5ns | 2.8% |  |
| match_64_arms | 3.1ns | 236.9ns | 1.3% |  |
| match_8_arms | 2.9ns | 160.6ns | 1.8% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 93.3-105.8 ns)
     93.3 |#############
     93.9 |###########
     94.5 |###########
     95.1 |######
     95.8 |####################
     96.4 |######
     97.0 |###############################
     97.7 |########
     98.3 |########################
     98.9 |########
     99.5 |########################################
    100.2 |########
    100.8 |########################
    101.4 |########
    102.0 |######################
    102.7 |####
    103.3 |######################
    103.9 |###########
    104.5 |###########
    105.2 |###########
  (11 below, 11 above range)

match_64_arms (n=160, range 231.6-242.6 ns)
    231.6 |#####################
    232.2 |#####
    232.7 |########################
    233.3 |#############################
    233.8 |########################
    234.4 |#############
    234.9 |##########################
    235.5 |#############
    236.0 |################
    236.6 |########################################
    237.1 |#############
    237.7 |########
    238.2 |##########
    238.7 |#############################
    239.3 |#############
    239.8 |##########
    240.4 |################################
    240.9 |##########
    241.5 |#####
    242.0 |#############
  (11 below, 14 above range)

match_8_arms (n=160, range 153.2-168.3 ns)
    153.2 |###############
    154.0 |########################
    154.7 |########################
    155.5 |############
    156.2 |######
    157.0 |########################################
    157.7 |#####################
    158.5 |###########################
    159.2 |#########
    160.0 |##############################
    160.7 |########################
    161.5 |##################
    162.2 |###########################
    163.0 |##################
    163.7 |#################################
    164.5 |###############
    165.3 |#################################
    166.0 |##################
    166.8 |############
    167.5 |###
  (13 below, 11 above range)

```
