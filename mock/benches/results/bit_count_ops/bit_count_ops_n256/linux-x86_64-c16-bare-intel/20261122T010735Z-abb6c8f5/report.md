# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ctz_debruijn, ctz_intrinsic) are a dead heat (<1%)

ctz_debruijn (20 ns) and ctz_intrinsic (20 ns) differ by 0.00%, inside the noise, even though the wider field spreads 68.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (14 ns) is smaller than the fastest variant's own run-to-run std-dev (18 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (20 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### ctz_intrinsic's comparison is tie-heavy (23% tied pairs)

23% of paired samples for ctz_intrinsic are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### ctz_loop is inconsistent: worst-20% is 4.6x its best-20%

ctz_loop's best 20% of batches run at 24 ns but its worst 20% at 111 ns (4.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 20.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.68x (fastest 20.0 ns, slowest 33.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6f1038d492c5ab07 |
| Counter frequency | 2112774681 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 2592ns | 1895ns | 1551ns | 1837ns | 5899ns | base |
| ctz_intrinsic | 2640ns | 1897ns | 1552ns | 1846ns | 6110ns | +1.83% |
| ctz_loop | 2621ns | 1577ns | 1559ns | 1933ns | 5746ns | +1.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 27ns | 16ns | 61ns | base | 9.442 |
| ctz_intrinsic | 28ns | 16ns | 64ns | +2.39% | 9.221 |
| ctz_loop | 50ns | 24ns | 111ns | +83.73% | 5.139 |

## Performance model

- Peak throughput: **15.684 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 12.800 | 81.6% |
| ctz_intrinsic | 12.800 | 81.6% |
| ctz_loop | 7.619 | 48.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2592ns | 2592ns | base |
| ctz_intrinsic | 2640ns | 2640ns | +1.83% |
| ctz_loop | 2621ns | 2621ns | +1.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 20ns | base | --- | [20, 20] | --- | --- | --- | --- |
| ctz_intrinsic | 20ns | no significant difference | [+0, +0]ns | [20, 20] | no (adj: YES) | 0.0300 | 0.0300 | **37** (23%, HIGH) |
| ctz_loop | 34ns | +13.4ns (+67.0%) | [+11, +21]ns | [30, 38] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 20ns | +0.3% | +16.2% |
| 1 | 2 | 16ns | +22.6% | +64.9% |
| 1 | 3 | 20ns | -18.2% | +47.9% |
| 1 | 4 | 20ns | +0.5% | +52.0% |
| 1 | 5 | 16ns | +0.0% | +132.3% |
| 1 | 6 | 16ns | -0.6% | +167.1% |
| 1 | 7 | 16ns | +22.3% | +50.6% |
| 1 | 8 | 20ns | +0.5% | +68.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.123 | ok |
| ctz_intrinsic | -0.141 | ok |
| ctz_loop | 0.070 | ok |

**Consistency summary:**

- **ctz_intrinsic**: won 2/8, lost 5/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 32.3ns | 27.1ns | 119.1% | HIGH |
| ctz_intrinsic | 32.9ns | 27.8ns | 118.6% | HIGH |
| ctz_loop | 33.4ns | 49.8ns | 67.0% | HIGH |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 16.3-61.1 ns)
     16.3 |##########################
     18.6 |########################################
     20.8 |#
     23.1 |#
     25.3 |
     27.5 |
     29.8 |
     32.0 |
     34.3 |
     36.5 |
     38.7 |
     41.0 |
     43.2 |#
     45.4 |
     47.7 |
     49.9 |##
     52.2 |
     54.4 |#
     56.6 |#
     58.9 |###
  (16 below, 13 above range)

ctz_intrinsic (n=160, range 16.3-63.8 ns)
     16.3 |##############
     18.7 |########################################
     21.1 |
     23.4 |
     25.8 |
     28.2 |
     30.6 |
     32.9 |
     35.3 |
     37.7 |
     40.0 |
     42.4 |
     44.8 |
     47.2 |#
     49.5 |
     51.9 |
     54.3 |
     56.7 |#
     59.0 |##
     61.4 |
  (19 below, 19 above range)

ctz_loop (n=160, range 24.0-111.4 ns)
     24.0 |########################################
     28.4 |#################################
     32.7 |####################
     37.1 |########################
     41.5 |#####################
     45.8 |#
     50.2 |#
     54.6 |
     59.0 |
     63.3 |#
     67.7 |
     72.1 |#
     76.4 |##
     80.8 |##
     85.2 |#####
     89.5 |##
     93.9 |#####
     98.3 |########
    102.7 |####
    107.0 |##
  (15 below, 12 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=65.5% (high variance, measurements may be unstable)
- **ctz_debruijn**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ctz_debruijn**: bridge=117.5% of algo (FFI overhead may distort results)
- **ctz_intrinsic**: CV=67.6% (high variance, measurements may be unstable)
- **ctz_intrinsic**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **ctz_intrinsic**: bridge=117.5% of algo (FFI overhead may distort results)
- **ctz_loop**: CV=68.5% (high variance, measurements may be unstable)
- **ctz_loop**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **ctz_loop**: bridge=59.8% of algo (FFI overhead may distort results)
