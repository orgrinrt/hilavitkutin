# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 61% faster than the next best (match_8_arms)

match_2_arms (97 ns) leads match_8_arms (157 ns) by 61%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 2.4x slower than the field

match_64_arms (231 ns) is 2.4x the fastest (97 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (97 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 97.1 ns median
- 2 variants significantly slower than baseline
- Spread: 2.38x (fastest 97.1 ns, slowest 231.2 ns)

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
| match_2_arms | 2170ns | 2168ns | 2163ns | 2168ns | 2183ns | base |
| match_64_arms | 2310ns | 2302ns | 2295ns | 2302ns | 2348ns | +6.45% |
| match_8_arms | 2237ns | 2232ns | 2227ns | 2233ns | 2260ns | +3.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 97ns | 91ns | 103ns | base | 10.556 |
| match_64_arms | 231ns | 226ns | 237ns | +138.49% | 4.426 |
| match_8_arms | 157ns | 150ns | 164ns | +61.64% | 6.530 |

## Performance model

- Peak throughput: **11.248 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 10.546 | 93.8% |
| match_64_arms | 4.429 | 39.4% |
| match_8_arms | 6.535 | 58.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2170ns | 2170ns | base |
| match_64_arms | 2310ns | 2310ns | +6.45% |
| match_8_arms | 2237ns | 2237ns | +3.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 97ns | base | --- | [95, 98] | --- | --- | --- | --- |
| match_64_arms | 231ns | +134.0ns (+138.0%) | [+134, +135]ns | [231, 232] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 157ns | +59.6ns (+61.4%) | [+59, +60]ns | [156, 158] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 95ns | +140.6% | +65.4% |
| 1 | 2 | 98ns | +138.1% | +59.6% |
| 1 | 3 | 97ns | +138.4% | +58.0% |
| 1 | 4 | 96ns | +142.1% | +65.5% |
| 1 | 5 | 95ns | +142.6% | +65.1% |
| 1 | 6 | 97ns | +138.0% | +61.1% |
| 1 | 7 | 98ns | +135.2% | +59.6% |
| 1 | 8 | 96ns | +142.1% | +62.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.237 | moderate- |
| match_64_arms | -0.220 | moderate- |
| match_8_arms | -0.212 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 2.6ns | 97.0ns | 2.7% |  |
| match_64_arms | 3.0ns | 231.4ns | 1.3% |  |
| match_8_arms | 2.9ns | 156.8ns | 1.9% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 91.0-103.5 ns)
     91.0 |############
     91.7 |########################
     92.3 |########################################
     92.9 |##################
     93.5 |###############
     94.1 |###########################
     94.8 |########################################
     95.4 |########################
     96.0 |
     96.6 |#####################
     97.3 |#####################
     97.9 |###############
     98.5 |########################
     99.1 |#################################
     99.7 |############
    100.4 |####################################
    101.0 |#########
    101.6 |########################
    102.2 |######
    102.9 |###############
  (10 below, 12 above range)

match_64_arms (n=160, range 226.0-237.4 ns)
    226.0 |######################
    226.6 |####################
    227.1 |############
    227.7 |##########
    228.3 |#################
    228.9 |##########
    229.4 |#####################################
    230.0 |#######
    230.6 |###############
    231.1 |########################################
    231.7 |####################
    232.3 |################################
    232.9 |################################
    233.4 |##########
    234.0 |#####
    234.6 |###############
    235.1 |######################
    235.7 |#####
    236.3 |##
    236.9 |#######
  (12 below, 10 above range)

match_8_arms (n=160, range 149.8-164.1 ns)
    149.8 |#########
    150.5 |##################
    151.2 |#########
    151.9 |#####################
    152.7 |##############################
    153.4 |############
    154.1 |########################
    154.8 |########################################
    155.5 |####################################
    156.2 |############
    156.9 |#####################
    157.7 |########################################
    158.4 |#####################
    159.1 |###########################
    159.8 |######
    160.5 |#################################
    161.2 |#########
    162.0 |#####################
    162.7 |############
    163.4 |
  (13 below, 14 above range)

```
