# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.4% (fastest 390 ns to slowest 392 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### branch_if shows alternating (throttle bounce) (autocorr -0.55)

branch_if's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (12 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (branch_cmov)

The baseline branch_cmov is the fastest (390 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader branch_cmov vs stability leader branch_if (+0% speed for 1.5x steadier)

branch_cmov is fastest (390 ns, CV 3.1%); branch_if gives up 0.4% median for 1.5x lower variance (CV 2.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (branch_cmov) is the fastest** at 390.4 ns median
- Spread: 1.00x (fastest 390.4 ns, slowest 392.1 ns)

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
| branch_cmov | 2500ns | 2477ns | 2469ns | 2488ns | 2567ns | base |
| branch_if | 2508ns | 2490ns | 2471ns | 2499ns | 2574ns | +0.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 393ns | 386ns | 408ns | base | 0.651 |
| branch_if | 394ns | 386ns | 406ns | +0.19% | 0.649 |

## Performance model

- Peak throughput: **0.664 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.656 | 98.8% |
| branch_if | 0.653 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2500ns | 2500ns | base |
| branch_if | 2508ns | 2508ns | +0.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 390ns | base | --- | [390, 392] | --- | --- | --- | --- |
| branch_if | 392ns | no significant difference | [-0, +1]ns | [391, 393] | no | 0.2917 | 0.2917 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 393ns | +1.3% |
| 1 | 2 | 396ns | -1.5% |
| 1 | 3 | 389ns | +0.2% |
| 1 | 4 | 402ns | -1.6% |
| 1 | 5 | 390ns | +0.0% |
| 1 | 6 | 391ns | +3.0% |
| 1 | 7 | 390ns | +0.1% |
| 1 | 8 | 390ns | +1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.383 | moderate- |
| branch_if | -0.551 | HIGH- (thermal bounce) |

**Consistency summary:**

- **branch_if**: won 2/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 3.2ns | 393.4ns | 0.8% |  |
| branch_if | 3.2ns | 394.2ns | 0.8% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 385.7-408.0 ns)
    385.7 |##############################
    386.8 |##############################
    387.9 |##########
    389.0 |########################################
    390.1 |##################################
    391.3 |##############
    392.4 |####################
    393.5 |############
    394.6 |#####
    395.7 |#########
    396.9 |#########
    398.0 |##########
    399.1 |
    400.2 |#####
    401.3 |#####
    402.4 |
    403.6 |#####
    404.7 |#########
    405.8 |###
    406.9 |
  (10 below, 8 above range)

branch_if (n=160, range 386.4-405.9 ns)
    386.4 |##########
    387.4 |###################################
    388.4 |##############
    389.3 |###############################
    390.3 |########################################
    391.3 |##########
    392.3 |#####################
    393.2 |############
    394.2 |########
    395.2 |####
    396.2 |#####################
    397.1 |##################
    398.1 |################
    399.1 |##############
    400.1 |
    401.0 |
    402.0 |############
    403.0 |####
    404.0 |####
    404.9 |######
  (13 below, 10 above range)

```
