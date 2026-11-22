# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### minmax_intrinsic shows alternating (throttle bounce) (autocorr -0.67)

minmax_intrinsic's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (7 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.8% of the fastest

All 3 variants sit between 392 ns and 400 ns - a 1.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### minmax_branchless's edge over baseline is significant but tiny (4 ns, 0.96%)

minmax_branchless differs from baseline minmax_branch by 4 ns (0.96%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: minmax_intrinsic** at 392.5 ns median (-0.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 392.5 ns, slowest 399.6 ns)

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
| minmax_branch | 2496ns | 2492ns | 2486ns | 2493ns | 2514ns | base |
| minmax_branchless | 2504ns | 2498ns | 2491ns | 2498ns | 2534ns | +0.34% |
| minmax_intrinsic | 2497ns | 2492ns | 2485ns | 2492ns | 2521ns | +0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 396ns | 389ns | 407ns | base | 41.341 |
| minmax_branchless | 399ns | 389ns | 409ns | +0.64% | 41.079 |
| minmax_intrinsic | 393ns | 382ns | 404ns | -0.88% | 41.706 |

## Performance model

- Peak throughput: **42.849 Gops/s** (minmax_intrinsic; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 41.437 | 96.7% |
| minmax_branchless | 41.001 | 95.7% |
| minmax_intrinsic | 41.743 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2496ns | 2496ns | base |
| minmax_branchless | 2504ns | 2504ns | +0.34% |
| minmax_intrinsic | 2497ns | 2497ns | +0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 395ns | base | --- | [394, 396] | --- | --- | --- | --- |
| minmax_branchless | 400ns | +3.8ns (+1.0%) | [+3, +5]ns | [398, 400] | YES | 0.0000 | 0.0000 | 5 |
| minmax_intrinsic | 392ns | -2.1ns (-0.5%) | [-3, -1]ns | [392, 394] | YES | 0.0002 | 0.0002 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 394ns | +1.8% | +0.1% |
| 1 | 2 | 396ns | +1.6% | +0.1% |
| 1 | 3 | 395ns | -1.3% | -2.5% |
| 1 | 4 | 395ns | +1.6% | +0.7% |
| 1 | 5 | 396ns | +1.1% | -2.5% |
| 1 | 6 | 396ns | +1.3% | -0.1% |
| 1 | 7 | 395ns | +1.4% | -0.2% |
| 1 | 8 | 396ns | -1.5% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.280 | moderate- |
| minmax_branchless | -0.259 | moderate- |
| minmax_intrinsic | -0.671 | HIGH- (thermal bounce) |

**Consistency summary:**

- **minmax_branchless**: won 2/8, lost 6/8
- **minmax_intrinsic**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.3ns | 396.3ns | 0.6% |  |
| minmax_branchless | 2.6ns | 398.8ns | 0.6% |  |
| minmax_intrinsic | 2.6ns | 392.8ns | 0.7% |  |

## Distribution (algo ns)

```
minmax_branch (n=160, range 388.7-406.5 ns)
    388.7 |#################
    389.6 |############
    390.5 |######################
    391.4 |####################
    392.3 |#########################
    393.2 |########################################
    394.1 |############
    394.9 |########################################
    395.8 |######################
    396.7 |##############################
    397.6 |############
    398.5 |##########
    399.4 |#######
    400.3 |####################
    401.2 |##########
    402.1 |#################
    403.0 |###############
    403.8 |#####
    404.7 |
    405.6 |##########
  (13 below, 7 above range)

minmax_branchless (n=160, range 389.0-408.7 ns)
    389.0 |###########
    390.0 |#################
    391.0 |########
    392.0 |####################
    393.0 |##############
    394.0 |########
    395.0 |##################################
    395.9 |##############
    396.9 |#################
    397.9 |######################
    398.9 |##################################
    399.9 |########################################
    400.9 |#####################################
    401.8 |#################
    402.8 |##############
    403.8 |####################
    404.8 |#####
    405.8 |#########################
    406.8 |##
    407.7 |##############
  (16 below, 11 above range)

minmax_intrinsic (n=160, range 382.4-404.4 ns)
    382.4 |#####
    383.5 |################
    384.6 |##########
    385.7 |################
    386.8 |#############
    387.9 |##################
    389.0 |##########################
    390.1 |##################
    391.2 |########################################
    392.3 |################################
    393.4 |#####################
    394.5 |#############################
    395.6 |#####################
    396.7 |########################
    397.8 |################
    398.9 |#####
    400.0 |##
    401.1 |##################
    402.2 |################
    403.3 |#####
  (17 below, 9 above range)

```
