# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 112 ns to slowest 112 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### minmax_branch is fastest but the noisiest (CV 5.1%)

minmax_branch wins on median (112 ns) yet has the highest variance (CV 5.1%), while minmax_intrinsic is the steadiest (CV 3.6%, 112 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### minmax_intrinsic shows alternating (throttle bounce) (autocorr -0.78)

minmax_intrinsic's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (6 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (112 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader minmax_branch vs stability leader minmax_intrinsic (+0% speed for 1.4x steadier)

minmax_branch is fastest (112 ns, CV 5.1%); minmax_intrinsic gives up 0.0% median for 1.4x lower variance (CV 3.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 112.1 ns median
- Spread: 1.00x (fastest 112.1 ns, slowest 112.1 ns)

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
| minmax_branch | 2214ns | 2209ns | 2201ns | 2209ns | 2240ns | base |
| minmax_branchless | 2215ns | 2212ns | 2205ns | 2212ns | 2234ns | +0.05% |
| minmax_intrinsic | 2216ns | 2212ns | 2206ns | 2213ns | 2237ns | +0.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 113ns | 107ns | 121ns | base | 36.287 |
| minmax_branchless | 112ns | 107ns | 118ns | -0.57% | 36.495 |
| minmax_intrinsic | 113ns | 108ns | 119ns | -0.17% | 36.348 |

## Performance model

- Peak throughput: **38.360 Gops/s** (minmax_branchless; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 36.539 | 95.3% |
| minmax_branchless | 36.539 | 95.3% |
| minmax_intrinsic | 36.539 | 95.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2214ns | 2214ns | base |
| minmax_branchless | 2215ns | 2215ns | +0.05% |
| minmax_intrinsic | 2216ns | 2216ns | +0.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 112ns | base | --- | [111, 113] | --- | --- | --- | --- |
| minmax_branchless | 112ns | no significant difference | [-1, +0]ns | [111, 113] | no | 0.4655 | 0.4655 | 8 |
| minmax_intrinsic | 112ns | no significant difference | [-0, +1]ns | [111, 113] | no | 0.3410 | 0.1705 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 112ns | -0.6% | -0.6% |
| 1 | 2 | 113ns | +0.0% | +0.4% |
| 1 | 3 | 113ns | -0.0% | -2.2% |
| 1 | 4 | 111ns | +1.3% | +1.7% |
| 1 | 5 | 110ns | +0.5% | +1.0% |
| 1 | 6 | 112ns | -0.4% | +0.5% |
| 1 | 7 | 112ns | -0.4% | -0.6% |
| 1 | 8 | 111ns | -0.7% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.181 | ok |
| minmax_branchless | 0.319 | moderate+ |
| minmax_intrinsic | -0.783 | HIGH- (thermal bounce) |

**Consistency summary:**

- **minmax_branchless**: won 4/8, lost 2/8
- **minmax_intrinsic**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.1ns | 112.9ns | 1.9% |  |
| minmax_branchless | 2.0ns | 112.2ns | 1.8% |  |
| minmax_intrinsic | 2.0ns | 112.7ns | 1.8% |  |

## Distribution (algo ns)

```
minmax_branch (n=160, range 107.1-121.0 ns)
    107.1 |#####
    107.8 |########################
    108.5 |##################
    109.2 |################################
    109.9 |########################################
    110.6 |################################
    111.3 |##########
    112.0 |#############################
    112.7 |################################
    113.3 |########
    114.0 |##########################
    114.7 |##################
    115.4 |#####
    116.1 |########
    116.8 |##########
    117.5 |########
    118.2 |#####################
    118.9 |#############
    119.6 |##########
    120.3 |##
  (18 below, 8 above range)

minmax_branchless (n=160, range 106.8-118.4 ns)
    106.8 |##################
    107.4 |##################
    107.9 |################
    108.5 |##################
    109.1 |#####################
    109.7 |########
    110.3 |##########################
    110.8 |########################
    111.4 |################
    112.0 |########################################
    112.6 |################
    113.2 |########################
    113.7 |##################
    114.3 |#############
    114.9 |#############
    115.5 |#############
    116.0 |##########
    116.6 |##################
    117.2 |########
    117.8 |################
  (14 below, 11 above range)

minmax_intrinsic (n=160, range 108.1-119.0 ns)
    108.1 |############
    108.7 |#################################
    109.2 |#######################
    109.8 |################
    110.3 |###############################
    110.8 |############
    111.4 |##
    111.9 |################
    112.5 |########################################
    113.0 |######
    113.5 |##################
    114.1 |#######################
    114.6 |
    115.2 |########
    115.7 |##########
    116.3 |##########
    116.8 |####
    117.3 |####
    117.9 |######
    118.4 |######
  (10 below, 13 above range)

```
