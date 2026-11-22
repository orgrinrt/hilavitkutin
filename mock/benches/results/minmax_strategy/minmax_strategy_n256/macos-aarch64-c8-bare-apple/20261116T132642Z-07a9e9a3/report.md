# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.9% (fastest 23 ns to slowest 23 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### minmax_branchless shows alternating (throttle bounce) (autocorr -0.68)

minmax_branchless's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (2 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### minmax_intrinsic's comparison is tie-heavy (11% tied pairs)

11% of paired samples for minmax_intrinsic are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

## Key findings

- **Fastest: minmax_intrinsic** at 22.7 ns median (-0.9% vs baseline)
- Spread: 1.01x (fastest 22.7 ns, slowest 22.9 ns)

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
| minmax_branch | 2109ns | 2106ns | 2100ns | 2106ns | 2128ns | base |
| minmax_branchless | 2111ns | 2104ns | 2099ns | 2104ns | 2140ns | +0.07% |
| minmax_intrinsic | 2112ns | 2106ns | 2101ns | 2106ns | 2141ns | +0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 23ns | 20ns | 26ns | base | 11.138 |
| minmax_branchless | 23ns | 20ns | 27ns | +0.24% | 11.111 |
| minmax_intrinsic | 23ns | 20ns | 26ns | -0.70% | 11.217 |

## Performance model

- Peak throughput: **13.122 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 11.179 | 85.2% |
| minmax_branchless | 11.179 | 85.2% |
| minmax_intrinsic | 11.278 | 85.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2109ns | 2109ns | base |
| minmax_branchless | 2111ns | 2111ns | +0.07% |
| minmax_intrinsic | 2112ns | 2112ns | +0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 23ns | base | --- | [22, 23] | --- | --- | --- | --- |
| minmax_branchless | 23ns | no significant difference | [-0, +0]ns | [22, 23] | no | 0.5095 | 0.5095 | 13 |
| minmax_intrinsic | 23ns | no significant difference | [-1, +0]ns | [22, 23] | no | 0.5095 | 0.5036 | **17** (11%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 23ns | +2.6% | +0.9% |
| 1 | 2 | 23ns | -3.5% | +0.0% |
| 1 | 3 | 23ns | +1.7% | -1.7% |
| 1 | 4 | 22ns | -1.8% | +3.6% |
| 1 | 5 | 23ns | -0.9% | -5.2% |
| 1 | 6 | 23ns | -2.8% | -1.7% |
| 1 | 7 | 23ns | +0.9% | +0.9% |
| 1 | 8 | 23ns | -2.6% | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.470 | moderate- |
| minmax_branchless | -0.685 | HIGH- (thermal bounce) |
| minmax_intrinsic | -0.002 | ok |

**Consistency summary:**

- **minmax_branchless**: won 5/8, lost 3/8
- **minmax_intrinsic**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 3.4ns | 23.0ns | 14.9% | HIGH |
| minmax_branchless | 3.5ns | 23.0ns | 15.0% | HIGH |
| minmax_intrinsic | 3.5ns | 22.8ns | 15.3% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 19.5-26.4 ns)
     19.5 |#############
     19.9 |#############
     20.2 |##########################
     20.5 |##################
     20.9 |##########
     21.2 |
     21.6 |##################
     21.9 |#############
     22.2 |########################################
     22.6 |##################################
     22.9 |
     23.3 |##################################
     23.6 |#####################
     24.0 |#####################################
     24.3 |########################
     24.6 |
     25.0 |################
     25.3 |########
     25.7 |########################
     26.0 |##########
  (11 below, 12 above range)

minmax_branchless (n=160, range 19.8-26.6 ns)
     19.8 |################
     20.1 |##########
     20.5 |#############################
     20.8 |
     21.2 |#####################################
     21.5 |#####################
     21.8 |#############
     22.2 |########################################
     22.5 |
     22.9 |################
     23.2 |#####################
     23.6 |#############################
     23.9 |##########
     24.2 |
     24.6 |##########################
     24.9 |##################
     25.3 |##################
     25.6 |#####################
     26.0 |##########
     26.3 |
  (16 below, 16 above range)

minmax_intrinsic (n=160, range 19.7-26.0 ns)
     19.7 |################
     20.1 |
     20.4 |################################
     20.7 |################
     21.0 |################################
     21.3 |
     21.6 |#####################
     21.9 |#####################
     22.3 |##################################
     22.6 |
     22.9 |##########
     23.2 |#####################################
     23.5 |##########################
     23.8 |
     24.2 |##################
     24.5 |########################################
     24.8 |########################
     25.1 |########
     25.4 |
     25.7 |#############
  (15 below, 13 above range)

```

## Diagnostics

- **minmax_branch**: bridge=14.4% of algo (FFI overhead may distort results)
- **minmax_branchless**: bridge=16.2% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: bridge=14.5% of algo (FFI overhead may distort results)
