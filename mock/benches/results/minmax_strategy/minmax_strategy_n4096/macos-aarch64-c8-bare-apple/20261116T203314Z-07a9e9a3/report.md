# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.7% (fastest 109 ns to slowest 110 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### minmax_branch shows alternating (throttle bounce) (autocorr -0.56)

minmax_branch's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (109 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 109.2 ns median
- Spread: 1.01x (fastest 109.2 ns, slowest 110.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5d49a3f1b7c06e82 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2188ns | 2180ns | 2175ns | 2181ns | 2223ns | base |
| minmax_branchless | 2213ns | 2181ns | 2176ns | 2183ns | 2341ns | +1.13% |
| minmax_intrinsic | 2189ns | 2181ns | 2175ns | 2182ns | 2226ns | +0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 110ns | 105ns | 116ns | base | 37.310 |
| minmax_branchless | 112ns | 105ns | 123ns | +1.72% | 36.680 |
| minmax_intrinsic | 110ns | 105ns | 116ns | +0.15% | 37.254 |

## Performance model

- Peak throughput: **39.084 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 37.509 | 96.0% |
| minmax_branchless | 37.236 | 95.3% |
| minmax_intrinsic | 37.509 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2188ns | 2188ns | base |
| minmax_branchless | 2213ns | 2213ns | +1.13% |
| minmax_intrinsic | 2189ns | 2189ns | +0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 109ns | base | --- | [108, 110] | --- | --- | --- | --- |
| minmax_branchless | 110ns | no significant difference | [+0, +1]ns | [109, 111] | no | 0.1461 | 0.0730 | 9 |
| minmax_intrinsic | 109ns | no significant difference | [-0, +0]ns | [109, 110] | no | 0.4595 | 0.4595 | 12 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 110ns | +0.0% | -0.2% |
| 1 | 2 | 110ns | -0.4% | +0.4% |
| 1 | 3 | 109ns | +6.5% | +0.5% |
| 1 | 4 | 111ns | -0.9% | -1.1% |
| 1 | 5 | 109ns | -0.4% | +0.6% |
| 1 | 6 | 110ns | +0.0% | -0.8% |
| 1 | 7 | 108ns | +0.0% | +1.4% |
| 1 | 8 | 110ns | -0.8% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.559 | HIGH- (thermal bounce) |
| minmax_branchless | -0.049 | ok |
| minmax_intrinsic | -0.376 | moderate- |

**Consistency summary:**

- **minmax_branchless**: won 4/8, lost 1/8
- **minmax_intrinsic**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.2ns | 109.8ns | 2.0% |  |
| minmax_branchless | 2.1ns | 111.7ns | 1.9% |  |
| minmax_intrinsic | 2.1ns | 109.9ns | 1.9% |  |

## Distribution (algo ns)

```
minmax_branch (n=160, range 104.8-115.8 ns)
    104.8 |##############
    105.4 |#########################
    105.9 |##############
    106.5 |##############
    107.0 |########################################
    107.6 |####################
    108.1 |####################
    108.7 |#####################################
    109.2 |####################
    109.8 |####################
    110.3 |##################################
    110.9 |###########
    111.4 |###########
    112.0 |####################
    112.5 |########
    113.1 |#################
    113.6 |#####
    114.2 |###########
    114.7 |###########
    115.3 |####################
  (16 below, 12 above range)

minmax_branchless (n=160, range 105.1-123.1 ns)
    105.1 |######################
    106.0 |##########################
    106.9 |###############################
    107.8 |########################################
    108.7 |#################
    109.6 |############################
    110.5 |####################
    111.4 |####################
    112.3 |###########
    113.2 |#################
    114.1 |##########################
    115.0 |###########
    115.9 |#############
    116.8 |
    117.7 |###########
    118.6 |####
    119.5 |##
    120.4 |##
    121.3 |
    122.2 |####
  (16 below, 4 above range)

minmax_intrinsic (n=160, range 104.9-116.1 ns)
    104.9 |##################################
    105.4 |#############
    106.0 |##########
    106.5 |##########################
    107.1 |#####################
    107.7 |################
    108.2 |#####################################
    108.8 |#############################
    109.3 |################
    109.9 |########################################
    110.5 |##########
    111.0 |#####
    111.6 |##################
    112.1 |
    112.7 |########
    113.3 |##########################
    113.8 |########
    114.4 |#####
    114.9 |##########################
    115.5 |########
  (11 below, 13 above range)

```
