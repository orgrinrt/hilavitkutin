# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.3% (fastest 34 ns to slowest 34 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (29 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### minmax_intrinsic's comparison is tie-heavy (10% tied pairs)

10% of paired samples for minmax_intrinsic are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### minmax_branchless is inconsistent: worst-20% is 3.8x its best-20%

minmax_branchless's best 20% of batches run at 28 ns but its worst 20% at 105 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: minmax_intrinsic** at 33.7 ns median (-0.3% vs baseline)
- Spread: 1.00x (fastest 33.7 ns, slowest 33.8 ns)

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
| Counter frequency | 2112833092 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2648ns | 1911ns | 1561ns | 1961ns | 5794ns | base |
| minmax_branchless | 2685ns | 1912ns | 1563ns | 1952ns | 6009ns | +1.43% |
| minmax_intrinsic | 2544ns | 1908ns | 1562ns | 1797ns | 5766ns | -3.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 47ns | 28ns | 101ns | base | 5.484 |
| minmax_branchless | 47ns | 28ns | 105ns | +1.57% | 5.400 |
| minmax_intrinsic | 45ns | 28ns | 100ns | -4.58% | 5.748 |

## Performance model

- Peak throughput: **9.261 Gops/s** (minmax_branchless; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 7.574 | 81.8% |
| minmax_branchless | 7.574 | 81.8% |
| minmax_intrinsic | 7.596 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2648ns | 2648ns | base |
| minmax_branchless | 2685ns | 2685ns | +1.43% |
| minmax_intrinsic | 2544ns | 2544ns | -3.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 34ns | base | --- | [34, 34] | --- | --- | --- | --- |
| minmax_branchless | 34ns | no significant difference | [-0, +0]ns | [34, 34] | no | 0.9350 | 0.9350 | 10 |
| minmax_intrinsic | 34ns | no significant difference | [-0, +0]ns | [28, 34] | no | 0.5572 | 0.2786 | 16 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 28ns | +0.0% | +22.0% |
| 1 | 2 | 34ns | -2.2% | -0.3% |
| 1 | 3 | 34ns | +0.0% | -17.9% |
| 1 | 4 | 34ns | -18.0% | -18.0% |
| 1 | 5 | 34ns | -18.0% | -18.0% |
| 1 | 6 | 28ns | +22.4% | +22.0% |
| 1 | 7 | 28ns | +25.3% | +0.0% |
| 1 | 8 | 28ns | +22.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.371 | moderate+ |
| minmax_branchless | 0.124 | ok |
| minmax_intrinsic | 0.060 | ok |

**Consistency summary:**

- **minmax_branchless**: won 3/8, lost 3/8
- **minmax_intrinsic**: won 4/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 32.3ns | 46.7ns | 69.3% | HIGH |
| minmax_branchless | 32.9ns | 47.4ns | 69.3% | HIGH |
| minmax_intrinsic | 30.8ns | 44.5ns | 69.2% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 27.7-101.2 ns)
     27.7 |#########################
     31.3 |########################################
     35.0 |
     38.7 |
     42.4 |
     46.0 |#
     49.7 |
     53.4 |
     57.1 |
     60.8 |
     64.4 |
     68.1 |
     71.8 |
     75.5 |
     79.1 |#
     82.8 |
     86.5 |###
     90.2 |###
     93.9 |
     97.5 |#
  (16 below, 16 above range)

minmax_branchless (n=160, range 27.6-105.4 ns)
     27.6 |########################
     31.5 |########################################
     35.4 |#
     39.3 |
     43.2 |
     47.1 |
     51.0 |
     54.9 |
     58.7 |
     62.6 |
     66.5 |
     70.4 |###
     74.3 |#
     78.2 |#
     82.1 |
     85.9 |
     89.8 |
     93.7 |#
     97.6 |
    101.5 |#
  (17 below, 19 above range)

minmax_intrinsic (n=160, range 27.6-99.8 ns)
     27.6 |########################################
     31.3 |#####################################
     34.9 |
     38.5 |
     42.1 |
     45.7 |
     49.3 |
     52.9 |
     56.5 |
     60.1 |
     63.7 |
     67.3 |
     70.9 |
     74.5 |
     78.1 |##
     81.7 |#
     85.4 |
     89.0 |##
     92.6 |##
     96.2 |##
  (16 below, 14 above range)

```

## Diagnostics

- **minmax_branch**: CV=62.2% (high variance, measurements may be unstable)
- **minmax_branch**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **minmax_branch**: bridge=68.9% of algo (FFI overhead may distort results)
- **minmax_branchless**: CV=65.1% (high variance, measurements may be unstable)
- **minmax_branchless**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **minmax_branchless**: bridge=68.9% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: CV=64.6% (high variance, measurements may be unstable)
- **minmax_intrinsic**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **minmax_intrinsic**: bridge=68.8% of algo (FFI overhead may distort results)
