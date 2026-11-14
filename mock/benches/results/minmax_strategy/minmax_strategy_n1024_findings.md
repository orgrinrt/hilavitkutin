# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.0% (fastest 42 ns to slowest 42 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (42 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 42.1 ns median
- Spread: 1.01x (fastest 42.1 ns, slowest 42.5 ns)

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
| minmax_branch | 2199ns | 2251ns | 2100ns | 2202ns | 2288ns | base |
| minmax_branchless | 2254ns | 2254ns | 2108ns | 2243ns | 2431ns | +2.49% |
| minmax_intrinsic | 2217ns | 2166ns | 2097ns | 2185ns | 2431ns | +0.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 42ns | 36ns | 49ns | base | 24.177 |
| minmax_branchless | 43ns | 37ns | 50ns | +1.67% | 23.779 |
| minmax_intrinsic | 43ns | 36ns | 51ns | +1.06% | 23.923 |

## Performance model

- Peak throughput: **28.449 Gops/s** (minmax_intrinsic; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 24.323 | 85.5% |
| minmax_branchless | 24.094 | 84.7% |
| minmax_intrinsic | 24.323 | 85.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2199ns | 2199ns | base |
| minmax_branchless | 2254ns | 2254ns | +2.49% |
| minmax_intrinsic | 2217ns | 2217ns | +0.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 42ns | base | --- | [41, 43] | --- | --- | --- | --- |
| minmax_branchless | 42ns | no significant difference | [+0, +2]ns | [42, 43] | no | 0.1806 | 0.0903 | 6 |
| minmax_intrinsic | 42ns | no significant difference | [-1, +1]ns | [41, 43] | no | 0.6871 | 0.6871 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 42ns | -3.1% | -9.9% |
| 1 | 2 | 44ns | -5.8% | -6.8% |
| 1 | 3 | 41ns | +1.2% | -5.8% |
| 1 | 4 | 44ns | -1.0% | -4.8% |
| 1 | 5 | 43ns | -4.9% | +4.0% |
| 1 | 6 | 41ns | +13.4% | +8.9% |
| 1 | 7 | 39ns | +11.1% | +12.2% |
| 1 | 8 | 44ns | +2.3% | -1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.183 | ok |
| minmax_branchless | -0.068 | ok |
| minmax_intrinsic | 0.443 | moderate+ |

**Consistency summary:**

- **minmax_branchless**: won 4/8, lost 4/8
- **minmax_intrinsic**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.4ns | 42.4ns | 5.7% | HIGH |
| minmax_branchless | 2.4ns | 43.1ns | 5.6% | HIGH |
| minmax_intrinsic | 2.6ns | 42.8ns | 6.0% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 36.2-48.5 ns)
     36.2 |###########
     36.9 |##############
     37.5 |####################
     38.1 |##############
     38.7 |######################
     39.3 |########
     39.9 |#########################
     40.5 |##############
     41.1 |########################################
     41.8 |####################
     42.4 |##################################
     43.0 |########
     43.6 |##################################
     44.2 |########
     44.8 |#########################
     45.4 |########
     46.1 |#################
     46.7 |#########################
     47.3 |##############
     47.9 |##############
  (14 below, 12 above range)

minmax_branchless (n=160, range 37.2-49.6 ns)
     37.2 |#####
     37.8 |############
     38.4 |#####
     39.0 |#########################
     39.7 |##########
     40.3 |#####################################
     40.9 |#################
     41.5 |########################################
     42.1 |#######
     42.8 |##############################
     43.4 |#####
     44.0 |#########################
     44.6 |#################
     45.3 |####################
     45.9 |#######
     46.5 |#########################
     47.1 |#######
     47.8 |############
     48.4 |##
     49.0 |############
  (17 below, 13 above range)

minmax_intrinsic (n=160, range 36.0-51.3 ns)
     36.0 |#################
     36.8 |#####
     37.5 |##############
     38.3 |########################################
     39.1 |##############
     39.8 |##################################
     40.6 |########################################
     41.4 |######################
     42.1 |######################
     42.9 |#################
     43.7 |###############################
     44.4 |####################
     45.2 |############################
     46.0 |####################
     46.7 |############################
     47.5 |#####
     48.3 |########
     49.0 |#####
     49.8 |########
     50.6 |#####
  (16 below, 7 above range)

```
