# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.3% (fastest 34 ns to slowest 34 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### minmax_branchless shows alternating (throttle bounce) (autocorr -0.63)

minmax_branchless's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (29 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (34 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### minmax_intrinsic's comparison is tie-heavy (13% tied pairs)

13% of paired samples for minmax_intrinsic are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### minmax_intrinsic is inconsistent: worst-20% is 3.8x its best-20%

minmax_intrinsic's best 20% of batches run at 28 ns but its worst 20% at 106 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 33.8 ns median
- Spread: 1.00x (fastest 33.8 ns, slowest 33.9 ns)

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
| Counter frequency | 2112457735 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2606ns | 1905ns | 1560ns | 1940ns | 5651ns | base |
| minmax_branchless | 2684ns | 1911ns | 1561ns | 1984ns | 5908ns | +3.00% |
| minmax_intrinsic | 2703ns | 1910ns | 1561ns | 1974ns | 6032ns | +3.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 46ns | 28ns | 99ns | base | 5.551 |
| minmax_branchless | 47ns | 28ns | 104ns | +2.59% | 5.411 |
| minmax_intrinsic | 48ns | 28ns | 106ns | +3.37% | 5.370 |

## Performance model

- Peak throughput: **9.256 Gops/s** (minmax_branchless; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 7.574 | 81.8% |
| minmax_branchless | 7.574 | 81.8% |
| minmax_intrinsic | 7.552 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2606ns | 2606ns | base |
| minmax_branchless | 2684ns | 2684ns | +3.00% |
| minmax_intrinsic | 2703ns | 2703ns | +3.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 34ns | base | --- | [28, 34] | --- | --- | --- | --- |
| minmax_branchless | 34ns | no significant difference | [-0, +0]ns | [34, 34] | no | 0.9327 | 0.9327 | **20** (12%, HIGH) |
| minmax_intrinsic | 34ns | no significant difference | [+0, +0]ns | [34, 34] | no | 0.0541 | 0.0271 | **21** (13%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 28ns | +22.4% | +0.0% |
| 1 | 2 | 28ns | -0.4% | +22.1% |
| 1 | 3 | 34ns | -18.2% | -18.2% |
| 1 | 4 | 28ns | +22.2% | -0.2% |
| 1 | 5 | 28ns | +0.0% | +22.4% |
| 1 | 6 | 28ns | +22.2% | +22.0% |
| 1 | 7 | 34ns | -18.3% | +0.0% |
| 1 | 8 | 34ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.067 | ok |
| minmax_branchless | -0.625 | HIGH- (thermal bounce) |
| minmax_intrinsic | 0.050 | ok |

**Consistency summary:**

- **minmax_branchless**: won 3/8, lost 3/8
- **minmax_intrinsic**: won 2/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 31.6ns | 46.1ns | 68.6% | HIGH |
| minmax_branchless | 37.0ns | 47.3ns | 78.2% | HIGH |
| minmax_intrinsic | 33.0ns | 47.7ns | 69.2% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 27.7-99.0 ns)
     27.7 |########################################
     31.2 |################################
     34.8 |##
     38.4 |
     41.9 |
     45.5 |
     49.1 |
     52.6 |
     56.2 |
     59.8 |
     63.3 |
     66.9 |
     70.5 |
     74.0 |#
     77.6 |
     81.2 |##
     84.7 |###
     88.3 |
     91.9 |####
     95.4 |##
  (10 below, 15 above range)

minmax_branchless (n=160, range 27.7-103.7 ns)
     27.7 |#############################
     31.5 |########################################
     35.3 |
     39.1 |
     42.9 |
     46.7 |
     50.5 |
     54.3 |
     58.1 |
     61.9 |
     65.7 |
     69.5 |
     73.3 |
     77.1 |
     80.9 |#
     84.7 |##
     88.5 |
     92.3 |#
     96.1 |#
     99.9 |#####
  (13 below, 13 above range)

minmax_intrinsic (n=160, range 27.7-105.7 ns)
     27.7 |###################
     31.6 |########################################
     35.5 |
     39.4 |
     43.3 |
     47.2 |
     51.1 |
     55.0 |
     58.9 |
     62.8 |#
     66.7 |
     70.6 |
     74.5 |
     78.4 |
     82.3 |
     86.2 |##
     90.1 |##
     94.0 |#
     97.9 |##
    101.8 |#
  (5 below, 14 above range)

```

## Diagnostics

- **minmax_branch**: CV=61.8% (high variance, measurements may be unstable)
- **minmax_branch**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **minmax_branch**: bridge=68.5% of algo (FFI overhead may distort results)
- **minmax_branchless**: CV=63.2% (high variance, measurements may be unstable)
- **minmax_branchless**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **minmax_branchless**: bridge=68.6% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: CV=63.9% (high variance, measurements may be unstable)
- **minmax_intrinsic**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **minmax_intrinsic**: bridge=68.4% of algo (FFI overhead may distort results)
