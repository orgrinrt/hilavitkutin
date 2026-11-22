# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.0% (fastest 82 ns to slowest 83 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (branch_cmov)

The baseline branch_cmov is the fastest (82 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader branch_cmov vs stability leader branch_if (+1% speed for 1.1x steadier)

branch_cmov is fastest (82 ns, CV 4.3%); branch_if gives up 1.0% median for 1.1x lower variance (CV 3.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (branch_cmov) is the fastest** at 82.5 ns median
- Spread: 1.01x (fastest 82.5 ns, slowest 83.3 ns)

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
| branch_cmov | 2169ns | 2155ns | 2145ns | 2162ns | 2212ns | base |
| branch_if | 2185ns | 2191ns | 2151ns | 2185ns | 2216ns | +0.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 83ns | 78ns | 87ns | base | 0.775 |
| branch_if | 83ns | 78ns | 87ns | +0.69% | 0.769 |

## Performance model

- Peak throughput: **0.818 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.776 | 94.8% |
| branch_if | 0.768 | 93.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2169ns | 2169ns | base |
| branch_if | 2185ns | 2185ns | +0.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 82ns | base | --- | [82, 83] | --- | --- | --- | --- |
| branch_if | 83ns | no significant difference | [+0, +1]ns | [83, 84] | no (adj: YES) | 0.0282 | 0.0282 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 84ns | +0.0% |
| 1 | 2 | 83ns | +0.5% |
| 1 | 3 | 83ns | +0.5% |
| 1 | 4 | 82ns | +2.0% |
| 1 | 5 | 81ns | +1.5% |
| 1 | 6 | 82ns | +3.1% |
| 1 | 7 | 83ns | +2.1% |
| 1 | 8 | 83ns | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | 0.394 | moderate+ |
| branch_if | -0.211 | moderate- |

**Consistency summary:**

- **branch_if**: won 1/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.7ns | 82.6ns | 3.3% |  |
| branch_if | 2.7ns | 83.2ns | 3.2% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 78.2-87.2 ns)
     78.2 |######
     78.6 |######
     79.1 |############
     79.5 |########################
     80.0 |#################################
     80.4 |##################
     80.9 |########################
     81.3 |##############################
     81.8 |##############################
     82.2 |########################################
     82.7 |##############################
     83.1 |###########################
     83.6 |#####################
     84.0 |############
     84.5 |##############################
     84.9 |#########
     85.4 |####################################
     85.8 |#########
     86.3 |##################
     86.7 |#########
  (12 below, 7 above range)

branch_if (n=160, range 78.4-87.4 ns)
     78.4 |#####
     78.9 |
     79.3 |###
     79.8 |#####
     80.2 |###
     80.7 |##########
     81.1 |###########
     81.6 |##########
     82.0 |#############
     82.4 |#####################
     82.9 |########################################
     83.3 |#############
     83.8 |###########
     84.2 |############################
     84.7 |########
     85.1 |######
     85.6 |########
     86.0 |##########
     86.5 |#####
     86.9 |######
  (13 below, 14 above range)

```
