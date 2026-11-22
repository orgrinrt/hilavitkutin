# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.0% (fastest 40 ns to slowest 40 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### minmax_branch is fastest but the noisiest (CV 10.4%)

minmax_branch wins on median (40 ns) yet has the highest variance (CV 10.4%), while minmax_intrinsic is the steadiest (CV 9.5%, 40 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### minmax_branchless shows alternating (throttle bounce) (autocorr -0.83)

minmax_branchless's per-pass series has lag-1 autocorrelation -0.83, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (40 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader minmax_branch vs stability leader minmax_intrinsic (+1% speed for 1.1x steadier)

minmax_branch is fastest (40 ns, CV 10.4%); minmax_intrinsic gives up 1.0% median for 1.1x lower variance (CV 9.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 39.8 ns median
- Spread: 1.01x (fastest 39.8 ns, slowest 40.2 ns)

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
| minmax_branch | 2117ns | 2112ns | 2107ns | 2112ns | 2143ns | base |
| minmax_branchless | 2120ns | 2112ns | 2108ns | 2113ns | 2155ns | +0.13% |
| minmax_intrinsic | 2120ns | 2112ns | 2108ns | 2113ns | 2154ns | +0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 40ns | 35ns | 47ns | base | 25.334 |
| minmax_branchless | 40ns | 35ns | 47ns | +0.04% | 25.324 |
| minmax_intrinsic | 41ns | 36ns | 46ns | +0.33% | 25.251 |

## Performance model

- Peak throughput: **29.114 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 25.729 | 88.4% |
| minmax_branchless | 25.600 | 87.9% |
| minmax_intrinsic | 25.473 | 87.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2117ns | 2117ns | base |
| minmax_branchless | 2120ns | 2120ns | +0.13% |
| minmax_intrinsic | 2120ns | 2120ns | +0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 40ns | base | --- | [39, 41] | --- | --- | --- | --- |
| minmax_branchless | 40ns | no significant difference | [-0, +0]ns | [39, 40] | no | 0.6832 | 0.5703 | 8 |
| minmax_intrinsic | 40ns | no significant difference | [-0, +1]ns | [40, 41] | no | 0.6832 | 0.6832 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 40ns | +0.5% | -0.5% |
| 1 | 2 | 40ns | +0.5% | +1.5% |
| 1 | 3 | 40ns | -3.0% | +0.0% |
| 1 | 4 | 40ns | +1.5% | +1.5% |
| 1 | 5 | 40ns | -2.5% | -0.5% |
| 1 | 6 | 40ns | +2.5% | +4.2% |
| 1 | 7 | 39ns | +1.0% | +3.6% |
| 1 | 8 | 39ns | +2.5% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.481 | moderate+ |
| minmax_branchless | -0.833 | HIGH- (thermal bounce) |
| minmax_intrinsic | -0.084 | ok |

**Consistency summary:**

- **minmax_branchless**: won 2/8, lost 6/8
- **minmax_intrinsic**: won 2/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.4ns | 40.4ns | 6.0% | HIGH |
| minmax_branchless | 2.4ns | 40.4ns | 5.9% | HIGH |
| minmax_intrinsic | 2.3ns | 40.6ns | 5.6% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 35.2-47.0 ns)
     35.2 |######
     35.8 |###############
     36.4 |########
     37.0 |########################
     37.5 |###############
     38.1 |#############
     38.7 |########################################
     39.3 |####################
     39.9 |########################
     40.5 |###############
     41.1 |########################
     41.7 |########################
     42.3 |######
     42.9 |######
     43.5 |
     44.1 |###########
     44.7 |##
     45.3 |###############
     45.9 |######
     46.4 |####
  (15 below, 16 above range)

minmax_branchless (n=160, range 35.2-46.7 ns)
     35.2 |#####
     35.8 |###############################
     36.4 |########
     36.9 |#####################################
     37.5 |####################
     38.1 |###############################
     38.7 |######################
     39.2 |####################
     39.8 |###############################
     40.4 |########################################
     41.0 |#################
     41.5 |##############
     42.1 |#####
     42.7 |#################
     43.3 |######################
     43.8 |###########
     44.4 |#################
     45.0 |###########
     45.5 |#####
     46.1 |##
  (13 below, 16 above range)

minmax_intrinsic (n=160, range 35.6-46.4 ns)
     35.6 |#####
     36.2 |#################
     36.7 |######################
     37.2 |#####
     37.8 |#################
     38.3 |####################
     38.9 |##############################
     39.4 |######################
     40.0 |###################################
     40.5 |###############
     41.0 |######################
     41.6 |########################################
     42.1 |#####
     42.7 |#######
     43.2 |#######
     43.7 |###############
     44.3 |############
     44.8 |#######
     45.4 |##########
     45.9 |
  (18 below, 15 above range)

```
