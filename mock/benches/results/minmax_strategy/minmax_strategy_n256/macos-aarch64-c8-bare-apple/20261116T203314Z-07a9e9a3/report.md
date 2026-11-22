# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.8% (fastest 22 ns to slowest 23 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### minmax_intrinsic is fastest but the noisiest (CV 17.3%)

minmax_intrinsic wins on median (22 ns) yet has the highest variance (CV 17.3%), while minmax_branchless is the steadiest (CV 9.9%, 23 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader minmax_intrinsic vs stability leader minmax_branchless (+2% speed for 1.7x steadier)

minmax_intrinsic is fastest (22 ns, CV 17.3%); minmax_branchless gives up 1.8% median for 1.7x lower variance (CV 9.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: minmax_intrinsic** at 22.5 ns median (-1.7% vs baseline)
- Spread: 1.02x (fastest 22.5 ns, slowest 22.9 ns)

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
| minmax_branch | 2115ns | 2106ns | 2101ns | 2106ns | 2157ns | base |
| minmax_branchless | 2111ns | 2104ns | 2099ns | 2105ns | 2141ns | -0.21% |
| minmax_intrinsic | 2119ns | 2108ns | 2101ns | 2109ns | 2165ns | +0.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 23ns | 19ns | 26ns | base | 11.259 |
| minmax_branchless | 23ns | 20ns | 26ns | +0.63% | 11.189 |
| minmax_intrinsic | 23ns | 20ns | 27ns | +0.57% | 11.195 |

## Performance model

- Peak throughput: **13.166 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 11.179 | 84.9% |
| minmax_branchless | 11.179 | 84.9% |
| minmax_intrinsic | 11.378 | 86.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2115ns | 2115ns | base |
| minmax_branchless | 2111ns | 2111ns | -0.21% |
| minmax_intrinsic | 2119ns | 2119ns | +0.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 23ns | base | --- | [22, 23] | --- | --- | --- | --- |
| minmax_branchless | 23ns | no significant difference | [-0, +0]ns | [22, 23] | no | 0.5625 | 0.5625 | 14 |
| minmax_intrinsic | 22ns | no significant difference | [-1, +0]ns | [22, 23] | no | 0.5625 | 0.5124 | 11 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 22ns | +1.8% | +0.0% |
| 1 | 2 | 22ns | +0.0% | -0.9% |
| 1 | 3 | 22ns | +6.4% | +0.0% |
| 1 | 4 | 23ns | -0.9% | -5.2% |
| 1 | 5 | 23ns | +0.0% | +1.7% |
| 1 | 6 | 23ns | -0.9% | -1.7% |
| 1 | 7 | 22ns | -0.9% | +5.3% |
| 1 | 8 | 23ns | -0.9% | -3.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.030 | ok |
| minmax_branchless | -0.042 | ok |
| minmax_intrinsic | -0.192 | ok |

**Consistency summary:**

- **minmax_branchless**: won 4/8, lost 2/8
- **minmax_intrinsic**: won 4/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 3.6ns | 22.7ns | 15.7% | HIGH |
| minmax_branchless | 3.4ns | 22.9ns | 15.1% | HIGH |
| minmax_intrinsic | 3.5ns | 22.9ns | 15.4% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 19.4-25.9 ns)
     19.4 |#########
     19.8 |########################
     20.1 |##############################
     20.4 |
     20.7 |#########
     21.1 |#####################
     21.4 |##############################
     21.7 |
     22.0 |########################################
     22.4 |####################################
     22.7 |########################################
     23.0 |##################
     23.3 |
     23.7 |########################################
     24.0 |##############################
     24.3 |##################
     24.6 |
     25.0 |#####################
     25.3 |#####################
     25.6 |#####################
  (13 below, 12 above range)

minmax_branchless (n=160, range 19.8-26.3 ns)
     19.8 |####################
     20.1 |####################
     20.4 |
     20.8 |#################
     21.1 |###########################
     21.4 |#################
     21.7 |
     22.1 |######################
     22.4 |########################################
     22.7 |#####################################
     23.0 |########################################
     23.3 |
     23.7 |#################
     24.0 |##########
     24.3 |###############
     24.6 |
     25.0 |##########
     25.3 |####################
     25.6 |######################
     25.9 |#####
  (11 below, 12 above range)

minmax_intrinsic (n=160, range 19.7-26.7 ns)
     19.7 |###########
     20.1 |##############
     20.4 |
     20.8 |#####################
     21.1 |#####################
     21.5 |########################################
     21.8 |##################
     22.2 |#####################################
     22.5 |
     22.9 |#########################
     23.2 |###########
     23.6 |#########################
     23.9 |#########################
     24.3 |#######################
     24.6 |
     25.0 |#########################
     25.3 |#######
     25.7 |#########
     26.0 |##
     26.4 |#########
  (16 below, 3 above range)

```

## Diagnostics

- **minmax_branch**: bridge=14.4% of algo (FFI overhead may distort results)
- **minmax_branchless**: bridge=14.4% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: bridge=15.6% of algo (FFI overhead may distort results)
