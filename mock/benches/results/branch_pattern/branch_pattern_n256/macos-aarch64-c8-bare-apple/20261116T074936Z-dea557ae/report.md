# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 392 ns to slowest 392 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (branch_cmov)

The baseline branch_cmov is the fastest (392 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (branch_cmov) is the fastest** at 392.5 ns median
- Spread: 1.00x (fastest 392.5 ns, slowest 392.5 ns)

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
| branch_cmov | 2491ns | 2471ns | 2455ns | 2482ns | 2554ns | base |
| branch_if | 2494ns | 2468ns | 2454ns | 2480ns | 2577ns | +0.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 393ns | 387ns | 400ns | base | 0.651 |
| branch_if | 396ns | 387ns | 410ns | +0.62% | 0.647 |

## Performance model

- Peak throughput: **0.661 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.652 | 98.6% |
| branch_if | 0.652 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2491ns | 2491ns | base |
| branch_if | 2494ns | 2494ns | +0.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 392ns | base | --- | [392, 394] | --- | --- | --- | --- |
| branch_if | 392ns | no significant difference | [+0, +2]ns | [392, 394] | no | 0.0551 | 0.0551 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 391ns | -0.2% |
| 1 | 2 | 392ns | +0.9% |
| 1 | 3 | 395ns | -1.0% |
| 1 | 4 | 394ns | +1.7% |
| 1 | 5 | 390ns | +0.0% |
| 1 | 6 | 398ns | +1.3% |
| 1 | 7 | 393ns | +0.9% |
| 1 | 8 | 391ns | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.293 | moderate- |
| branch_if | -0.478 | moderate- |

**Consistency summary:**

- **branch_if**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.6ns | 393.1ns | 0.7% |  |
| branch_if | 2.7ns | 395.5ns | 0.7% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 387.1-399.6 ns)
    387.1 |##########
    387.7 |##########
    388.3 |##################
    389.0 |##################
    389.6 |########################
    390.2 |#####################
    390.8 |#####################
    391.5 |########################
    392.1 |#####################################
    392.7 |##########################
    393.3 |#####
    394.0 |########
    394.6 |##################
    395.2 |########################################
    395.9 |#####
    396.5 |#############################
    397.1 |################
    397.7 |########
    398.4 |##
    399.0 |#####################
  (13 below, 9 above range)

branch_if (n=160, range 387.2-410.1 ns)
    387.2 |############################
    388.4 |################################
    389.5 |########################################
    390.7 |################################
    391.8 |################################
    393.0 |###########
    394.1 |#####################
    395.2 |#########
    396.4 |#######################
    397.5 |#########
    398.7 |#########################
    399.8 |###########
    400.9 |####
    402.1 |#######################
    403.2 |####
    404.4 |####
    405.5 |####
    406.6 |##
    407.8 |
    408.9 |####
  (14 below, 6 above range)

```
