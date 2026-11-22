# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 107 ns to slowest 107 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (107 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (load_aligned) is the fastest** at 107.1 ns median
- Spread: 1.00x (fastest 107.1 ns, slowest 107.1 ns)

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
| load_aligned | 2212ns | 2208ns | 2201ns | 2208ns | 2235ns | base |
| load_byte_pack | 2209ns | 2206ns | 2202ns | 2206ns | 2224ns | -0.14% |
| load_unaligned | 2221ns | 2205ns | 2200ns | 2205ns | 2294ns | +0.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 107ns | 103ns | 111ns | base | 9.551 |
| load_byte_pack | 107ns | 103ns | 111ns | +0.05% | 9.546 |
| load_unaligned | 109ns | 103ns | 122ns | +1.91% | 9.372 |

## Performance model

- Peak throughput: **9.928 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 9.561 | 96.3% |
| load_byte_pack | 9.561 | 96.3% |
| load_unaligned | 9.561 | 96.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2212ns | 2212ns | base |
| load_byte_pack | 2209ns | 2209ns | -0.14% |
| load_unaligned | 2221ns | 2221ns | +0.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 107ns | base | --- | [107, 108] | --- | --- | --- | --- |
| load_byte_pack | 107ns | no significant difference | [-0, +0]ns | [106, 108] | no | 0.8078 | 0.6184 | 15 |
| load_unaligned | 107ns | no significant difference | [-0, +0]ns | [107, 108] | no | 0.8078 | 0.8078 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 107ns | +1.5% | -0.6% |
| 1 | 2 | 108ns | -0.4% | -0.2% |
| 1 | 3 | 108ns | -0.7% | -0.4% |
| 1 | 4 | 107ns | -0.4% | +0.4% |
| 1 | 5 | 108ns | -0.4% | -0.2% |
| 1 | 6 | 107ns | -0.7% | +0.7% |
| 1 | 7 | 107ns | +0.7% | +0.2% |
| 1 | 8 | 106ns | -0.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.250 | moderate+ |
| load_byte_pack | -0.215 | moderate- |
| load_unaligned | 0.237 | moderate+ |

**Consistency summary:**

- **load_byte_pack**: won 6/8, lost 2/8
- **load_unaligned**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.7ns | 107.2ns | 2.5% |  |
| load_byte_pack | 2.9ns | 107.3ns | 2.7% |  |
| load_unaligned | 2.9ns | 109.3ns | 2.7% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 103.1-111.5 ns)
    103.1 |#########
    103.6 |##################
    104.0 |#########
    104.4 |#####################
    104.8 |########################
    105.2 |########################
    105.6 |##################
    106.1 |#################################
    106.5 |####################################
    106.9 |#####################
    107.3 |##################
    107.7 |########################################
    108.1 |#####################
    108.6 |#####################
    109.0 |#####################
    109.4 |#####################
    109.8 |##################
    110.2 |###############
    110.6 |######
    111.1 |#########
  (13 below, 13 above range)

load_byte_pack (n=160, range 103.2-111.5 ns)
    103.2 |#############
    103.7 |##########
    104.1 |##########
    104.5 |#################################
    104.9 |#######################
    105.3 |########################################
    105.7 |####################################
    106.1 |####################
    106.5 |####################
    107.0 |####################################
    107.4 |################
    107.8 |#############
    108.2 |####################
    108.6 |#################################
    109.0 |##############################
    109.4 |#############
    109.8 |#######################
    110.3 |#############
    110.7 |#######################
    111.1 |##########
  (13 below, 15 above range)

load_unaligned (n=160, range 103.4-121.5 ns)
    103.4 |###########
    104.3 |#############
    105.2 |###############
    106.1 |###################
    107.0 |########################################
    107.9 |#############
    108.8 |###########
    109.8 |#####
    110.7 |######
    111.6 |####
    112.5 |########
    113.4 |#
    114.3 |#
    115.2 |
    116.1 |
    117.0 |
    117.9 |
    118.8 |
    119.7 |
    120.6 |
  (12 below, 2 above range)

```
