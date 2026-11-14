# Match arm count: 2 vs 8 vs 64 — jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 58% faster than the next best (match_8_arms)

match_2_arms (107 ns) leads match_8_arms (169 ns) by 58%, a clear separation rather than a photo finish. CV 8.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### match_64_arms is an outlier: 2.3x slower than the field

match_64_arms (249 ns) is 2.3x the fastest (107 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### match_2_arms is fastest but the noisiest (CV 8.8%)

match_2_arms wins on median (107 ns) yet has the highest variance (CV 8.8%), while match_64_arms is the steadiest (CV 5.9%, 249 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (107 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 107.1 ns median
- 2 variants significantly slower than baseline
- Spread: 2.33x (fastest 107.1 ns, slowest 249.2 ns)

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
| match_2_arms | 2380ns | 2482ns | 2181ns | 2403ns | 2509ns | base |
| match_64_arms | 2490ns | 2461ns | 2297ns | 2498ns | 2661ns | +4.63% |
| match_8_arms | 2405ns | 2378ns | 2232ns | 2403ns | 2586ns | +1.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 107ns | 96ns | 120ns | base | 9.545 |
| match_64_arms | 251ns | 230ns | 271ns | +133.62% | 4.086 |
| match_8_arms | 169ns | 156ns | 184ns | +57.86% | 6.047 |

## Performance model

- Peak throughput: **10.624 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 9.561 | 90.0% |
| match_64_arms | 4.109 | 38.7% |
| match_8_arms | 6.052 | 57.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2380ns | 2380ns | base |
| match_64_arms | 2490ns | 2490ns | +4.63% |
| match_8_arms | 2405ns | 2405ns | +1.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 107ns | base | --- | [106, 109] | --- | --- | --- | --- |
| match_64_arms | 249ns | +142.9ns (+133.4%) | [+138, +146]ns | [247, 251] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 169ns | +62.5ns (+58.4%) | [+60, +65]ns | [167, 172] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 98ns | +145.1% | +79.7% |
| 1 | 2 | 106ns | +119.7% | +58.8% |
| 1 | 3 | 106ns | +148.7% | +58.5% |
| 1 | 4 | 109ns | +146.2% | +50.5% |
| 1 | 5 | 111ns | +124.4% | +52.4% |
| 1 | 6 | 109ns | +118.0% | +58.7% |
| 1 | 7 | 99ns | +165.5% | +80.7% |
| 1 | 8 | 109ns | +140.8% | +43.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.102 | ok |
| match_64_arms | 0.110 | ok |
| match_8_arms | -0.229 | moderate- |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 3.0ns | 107.3ns | 2.8% |  |
| match_64_arms | 3.2ns | 250.6ns | 1.3% |  |
| match_8_arms | 3.2ns | 169.3ns | 1.9% |  |

## Distribution (algo ns)

```
match_2_arms (n=160, range 96.4-119.5 ns)
     96.4 |#############
     97.5 |##########################
     98.7 |#######################
     99.9 |##########################
    101.0 |#######################
    102.2 |################
    103.3 |##########
    104.5 |##############################
    105.6 |#################################
    106.8 |########################################
    108.0 |##############################
    109.1 |####################################
    110.3 |#############
    111.4 |################
    112.6 |#######################
    113.7 |#######################
    114.9 |########################################
    116.1 |#############
    117.2 |#############
    118.4 |##########
  (16 below, 5 above range)

match_64_arms (n=160, range 230.2-271.0 ns)
    230.2 |##############################
    232.3 |#######################
    234.3 |######
    236.3 |####################
    238.4 |#############
    240.4 |#######################
    242.5 |##########################
    244.5 |#################################
    246.5 |#################################
    248.6 |##########################
    250.6 |####################################
    252.7 |#############
    254.7 |###
    256.7 |##########
    258.8 |######
    260.8 |##############################
    262.8 |#################################
    264.9 |######
    266.9 |##############################
    269.0 |########################################
  (14 below, 12 above range)

match_8_arms (n=160, range 155.8-183.9 ns)
    155.8 |#####################
    157.2 |###############
    158.6 |############
    160.0 |########################################
    161.4 |###############
    162.8 |##################
    164.2 |###########################
    165.6 |###############
    167.0 |####################################
    168.4 |#####################
    169.8 |###############
    171.2 |###########################
    172.6 |#####################
    174.0 |##################
    175.4 |#####################
    176.9 |##################
    178.3 |########################
    179.7 |############
    181.1 |###############
    182.5 |############
  (13 below, 13 above range)

```
