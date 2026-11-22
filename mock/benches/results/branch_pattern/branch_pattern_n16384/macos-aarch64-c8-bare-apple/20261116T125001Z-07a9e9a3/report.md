# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 26.06 us to slowest 26.07 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (241 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (branch_cmov)

The baseline branch_cmov is the fastest (26.06 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (branch_cmov) is the fastest** at 26059.8 ns median
- Spread: 1.00x (fastest 26059.8 ns, slowest 26065.5 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 28253ns | 28162ns | 28119ns | 28170ns | 28634ns | base |
| branch_if | 28254ns | 28172ns | 28117ns | 28174ns | 28628ns | +0.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 26144ns | 26026ns | 26489ns | base | 0.627 |
| branch_if | 26142ns | 26024ns | 26484ns | -0.01% | 0.627 |

## Performance model

- Peak throughput: **0.630 Gops/s** (branch_if; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.629 | 99.9% |
| branch_if | 0.629 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 28253ns | 28253ns | base |
| branch_if | 28254ns | 28254ns | +0.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 26060ns | base | --- | [26052, 26073] | --- | --- | --- | --- |
| branch_if | 26065ns | no significant difference | [-9, +2]ns | [26051, 26078] | no | 0.5801 | 0.5801 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 26292ns | +2.4% |
| 1 | 2 | 26058ns | -0.0% |
| 1 | 3 | 26060ns | -0.0% |
| 1 | 4 | 26069ns | +0.0% |
| 1 | 5 | 26051ns | -0.0% |
| 1 | 6 | 26066ns | -0.0% |
| 1 | 7 | 26059ns | -0.1% |
| 1 | 8 | 26044ns | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.028 | ok |
| branch_if | -0.023 | ok |

**Consistency summary:**

- **branch_if**: won 0/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.9ns | 26143.6ns | 0.0% |  |
| branch_if | 3.0ns | 26142.1ns | 0.0% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 26025.6-26489.2 ns)
  26025.6 |########################################
  26048.8 |######################
  26072.0 |##############
  26095.2 |############
  26118.4 |#######
  26141.5 |
  26164.7 |##
  26187.9 |##
  26211.1 |
  26234.2 |#
  26257.4 |
  26280.6 |
  26303.8 |
  26326.9 |
  26350.1 |
  26373.3 |
  26396.5 |
  26419.6 |
  26442.8 |
  26466.0 |
  (13 below, 14 above range)

branch_if (n=160, range 26023.6-26483.8 ns)
  26023.6 |########################################
  26046.6 |#######################
  26069.6 |#######################
  26092.6 |################
  26115.7 |#######
  26138.7 |
  26161.7 |##
  26184.7 |
  26207.7 |
  26230.7 |##
  26253.7 |
  26276.7 |
  26299.7 |
  26322.8 |
  26345.8 |
  26368.8 |
  26391.8 |
  26414.8 |
  26437.8 |
  26460.8 |
  (12 below, 13 above range)

```
