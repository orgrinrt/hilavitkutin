# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 26.07 us to slowest 26.07 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (151 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (branch_cmov)

The baseline branch_cmov is the fastest (26.07 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (branch_cmov) is the fastest** at 26065.6 ns median
- Spread: 1.00x (fastest 26065.6 ns, slowest 26068.8 ns)

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
| branch_cmov | 28179ns | 28153ns | 28084ns | 28153ns | 28353ns | base |
| branch_if | 28260ns | 28161ns | 28089ns | 28175ns | 28687ns | +0.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 26085ns | 26000ns | 26238ns | base | 0.628 |
| branch_if | 26157ns | 26004ns | 26549ns | +0.28% | 0.626 |

## Performance model

- Peak throughput: **0.630 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.629 | 99.7% |
| branch_if | 0.628 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 28179ns | 28179ns | base |
| branch_if | 28260ns | 28260ns | +0.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 26066ns | base | --- | [26051, 26072] | --- | --- | --- | --- |
| branch_if | 26069ns | no significant difference | [-5, +26]ns | [26054, 26082] | no | 0.3816 | 0.3816 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 26064ns | +0.2% |
| 1 | 2 | 26032ns | +0.2% |
| 1 | 3 | 26070ns | -0.0% |
| 1 | 4 | 26112ns | -0.2% |
| 1 | 5 | 26051ns | +0.2% |
| 1 | 6 | 26067ns | -0.0% |
| 1 | 7 | 26046ns | +0.0% |
| 1 | 8 | 26052ns | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.095 | ok |
| branch_if | 0.167 | ok |

**Consistency summary:**

- **branch_if**: won 2/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.9ns | 26085.1ns | 0.0% |  |
| branch_if | 3.0ns | 26157.0ns | 0.0% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 25999.7-26238.4 ns)
  25999.7 |############################
  26011.6 |################################
  26023.6 |##############################
  26035.5 |#####################
  26047.4 |############################
  26059.4 |###################################
  26071.3 |########################################
  26083.2 |#######################
  26095.2 |################
  26107.1 |#####################
  26119.1 |##################
  26131.0 |#######
  26142.9 |####
  26154.9 |
  26166.8 |####
  26178.8 |#######
  26190.7 |##
  26202.6 |##
  26214.6 |
  26226.5 |
  (14 below, 8 above range)

branch_if (n=160, range 26004.1-26549.0 ns)
  26004.1 |##############################
  26031.4 |########################################
  26058.6 |############################
  26085.8 |############
  26113.1 |#################
  26140.3 |##########
  26167.6 |###
  26194.8 |###
  26222.1 |##
  26249.3 |#
  26276.6 |##
  26303.8 |#
  26331.1 |#
  26358.3 |##
  26385.6 |
  26412.8 |#
  26440.0 |#
  26467.3 |#
  26494.5 |##
  26521.8 |
  (16 below, 13 above range)

```
