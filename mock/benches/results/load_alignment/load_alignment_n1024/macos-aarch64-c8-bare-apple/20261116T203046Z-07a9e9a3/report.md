# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.4% (fastest 107 ns to slowest 108 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (107 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (load_aligned) is the fastest** at 107.1 ns median
- Spread: 1.00x (fastest 107.1 ns, slowest 107.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x2c8af49106b7ed53 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 2213ns | 2205ns | 2201ns | 2206ns | 2247ns | base |
| load_byte_pack | 2238ns | 2208ns | 2201ns | 2209ns | 2360ns | +1.11% |
| load_unaligned | 2212ns | 2208ns | 2202ns | 2208ns | 2234ns | -0.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 107ns | 103ns | 112ns | base | 9.532 |
| load_byte_pack | 109ns | 104ns | 116ns | +1.18% | 9.421 |
| load_unaligned | 107ns | 104ns | 111ns | -0.03% | 9.536 |

## Performance model

- Peak throughput: **9.903 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 9.561 | 96.5% |
| load_byte_pack | 9.526 | 96.2% |
| load_unaligned | 9.543 | 96.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2213ns | 2213ns | base |
| load_byte_pack | 2238ns | 2238ns | +1.11% |
| load_unaligned | 2212ns | 2212ns | -0.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 107ns | base | --- | [107, 108] | --- | --- | --- | --- |
| load_byte_pack | 108ns | no significant difference | [+0, +1]ns | [107, 108] | no | 0.0704 | 0.0352 | 7 |
| load_unaligned | 107ns | no significant difference | [-1, +0]ns | [107, 108] | no | 0.2267 | 0.2267 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 107ns | +0.2% | +1.2% |
| 1 | 2 | 107ns | +0.6% | +0.4% |
| 1 | 3 | 108ns | -0.4% | -1.1% |
| 1 | 4 | 108ns | -1.2% | -0.4% |
| 1 | 5 | 108ns | -0.2% | -1.3% |
| 1 | 6 | 107ns | +0.2% | +0.9% |
| 1 | 7 | 107ns | +0.9% | +0.4% |
| 1 | 8 | 107ns | +9.9% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.394 | moderate+ |
| load_byte_pack | 0.018 | ok |
| load_unaligned | -0.375 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 3/8, lost 5/8
- **load_unaligned**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.9ns | 107.4ns | 2.7% |  |
| load_byte_pack | 2.8ns | 108.7ns | 2.6% |  |
| load_unaligned | 2.8ns | 107.4ns | 2.6% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 103.4-111.7 ns)
    103.4 |##
    103.8 |#######
    104.2 |#################
    104.7 |#####
    105.1 |###########################
    105.5 |###########################
    105.9 |######################
    106.3 |########################################
    106.7 |#################
    107.1 |#########################
    107.6 |######################
    108.0 |######################
    108.4 |####################
    108.8 |##########
    109.2 |############
    109.6 |#####
    110.1 |#################
    110.5 |#################
    110.9 |#######
    111.3 |##########
  (14 below, 11 above range)

load_byte_pack (n=160, range 103.7-116.0 ns)
    103.7 |##############
    104.4 |######
    105.0 |#############################
    105.6 |################
    106.2 |########################################
    106.8 |################
    107.4 |###############################
    108.0 |################
    108.6 |#####################
    109.3 |##########
    109.9 |###########################
    110.5 |
    111.1 |############
    111.7 |######
    112.3 |########
    112.9 |####
    113.6 |######
    114.2 |######
    114.8 |##
    115.4 |
  (13 below, 15 above range)

load_unaligned (n=160, range 103.5-111.5 ns)
    103.5 |###
    103.9 |####################
    104.3 |#######################
    104.7 |##############################
    105.1 |####################
    105.5 |########################################
    105.9 |##############################
    106.3 |##############################
    106.7 |#################################
    107.1 |########################################
    107.5 |####################################
    107.9 |####################
    108.3 |################
    108.7 |
    109.1 |####################
    109.5 |##############################
    109.9 |#############
    110.3 |##########
    110.7 |################
    111.1 |##########
  (11 below, 16 above range)

```
