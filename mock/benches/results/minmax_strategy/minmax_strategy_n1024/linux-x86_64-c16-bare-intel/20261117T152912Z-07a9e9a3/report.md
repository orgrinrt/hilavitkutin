# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (43 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### minmax_intrinsic is inconsistent: worst-20% is 4.0x its best-20%

minmax_intrinsic's best 20% of batches run at 42 ns but its worst 20% at 167 ns (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 2.8% of the fastest

All 3 variants sit between 50 ns and 52 ns - a 2.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: minmax_branchless** at 50.1 ns median (-2.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.03x (fastest 50.1 ns, slowest 51.5 ns)

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
| Counter frequency | 2112775109 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2730ns | 1939ns | 1587ns | 2141ns | 5640ns | base |
| minmax_branchless | 2680ns | 1936ns | 1639ns | 1982ns | 5815ns | -1.84% |
| minmax_intrinsic | 2909ns | 1937ns | 1583ns | 2249ns | 6213ns | +6.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 74ns | 44ns | 154ns | base | 13.879 |
| minmax_branchless | 69ns | 43ns | 150ns | -6.12% | 14.784 |
| minmax_intrinsic | 78ns | 42ns | 167ns | +5.75% | 13.124 |

## Performance model

- Peak throughput: **24.593 Gops/s** (minmax_intrinsic; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 19.883 | 80.8% |
| minmax_branchless | 20.439 | 83.1% |
| minmax_intrinsic | 19.883 | 80.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2730ns | 2730ns | base |
| minmax_branchless | 2680ns | 2680ns | -1.84% |
| minmax_intrinsic | 2909ns | 2909ns | +6.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 52ns | base | --- | [51, 52] | --- | --- | --- | --- |
| minmax_branchless | 50ns | -1.4ns (-2.7%) | [-2, -1]ns | [50, 50] | YES | 0.0000 | 0.0000 | 2 |
| minmax_intrinsic | 52ns | no significant difference | [-0, +0]ns | [51, 52] | no | 1.0000 | 1.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 63ns | -20.7% | -33.8% |
| 1 | 2 | 51ns | -1.5% | -19.0% |
| 1 | 3 | 52ns | -3.9% | +122.5% |
| 1 | 4 | 52ns | -2.7% | -0.3% |
| 1 | 5 | 43ns | -3.4% | +21.8% |
| 1 | 6 | 52ns | -3.1% | -0.3% |
| 1 | 7 | 51ns | -18.3% | +1.0% |
| 1 | 8 | 48ns | +4.9% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.017 | ok |
| minmax_branchless | -0.360 | moderate- |
| minmax_intrinsic | -0.207 | moderate- |

**Consistency summary:**

- **minmax_branchless**: won 7/8, lost 1/8
- **minmax_intrinsic**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 34.5ns | 73.8ns | 46.8% | HIGH |
| minmax_branchless | 32.9ns | 69.3ns | 47.6% | HIGH |
| minmax_intrinsic | 36.5ns | 78.0ns | 46.8% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 43.9-154.0 ns)
     43.9 |######
     49.4 |########################################
     54.9 |
     60.4 |
     65.9 |
     71.4 |
     77.0 |
     82.5 |
     88.0 |
     93.5 |
     99.0 |
    104.5 |###
    110.0 |#
    115.5 |#
    121.0 |#
    126.5 |
    132.0 |
    137.5 |
    143.0 |
    148.5 |
  (21 below, 15 above range)

minmax_branchless (n=160, range 42.6-149.7 ns)
     42.6 |
     48.0 |########################################
     53.4 |
     58.7 |
     64.1 |
     69.4 |
     74.8 |
     80.1 |
     85.5 |
     90.8 |#
     96.2 |
    101.5 |
    106.9 |
    112.3 |
    117.6 |
    123.0 |
    128.3 |
    133.7 |
    139.0 |##
    144.4 |
  (27 below, 16 above range)

minmax_intrinsic (n=160, range 41.6-167.0 ns)
     41.6 |###############
     47.9 |########################################
     54.2 |
     60.4 |
     66.7 |#
     73.0 |
     79.2 |##
     85.5 |#
     91.8 |
     98.1 |#
    104.3 |
    110.6 |#
    116.9 |#
    123.1 |
    129.4 |#
    135.7 |
    141.9 |#
    148.2 |
    154.5 |###
    160.7 |##
  (21 below, 17 above range)

```

## Diagnostics

- **minmax_branch**: CV=60.8% (high variance, measurements may be unstable)
- **minmax_branch**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **minmax_branch**: bridge=47.0% of algo (FFI overhead may distort results)
- **minmax_branchless**: CV=61.6% (high variance, measurements may be unstable)
- **minmax_branchless**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **minmax_branchless**: bridge=47.7% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: CV=61.9% (high variance, measurements may be unstable)
- **minmax_intrinsic**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **minmax_intrinsic**: bridge=47.2% of algo (FFI overhead may distort results)
