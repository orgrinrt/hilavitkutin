# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_intrinsic dominates: 21% faster than the next best (ctz_debruijn)

ctz_intrinsic (16 ns) leads ctz_debruijn (20 ns) by 21%, a clear separation rather than a photo finish. CV 110.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ctz_loop is an outlier: 2.3x slower than the field

ctz_loop (38 ns) is 2.3x the fastest (16 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_intrinsic is fastest but the noisiest (CV 110.0%)

ctz_intrinsic wins on median (16 ns) yet has the highest variance (CV 110.0%), while ctz_loop is the steadiest (CV 89.3%, 38 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ctz_debruijn shows alternating (throttle bounce) (autocorr -0.68)

ctz_debruijn's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### ctz_loop is inconsistent: worst-20% is 4.1x its best-20%

ctz_loop's best 20% of batches run at 27 ns but its worst 20% at 114 ns (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ctz_intrinsic** at 16.5 ns median (-17.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.28x (fastest 16.5 ns, slowest 37.6 ns)

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
| Counter frequency | 2112792693 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 2611ns | 1894ns | 1551ns | 1839ns | 5986ns | base |
| ctz_intrinsic | 2636ns | 1561ns | 1548ns | 1915ns | 5885ns | +0.96% |
| ctz_loop | 2702ns | 1914ns | 1607ns | 1959ns | 6025ns | +3.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 27ns | 16ns | 62ns | base | 9.331 |
| ctz_intrinsic | 28ns | 16ns | 61ns | +0.87% | 9.251 |
| ctz_loop | 50ns | 27ns | 114ns | +84.01% | 5.071 |

## Performance model

- Peak throughput: **15.693 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 12.800 | 81.6% |
| ctz_intrinsic | 15.515 | 98.9% |
| ctz_loop | 6.809 | 43.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2611ns | 2611ns | base |
| ctz_intrinsic | 2636ns | 2636ns | +0.96% |
| ctz_loop | 2702ns | 2702ns | +3.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 20ns | base | --- | [20, 20] | --- | --- | --- | --- |
| ctz_intrinsic | 16ns | -3.6ns (-18.0%) | [-4, -3]ns | [16, 20] | YES | 0.0001 | 0.0001 | 8 |
| ctz_loop | 38ns | +17.6ns (+88.0%) | [+15, +18]ns | [36, 38] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 20ns | -18.4% | +40.3% |
| 1 | 2 | 16ns | +22.6% | +63.7% |
| 1 | 3 | 20ns | -18.2% | +82.0% |
| 1 | 4 | 16ns | +0.0% | +83.5% |
| 1 | 5 | 20ns | -18.2% | +89.8% |
| 1 | 6 | 16ns | +22.0% | +166.8% |
| 1 | 7 | 20ns | -18.0% | +49.3% |
| 1 | 8 | 20ns | -18.2% | +97.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.678 | HIGH- (thermal bounce) |
| ctz_intrinsic | -0.375 | moderate- |
| ctz_loop | -0.099 | ok |

**Consistency summary:**

- **ctz_intrinsic**: won 5/8, lost 2/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 32.2ns | 27.4ns | 117.3% | HIGH |
| ctz_intrinsic | 32.8ns | 27.7ns | 118.4% | HIGH |
| ctz_loop | 34.4ns | 50.5ns | 68.1% | HIGH |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 16.3-62.0 ns)
     16.3 |###############
     18.6 |########################################
     20.9 |
     23.2 |
     25.5 |
     27.8 |
     30.1 |
     32.3 |
     34.6 |
     36.9 |#
     39.2 |
     41.5 |
     43.8 |
     46.1 |
     48.3 |
     50.6 |#
     52.9 |
     55.2 |#
     57.5 |#
     59.8 |##
  (18 below, 16 above range)

ctz_intrinsic (n=160, range 16.3-61.2 ns)
     16.3 |########################################
     18.6 |######################
     20.8 |
     23.0 |
     25.3 |
     27.5 |
     29.8 |
     32.0 |
     34.3 |
     36.5 |
     38.7 |
     41.0 |
     43.2 |
     45.5 |
     47.7 |##
     50.0 |###
     52.2 |
     54.4 |####
     56.7 |#
     58.9 |###
  (28 below, 14 above range)

ctz_loop (n=160, range 27.5-113.5 ns)
     27.5 |########################################
     31.8 |#######
     36.1 |####################################
     40.4 |################
     44.7 |
     49.0 |
     53.3 |
     57.6 |
     61.9 |
     66.2 |
     70.5 |
     74.8 |
     79.1 |
     83.4 |
     87.7 |#
     92.0 |
     96.3 |###
    100.6 |
    104.9 |##
    109.2 |#
  (20 below, 16 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=65.8% (high variance, measurements may be unstable)
- **ctz_debruijn**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **ctz_debruijn**: bridge=117.5% of algo (FFI overhead may distort results)
- **ctz_intrinsic**: CV=65.6% (high variance, measurements may be unstable)
- **ctz_intrinsic**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **ctz_intrinsic**: bridge=117.6% of algo (FFI overhead may distort results)
- **ctz_loop**: CV=66.5% (high variance, measurements may be unstable)
- **ctz_loop**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **ctz_loop**: bridge=64.6% of algo (FFI overhead may distort results)
