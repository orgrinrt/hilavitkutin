# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.0% (fastest 40 ns to slowest 41 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (40 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### minmax_intrinsic's comparison is tie-heavy (10% tied pairs)

10% of paired samples for minmax_intrinsic are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader minmax_branch vs stability leader minmax_intrinsic (+0% speed for 1.1x steadier)

minmax_branch is fastest (40 ns, CV 11.8%); minmax_intrinsic gives up 0.0% median for 1.1x lower variance (CV 11.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 40.0 ns median
- Spread: 1.02x (fastest 40.0 ns, slowest 40.8 ns)

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
| minmax_branch | 2150ns | 2139ns | 2131ns | 2139ns | 2199ns | base |
| minmax_branchless | 2156ns | 2139ns | 2132ns | 2140ns | 2229ns | +0.29% |
| minmax_intrinsic | 2141ns | 2139ns | 2134ns | 2139ns | 2153ns | -0.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 41ns | 35ns | 48ns | base | 25.138 |
| minmax_branchless | 42ns | 36ns | 51ns | +2.41% | 24.546 |
| minmax_intrinsic | 41ns | 36ns | 47ns | -0.12% | 25.168 |

## Performance model

- Peak throughput: **29.088 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 25.600 | 88.0% |
| minmax_branchless | 25.098 | 86.3% |
| minmax_intrinsic | 25.600 | 88.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2150ns | 2150ns | base |
| minmax_branchless | 2156ns | 2156ns | +0.29% |
| minmax_intrinsic | 2141ns | 2141ns | -0.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 40ns | base | --- | [39, 41] | --- | --- | --- | --- |
| minmax_branchless | 41ns | no significant difference | [+0, +1]ns | [40, 41] | no (adj: YES) | 0.0337 | 0.0168 | 12 |
| minmax_intrinsic | 40ns | no significant difference | [-0, +1]ns | [39, 41] | no | 0.5598 | 0.5598 | 16 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 40ns | +0.5% | -6.2% |
| 1 | 2 | 41ns | +0.1% | -5.0% |
| 1 | 3 | 39ns | +7.9% | +2.6% |
| 1 | 4 | 40ns | +1.5% | +0.0% |
| 1 | 5 | 39ns | +3.2% | +4.8% |
| 1 | 6 | 39ns | +4.1% | +4.1% |
| 1 | 7 | 39ns | +3.6% | +3.6% |
| 1 | 8 | 40ns | +0.0% | -3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.012 | ok |
| minmax_branchless | 0.133 | ok |
| minmax_intrinsic | 0.365 | moderate+ |

**Consistency summary:**

- **minmax_branchless**: won 0/8, lost 7/8
- **minmax_intrinsic**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 3.7ns | 40.7ns | 9.1% | HIGH |
| minmax_branchless | 3.6ns | 41.7ns | 8.6% | HIGH |
| minmax_intrinsic | 3.3ns | 40.7ns | 8.1% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 35.2-47.6 ns)
     35.2 |########################
     35.8 |########
     36.4 |#############
     37.1 |#####################
     37.7 |########################################
     38.3 |########################
     38.9 |##########################
     39.6 |################################
     40.2 |##########################
     40.8 |#####################
     41.4 |##########
     42.0 |##################
     42.7 |################
     43.3 |##################
     43.9 |########
     44.5 |#####################
     45.1 |##
     45.8 |########################
     46.4 |#############
     47.0 |#####
  (12 below, 7 above range)

minmax_branchless (n=160, range 35.7-51.4 ns)
     35.7 |######################
     36.4 |####################
     37.2 |####################
     38.0 |##################
     38.8 |################
     39.6 |######################
     40.4 |########################################
     41.2 |########################
     41.9 |##############################
     42.7 |##################
     43.5 |##############
     44.3 |########
     45.1 |############
     45.9 |####
     46.7 |##########
     47.4 |########
     48.2 |##
     49.0 |##
     49.8 |##
     50.6 |
  (12 below, 2 above range)

minmax_intrinsic (n=160, range 35.8-47.3 ns)
     35.8 |###############
     36.4 |#############
     37.0 |###################################
     37.6 |#############
     38.1 |###############
     38.7 |###############################
     39.3 |########
     39.8 |########################################
     40.4 |###############
     41.0 |########
     41.6 |############################
     42.1 |#################
     42.7 |####
     43.3 |########
     43.9 |######
     44.4 |###########
     45.0 |####
     45.6 |######
     46.2 |#################
     46.7 |####
  (13 below, 8 above range)

```

## Diagnostics

- **minmax_branchless**: CV=31.3% (high variance, measurements may be unstable)
