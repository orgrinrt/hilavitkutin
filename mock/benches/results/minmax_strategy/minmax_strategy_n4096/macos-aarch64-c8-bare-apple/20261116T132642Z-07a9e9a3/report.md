# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### minmax_branchless shows alternating (throttle bounce) (autocorr -0.61)

minmax_branchless's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (109 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### minmax_branchless's comparison is tie-heavy (11% tied pairs)

11% of paired samples for minmax_branchless are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader minmax_branch vs stability leader minmax_intrinsic (+1% speed for 1.0x steadier)

minmax_branch is fastest (109 ns, CV 3.6%); minmax_intrinsic gives up 0.6% median for 1.0x lower variance (CV 3.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.6% of the fastest

All 3 variants sit between 109 ns and 110 ns - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### minmax_intrinsic's edge over baseline is significant but tiny (1 ns, 0.73%)

minmax_intrinsic differs from baseline minmax_branch by 1 ns (0.73%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 109.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 109.0 ns, slowest 109.6 ns)

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
| minmax_branch | 2187ns | 2181ns | 2175ns | 2181ns | 2213ns | base |
| minmax_branchless | 2185ns | 2181ns | 2176ns | 2181ns | 2206ns | -0.08% |
| minmax_intrinsic | 2195ns | 2182ns | 2176ns | 2183ns | 2249ns | +0.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 110ns | 105ns | 116ns | base | 37.342 |
| minmax_branchless | 110ns | 105ns | 116ns | +0.16% | 37.282 |
| minmax_intrinsic | 110ns | 105ns | 116ns | +0.66% | 37.096 |

## Performance model

- Peak throughput: **39.145 Gops/s** (minmax_branchless; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 37.595 | 96.0% |
| minmax_branchless | 37.509 | 95.8% |
| minmax_intrinsic | 37.372 | 95.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2187ns | 2187ns | base |
| minmax_branchless | 2185ns | 2185ns | -0.08% |
| minmax_intrinsic | 2195ns | 2195ns | +0.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 109ns | base | --- | [108, 110] | --- | --- | --- | --- |
| minmax_branchless | 109ns | no significant difference | [+0, +0]ns | [109, 110] | no | 0.3560 | 0.3560 | **18** (11%, HIGH) |
| minmax_intrinsic | 110ns | no significant difference | [+0, +1]ns | [109, 110] | no (adj: YES) | 0.0233 | 0.0117 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 108ns | -0.2% | +0.2% |
| 1 | 2 | 109ns | +1.1% | +0.8% |
| 1 | 3 | 110ns | -1.0% | +0.4% |
| 1 | 4 | 109ns | +1.2% | +0.0% |
| 1 | 5 | 108ns | +1.0% | +1.0% |
| 1 | 6 | 108ns | +1.2% | +0.7% |
| 1 | 7 | 110ns | -0.5% | +0.5% |
| 1 | 8 | 109ns | +0.1% | +3.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.063 | ok |
| minmax_branchless | -0.606 | HIGH- (thermal bounce) |
| minmax_intrinsic | 0.136 | ok |

**Consistency summary:**

- **minmax_branchless**: won 3/8, lost 5/8
- **minmax_intrinsic**: won 0/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.1ns | 109.7ns | 1.9% |  |
| minmax_branchless | 2.1ns | 109.9ns | 1.9% |  |
| minmax_intrinsic | 2.0ns | 110.4ns | 1.8% |  |

## Distribution (algo ns)

```
minmax_branch (n=160, range 104.7-115.8 ns)
    104.7 |#######
    105.2 |######################
    105.8 |###################################
    106.3 |##########
    106.9 |#######
    107.5 |########################################
    108.0 |###################################
    108.6 |############
    109.1 |##############################
    109.7 |#########################
    110.2 |#####
    110.8 |##########
    111.3 |#######
    111.9 |###############
    112.4 |#################
    113.0 |##########
    113.5 |#######
    114.1 |#################
    114.6 |#####
    115.2 |####################
  (12 below, 12 above range)

minmax_branchless (n=160, range 104.6-115.9 ns)
    104.6 |#####################
    105.2 |#########
    105.8 |##############################
    106.3 |#####################
    106.9 |#########
    107.4 |###########################
    108.0 |####################################
    108.6 |##################
    109.1 |########################################
    109.7 |########################
    110.2 |#####################
    110.8 |##################
    111.4 |###############
    111.9 |###
    112.5 |########################
    113.1 |###############
    113.6 |############
    114.2 |###########################
    114.7 |#########
    115.3 |##################
  (16 below, 12 above range)

minmax_intrinsic (n=160, range 105.4-116.4 ns)
    105.4 |##################
    105.9 |#####################
    106.5 |########
    107.0 |##########################
    107.6 |##########################
    108.1 |#####################
    108.7 |########################################
    109.2 |##########################
    109.8 |################
    110.3 |#####################
    110.9 |########
    111.4 |##########
    112.0 |#############################
    112.5 |#####
    113.1 |########
    113.6 |##########
    114.2 |########################
    114.7 |##################
    115.3 |##########
    115.8 |##########
  (12 below, 12 above range)

```
