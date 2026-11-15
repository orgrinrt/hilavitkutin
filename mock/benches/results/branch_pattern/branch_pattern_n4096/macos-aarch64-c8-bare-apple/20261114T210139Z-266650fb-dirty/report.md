# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 6.72 us to slowest 6.72 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (208 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (branch_cmov)

The baseline branch_cmov is the fastest (6.72 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader branch_cmov vs stability leader branch_if (+0% speed for 1.1x steadier)

branch_cmov is fastest (6.72 us, CV 3.1%); branch_if gives up 0.0% median for 1.1x lower variance (CV 2.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (branch_cmov) is the fastest** at 6716.0 ns median
- Spread: 1.00x (fastest 6716.0 ns, slowest 6717.5 ns)

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
| branch_cmov | 8866ns | 8864ns | 8566ns | 8840ns | 9245ns | base |
| branch_if | 8879ns | 8868ns | 8578ns | 8866ns | 9219ns | +0.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 6715ns | 6490ns | 7000ns | base | 0.610 |
| branch_if | 6724ns | 6496ns | 6984ns | +0.12% | 0.609 |

## Performance model

- Peak throughput: **0.631 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.610 | 96.6% |
| branch_if | 0.610 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 8866ns | 8866ns | base |
| branch_if | 8879ns | 8879ns | +0.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 6716ns | base | --- | [6614, 6729] | --- | --- | --- | --- |
| branch_if | 6718ns | no significant difference | [-17, +11]ns | [6698, 6727] | no | 0.8126 | 0.8126 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 6500ns | +1.4% |
| 1 | 2 | 6956ns | -5.6% |
| 1 | 3 | 6542ns | +3.8% |
| 1 | 4 | 6538ns | +4.0% |
| 1 | 5 | 6959ns | -3.4% |
| 1 | 6 | 6955ns | -3.5% |
| 1 | 7 | 6491ns | +3.4% |
| 1 | 8 | 6744ns | +1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.321 | moderate- |
| branch_if | 0.201 | moderate+ |

**Consistency summary:**

- **branch_if**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.8ns | 6715.4ns | 0.0% |  |
| branch_if | 2.6ns | 6723.7ns | 0.0% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 6489.7-7000.5 ns)
   6489.7 |##################################
   6515.2 |########
   6540.7 |#######
   6566.3 |###
   6591.8 |#####
   6617.4 |###
   6642.9 |#
   6668.4 |###
   6694.0 |#######
   6719.5 |################
   6745.1 |#####
   6770.6 |##
   6796.1 |###
   6821.7 |##
   6847.2 |##
   6872.8 |#
   6898.3 |#
   6923.8 |#
   6949.4 |########################################
   6974.9 |####
  (14 below, 4 above range)

branch_if (n=160, range 6495.7-6983.9 ns)
   6495.7 |#######################
   6520.1 |############
   6544.5 |#########
   6568.9 |#####
   6593.3 |############
   6617.7 |############
   6642.1 |#####
   6666.6 |###
   6691.0 |####################
   6715.4 |########################################
   6739.8 |#######
   6764.2 |#######
   6788.6 |#####
   6813.0 |#
   6837.5 |#####
   6861.9 |#
   6886.3 |#####
   6910.7 |###
   6935.1 |#########################
   6959.5 |################################
  (20 below, 7 above range)

```
