# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (branch_cmov)

The baseline branch_cmov is the fastest (27.44 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (branch_cmov) is the fastest** at 27442.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 27442.9 ns, slowest 29932.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branchful if/else vs branchless cmov/csel under data-dependent input |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x76543210fedcba98 |
| Counter frequency | 2112793168 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 28701ns | 29336ns | 25971ns | 29337ns | 29521ns | base |
| branch_if | 31279ns | 31854ns | 27630ns | 31807ns | 33344ns | +8.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 26847ns | 24295ns | 27611ns | base | 0.610 |
| branch_if | 29415ns | 25943ns | 31414ns | +9.56% | 0.557 |

## Performance model

- Peak throughput: **0.674 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.597 | 88.5% |
| branch_if | 0.547 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 28701ns | 28701ns | base |
| branch_if | 31279ns | 31279ns | +8.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 27443ns | base | --- | [27432, 27461] | --- | --- | --- | --- |
| branch_if | 29932ns | +2680.0ns (+9.8%) | [+2469, +3049]ns | [29689, 30114] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 22485ns | +31.6% |
| 1 | 2 | 27477ns | +11.8% |
| 1 | 3 | 27507ns | +13.5% |
| 1 | 4 | 27443ns | +6.5% |
| 1 | 5 | 27503ns | +3.6% |
| 1 | 6 | 27478ns | +8.6% |
| 1 | 7 | 27444ns | +9.8% |
| 1 | 8 | 27433ns | -8.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.019 | ok |
| branch_if | -0.013 | ok |

**Consistency summary:**

- **branch_if**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 23.3ns | 26847.4ns | 0.1% |  |
| branch_if | 24.0ns | 29414.9ns | 0.1% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 24295.0-27611.2 ns)
  24295.0 |
  24460.8 |
  24626.6 |
  24792.4 |
  24958.2 |
  25124.0 |
  25289.8 |
  25455.7 |
  25621.5 |
  25787.3 |
  25953.1 |
  26118.9 |
  26284.7 |
  26450.5 |
  26616.3 |
  26782.2 |
  26948.0 |
  27113.8 |#
  27279.6 |#################################
  27445.4 |########################################
  (20 below, 7 above range)

branch_if (n=160, range 25942.8-31414.2 ns)
  25942.8 |
  26216.3 |
  26489.9 |
  26763.5 |
  27037.1 |#######
  27310.6 |##
  27584.2 |##
  27857.8 |
  28131.3 |#####################
  28404.9 |###########
  28678.5 |####
  28952.1 |################
  29225.6 |#######################
  29499.2 |################################
  29772.8 |###################################
  30046.3 |################################
  30319.9 |###########
  30593.5 |#######################
  30867.0 |###################################
  31140.6 |########################################
  (20 below, 12 above range)

```
